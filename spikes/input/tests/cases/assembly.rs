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

//! The header-assembly cases of plan Phase 3, task 1, and the parsing contract's tests of plan section 7.4 that the
//! spike can run: a header split at every byte position, one-byte deliveries, two packets back to back, a packet far
//! larger than the assembler limit, a header of two units whose first unit's bytes were dropped, a long run of
//! headerless payload, and the per-stream limit.

use std::io::Cursor;

use cc_media_probe::Limit;
use input::blocking;
use input::builder::{ToyFile, parameters_unit, payload_unit, setup_unit};
use input::codec::{
  self, Consumption, Context, Continuation, Framing, Input, NEEDS_DECODING_FLAG, Outcome, Representation, Setup,
};
use input::report::{HeaderStatus, Reason};

use crate::support::{parsed, size, small_options};

/// The bytes of a whole header: a setup with three parameters and the parameters.
fn header_bytes() -> Vec<u8> {
  [setup_unit(320, 240, 3, 0), parameters_unit(&[4, 5, 6]).unwrap()].concat()
}

/// The header of stream 1 of `file`.
fn header_of(file: &[u8], options: &input::Options) -> HeaderStatus {
  let report = blocking::probe_bytes(file, options).unwrap();
  report.streams[0].header.clone()
}

#[test]
fn a_header_split_at_every_position_of_a_stream_is_parsed() {
  let bytes = header_bytes();
  for cut in 0..=bytes.len() {
    let (head, tail) = bytes.split_at(cut);
    let file = ToyFile::new()
      .stream(1, Framing::Stream)
      .packet(1, head)
      .packet(1, tail)
      .build();
    assert_eq!(
      header_of(&file, &small_options()),
      parsed(320, 240, &[4, 5, 6]),
      "cut at {cut}"
    );
  }
}

#[test]
fn a_header_split_into_three_deliveries_at_every_pair_of_positions_is_parsed() {
  let bytes = header_bytes();
  for first in 0..=bytes.len() {
    for second in first..=bytes.len() {
      let file = ToyFile::new()
        .stream(1, Framing::Stream)
        .packet(1, &bytes[..first])
        .packet(1, &bytes[first..second])
        .packet(1, &bytes[second..])
        .build();
      assert_eq!(
        header_of(&file, &small_options()),
        parsed(320, 240, &[4, 5, 6]),
        "cuts at {first}, {second}"
      );
    }
  }
}

#[test]
fn one_byte_deliveries_and_one_byte_pieces_give_the_header() {
  let bytes = header_bytes();
  let file = bytes
    .iter()
    .fold(ToyFile::new().stream(1, Framing::Stream), |file, byte| {
      file.packet(1, &[*byte])
    })
    .build();
  assert_eq!(header_of(&file, &small_options()), parsed(320, 240, &[4, 5, 6]));
  let options = input::Options {
    piece: size(1),
    ..small_options()
  };
  let packet = ToyFile::new().stream(1, Framing::Packet).packet(1, &bytes).build();
  assert_eq!(header_of(&packet, &options), parsed(320, 240, &[4, 5, 6]));
}

#[test]
fn two_packets_back_to_back_stay_two_and_the_continuation_joins_them() {
  let file = ToyFile::new()
    .stream(1, Framing::Packet)
    .packet(1, &[setup_unit(320, 240, 3, 0), payload_unit(2)].concat())
    .packet(1, &parameters_unit(&[4, 5, 6]).unwrap())
    .build();
  let report = blocking::probe_bytes(&file, &small_options()).unwrap();
  assert_eq!(report.streams[0].header, parsed(320, 240, &[4, 5, 6]));
  assert_eq!(report.streams[0].packets_offered, 2);
  let packets = Input {
    framing: Framing::Packet,
    representation: Representation::OneByteLengths,
  };
  let setup = codec::parse(
    packets,
    Context::default(),
    &setup_unit(320, 240, 3, 0),
    Continuation::default(),
    true,
  );
  assert_eq!(setup.outcome, Outcome::NeedsMoreBytes { total: None });
  assert_eq!(setup.consumption, Consumption::Packet { finished: true });
  let second = codec::parse(
    packets,
    Context::default(),
    &parameters_unit(&[4, 5, 6]).unwrap(),
    setup.continuation,
    true,
  );
  assert!(matches!(second.outcome, Outcome::Parsed(_)));
}

