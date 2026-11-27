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

//! The two sample fixtures of `docs/design/test-kit.md` section 3, each checked against a second transcription of
//! its bytes: the RFC's own dump, and the builder.

use kit::builder::{ToyElement, toy_file};
use kit::fixture::Fixture;

/// RFC 9639, Appendix D.1.1, "Example File 1 in Hexadecimal Representation", transcribed as the RFC prints it.
const RFC_9639_DUMP: &str = "664c 6143 8000 0022 1000 1000 0000 0f00 000f 0ac4 42f0 0000 0001 3e84 b418 07dc 6903 0758 \
                             6a3d ad1a 2e0f fff8 6918 0000 bf03 58fd 0312 8baa 9a";

fn dump_bytes(dump: &str) -> Vec<u8> {
  let digits: String = dump.split_whitespace().collect();
  digits
    .as_bytes()
    .chunks(2)
    .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap())
    .collect()
}

#[test]
fn the_flac_worked_example_assembles_to_the_rfc_dump() {
  let fixture = Fixture::load("flac/decoding-example-1").unwrap();
  assert_eq!(fixture.body.bytes, dump_bytes(RFC_9639_DUMP));
  assert_eq!(
    fixture.header.source,
    "RFC 9639, Appendix D.1, \"Decoding Example 1\", from its tables 27 to 31 and its text"
  );
  assert_eq!(
    fixture.header.route.as_ref().map(|route| route.target.as_str()),
    Some("flac")
  );
  assert_eq!(fixture.header.errata, Vec::<String>::new());
  let region = &fixture.body.regions[0];
  assert_eq!((region.start, region.end), (4, 42));
  assert_eq!(&fixture.body.bytes[region.start..region.start + 4], [0x80, 0, 0, 0x22]);
  assert_eq!(region.route.target, "flac_codec");
  assert_eq!(region.route.representation.as_deref(), Some("metadata_blocks"));
}

#[test]
fn the_toy_fixture_assembles_to_what_the_builder_writes() {
  let fixture = Fixture::load("toy/setup-and-parameters").unwrap();
  let header = [1, 0x06, 0x01, 0x40, 0x00, 0xf0, 0x01, 0x00, 0x02, 0x02, 0x00, 0x07];
  let built = toy_file(&[
    ToyElement::new(*b"STRM", [&[1_u8, 0, 0][..], b"TCOD"].concat()),
    ToyElement::new(*b"PACK", [&[1_u8][..], &header].concat()),
  ])
  .unwrap();
  assert_eq!(fixture.body.bytes, built);
  let region = &fixture.body.regions[0];
  assert_eq!(&fixture.body.bytes[region.start..region.end], header);
}

#[test]
fn a_builder_writes_a_wrong_length_on_purpose() {
  let built = toy_file(&[ToyElement::new(*b"TITL", b"x".to_vec()).with_length(9)]).unwrap();
  assert_eq!(&built[12..], [b'T', b'I', b'T', b'L', 0, 0, 0, 9, b'x']);
}
