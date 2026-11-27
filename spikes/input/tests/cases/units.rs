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

//! What a unit's first bytes decide (plan section 7.4), after the second review of the spike. At a unit boundary
//! in packet mode the parser asks for the kind byte alone, since a payload kind ends the packet's part of the
//! header, so the same header parses whatever the size of the pieces. A unit whose kind, length or place already
//! contradicts the syntax is malformed as soon as those bytes are there, so no limit is named for a body that was
//! never needed. The async probe, the hand-written one and the state machine agree.

use cc_media_probe::Limits;
use input::builder::{ToyFile, parameters_unit, payload_unit, setup_unit, unit};
use input::codec::{CONTINUATION_SIZE, Framing, Representation};
use input::report::{HeaderStatus, ToyReport};
use input::{blocking, handwritten, machine};

use crate::support::{parsed, size, small_options};

/// The report of `file` in memory, the same from the three walkers.
fn all(file: &[u8], options: &input::Options) -> ToyReport {
  let report = blocking::probe_bytes(file, options).unwrap();
  assert_eq!(handwritten::probe_bytes(file, options).unwrap(), report);
  assert_eq!(machine::probe_bytes(file, options).unwrap(), report);
  report
}

/// The continuation value's size as a count of bytes.
fn continuation() -> u64 {
  u64::try_from(CONTINUATION_SIZE).unwrap()
}

#[test]
fn a_payload_kind_after_a_setup_ends_the_packet_whatever_the_piece_size() {
  let file = ToyFile::new()
    .stream(1, Framing::Packet)
    .packet(1, &[setup_unit(320, 240, 1, 0), payload_unit(20)].concat())
    .packet(1, &parameters_unit(&[7]).unwrap())
    .build();
  for piece in [8, 9] {
    let options = input::Options {
      limits: Limits::default().with_header_bytes_per_stream(9 + continuation()),
      piece: size(piece),
      ..small_options()
    };
    assert_eq!(
      all(&file, &options).streams[0].header,
      parsed(320, 240, &[7]),
      "pieces of {piece} bytes"
    );
  }
}

#[test]
fn a_unit_that_contradicts_the_syntax_is_malformed_before_its_body_arrives() {
  let long = [0_u8; 200];
  let one = Representation::OneByteLengths;
  let cases = [
    ("an unknown kind", unit(one, 9, &long).unwrap()),
    ("a setup of another length", unit(one, 1, &long).unwrap()),
    ("parameters before any setup", unit(one, 2, &long).unwrap()),
    (
      "parameters that the setup's count contradicts",
      [setup_unit(320, 240, 2, 0), unit(one, 2, &long).unwrap()].concat(),
    ),
  ];
  let options = input::Options {
    limits: Limits::default().with_header_bytes_per_stream(32 + continuation()),
    ..small_options()
  };
  for framing in [Framing::Packet, Framing::Stream] {
    for (case, header) in &cases {
      let file = ToyFile::new().stream(1, framing).packet(1, header).build();
      let report = all(&file, &options);
      assert_eq!(
        report.streams[0].header,
        HeaderStatus::Malformed,
        "{case} in {framing:?}"
      );
      assert_eq!(report.exhausted, [], "{case} in {framing:?}");
    }
  }
}

#[test]
fn a_two_byte_length_that_contradicts_the_setup_is_malformed_at_once() {
  let file = ToyFile::new()
    .stream_with_representation(1, Framing::Stream, Representation::TwoByteLengths)
    .packet(
      1,
      &[
        unit(Representation::TwoByteLengths, 1, &[1, 64, 0, 240, 3, 0]).unwrap(),
        unit(Representation::TwoByteLengths, 2, &[0; 1000]).unwrap(),
      ]
      .concat(),
    )
    .build();
  let options = input::Options {
    limits: Limits::default().with_header_bytes_per_stream(64),
    ..small_options()
  };
  assert_eq!(all(&file, &options).streams[0].header, HeaderStatus::Malformed);
}
