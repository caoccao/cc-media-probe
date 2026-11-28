/*
* Copyright (c) 2026. caoccao.com Sam Cao
* All rights reserved.

* Licensed under the Apache License, Version 2.0 (the "License");
* you may not use this file except in compliance with the License.
* You may obtain a copy of the License at

* http://www.apache.org/licenses/LICENSE-2.0

* Unless required by applicable law or agreed to in writing, software
* distributed under the License is distributed on an "AS IS" BASIS,
* WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
* See the License for the specific language governing permissions and
* limitations under the License.
*/

//! The test's own sources and readers (plan section 7.1): the custom positional sources over bytes in memory,
//! blocking and async, which deliver in pieces, count their reads and give no bytes at and beyond their end, and the
//! async reader of both runtimes, which is pending once before each read and serves what the reader inside it
//! delivers.

use std::io::{Cursor, ErrorKind, SeekFrom};
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

use cc_media_probe::asynchronous::AsyncPositionalSource;
use cc_media_probe::source::PositionalSource;
use cc_media_probe_testkit::readers::{CountingReader, Counts, FaultingReader, Faults};
use futures_lite::future::block_on;

use crate::common::memory::{AsyncMemory, Memory};
use crate::common::pending::Pending;

#[test]
fn a_custom_source_delivers_in_pieces_and_gives_no_bytes_at_and_beyond_its_end() {
  let mut memory = Memory::new(&[1, 2, 3], &[2]);
  let mut buffer = [0_u8; 8];
  assert_eq!(memory.read_at(0, &mut buffer).unwrap(), 2);
  assert_eq!(memory.read_at(2, &mut buffer).unwrap(), 1);
  assert_eq!(buffer[0], 3);
  assert_eq!(memory.read_at(3, &mut buffer).unwrap(), 0);
  assert_eq!(memory.read_at(u64::MAX, &mut buffer).unwrap(), 0);
  assert_eq!(memory.length().unwrap(), Some(3));
  assert_eq!(
    memory.counts(),
    Counts {
      read_calls: 4,
      delivered: 3,
      seeks: 0
    }
  );
}

#[test]
fn a_custom_source_fails_where_its_faults_say() {
  let faults = Faults::new().with_error_at(2, ErrorKind::TimedOut);
  let mut memory = Memory::with_faults(&[1, 2, 3, 4], faults);
  let mut buffer = [0_u8; 8];
  assert_eq!(memory.read_at(0, &mut buffer).unwrap(), 2);
  assert_eq!(memory.read_at(2, &mut buffer).unwrap_err().kind(), ErrorKind::TimedOut);
  assert_eq!(memory.read_at(3, &mut buffer).unwrap(), 1);
}

#[test]
fn a_custom_source_counts_a_failed_read_as_the_counting_reader_counts_one() {
  // The review of plan Phase 3, task 4: one interrupted read gave no call, where the reader counts one of 0 bytes.
  let faults = || Faults::new().with_interruption_at(0);
  let mut memory = Memory::with_faults(&[1, 2], faults());
  let mut reader = CountingReader::new(FaultingReader::with_faults(Cursor::new(vec![1, 2]), faults()));
  let mut buffer = [0_u8; 8];
  assert_eq!(
    memory.read_at(0, &mut buffer).unwrap_err().kind(),
    ErrorKind::Interrupted
  );
  let interrupted = std::io::Read::read(&mut reader, &mut buffer).unwrap_err();
  assert_eq!(interrupted.kind(), ErrorKind::Interrupted);
  let once = Counts {
    read_calls: 1,
    delivered: 0,
    seeks: 0,
  };
  assert_eq!((memory.counts(), reader.counts()), (once, once));
  assert_eq!(memory.read_at(0, &mut buffer).unwrap(), 2);
  assert_eq!(memory.counts().read_calls, 2);
}

#[test]
fn a_custom_source_gives_an_empty_read_0_and_its_faults_at_its_end() {
  // The second review of plan Phase 3, task 4: an empty read consumed the interruption, and the end hid the error.
  let mut memory = Memory::with_faults(&[1, 2], Faults::new().with_interruption_at(0));
  let mut buffer = [0_u8; 8];
  assert_eq!(memory.read_at(0, &mut []).unwrap(), 0);
  assert_eq!(
    memory.read_at(0, &mut buffer).unwrap_err().kind(),
    ErrorKind::Interrupted
  );
  assert_eq!(memory.read_at(0, &mut buffer).unwrap(), 2);
  let mut ended = Memory::with_faults(&[1, 2], Faults::new().with_error_at(2, ErrorKind::TimedOut));
  assert_eq!(ended.read_at(2, &mut []).unwrap(), 0);
  assert_eq!(ended.read_at(2, &mut buffer).unwrap_err().kind(), ErrorKind::TimedOut);
  assert_eq!(ended.read_at(3, &mut buffer).unwrap(), 0);
  assert_eq!((memory.counts().read_calls, ended.counts().read_calls), (3, 3));
}