#[test]
fn a_packet_far_larger_than_the_limit_whose_header_fits_in_its_first_piece_is_read_no_further() {
  let mut payload = [setup_unit(320, 240, 0, 0), payload_unit(200)].concat();
  payload.resize(1 << 20, 0x5A);
  let file = ToyFile::new()
    .stream(1, Framing::Packet)
    .packet(1, &payload)
    .title("After")
    .build();
  let options = input::Options {
    limits: cc_media_probe::Limits::default().with_header_bytes_per_stream(64),
    ..small_options()
  };
  let report = blocking::probe_reader(Cursor::new(file.clone()), &options).unwrap();
  assert_eq!(report.streams[0].header, parsed(320, 240, &[]));
  assert_eq!(report.titles[0].text, "After");
  assert!(report.usage.bytes_read < 1024, "read {}", report.usage.bytes_read);
  assert_eq!(report.exhausted, [] as [Limit; 0]);
  let forward = blocking::probe_stream(Cursor::new(file.clone()), &options).unwrap();
  assert_eq!(forward.streams[0].header, parsed(320, 240, &[]));
  assert_eq!(forward.usage.bytes_read, u64::try_from(file.len()).unwrap());
  assert!(
    forward.usage.peak_working_memory < 4096,
    "held {}",
    forward.usage.peak_working_memory
  );
}

#[test]
fn a_second_unit_is_read_with_the_fact_of_a_first_unit_whose_bytes_were_dropped() {
  let mut file = ToyFile::new()
    .stream(1, Framing::Stream)
    .packet(1, &setup_unit(320, 240, 3, 0));
  for _ in 0..8 {
    file = file.packet(1, &payload_unit(30));
  }
  let file = file.packet(1, &parameters_unit(&[4, 5, 6]).unwrap()).build();
  let options = input::Options {
    limits: cc_media_probe::Limits::default().with_header_bytes_per_stream(40),
    ..small_options()
  };
  assert_eq!(header_of(&file, &options), parsed(320, 240, &[4, 5, 6]));
}

#[test]
fn configuration_after_several_headerless_packets_is_found() {
  let mut file = ToyFile::new().stream(1, Framing::Packet);
  for _ in 0..5 {
    file = file.packet(1, &payload_unit(8));
  }
  let file = file.packet(1, &header_bytes()).build();
  let report = blocking::probe_bytes(&file, &small_options()).unwrap();
  assert_eq!(report.streams[0].header, parsed(320, 240, &[4, 5, 6]));
  assert_eq!(report.streams[0].packets_offered, 6);
}

#[test]
fn a_long_run_of_headerless_payload_charges_its_work_and_meets_the_cap() {
  let mut file = ToyFile::new().stream(1, Framing::Stream);
  for _ in 0..200 {
    file = file.packet(1, &payload_unit(20));
  }
  let file = file.build();
  let options = input::Options {
    limits: cc_media_probe::Limits::default().with_header_bytes_per_stream(32),
    ..small_options()
  };
  let report = blocking::probe_bytes(&file, &options).unwrap();
  assert_eq!(
    report.streams[0].header,
    HeaderStatus::NotFound(Reason::Limit(Limit::HeaderBytesPerStream))
  );
  assert!(report.usage.work_units > 32 * 16);
}

#[test]
fn the_end_of_the_input_while_bytes_are_needed_cuts_the_header_short() {
  let file = ToyFile::new()
    .stream(1, Framing::Stream)
    .packet(1, &setup_unit(320, 240, 3, 0))
    .build();
  let setup = Setup {
    width: 320,
    height: 240,
    parameter_count: 3,
    flags: 0,
  };
  assert_eq!(
    header_of(&file, &small_options()),
    HeaderStatus::PartiallyParsedCutShort(setup, Reason::TruncatedInput)
  );
  let empty = ToyFile::new().stream(1, Framing::Packet).build();
  assert_eq!(
    header_of(&empty, &small_options()),
    HeaderStatus::NotFound(Reason::TruncatedInput)
  );
}

