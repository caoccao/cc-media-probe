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

//! Every entry point takes its reader or source by value, and a caller who passes `&mut` of it keeps it (C-RW-VALUE,
//! `api.md` sections 4.1, 4.4 and 9.1): a blocking reader, a forward-only reader and a positional source, and the
//! async readers of both runtimes and an async source. A forward-only reader is left after exactly the bytes that
//! the probe counts as read. Until the detection inventory, every input gives the error `UnknownFormat` or the
//! outcome `Unknown`, and nothing is read.

use std::io::{self, Cursor, Read};
use std::pin::Pin;
use std::task::{Context, Poll};

use cc_media_probe::asynchronous::AsyncPositionalSource;
use cc_media_probe::model::DetectionOutcome;
use cc_media_probe::source::PositionalSource;
use cc_media_probe::{Detection, Error, ErrorKind, MediaReport, Prober};

/// The bytes that each test probes.
const INPUT: &[u8] = b"not a media file";

/// A source of bytes in memory, blocking and async, that counts the reads it serves.
struct Object {
  data: Vec<u8>,
  reads: usize,
}

impl Object {
  fn new() -> Object {
    Object {
      data: INPUT.to_vec(),
      reads: 0,
    }
  }
}

impl PositionalSource for Object {
  fn read_at(&mut self, position: u64, buffer: &mut [u8]) -> io::Result<usize> {
    self.reads = self.reads.saturating_add(1);
    let start = usize::try_from(position).map_or(self.data.len(), |start| start.min(self.data.len()));
    let available = &self.data[start..];
    let count = available.len().min(buffer.len());
    buffer[..count].copy_from_slice(&available[..count]);
    Ok(count)
  }

  fn length(&mut self) -> io::Result<Option<u64>> {
    Ok(u64::try_from(self.data.len()).ok())
  }
}

impl AsyncPositionalSource for Object {
  fn poll_read_at(
    self: Pin<&mut Self>,
    _context: &mut Context<'_>,
    position: u64,
    buffer: &mut [u8],
  ) -> Poll<io::Result<usize>> {
    Poll::Ready(self.get_mut().read_at(position, buffer))
  }

  fn poll_length(self: Pin<&mut Self>, _context: &mut Context<'_>) -> Poll<io::Result<Option<u64>>> {
    Poll::Ready(self.get_mut().length())
  }
}

/// Whether the probe ended as every probe of this version does.
fn unknown_format(probed: Result<MediaReport, Error>) -> bool {
  probed.is_err_and(|error| *error.kind() == ErrorKind::UnknownFormat)
}

/// Whether the detection found nothing, as every detection of this version does without a hint.
fn unknown(detected: Result<Detection, Error>) -> bool {
  detected.is_ok_and(|detection| *detection.outcome() == DetectionOutcome::Unknown)
}

/// The bytes that a probe that ended in an error counts as read. Before detection finds a format, the primary input
/// is the only source, so they are all its own.
fn probe_count(probed: &Result<MediaReport, Error>) -> Option<u64> {
  probed.as_ref().err().map(|error| error.usage().bytes_read())
}

/// The bytes that a detection counts as read, all of them the primary input's.
fn detection_count(detected: &Result<Detection, Error>) -> Option<u64> {
  detected.as_ref().ok().map(|detection| detection.usage().bytes_read())
}

#[test]
fn a_blocking_probe_leaves_each_input_with_its_caller() -> io::Result<()> {
  let prober = Prober::new();
  let mut reader = Cursor::new(INPUT.to_vec());
  assert!(unknown_format(prober.probe_reader(&mut reader)));
  assert!(unknown(prober.detect_reader(&mut reader)));
  reader.set_position(0);
  let mut text = String::new();
  reader.read_to_string(&mut text)?;
  assert_eq!(text.as_bytes(), INPUT);

  // A forward-only reader is left after exactly the bytes that the probe counts as read, so the rest is still the
  // caller's: a probe that consumed more than it counts would leave the cursor further on.
  let mut stream = Cursor::new(INPUT.to_vec());
  let outcome = prober.probe_stream(&mut stream);
  assert_eq!(probe_count(&outcome), Some(stream.position()));
  assert!(unknown_format(outcome));
  let read = usize::try_from(stream.position()).unwrap();
  let mut rest = Vec::new();
  stream.read_to_end(&mut rest)?;
  assert_eq!(rest, INPUT[read..]);
  let mut stream = Cursor::new(INPUT.to_vec());
  let detected = prober.detect_stream(&mut stream);
  assert_eq!(detection_count(&detected), Some(stream.position()));
  assert!(unknown(detected));

  let mut source = Object::new();
  assert!(unknown_format(prober.probe_source(&mut source)));
  assert!(unknown(prober.detect_source(&mut source)));
  let served = source.reads;
  assert!(unknown_format(cc_media_probe::probe_source(&mut source)));
  assert!(source.reads >= served);
  Ok(())
}

#[test]
fn an_async_source_stays_with_its_caller() {
  let mut source = Object::new();
  let asynchronous = Prober::new();
  let handle = asynchronous.asynchronous();
  assert!(unknown_format(futures_lite::future::block_on(
    handle.probe_source(&mut source)
  )));
  assert!(unknown(futures_lite::future::block_on(
    handle.detect_source(&mut source)
  )));
  assert!(unknown_format(futures_lite::future::block_on(
    cc_media_probe::asynchronous::probe_source(&mut source)
  )));
  assert_eq!(source.data, INPUT);
}

#[cfg(feature = "futures-io")]
#[test]
fn a_futures_reader_stays_with_its_caller() {
  use cc_media_probe::asynchronous::futures;
  use futures_lite::future::block_on;

  let mut reader = futures_lite::io::Cursor::new(INPUT.to_vec());
  assert!(unknown_format(block_on(futures::probe_reader(&mut reader))));
  assert!(unknown(block_on(futures::detect_reader(&mut reader))));
  let mut stream = futures_lite::io::Cursor::new(INPUT.to_vec());
  let probed = block_on(futures::probe_stream(&mut stream));
  assert_eq!(probe_count(&probed), Some(stream.position()));
  assert!(unknown_format(probed));
  let mut stream = futures_lite::io::Cursor::new(INPUT.to_vec());
  let detected = block_on(futures::detect_stream(&mut stream));
  assert_eq!(detection_count(&detected), Some(stream.position()));
  assert!(unknown(detected));
  assert_eq!(reader.into_inner(), INPUT);
}

#[cfg(feature = "tokio")]
#[tokio::test]
async fn a_tokio_reader_stays_with_its_caller() {
  use cc_media_probe::asynchronous::tokio;

  let mut reader = Cursor::new(INPUT.to_vec());
  assert!(unknown_format(tokio::probe_reader(&mut reader).await));
  assert!(unknown(tokio::detect_reader(&mut reader).await));
  let mut stream = Cursor::new(INPUT.to_vec());
  let probed = tokio::probe_stream(&mut stream).await;
  assert_eq!(probe_count(&probed), Some(stream.position()));
  assert!(unknown_format(probed));
  let mut stream = Cursor::new(INPUT.to_vec());
  let detected = tokio::detect_stream(&mut stream).await;
  assert_eq!(detection_count(&detected), Some(stream.position()));
  assert!(unknown(detected));
  assert_eq!(reader.into_inner(), INPUT);
}
