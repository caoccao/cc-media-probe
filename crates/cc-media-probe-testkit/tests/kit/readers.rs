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

//! The test kit's readers (`docs/design/test-kit.md` section 5): pieces that cut the input at fixed bytes, whatever
//! the reads ask for, as the third review of the G3.2 proposal asked; an error at a chosen offset, of a chosen kind
//! or wrapping any Rust error; an interruption once; and the counts of a counting reader.

use std::error::Error;
use std::fmt;
use std::io::{Cursor, ErrorKind, Read, Seek, SeekFrom};

use cc_media_probe_testkit::readers::{CountingReader, Counts, FaultingReader, Faults, Pieces};

#[test]
fn pieces_end_reads_at_fixed_bytes_whatever_sizes_the_reads_ask_for() {
  let pieces = Pieces::new(&[40, 8]);
  let rooms: Vec<Option<u64>> = [0, 8, 39, 40, 47, 48, 52, 63, 64]
    .iter()
    .map(|position| pieces.room(*position))
    .collect();
  assert_eq!(
    rooms,
    [
      Some(40),
      Some(32),
      Some(1),
      Some(8),
      Some(1),
      Some(8),
      Some(4),
      Some(1),
      Some(8)
    ]
  );
  assert_eq!(Pieces::default().room(5), None);
  let mut buffer = [0_u8; 64];
  assert_eq!(pieces.cut(8, &mut buffer).len(), 32);
  assert_eq!(pieces.cut(40, &mut buffer[..3]).len(), 3);
}

#[test]
fn a_faulting_reader_cuts_at_the_bytes_of_its_pieces_across_seeks() {
  let bytes: Vec<u8> = (0..=99).collect();
  let mut reader = FaultingReader::new(Cursor::new(bytes)).with_pieces(&[40, 8]);
  let mut buffer = [0_u8; 64];
  let mut delivered = Vec::new();
  for _ in 0..3 {
    delivered.push(reader.read(&mut buffer[..8]).unwrap());
  }
  delivered.push(reader.read(&mut buffer).unwrap());
  delivered.push(reader.read(&mut buffer).unwrap());
  assert_eq!(delivered, [8, 8, 8, 16, 8]);
  reader.seek(SeekFrom::Start(44)).unwrap();
  assert_eq!(reader.read(&mut buffer).unwrap(), 4);
  assert_eq!(buffer[0], 44);
}

/// A cause of the test's own type, which a wrapped error gives back.
#[derive(Debug, PartialEq, Eq)]
struct Cause(u32);

impl fmt::Display for Cause {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(formatter, "cause {}", self.0)
  }
}

impl Error for Cause {}

#[test]
fn a_read_that_starts_at_the_error_offset_fails_every_time_with_its_kind() {
  let bytes: Vec<u8> = (0..=99).collect();
  let mut reader = FaultingReader::new(Cursor::new(bytes)).with_error_at(10, ErrorKind::PermissionDenied);
  let mut buffer = [0_u8; 64];
  assert_eq!(reader.read(&mut buffer).unwrap(), 10);
  for _ in 0..2 {
    assert_eq!(
      reader.read(&mut buffer).unwrap_err().kind(),
      ErrorKind::PermissionDenied
    );
  }
  assert_eq!(reader.position(), 10);
  reader.seek(SeekFrom::Start(11)).unwrap();
  assert_eq!(reader.read(&mut buffer).unwrap(), 64);
  reader.seek(SeekFrom::Start(4)).unwrap();
  assert_eq!(reader.read(&mut buffer).unwrap(), 6);
  assert_eq!(
    reader.read(&mut buffer).unwrap_err().kind(),
    ErrorKind::PermissionDenied
  );
}

#[test]
fn a_reader_already_moved_applies_its_faults_at_the_positions_of_its_bytes() {
  // The review of plan Phase 3, task 4, wrapped a cursor at byte 5, whose position read 0 until a seek.
  let bytes: Vec<u8> = (0..=19).collect();
  let mut cursor = Cursor::new(bytes);
  cursor.set_position(5);
  let mut reader = FaultingReader::new(cursor)
    .with_position(5)
    .with_error_at(8, ErrorKind::PermissionDenied);
  assert_eq!(reader.position(), 5);
  let mut buffer = [0_u8; 16];
  assert_eq!(reader.read(&mut buffer).unwrap(), 3);
  assert_eq!(&buffer[..3], [5, 6, 7]);
  assert_eq!(
    reader.read(&mut buffer).unwrap_err().kind(),
    ErrorKind::PermissionDenied
  );
  assert_eq!((reader.position(), reader.stream_position().unwrap()), (8, 8));
}

