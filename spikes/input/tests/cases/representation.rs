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

//! The parser's inputs of plan section 7.4 beyond the bytes: the representation that the stream declaration states,
//! units with one-byte or two-byte lengths, and the packet's known length, which lets a unit that runs past its
//! packet be malformed on its first piece. Also a payload unit known by its kind byte alone, and the builders that
//! refuse a length their field cannot hold.

use cc_media_probe::Limits;
use input::blocking;
use input::builder::{ToyFile, parameters_body, parameters_unit, payload_unit, setup_body, setup_unit, unit};
use input::codec::{
  self, CONTINUATION_SIZE, Context, Continuation, Framing, Input, Outcome, PARAMETERS_KIND, Representation, SETUP_KIND,
};
use input::report::{DiagnosticKind, HeaderStatus, Reason};

use crate::support::{parsed, size, small_options};

/// A whole header of `parameters` in `representation`.
fn header_in(representation: Representation, parameters: &[u16]) -> Vec<u8> {
  let count = u8::try_from(parameters.len()).unwrap();
  [
    unit(representation, SETUP_KIND, &setup_body(320, 240, count, 0)).unwrap(),
    unit(representation, PARAMETERS_KIND, &parameters_body(parameters)).unwrap(),
  ]
  .concat()
}

/// The header of a stream in `representation` and `framing` whose one packet or delivery is `bytes`.
fn header_of(representation: Representation, framing: Framing, bytes: &[u8]) -> HeaderStatus {
  let file = ToyFile::new()
    .stream_with_representation(1, framing, representation)
    .packet(1, bytes)
    .build();
  blocking::probe_bytes(&file, &small_options()).unwrap().streams[0]
    .header
    .clone()
}

#[test]
fn two_byte_lengths_hold_a_parameters_unit_that_one_byte_lengths_cannot() {
  let parameters: Vec<u16> = (1..=200).collect();
  assert_eq!(parameters_unit(&parameters), None);
  let bytes = header_in(Representation::TwoByteLengths, &parameters);
  for framing in [Framing::Packet, Framing::Stream] {
    assert_eq!(
      header_of(Representation::TwoByteLengths, framing, &bytes),
      parsed(320, 240, &parameters),
      "{framing:?}"
    );
  }
}

#[test]
fn the_same_header_through_each_representation_gives_the_same_fields() {
  for representation in [Representation::OneByteLengths, Representation::TwoByteLengths] {
    for framing in [Framing::Packet, Framing::Stream] {
      let bytes = header_in(representation, &[4, 5, 6]);
      assert_eq!(
        header_of(representation, framing, &bytes),
        parsed(320, 240, &[4, 5, 6]),
        "{representation:?}, {framing:?}"
      );
    }
  }
}

#[test]
fn units_read_in_the_wrong_representation_run_past_their_packet() {
  let bytes = header_in(Representation::OneByteLengths, &[4, 5, 6]);
  assert_eq!(
    header_of(Representation::TwoByteLengths, Framing::Packet, &bytes),
    HeaderStatus::Malformed
  );
}

#[test]
fn an_unknown_representation_byte_is_a_bad_body() {
  let file = ToyFile::new()
    .element(*b"STRM", &[1, 0, 7, b'T', b'C', b'O', b'D'])
    .title("After")
    .build();
  let report = blocking::probe_bytes(&file, &small_options()).unwrap();
  assert_eq!(report.streams, []);
  assert_eq!(report.diagnostics[0].kind, DiagnosticKind::BadBody);
  assert_eq!(report.titles.len(), 1);
}

#[test]
fn a_known_packet_length_makes_a_unit_that_runs_past_it_malformed_at_once() {
  let mut payload = [setup_unit(320, 240, 125, 0), vec![PARAMETERS_KIND, 250]].concat();
  payload.resize(100, 0);
  let packets = Input {
    framing: Framing::Packet,
    representation: Representation::OneByteLengths,
  };
  let known = Context {
    packet_length: Some(100),
  };
  let first_piece = &payload[..16];
  let at_once = codec::parse(packets, known, first_piece, Continuation::default(), false);
  assert_eq!(at_once.outcome, Outcome::Malformed);
  assert_eq!(at_once.examined, 8 + 2);
  let unknown = codec::parse(packets, Context::default(), first_piece, Continuation::default(), false);
  assert_eq!(
    unknown.outcome,
    Outcome::NeedsMoreBytes {
      total: Some(8 + 2 + 250)
    }
  );
  let file = ToyFile::new().stream(1, Framing::Packet).packet(1, &payload).build();
  let options = input::Options {
    limits: Limits::default().with_header_bytes_per_stream(64),
    ..small_options()
  };
  let report = blocking::probe_bytes(&file, &options).unwrap();
  assert_eq!(report.streams[0].header, HeaderStatus::Malformed);
  assert!(report.usage.bytes_read < 80, "read {}", report.usage.bytes_read);
}

#[test]
fn a_payload_unit_is_known_by_its_kind_byte_alone() {
  let packets = Input {
    framing: Framing::Packet,
    representation: Representation::OneByteLengths,
  };
  let result = codec::parse(packets, Context::default(), &[0], Continuation::default(), false);
  assert_eq!(result.outcome, Outcome::NoHeaderPresent);
  assert_eq!(result.examined, 1);
  let file = ToyFile::new()
    .stream(1, Framing::Packet)
    .packet(1, &payload_unit(4))
    .packet(1, &payload_unit(4))
    .packet(1, &setup_unit(320, 240, 0, 0))
    .build();
  let options = input::Options {
    limits: Limits::default().with_header_bytes_per_stream(u64::try_from(CONTINUATION_SIZE).unwrap() + 1),
    piece: size(1),
    ..small_options()
  };
  let report = blocking::probe_bytes(&file, &options).unwrap();
  assert_eq!(report.streams[0].packets_offered, 3);
  assert_eq!(
    report.streams[0].header,
    HeaderStatus::NotFound(Reason::Limit(cc_media_probe::Limit::HeaderBytesPerStream))
  );
}

#[test]
fn the_builders_refuse_a_length_that_the_field_cannot_hold() {
  assert_eq!(parameters_unit(&[0; 127]).unwrap().len(), 256);
  assert_eq!(parameters_unit(&[0; 128]), None);
  let long = unit(Representation::TwoByteLengths, PARAMETERS_KIND, &[0; 256]).unwrap();
  assert_eq!(&long[..3], &[PARAMETERS_KIND, 1, 0]);
  assert_eq!(long.len(), 259);
  assert_eq!(
    unit(Representation::TwoByteLengths, PARAMETERS_KIND, &vec![0; 65_536]),
    None
  );
}
