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

//! The outcomes of the toy walker and codec parser against the accepted grammar (`spikes/input/README.md` sections
//! 2 and 3), through the fuzz targets' entry points and the repository-only ones, built with the test kit's toy
//! builder. Each is a case that a review of the G3.2 proposal found the spike's reduced walker or parser to get
//! wrong: the first review the grammar, and the third an element that the input's end cuts short, a context that
//! misstates its packet's length, and a header split across deliveries to the codec.

use kit::builder::{ToyElement, toy_file};
use kit::fixture::Fixture;
use library::fuzzing_entry_points::{Parser, Target, run};
use library::repository_entry_points::{toy_outcome, toy_stream_outcomes};

/// A setup of width 320 and height 240 with `count` parameters and `flags`.
fn setup(count: u8, flags: u8) -> Vec<u8> {
  vec![1, 6, 0x01, 0x40, 0x00, 0xf0, count, flags]
}

/// The parameters unit of one parameter.
const PARAMETERS: [u8; 4] = [2, 2, 0x00, 0x07];

/// The toy codec target's outcome for `bytes` in packet mode (framing 0) with their length as the packet's, or in
/// stream mode (framing 1), which knows no packet's length.
fn codec(framing: u8, bytes: &[u8]) -> String {
  let length = if framing == 0 { bytes.len() } else { 0 };
  let length = u32::try_from(length).unwrap().to_be_bytes();
  let input = [&[0, framing][..], &length, bytes].concat();
  run(Target::ToyCodec, &input).outcome
}

/// A toy file of one stream, number 1, in `framing` with the codec `codec`, and a packet for each payload.
fn file(framing: u8, codec: [u8; 4], payloads: &[&[u8]]) -> Vec<u8> {
  let mut elements = vec![ToyElement::new(*b"STRM", [&[1, framing, 0][..], &codec].concat())];
  elements.extend(
    payloads
      .iter()
      .map(|payload| ToyElement::new(*b"PACK", [&[1][..], payload].concat())),
  );
  toy_file(&elements).unwrap()
}

#[test]
fn a_packet_that_ends_after_its_setup_leaves_the_parameters_to_a_later_packet() {
  assert_eq!(codec(0, &setup(1, 0)), "needs_more_bytes");
  assert_eq!(codec(0, &[setup(1, 0), PARAMETERS.to_vec()].concat()), "parsed");
  assert_eq!(codec(0, &setup(1, 0)[..5]), "malformed");
  let split = file(0, *b"TCOD", &[&setup(1, 0), &PARAMETERS]);
  assert_eq!(toy_stream_outcomes(&split), ["parsed"]);
  let without = file(0, *b"TCOD", &[&setup(1, 0)]);
  assert_eq!(toy_stream_outcomes(&without), ["partially_parsed_cut_short"]);
}

#[test]
fn a_stream_skips_its_payload_units_before_the_setup() {
  let bytes = [&[0, 1, 0xaa][..], &setup(0, 0)].concat();
  assert_eq!(codec(1, &bytes), "parsed");
  assert_eq!(codec(0, &bytes), "no_header_present");
  let streamed = file(1, *b"TCOD", &[&bytes[..5], &bytes[5..]]);
  assert_eq!(toy_stream_outcomes(&streamed), ["parsed"]);
  assert_eq!(codec(1, &setup(1, 0)[..5]), "needs_more_bytes");
}

#[test]
fn a_setup_that_needs_decoding_ends_partially_parsed_by_design() {
  let bytes = [setup(1, 1), PARAMETERS.to_vec()].concat();
  assert_eq!(codec(0, &bytes), "partially_parsed_by_design");
  assert_eq!(codec(0, &setup(0, 1)), "parsed");
}

#[test]
fn detection_takes_only_a_header_with_a_known_first_element_for_a_toy_file() {
  let header = b"CCToy\0\x01\x00\x00\x00\x00\x0c";
  assert_eq!(toy_outcome(header), "insufficient_data");
  assert_eq!(run(Target::Toy, header).parser, None);
  let far = b"CCToy\0\x01\x00\x00\x00\x00\x64";
  assert_eq!(toy_outcome(far), "insufficient_data");
  assert_eq!(toy_outcome(&[&header[..], b"ST"].concat()), "insufficient_data");
  assert_eq!(toy_outcome(&[&header[..], b"SX"].concat()), "unknown_format");
  assert_eq!(toy_outcome(&[&header[..], b"XXXX\0\0\0\0"].concat()), "unknown_format");
  assert_eq!(toy_outcome(b"CCTo"), "insufficient_data");
  assert_eq!(toy_outcome(b"CCToy\0\x02"), "unknown_format");
  let walked = run(Target::Toy, &file(0, *b"TCOD", &[&setup(0, 0)]));
  assert_eq!(
    (walked.parser, walked.outcome.as_str()),
    (Some(Parser::ToyContainer), "report")
  );
}

