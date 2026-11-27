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

//! The forward-only cases of plan Phase 3, task 1 (D9): two interleaved streams whose second header passes while
//! the first still waits, a backward reference beyond the window followed by a readable sibling, and a detection
//! sample beyond the pinned prefix.

use std::io::Cursor;

use input::blocking;
use input::builder::{ToyFile, parameters_unit, payload_unit, setup_unit};
use input::codec::Framing;
use input::report::{DetectionOutcome, DiagnosticKind, Reason};

use crate::support::{parsed, small_options};

#[test]
fn the_second_streams_only_header_is_taken_while_the_first_still_waits() {
  let mut file = ToyFile::new()
    .stream(1, Framing::Packet)
    .stream(2, Framing::Packet)
    .stream_with_codec(3, Framing::Packet, *b"OTHR")
    .packet(1, &[setup_unit(320, 240, 2, 0), payload_unit(4)].concat())
    .packet(2, &setup_unit(640, 480, 0, 0));
  for _ in 0..30 {
    file = file.packet(3, &[0x5A; 60]);
  }
  let file = file.packet(1, &parameters_unit(&[4, 5]).unwrap()).build();
  let options = small_options();
  let report = blocking::probe_stream(Cursor::new(file), &options).unwrap();
  assert_eq!(report.streams[0].header, parsed(320, 240, &[4, 5]));
  assert_eq!(report.streams[1].header, parsed(640, 480, &[]));
  assert_eq!(report.diagnostics, []);
}

/// A file whose title reference, after 2,000 bytes of other elements, points back to the first title.
fn reference_file() -> (Vec<u8>, u64) {
  let file = ToyFile::new().title("Concert").element(*b"FILL", &[0; 2000]);
  let reference = file.position();
  (file.title_reference(12).title("After").build(), reference)
}

#[test]
fn a_reference_behind_the_window_is_reported_and_the_walk_goes_on_with_its_sibling() {
  let (file, reference) = reference_file();
  let options = small_options();
  let forward = blocking::probe_stream(Cursor::new(file.clone()), &options).unwrap();
  let titles: Vec<_> = forward.titles.iter().map(|title| title.text.as_str()).collect();
  assert_eq!(titles, ["Concert", "After"]);
  let unavailable: Vec<_> = forward
    .diagnostics
    .iter()
    .filter(|diagnostic| diagnostic.kind == DiagnosticKind::RangeUnavailable)
    .map(|diagnostic| diagnostic.offset)
    .collect();
  assert_eq!(unavailable, [reference]);
  let seekable = blocking::probe_reader(Cursor::new(file), &options).unwrap();
  let titles: Vec<_> = seekable
    .titles
    .iter()
    .map(|title| (title.text.as_str(), title.referenced))
    .collect();
  assert_eq!(titles, [("Concert", false), ("Concert", true), ("After", false)]);
}

#[test]
fn a_reference_within_the_window_is_read_on_a_forward_only_input() {
  let file = ToyFile::new().title("Concert").title_reference(12).build();
  let report = blocking::probe_stream(Cursor::new(file), &small_options()).unwrap();
  let titles: Vec<_> = report
    .titles
    .iter()
    .map(|title| (title.text.as_str(), title.referenced))
    .collect();
  assert_eq!(titles, [("Concert", false), ("Concert", true)]);
}

#[test]
fn a_detection_sample_beyond_the_pinned_prefix_gives_insufficient_data_on_a_forward_only_input() {
  let file = ToyFile::with_first_element_at(100).title("Late").build();
  let options = small_options();
  let forward = blocking::probe_stream(Cursor::new(file.clone()), &options).unwrap();
  assert_eq!(
    forward.detection,
    DetectionOutcome::InsufficientData(Reason::RangeUnavailable)
  );
  assert_eq!(forward.usage.bytes_read, 12);
  let seekable = blocking::probe_reader(Cursor::new(file), &options).unwrap();
  assert_eq!(seekable.detection, DetectionOutcome::Identified);
  assert_eq!(seekable.titles[0].text, "Late");
}

#[test]
fn a_forward_only_reader_starts_where_it_stands() {
  let file = ToyFile::new().title("Concert").build();
  let mut reader = Cursor::new([b"junk".as_slice(), &file].concat());
  std::io::Read::read_exact(&mut reader, &mut [0; 4]).unwrap();
  let report = blocking::probe_stream(&mut reader, &small_options()).unwrap();
  assert_eq!(report.titles[0].text, "Concert");
  assert_eq!(reader.position(), u64::try_from(file.len() + 4).unwrap());
}

#[test]
fn inputs_that_are_not_toy_files_or_end_early_are_told_apart() {
  let options = small_options();
  let other = blocking::probe_stream(Cursor::new(b"RIFF\0\0\0\0WAVEfmt ".to_vec()), &options).unwrap();
  assert_eq!(other.detection, DetectionOutcome::Unknown);
  let short = blocking::probe_stream(Cursor::new(b"CCTo".to_vec()), &options).unwrap();
  assert_eq!(
    short.detection,
    DetectionOutcome::InsufficientData(Reason::TruncatedInput)
  );
  let unknown_first = ToyFile::new().element(*b"ZZZZ", &[]).build();
  let report = blocking::probe_bytes(&unknown_first, &options).unwrap();
  assert_eq!(report.detection, DetectionOutcome::Unknown);
}
