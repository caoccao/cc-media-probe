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

//! The assembler's limits that the review of the spike found loose: a stream's deliveries are not its end, a
//! parser call examines no more than the work units and the cap on bytes examined leave, and the continuation
//! value counts toward both the per-stream header limit and the working memory (plan sections 7.4 and 7.5).

use cc_media_probe::{Limit, Limits};
use input::blocking;
use input::builder::{ToyFile, parameters_unit, payload_unit, setup_unit};
use input::codec::{CONTINUATION_SIZE, Framing, Setup};
use input::report::{HeaderStatus, Reason};

use crate::support::{parsed, small_options};

/// The continuation value's size as a count of bytes.
fn continuation() -> u64 {
  u64::try_from(CONTINUATION_SIZE).unwrap()
}

#[test]
fn one_byte_deliveries_of_a_stream_parse_as_one_delivery_does() {
  let parameters: Vec<u16> = (1..=32).collect();
  let header = [setup_unit(320, 240, 32, 0), parameters_unit(&parameters).unwrap()].concat();
  let options = input::Options {
    limits: Limits::default().with_header_bytes_per_stream(96),
    ..small_options()
  };
  let whole = ToyFile::new().stream(1, Framing::Stream).packet(1, &header).build();
  let split = header
    .iter()
    .fold(ToyFile::new().stream(1, Framing::Stream), |file, byte| {
      file.packet(1, &[*byte])
    })
    .build();
  for file in [whole, split] {
    let report = blocking::probe_bytes(&file, &options).unwrap();
    assert_eq!(report.streams[0].header, parsed(320, 240, &parameters));
  }
}

#[test]
fn a_parser_call_examines_no_more_than_the_work_units_left() {
  let file = ToyFile::new()
    .stream(1, Framing::Packet)
    .packet(
      1,
      &[setup_unit(320, 240, 1, 0), parameters_unit(&[5]).unwrap()].concat(),
    )
    .build();
  let options = input::Options {
    limits: Limits::default().with_work_units(5),
    ..small_options()
  };
  let report = blocking::probe_bytes(&file, &options).unwrap();
  assert_eq!(
    report.streams[0].header,
    HeaderStatus::NotFound(Reason::Limit(Limit::WorkUnits))
  );
  assert_eq!(report.usage.work_units, 5);
  assert_eq!(report.exhausted, [Limit::WorkUnits]);
  let enough = input::Options {
    limits: Limits::default().with_work_units(2 + 12),
    ..small_options()
  };
  let report = blocking::probe_bytes(&file, &enough).unwrap();
  assert_eq!(report.streams[0].header, parsed(320, 240, &[5]));
  assert_eq!(report.exhausted, []);
}

#[test]
fn the_bytes_examined_for_a_stream_never_pass_their_cap() {
  let limit = 64;
  let mut file = ToyFile::new().stream(1, Framing::Stream);
  let deliveries = 40;
  for _ in 0..deliveries {
    file = file.packet(1, &[payload_unit(28), payload_unit(28)].concat());
  }
  let options = input::Options {
    limits: Limits::default().with_header_bytes_per_stream(limit),
    ..small_options()
  };
  let report = blocking::probe_bytes(&file.build(), &options).unwrap();
  assert_eq!(
    report.streams[0].header,
    HeaderStatus::NotFound(Reason::Limit(Limit::HeaderBytesPerStream))
  );
  let elements = 1 + deliveries;
  let examined = report.usage.work_units - elements;
  assert!(examined <= 16 * limit, "examined {examined}");
}

#[test]
fn the_continuation_counts_toward_the_per_stream_header_limit() {
  let parameters = [1, 2, 3, 4, 5];
  let file = ToyFile::new()
    .stream(1, Framing::Stream)
    .packet(
      1,
      &[setup_unit(320, 240, 5, 0), parameters_unit(&parameters).unwrap()].concat(),
    )
    .build();
  let header_with = |limit: u64| {
    let options = input::Options {
      limits: Limits::default().with_header_bytes_per_stream(limit),
      ..small_options()
    };
    blocking::probe_bytes(&file, &options).unwrap().streams[0]
      .header
      .clone()
  };
  let setup = Setup {
    width: 320,
    height: 240,
    parameter_count: 5,
    flags: 0,
  };
  assert_eq!(
    header_with(12 + continuation() - 1),
    HeaderStatus::PartiallyParsedCutShort(setup, Reason::Limit(Limit::HeaderBytesPerStream))
  );
  assert_eq!(header_with(12 + continuation()), parsed(320, 240, &parameters));
  assert_eq!(
    header_with(continuation() - 1),
    HeaderStatus::NotFound(Reason::Limit(Limit::HeaderBytesPerStream))
  );
}

#[test]
fn the_continuation_is_charged_to_working_memory() {
  let file = ToyFile::new()
    .stream(1, Framing::Packet)
    .packet(1, &setup_unit(320, 240, 0, 0))
    .build();
  let scratch = 16 + 255;
  let header_with = |limit: u64| {
    let options = input::Options {
      limits: Limits::default().with_working_memory(limit),
      ..small_options()
    };
    let report = blocking::probe_bytes(&file, &options).unwrap();
    assert_eq!(report.usage.working_memory_at_end, 0);
    report.streams[0].header.clone()
  };
  assert_eq!(
    header_with(scratch + 8),
    HeaderStatus::NotFound(Reason::Limit(Limit::WorkingMemory))
  );
  assert_eq!(header_with(scratch + 8 + continuation()), parsed(320, 240, &[]));
}

#[test]
fn reaching_the_cap_on_bytes_examined_ends_a_waiting_stream_with_the_header_limit() {
  let limit = 64;
  let packets = 16 * limit;
  let mut file = ToyFile::new().stream(1, Framing::Packet);
  for _ in 0..packets {
    file = file.packet(1, &payload_unit(0));
  }
  let file = file.build();
  let report_with = |limits: Limits| {
    let options = input::Options {
      limits: limits.with_header_bytes_per_stream(limit),
      ..small_options()
    };
    blocking::probe_bytes(&file, &options).unwrap()
  };
  let report = report_with(Limits::default());
  assert_eq!(
    report.streams[0].header,
    HeaderStatus::NotFound(Reason::Limit(Limit::HeaderBytesPerStream))
  );
  assert_eq!(report.streams[0].packets_offered, packets);
  assert_eq!(report.exhausted, [Limit::HeaderBytesPerStream]);
  let elements = 1 + packets;
  let tied = report_with(Limits::default().with_work_units(elements + packets));
  assert_eq!(
    tied.streams[0].header,
    HeaderStatus::NotFound(Reason::Limit(Limit::WorkUnits))
  );
  assert_eq!(tied.exhausted, [Limit::WorkUnits]);
}
