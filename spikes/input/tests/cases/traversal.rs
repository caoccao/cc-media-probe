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

//! The traversal's edges: the end of the input inside an open list, an allowance of exactly the elements present, a
//! title reference that would read through itself, a forward-only window smaller than its chunk, a larger cache
//! under a tight working memory, and detection that bytes already rule out or that the allowance cut short. The
//! async probe and the hand-written one agree wherever both can read the input.

use std::io::Cursor;

use cc_media_probe::{Limit, Limits};
use input::builder::{ToyFile, payload_unit, setup_unit};
use input::codec::Framing;
use input::report::{DetectionOutcome, Diagnostic, DiagnosticKind, Reason, ToyReport};
use input::{blocking, handwritten, machine};

use crate::support::{parsed, size, small_options};

/// The report of `file` in memory, the same from the async probe and the hand-written one.
fn both(file: &[u8], options: &input::Options) -> ToyReport {
  let report = blocking::probe_bytes(file, options).unwrap();
  assert_eq!(handwritten::probe_bytes(file, options).unwrap(), report);
  report
}

/// A list element whose header declares `length` bytes of children.
fn list_header(length: u32) -> Vec<u8> {
  [b"LIST".to_vec(), length.to_be_bytes().to_vec()].concat()
}

#[test]
fn the_end_of_the_input_inside_an_open_list_cuts_that_list_short() {
  let title = ToyFile::new().title("x").build();
  let list = u64::try_from(title.len()).unwrap();
  let file = [title.clone(), list_header(20)].concat();
  assert_eq!(
    both(&file, &small_options()).diagnostics,
    [Diagnostic {
      source: 0,
      offset: list,
      kind: DiagnosticKind::TruncatedElement
    }]
  );
  let nested = [title, list_header(100), list_header(20)].concat();
  assert_eq!(
    both(&nested, &small_options()).diagnostics,
    [Diagnostic {
      source: 0,
      offset: list + 8,
      kind: DiagnosticKind::TruncatedElement
    }]
  );
  let closed = ToyFile::new().title("x").open_list().title("y").close_list().build();
  assert_eq!(both(&closed, &small_options()).diagnostics, []);
}

#[test]
fn an_allowance_of_exactly_the_elements_present_completes_the_traversal() {
  let file = ToyFile::new().title("Only").build();
  let with = |units| input::Options {
    limits: Limits::default().with_work_units(units),
    ..small_options()
  };
  let exact = both(&file, &with(1));
  assert_eq!(exact.titles.len(), 1);
  assert_eq!(exact.exhausted, []);
  let short = both(&file, &with(0));
  assert_eq!(short.titles.len(), 0);
  assert_eq!(short.exhausted, [Limit::WorkUnits]);
}

#[test]
fn a_title_reference_never_reads_through_itself() {
  let mut inner = b"TITL".to_vec();
  inner.extend_from_slice(&10_u32.to_be_bytes());
  let file = ToyFile::new()
    .title("x")
    .element(*b"PADX", &inner)
    .title_reference(29)
    .title("After")
    .title_reference(12)
    .build();
  let report = both(&file, &small_options());
  let titles: Vec<(&str, bool)> = report
    .titles
    .iter()
    .map(|title| (title.text.as_str(), title.referenced))
    .collect();
  assert_eq!(titles, [("x", false), ("After", false), ("x", true)]);
  assert_eq!(
    report.diagnostics,
    [
      Diagnostic {
        source: 0,
        offset: 21,
        kind: DiagnosticKind::UnknownElement(*b"PADX")
      },
      Diagnostic {
        source: 0,
        offset: 37,
        kind: DiagnosticKind::BadBody
      },
    ]
  );
}

#[test]
fn a_chunk_larger_than_the_window_keeps_the_bytes_requested() {
  let file = ToyFile::new().title("First").title("Second").title("Third").build();
  let options = input::Options {
    chunk: size(16),
    window: size(4),
    ..small_options()
  };
  let report = blocking::probe_stream(Cursor::new(file.clone()), &options).unwrap();
  assert_eq!(report.diagnostics, []);
  assert_eq!(report.titles, blocking::probe_bytes(&file, &options).unwrap().titles);
}

#[test]
fn a_larger_cache_never_reads_less_under_the_same_working_memory() {
  let file = ToyFile::new().title(&"A".repeat(40)).title(&"B".repeat(40)).build();
  let one_block = input::Options {
    cached_blocks: size(1),
    ..small_options()
  };
  let peak = blocking::probe_reader(Cursor::new(file.clone()), &one_block)
    .unwrap()
    .usage
    .peak_working_memory;
  for blocks in [1, 2, 8] {
    let options = input::Options {
      limits: Limits::default().with_working_memory(peak),
      cached_blocks: size(blocks),
      ..small_options()
    };
    let report = blocking::probe_reader(Cursor::new(file.clone()), &options).unwrap();
    assert_eq!(report.titles.len(), 2, "{blocks} blocks");
    assert_eq!(
      handwritten::probe_reader(Cursor::new(file.clone()), &options).unwrap(),
      report,
      "{blocks} blocks"
    );
  }
}

#[test]
fn detection_on_a_stream_judges_the_bytes_that_the_allowance_gave() {
  let options = input::Options {
    limits: Limits::default().with_bytes_read(1),
    ..small_options()
  };
  let other = b"elephants are not toy files";
  assert_eq!(both(other, &options).detection, DetectionOutcome::Unknown);
  let streamed = blocking::probe_stream(Cursor::new(other.to_vec()), &options).unwrap();
  assert_eq!(streamed.detection, DetectionOutcome::Unknown);
  let toy = ToyFile::new().title("x").build();
  let streamed = blocking::probe_stream(Cursor::new(toy), &options).unwrap();
  assert_eq!(
    streamed.detection,
    DetectionOutcome::InsufficientData(Reason::Limit(Limit::BytesRead))
  );
}

