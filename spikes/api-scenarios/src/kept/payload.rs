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

//! The members that a catch-all keeps, as its payload: a kind of an enum that carries data and an entry, a detection
//! or an error record of an outcome that a newer version wrote. No type is known for them, so the private map of
//! serde_json's `arbitrary_precision` feature among them is the number it spells, as the maintainer chose at the
//! fifteenth and sixteenth reviews: an integer within 64 bits as that integer, and any other number as the nearest
//! `f64`, which fails when it is not finite (`number_text`). So a number keeps its JSON type in every build, and an
//! object of the same shape in the input is that number too, since the Serde data model cannot tell the two apart.
//!
//! A value is kept as written until it becomes such a payload, as the eighteenth review asked: the members of a
//! known kind and of a tree's node are read into their types from what was kept, and a type that requires a number
//! refuses the map there as the object it is (`crate::neutral::read_kept`).

use serde::de::Error;

use super::members::KeptMembers;
use super::value::KeptValue;
use crate::constants::{ARBITRARY_PRECISION_NAME, NOT_JSON_VALUE_MESSAGE};
use crate::number_text::{self, NumberText};

impl KeptMembers {
  /// The members as a catch-all's payload, each private map among them, at any depth, read as its number.
  pub(crate) fn into_payload<Failure: Error>(self) -> Result<KeptMembers, Failure> {
    let members = self
      .into_iter()
      .map(|(name, value)| payload_value(value).map(|value| (name, value)))
      .collect::<Result<Vec<_>, Failure>>()?;
    Ok(KeptMembers::from_checked(members))
  }
}

/// A kept value as a payload: a private map as its number, and the values inside a sequence or a map so too.
fn payload_value<Failure: Error>(value: KeptValue) -> Result<KeptValue, Failure> {
  match value {
    KeptValue::Sequence(items) => items
      .into_iter()
      .map(payload_value)
      .collect::<Result<Vec<_>, Failure>>()
      .map(KeptValue::Sequence),
    KeptValue::Map(members) => match private_number(&members) {
      Some(NumberText::Unsigned(value)) => Ok(KeptValue::Unsigned(value)),
      Some(NumberText::Signed(value)) => Ok(KeptValue::Signed(value)),
      Some(NumberText::FloatingPoint(value)) if value.is_finite() => Ok(KeptValue::FloatingPoint(value.to_bits())),
      Some(NumberText::FloatingPoint(_)) => Err(Failure::custom(NOT_JSON_VALUE_MESSAGE)),
      None => members.into_payload().map(KeptValue::Map),
    },
    other => Ok(other),
  }
}

/// The number that a map spells when it is serde_json's private map: one member of that name holding a number's text.
fn private_number(members: &KeptMembers) -> Option<NumberText> {
  match members.as_slice() {
    [(name, KeptValue::Text(text))] if name == ARBITRARY_PRECISION_NAME => number_text::parse(text),
    _ => None,
  }
}