#[test]
fn only_the_toy_codec_gets_an_assembler_and_only_two_framings_declare_a_stream() {
  assert_eq!(
    toy_stream_outcomes(&file(0, *b"XCOD", &[&setup(0, 0)])),
    ["unsupported"]
  );
  assert_eq!(
    toy_stream_outcomes(&file(2, *b"TCOD", &[&setup(0, 0)])),
    Vec::<&str>::new()
  );
  assert_eq!(toy_stream_outcomes(&file(0, *b"TCOD", &[&[0, 1, 0xaa]])), ["not_found"]);
}

#[test]
fn a_list_is_entered_and_an_element_beyond_it_ends_it() {
  let packet = ToyElement::new(*b"PACK", [&[1][..], &setup(0, 0)].concat());
  let stream = ToyElement::new(*b"STRM", [&[1, 0, 0][..], b"TCOD"].concat());
  let inner = toy_file(&[packet]).unwrap();
  let list = ToyElement::new(*b"LIST", inner[12..].to_vec());
  let bytes = toy_file(&[stream, list]).unwrap();
  assert_eq!(
    (toy_outcome(&bytes), toy_stream_outcomes(&bytes)),
    ("report", vec!["parsed"])
  );
  assert_eq!(toy_outcome(&bytes[..bytes.len() - 1]), "truncated");
}

#[test]
fn a_child_that_runs_beyond_its_list_ends_the_list() {
  let stream = ToyElement::new(*b"STRM", [&[1, 0, 0][..], b"TCOD"].concat());
  let child = ToyElement::new(*b"PACK", [&[1][..], &setup(0, 0)].concat()).with_length(20);
  let inner = toy_file(&[child]).unwrap();
  let list = ToyElement::new(*b"LIST", inner[12..].to_vec());
  let title = ToyElement::new(*b"TITL", b"a title of sixteen".to_vec());
  let bytes = toy_file(&[stream, list, title]).unwrap();
  assert_eq!(
    (toy_outcome(&bytes), toy_stream_outcomes(&bytes)),
    ("report", vec!["not_found"])
  );
}

#[test]
fn a_list_that_the_input_cuts_short_keeps_the_children_before_the_end() {
  let stream = ToyElement::new(*b"STRM", [&[1, 0, 0][..], b"TCOD"].concat());
  let title = ToyElement::new(*b"TITL", b"a title".to_vec());
  let inner = toy_file(&[stream, title]).unwrap();
  let list = ToyElement::new(*b"LIST", inner[12..].to_vec());
  let bytes = toy_file(&[list]).unwrap();
  let cut = &bytes[..bytes.len() - 3];
  assert_eq!(
    (toy_outcome(cut), toy_stream_outcomes(cut)),
    ("truncated", vec!["not_found"])
  );
}

#[test]
fn a_packet_that_the_input_cuts_short_offers_the_bytes_before_the_end() {
  let header = [setup(1, 0), PARAMETERS.to_vec()].concat();
  for framing in [0, 1] {
    let bytes = file(framing, *b"TCOD", &[&[&header[..], &[0, 3, 9, 9, 9]].concat()]);
    let cut = &bytes[..bytes.len() - 2];
    assert_eq!(
      (toy_outcome(cut), toy_stream_outcomes(cut)),
      ("truncated", vec!["parsed"]),
      "framing {framing}"
    );
  }
  let bytes = file(0, *b"TCOD", &[&header]);
  let cut = &bytes[..bytes.len() - 2];
  assert_eq!(toy_stream_outcomes(cut), ["partially_parsed_cut_short"]);
}

#[test]
fn a_codec_input_whose_context_misstates_its_packets_length_is_rejected() {
  let bytes = setup(0, 0);
  let input = |framing: u8, length: u32| [&[0, framing][..], &length.to_be_bytes(), &bytes].concat();
  assert_eq!(run(Target::ToyCodec, &input(0, 8)).outcome, "parsed");
  for (framing, length) in [(0, 100), (0, 7), (1, 8)] {
    assert_eq!(
      run(Target::ToyCodec, &input(framing, length)).parser,
      None,
      "framing {framing}, length {length}"
    );
  }
  assert_eq!(run(Target::ToyCodec, &input(1, 0)).outcome, "parsed");
}

#[test]
fn the_codec_gets_the_header_of_the_stream_fixture_in_two_deliveries_joined_by_the_continuation() {
  let fixture = Fixture::load("toy/header-across-packets").unwrap();
  let bytes = &fixture.body.bytes;
  assert_eq!(toy_stream_outcomes(bytes), ["parsed"]);
  let second_alone = [&bytes[..27], &bytes[44..]].concat();
  assert_eq!(toy_stream_outcomes(&second_alone), ["malformed"]);
}