#[test]
fn bytes_that_already_rule_the_format_out_are_not_insufficient_data() {
  let options = small_options();
  assert_eq!(both(b"CCToy\0\x02", &options).detection, DetectionOutcome::Unknown);
  assert_eq!(
    both(b"CCToy\0\x01", &options).detection,
    DetectionOutcome::InsufficientData(Reason::TruncatedInput)
  );
  let start = ToyFile::new().build();
  let unknown = [start.clone(), b"ABCD\0\0".to_vec()].concat();
  assert_eq!(both(&unknown, &options).detection, DetectionOutcome::Unknown);
  for cut in [b"STR".to_vec(), b"STRM\0\0".to_vec()] {
    let known = [start.clone(), cut].concat();
    assert_eq!(
      both(&known, &options).detection,
      DetectionOutcome::InsufficientData(Reason::TruncatedInput)
    );
  }
}

#[test]
fn the_end_of_the_input_inside_a_skipped_element_truncates_it() {
  let file = ToyFile::new()
    .stream(1, Framing::Packet)
    .packet(1, &setup_unit(320, 240, 0, 0));
  let last = file.position();
  let file = file.packet(1, &payload_unit(40)).build();
  let truncated = &file[..file.len() - 25];
  let options = small_options();
  let expected = [Diagnostic {
    source: 0,
    offset: last,
    kind: DiagnosticKind::TruncatedElement,
  }];
  let reports = [
    ("bytes", both(truncated, &options)),
    ("state machine", machine::probe_bytes(truncated, &options).unwrap()),
    (
      "reader",
      blocking::probe_reader(Cursor::new(truncated), &options).unwrap(),
    ),
    (
      "hand-written reader",
      handwritten::probe_reader(Cursor::new(truncated), &options).unwrap(),
    ),
    (
      "state machine reader",
      machine::probe_reader(Cursor::new(truncated), &options).unwrap(),
    ),
    (
      "stream",
      blocking::probe_stream(Cursor::new(truncated), &options).unwrap(),
    ),
  ];
  for (input, report) in reports {
    assert_eq!(report.diagnostics, expected, "{input}");
    assert_eq!(report.streams[0].header, parsed(320, 240, &[]), "{input}");
  }
  for (input, report) in [
    ("bytes", both(&file, &options)),
    ("reader", blocking::probe_reader(Cursor::new(&file), &options).unwrap()),
    ("stream", blocking::probe_stream(Cursor::new(&file), &options).unwrap()),
  ] {
    assert_eq!(report.diagnostics, [], "the whole file as {input}");
  }
}

#[test]
fn a_list_too_short_for_another_header_is_left_without_reading_past_it() {
  let file = ToyFile::new().open_list().title("a").raw(&[0]).close_list();
  let stray = file.position() - 1;
  let file = file.title("b").build();
  let detection = 12 + 8;
  let walk = 8 + (8 + 1) + (8 + 1);
  let options = input::Options {
    limits: Limits::default().with_bytes_read(detection + walk),
    ..small_options()
  };
  let report = both(&file, &options);
  assert_eq!(machine::probe_bytes(&file, &options).unwrap(), report);
  let titles: Vec<&str> = report.titles.iter().map(|title| title.text.as_str()).collect();
  assert_eq!(titles, ["a", "b"]);
  assert_eq!(
    report.diagnostics,
    [Diagnostic {
      source: 0,
      offset: stray,
      kind: DiagnosticKind::ElementOverrunsParent,
    }]
  );
  assert_eq!(report.usage.bytes_read, detection + walk);
  assert_eq!(report.exhausted, []);
}

#[test]
fn a_title_reference_names_only_a_title_element_walked_before_it() {
  let file = ToyFile::new().title("t");
  let padding = file.position();
  let inner = [b"TITL".to_vec(), 1_u32.to_be_bytes().to_vec(), b"z".to_vec()].concat();
  let file = file.element(*b"PADX", &inner);
  let reference = file.position();
  let file = file.title_reference(padding + 8).build();
  let report = both(&file, &small_options());
  assert_eq!(machine::probe_bytes(&file, &small_options()).unwrap(), report);
  let titles: Vec<(&str, bool)> = report
    .titles
    .iter()
    .map(|title| (title.text.as_str(), title.referenced))
    .collect();
  assert_eq!(titles, [("t", false)]);
  assert_eq!(
    report.diagnostics,
    [
      Diagnostic {
        source: 0,
        offset: padding,
        kind: DiagnosticKind::UnknownElement(*b"PADX"),
      },
      Diagnostic {
        source: 0,
        offset: reference,
        kind: DiagnosticKind::BadBody,
      },
    ]
  );
}

#[test]
fn a_first_element_that_no_known_code_begins_rules_the_format_out() {
  let header = ToyFile::new().build();
  for (bytes, outcome) in [
    (&b"Z"[..], DetectionOutcome::Unknown),
    (&b"TZ"[..], DetectionOutcome::Unknown),
    (&b"TI"[..], DetectionOutcome::InsufficientData(Reason::TruncatedInput)),
    (&b""[..], DetectionOutcome::InsufficientData(Reason::TruncatedInput)),
  ] {
    let file = [header.clone(), bytes.to_vec()].concat();
    assert_eq!(both(&file, &small_options()).detection, outcome, "{bytes:?}");
  }
}