#[test]
fn an_async_source_is_pending_once_before_each_read() {
  let mut source = AsyncMemory::new(&[5, 6, 7], &[]);
  let mut context = Context::from_waker(Waker::noop());
  let mut buffer = [0_u8; 2];
  for (position, expected) in [(0, 2), (2, 1)] {
    let mut pinned = Pin::new(&mut source);
    assert!(
      pinned
        .as_mut()
        .poll_read_at(&mut context, position, &mut buffer)
        .is_pending()
    );
    let ready = pinned.as_mut().poll_read_at(&mut context, position, &mut buffer);
    assert!(matches!(ready, Poll::Ready(Ok(delivered)) if delivered == expected));
  }
  let length = Pin::new(&mut source).poll_length(&mut context);
  assert!(matches!(length, Poll::Ready(Ok(Some(3)))));
  assert_eq!(source.counts().read_calls, 2);
}

/// An async reader of `bytes` over the test kit's readers, in pieces of 3.
fn pending(bytes: &[u8]) -> Pending<CountingReader<FaultingReader<Cursor<Vec<u8>>>>> {
  Pending::new(CountingReader::new(
    FaultingReader::new(Cursor::new(bytes.to_vec())).with_pieces(&[3]),
  ))
}

#[test]
fn an_async_reader_is_pending_once_before_each_read_on_both_runtimes() {
  let bytes: Vec<u8> = (0..10).collect();
  let mut context = Context::from_waker(Waker::noop());
  let mut reader = pending(&bytes);
  let mut buffer = [0_u8; 8];
  let mut pinned = Pin::new(&mut reader);
  assert!(futures_io::AsyncRead::poll_read(pinned.as_mut(), &mut context, &mut buffer).is_pending());
  let ready = futures_io::AsyncRead::poll_read(pinned.as_mut(), &mut context, &mut buffer);
  assert!(matches!(ready, Poll::Ready(Ok(3))));
  let mut tokio_buffer = [0_u8; 8];
  let mut filled = tokio::io::ReadBuf::new(&mut tokio_buffer);
  assert!(tokio::io::AsyncRead::poll_read(pinned.as_mut(), &mut context, &mut filled).is_pending());
  assert!(tokio::io::AsyncRead::poll_read(pinned.as_mut(), &mut context, &mut filled).is_ready());
  assert_eq!(filled.filled(), [3, 4, 5]);
  assert_eq!(reader.get_ref().counts().read_calls, 2);
}

#[test]
fn an_async_reader_refuses_a_seek_on_tokio_while_another_waits() {
  // The second review of plan Phase 3, task 4: seeks of +2 and +3 both succeeded, and the reader stood at 3.
  let mut context = Context::from_waker(Waker::noop());
  let mut reader = pending(&[0; 10]);
  let mut pinned = Pin::new(&mut reader);
  tokio::io::AsyncSeek::start_seek(pinned.as_mut(), SeekFrom::Current(2)).unwrap();
  let refused = tokio::io::AsyncSeek::start_seek(pinned.as_mut(), SeekFrom::Current(3)).unwrap_err();
  assert_eq!(refused.kind(), ErrorKind::Other);
  let reached = tokio::io::AsyncSeek::poll_complete(pinned.as_mut(), &mut context);
  assert!(matches!(reached, Poll::Ready(Ok(2))));
  tokio::io::AsyncSeek::start_seek(pinned.as_mut(), SeekFrom::Current(3)).unwrap();
  let reached = tokio::io::AsyncSeek::poll_complete(pinned.as_mut(), &mut context);
  assert!(matches!(reached, Poll::Ready(Ok(5))));
  assert_eq!(reader.get_ref().counts().seeks, 2);
}

#[test]
fn an_async_reader_seeks_and_reads_whole_on_both_runtimes() {
  let bytes: Vec<u8> = (0..10).collect();
  let runtime = tokio::runtime::Builder::new_current_thread().build().unwrap();
  let mut tokio_reader = pending(&bytes);
  let tokio_rest = runtime.block_on(async {
    use tokio::io::{AsyncReadExt as _, AsyncSeekExt as _};
    assert_eq!(tokio_reader.seek(SeekFrom::Start(4)).await.unwrap(), 4);
    let mut rest = Vec::new();
    tokio_reader.read_to_end(&mut rest).await.unwrap();
    rest
  });
  assert_eq!(tokio_rest, &bytes[4..]);
  assert_eq!(tokio_reader.get_ref().counts().seeks, 1);
  let mut futures_reader = pending(&bytes);
  let futures_rest = block_on(async {
    use futures_lite::{AsyncReadExt as _, AsyncSeekExt as _};
    assert_eq!(futures_reader.seek(SeekFrom::Start(7)).await.unwrap(), 7);
    let mut rest = Vec::new();
    futures_reader.read_to_end(&mut rest).await.unwrap();
    rest
  });
  assert_eq!(futures_rest, &bytes[7..]);
  assert_eq!(futures_reader.get_ref().counts().seeks, 1);
}
