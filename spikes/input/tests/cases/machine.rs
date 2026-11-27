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

//! D8's fallback gives the async walker's report, usage included: the state machine through its blocking driver
//! from bytes and from a seekable reader, and through its async driver from a source that answers pending to every
//! other call, on files that reach the traversal's edges, under limits that stop it in several places.

use std::io::Cursor;

use cc_media_probe::Limits;
use futures_lite::future::block_on;
use input::builder::{ToyFile, parameters_body, payload_unit, setup_body, setup_unit, unit};
use input::codec::{Framing, PARAMETERS_KIND, Representation, SETUP_KIND};
use input::{asynchronous, blocking, machine};

use crate::support::{PendingSource, size, small_options, typical_file};

/// The files of the comparison.
fn files() -> Vec<Vec<u8>> {
  let mut inner = b"TITL".to_vec();
  inner.extend_from_slice(&10_u32.to_be_bytes());
  let overlapping = ToyFile::new()
    .title("x")
    .element(*b"PADX", &inner)
    .title_reference(29)
    .title("After")
    .title_reference(12)
    .build();
  let cut_list = [ToyFile::new().title("x").build(), b"LIST\0\0\0\x14".to_vec()].concat();
  let mut headerless = ToyFile::new().stream(1, Framing::Packet).stream(2, Framing::Stream);
  for number in [1, 2].into_iter().cycle().take(200) {
    headerless = headerless.packet(number, &payload_unit(30));
  }
  let long = [
    unit(
      Representation::TwoByteLengths,
      SETUP_KIND,
      &setup_body(320, 240, 150, 0),
    )
    .unwrap(),
    unit(
      Representation::TwoByteLengths,
      PARAMETERS_KIND,
      &parameters_body(&[3; 150]),
    )
    .unwrap(),
  ]
  .concat();
  let representations = ToyFile::new()
    .stream_with_representation(1, Framing::Packet, Representation::TwoByteLengths)
    .packet(1, &long)
    .packet(1, &[setup_unit(9, 9, 0, 0), vec![0; 4000]].concat())
    .build();
  vec![
    typical_file(),
    overlapping,
    cut_list,
    headerless.build(),
    representations,
    b"CCToy\0\x02".to_vec(),
    b"elephants are not toy files".to_vec(),
    Vec::new(),
  ]
}

/// The options of the comparison: the small buffers, and limits that stop the probe in several places.
fn option_sets() -> Vec<input::Options> {
  let with = |limits: Limits| input::Options {
    limits,
    ..small_options()
  };
  vec![
    small_options(),
    with(Limits::default().with_work_units(20)),
    with(Limits::default().with_header_bytes_per_stream(16)),
    with(Limits::default().with_bytes_read(50)),
    with(Limits::default().with_working_memory(300)),
    input::Options {
      piece: size(1),
      ..small_options()
    },
  ]
}

#[test]
fn the_state_machine_gives_the_async_walkers_report_from_every_input() {
  for (index, file) in files().iter().enumerate() {
    for (variant, options) in option_sets().iter().enumerate() {
      let expected = blocking::probe_bytes(file, options).unwrap();
      assert_eq!(
        machine::probe_bytes(file, options).unwrap(),
        expected,
        "bytes, file {index}, options {variant}"
      );
      let reader = blocking::probe_reader(Cursor::new(file.clone()), options).unwrap();
      assert_eq!(
        machine::probe_reader(Cursor::new(file.clone()), options).unwrap(),
        reader,
        "reader, file {index}, options {variant}"
      );
      let pending = block_on(asynchronous::probe_source(PendingSource::new(file), options)).unwrap();
      assert_eq!(
        block_on(machine::probe_source(PendingSource::new(file), options)).unwrap(),
        pending,
        "async source, file {index}, options {variant}"
      );
    }
  }
}
