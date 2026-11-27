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

//! The budget through every layer (plan section 7.5): a read allowance of zero issues no read, every allowance is
//! exact on every input, the stop names its limit, and the working memory and work units stop what they must.

use std::io::Cursor;

use cc_media_probe::{Limit, Limits};
use input::blocking;
use input::report::{DetectionOutcome, HeaderStatus, Reason};

use crate::support::{RecordingSource, small_options, typical_file};

/// The options with a read allowance of `limit`.
fn allowing(limit: u64) -> input::Options {
  input::Options {
    limits: Limits::default().with_bytes_read(limit),
    ..small_options()
  }
}

#[test]
fn an_allowance_of_zero_issues_no_read_and_names_the_limit() {
  let mut source = RecordingSource::new(&typical_file());
  let report = blocking::probe_source(&mut source, &allowing(0)).unwrap();
  assert_eq!(source.requests, [] as [(u64, usize); 0]);
  assert_eq!(
    report.detection,
    DetectionOutcome::InsufficientData(Reason::Limit(Limit::BytesRead))
  );
  assert_eq!(report.exhausted, [Limit::BytesRead]);
  assert_eq!(report.usage.bytes_read, 0);
}

#[test]
fn every_allowance_is_exact_on_every_blocking_input() {
  let file = typical_file();
  for limit in [1, 11, 12, 19, 20, 63, 64, 65, 100, 150] {
    let options = allowing(limit);
    let reports = [
      blocking::probe_bytes(&file, &options).unwrap(),
      blocking::probe_reader(Cursor::new(file.clone()), &options).unwrap(),
      blocking::probe_stream(Cursor::new(file.clone()), &options).unwrap(),
      blocking::probe_source(RecordingSource::new(&file), &options).unwrap(),
    ];
    for report in reports {
      assert!(
        report.usage.bytes_read <= limit,
        "limit {limit}: {}",
        report.usage.bytes_read
      );
      assert!(report.exhausted.contains(&Limit::BytesRead), "limit {limit}");
    }
  }
}

#[test]
fn a_stop_at_the_allowance_names_it_for_the_streams_that_still_wait() {
  let file = typical_file();
  let report = blocking::probe_bytes(&file, &allowing(70)).unwrap();
  assert_eq!(report.detection, DetectionOutcome::Identified);
  let waiting: Vec<_> = report
    .streams
    .iter()
    .filter(|stream| !matches!(stream.header, HeaderStatus::Parsed(_)))
    .map(|stream| stream.header.clone())
    .collect();
  assert_ne!(waiting.len(), 0);
  assert!(waiting.iter().all(|header| matches!(
    header,
    HeaderStatus::NotFound(Reason::Limit(Limit::BytesRead))
      | HeaderStatus::PartiallyParsedCutShort(_, Reason::Limit(Limit::BytesRead))
  )));
}

#[test]
fn the_working_memory_bounds_the_peak_and_stops_what_does_not_fit() {
  let file = typical_file();
  let unlimited = blocking::probe_reader(Cursor::new(file.clone()), &small_options()).unwrap();
  let with = |limit: u64| {
    let options = input::Options {
      limits: Limits::default().with_working_memory(limit),
      ..small_options()
    };
    let report = blocking::probe_reader(Cursor::new(file.clone()), &options).unwrap();
    assert!(report.usage.peak_working_memory <= limit, "limit {limit}");
    report
  };
  for limit in [0, 64, 200] {
    assert!(with(limit).exhausted.contains(&Limit::WorkingMemory), "limit {limit}");
  }
  let below = with(unlimited.usage.peak_working_memory - 1);
  assert_eq!(below.exhausted, [], "a cache of one block fewer stops nothing");
  assert_eq!(below.streams, unlimited.streams);
  assert_eq!(below.titles, unlimited.titles);
}

#[test]
fn the_work_units_stop_the_traversal() {
  let options = input::Options {
    limits: Limits::default().with_work_units(3),
    ..small_options()
  };
  let report = blocking::probe_bytes(&typical_file(), &options).unwrap();
  assert_eq!(report.usage.work_units, 3);
  assert!(report.exhausted.contains(&Limit::WorkUnits));
  assert!(report.titles.len() <= 1);
}

#[test]
fn every_buffer_returns_its_charge_by_the_end() {
  let file = typical_file();
  let options = small_options();
  let reports = [
    blocking::probe_bytes(&file, &options).unwrap(),
    blocking::probe_reader(Cursor::new(file.clone()), &options).unwrap(),
    blocking::probe_stream(Cursor::new(file.clone()), &options).unwrap(),
    blocking::probe_bytes(&file, &allowing(70)).unwrap(),
  ];
  for report in reports {
    assert!(report.usage.peak_working_memory > 0);
    assert_eq!(report.usage.working_memory_at_end, 0);
  }
}
