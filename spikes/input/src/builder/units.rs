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

//! The units of the toy codec. A unit whose body its length field cannot hold is refused rather than written with
//! a wrong length, so a builder's mistake never looks like a parser's failure.

use super::constants::PAYLOAD_FILLER;
use crate::codec::{PARAMETERS_KIND, PAYLOAD_KIND, Representation, SETUP_KIND, SETUP_LENGTH};

/// A unit of `kind` in `representation`, or `None` when its length field cannot hold the length of `body`.
#[must_use]
pub fn unit(representation: Representation, kind: u8, body: &[u8]) -> Option<Vec<u8>> {
  let mut unit = vec![kind];
  match representation {
    Representation::OneByteLengths => unit.push(u8::try_from(body.len()).ok()?),
    Representation::TwoByteLengths => unit.extend_from_slice(&u16::try_from(body.len()).ok()?.to_be_bytes()),
  }
  unit.extend_from_slice(body);
  Some(unit)
}

/// The body of a setup unit.
#[must_use]
pub fn setup_body(width: u16, height: u16, parameter_count: u8, flags: u8) -> Vec<u8> {
  let mut body = Vec::with_capacity(SETUP_LENGTH);
  body.extend_from_slice(&width.to_be_bytes());
  body.extend_from_slice(&height.to_be_bytes());
  body.push(parameter_count);
  body.push(flags);
  body
}

/// The body of a parameters unit.
#[must_use]
pub fn parameters_body(parameters: &[u16]) -> Vec<u8> {
  parameters
    .iter()
    .flat_map(|parameter| parameter.to_be_bytes())
    .collect()
}

/// A setup unit with one-byte lengths, which always hold its body.
#[must_use]
pub fn setup_unit(width: u16, height: u16, parameter_count: u8, flags: u8) -> Vec<u8> {
  let mut unit = vec![SETUP_KIND, u8::try_from(SETUP_LENGTH).unwrap_or(u8::MAX)];
  unit.extend_from_slice(&setup_body(width, height, parameter_count, flags));
  unit
}

/// A parameters unit with one-byte lengths, or `None` for more than 127 parameters, whose length a one-byte field
/// cannot hold.
#[must_use]
pub fn parameters_unit(parameters: &[u16]) -> Option<Vec<u8>> {
  unit(
    Representation::OneByteLengths,
    PARAMETERS_KIND,
    &parameters_body(parameters),
  )
}

/// A payload unit of `length` bytes, with a one-byte length.
#[must_use]
pub fn payload_unit(length: u8) -> Vec<u8> {
  let mut unit = vec![PAYLOAD_KIND, length];
  unit.resize(usize::from(length).saturating_add(unit.len()), PAYLOAD_FILLER);
  unit
}