#[test]
fn a_wrapped_error_gives_back_its_cause() {
  let mut reader =
    FaultingReader::new(Cursor::new(vec![7_u8; 8])).with_wrapped_error_at(0, ErrorKind::Other, || Cause(3));
  let error = reader.read(&mut [0_u8; 4]).unwrap_err();
  assert_eq!(error.kind(), ErrorKind::Other);
  assert_eq!(
    error.get_ref().and_then(|cause| cause.downcast_ref::<Cause>()),
    Some(&Cause(3))
  );
  assert_eq!(error.to_string(), "cause 3");
}

#[test]
fn an_interruption_comes_once_and_the_standard_library_retries_it() {
  let bytes: Vec<u8> = (0..=19).collect();
  let mut reader = FaultingReader::new(Cursor::new(bytes.clone())).with_interruption_at(5);
  let mut buffer = [0_u8; 16];
  assert_eq!(reader.read(&mut buffer).unwrap(), 5);
  assert_eq!(reader.read(&mut buffer).unwrap_err().kind(), ErrorKind::Interrupted);
  assert_eq!(reader.read(&mut buffer).unwrap(), 15);
  let mut whole = Vec::new();
  FaultingReader::new(Cursor::new(bytes.clone()))
    .with_interruption_at(5)
    .with_pieces(&[3])
    .read_to_end(&mut whole)
    .unwrap();
  assert_eq!(whole, bytes);
}

#[test]
fn a_read_into_an_empty_buffer_meets_no_fault_and_an_error_at_the_end_is_given() {
  // The second review of plan Phase 3, task 4: an empty read consumed a pending interruption.
  let mut reader = FaultingReader::new(Cursor::new(vec![1, 2]))
    .with_interruption_at(0)
    .with_error_at(2, ErrorKind::TimedOut);
  let mut buffer = [0_u8; 8];
  assert_eq!(reader.read(&mut []).unwrap(), 0);
  assert_eq!(reader.read(&mut buffer).unwrap_err().kind(), ErrorKind::Interrupted);
  assert_eq!(reader.read(&mut buffer).unwrap(), 2);
  assert_eq!(reader.read(&mut []).unwrap(), 0);
  assert_eq!(reader.read(&mut buffer).unwrap_err().kind(), ErrorKind::TimedOut);
  let mut faults = Faults::new()
    .with_interruption_at(4)
    .with_error_at(4, ErrorKind::TimedOut);
  assert_eq!(faults.apply(4, &mut []).unwrap().len(), 0);
  assert_eq!(faults.apply(4, &mut buffer).unwrap_err().kind(), ErrorKind::Interrupted);
}

#[test]
fn faults_cut_a_custom_sources_reads_at_its_own_positions() {
  let mut faults = Faults::new()
    .with_pieces(&[40, 8])
    .with_error_at(50, ErrorKind::UnexpectedEof);
  let mut buffer = [0_u8; 64];
  assert_eq!(faults.apply(0, &mut buffer).unwrap().len(), 40);
  assert_eq!(faults.apply(44, &mut buffer).unwrap().len(), 4);
  assert_eq!(faults.apply(48, &mut buffer).unwrap().len(), 2);
  assert_eq!(
    faults.apply(50, &mut buffer).unwrap_err().kind(),
    ErrorKind::UnexpectedEof
  );
  assert_eq!(faults.apply(51, &mut buffer).unwrap().len(), 5);
  assert_eq!(faults.pieces(), &Pieces::new(&[40, 8]));
}

#[test]
fn a_counting_reader_counts_reads_bytes_and_seeks() {
  let mut reader = CountingReader::new(Cursor::new(vec![1_u8; 10]));
  let mut buffer = [0_u8; 4];
  reader.read_exact(&mut buffer).unwrap();
  reader.seek(SeekFrom::Start(8)).unwrap();
  assert_eq!(reader.read(&mut buffer).unwrap(), 2);
  assert_eq!(reader.read(&mut buffer).unwrap(), 0);
  assert_eq!(
    reader.counts(),
    Counts {
      read_calls: 3,
      delivered: 6,
      seeks: 1
    }
  );
  let mut counts = Counts::default();
  counts.record(3);
  counts.record_seek();
  assert_eq!((counts.read_calls, counts.delivered, counts.seeks), (1, 3, 1));
}