#[test]
fn a_parser_that_needs_more_than_the_limit_names_it() {
  let parameters: Vec<u16> = (0..100).collect();
  let file = ToyFile::new()
    .stream(1, Framing::Packet)
    .packet(
      1,
      &[setup_unit(320, 240, 100, 0), parameters_unit(&parameters).unwrap()].concat(),
    )
    .build();
  let options = input::Options {
    limits: cc_media_probe::Limits::default().with_header_bytes_per_stream(64),
    ..small_options()
  };
  let report = blocking::probe_bytes(&file, &options).unwrap();
  assert!(matches!(
    report.streams[0].header,
    HeaderStatus::PartiallyParsedCutShort(_, Reason::Limit(Limit::HeaderBytesPerStream))
  ));
  assert_eq!(report.exhausted, [Limit::HeaderBytesPerStream]);
}

#[test]
fn a_setup_that_needs_decoding_is_final_and_bad_units_are_malformed() {
  let decoding = ToyFile::new()
    .stream(1, Framing::Packet)
    .packet(1, &setup_unit(320, 240, 3, NEEDS_DECODING_FLAG))
    .build();
  assert!(matches!(
    header_of(&decoding, &small_options()),
    HeaderStatus::PartiallyParsedByDesign(_)
  ));
  let early = ToyFile::new()
    .stream(1, Framing::Stream)
    .packet(1, &parameters_unit(&[1]).unwrap())
    .build();
  assert_eq!(header_of(&early, &small_options()), HeaderStatus::Malformed);
  let cut = ToyFile::new()
    .stream(1, Framing::Packet)
    .packet(1, &setup_unit(320, 240, 3, 0)[..5])
    .build();
  assert_eq!(header_of(&cut, &small_options()), HeaderStatus::Malformed);
}

#[test]
fn two_streams_compete_for_the_last_working_memory() {
  let file = ToyFile::new()
    .stream(1, Framing::Stream)
    .stream(2, Framing::Stream)
    .packet(1, &setup_unit(320, 240, 3, 0)[..4])
    .packet(2, &setup_unit(640, 480, 1, 0)[..4])
    .packet(
      1,
      &[
        setup_unit(320, 240, 3, 0)[4..].to_vec(),
        parameters_unit(&[4, 5, 6]).unwrap(),
      ]
      .concat(),
    )
    .packet(
      2,
      &[setup_unit(640, 480, 1, 0)[4..].to_vec(), parameters_unit(&[9]).unwrap()].concat(),
    )
    .build();
  let unlimited = blocking::probe_bytes(&file, &small_options()).unwrap();
  let scratch = unlimited.usage.peak_working_memory - 8;
  let options = input::Options {
    limits: cc_media_probe::Limits::default().with_working_memory(scratch),
    ..small_options()
  };
  let report = blocking::probe_bytes(&file, &options).unwrap();
  let headers: Vec<_> = report.streams.iter().map(|stream| stream.header.clone()).collect();
  assert!(headers.contains(&parsed(320, 240, &[4, 5, 6])) || headers.contains(&parsed(640, 480, &[9])));
  assert!(
    headers
      .iter()
      .any(|header| matches!(header, HeaderStatus::NotFound(Reason::Limit(Limit::WorkingMemory))))
  );
  assert!(report.usage.peak_working_memory <= scratch);
}

#[test]
fn a_stream_that_waits_through_many_packets_holds_no_more_than_one_piece() {
  let mut file = ToyFile::new().stream(1, Framing::Packet);
  for _ in 0..1000 {
    file = file.packet(1, &payload_unit(30));
  }
  let options = small_options();
  let report = blocking::probe_bytes(&file.build(), &options).unwrap();
  assert_eq!(report.streams[0].packets_offered, 1000);
  let scratch = u64::try_from(options.piece.get() + 255).unwrap();
  let held = u64::try_from(options.piece.get() + codec::CONTINUATION_SIZE).unwrap();
  assert!(
    report.usage.peak_working_memory <= scratch + held,
    "held {}",
    report.usage.peak_working_memory
  );
}
