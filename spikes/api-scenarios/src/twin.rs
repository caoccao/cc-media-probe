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

//! The exact twins of `api.md` section 12. A quantity that a record cannot omit is a number within 2^53 - 1; a
//! larger one leaves the number out, and a member with the suffix `_exact` holds its canonical decimal text. Reading
//! takes exactly one of the two, and each only in its own range.

use crate::constants::{LARGEST_EXACT, MINUS_SIGN, QUANTITY_MESSAGE, TWIN_MESSAGE};
use crate::document::InvalidDocument;

/// The number and the twin that an unsigned quantity is written as.
pub(crate) fn unsigned_members(value: Option<u64>) -> (Option<u64>, Option<String>) {
  match value {
    Some(value) if value > LARGEST_EXACT => (None, Some(value.to_string())),
    other => (other, None),
  }
}

/// An unsigned quantity read from its number or its twin, or absent with neither.
pub(crate) fn unsigned_value(number: Option<u64>, twin: Option<String>) -> Result<Option<u64>, InvalidDocument> {
  match (number, twin) {
    (None, None) => Ok(None),
    (Some(number), None) if number <= LARGEST_EXACT => Ok(Some(number)),
    (None, Some(text)) => large_magnitude(&text).map(Some),
    _ => Err(InvalidDocument::new(TWIN_MESSAGE)),
  }
}

/// The number and the twin that a signed quantity, a magnitude with its sign, is written as.
pub(crate) fn signed_members(magnitude: u64, negative: bool) -> (Option<i64>, Option<String>) {
  match i64::try_from(magnitude) {
    Ok(number) if magnitude <= LARGEST_EXACT => (Some(if negative { number.wrapping_neg() } else { number }), None),
    _ if negative => (None, Some(format!("{MINUS_SIGN}{magnitude}"))),
    _ => (None, Some(magnitude.to_string())),
  }
}

/// A signed quantity read from its number or its twin, as its magnitude and its sign.
pub(crate) fn signed_value(number: Option<i64>, twin: Option<String>) -> Result<(u64, bool), InvalidDocument> {
  match (number, twin) {
    (Some(number), None) if number.unsigned_abs() <= LARGEST_EXACT => Ok((number.unsigned_abs(), number < 0)),
    (None, Some(text)) => match text.strip_prefix(MINUS_SIGN) {
      Some(digits) => match large_magnitude(digits) {
        Ok(magnitude) if magnitude <= i64::MIN.unsigned_abs() => Ok((magnitude, true)),
        _ => Err(InvalidDocument::new(TWIN_MESSAGE)),
      },
      None => large_magnitude(&text).map(|magnitude| (magnitude, false)),
    },
    _ => Err(InvalidDocument::new(TWIN_MESSAGE)),
  }
}

/// A magnitude beyond the exact range, from its canonical decimal text: digits only, without a leading zero.
fn large_magnitude(text: &str) -> Result<u64, InvalidDocument> {
  match text.parse::<u64>() {
    Ok(value) if value > LARGEST_EXACT && value.to_string() == text => Ok(value),
    _ => Err(InvalidDocument::new(TWIN_MESSAGE)),
  }
}

/// A normalized quantity that is present, read only within 2^53 - 1, which every JSON reader holds exactly; one
/// beyond it is absent in the report and kept in the details (`json-output.md` section 5.5; fourteenth review).
pub(crate) fn exact_quantity<'input, Input: serde::Deserializer<'input>>(
  deserializer: Input,
) -> Result<Option<u64>, Input::Error> {
  let quantity = <u64 as serde::Deserialize>::deserialize(deserializer)?;
  if quantity > LARGEST_EXACT {
    return Err(<Input::Error as serde::de::Error>::custom(QUANTITY_MESSAGE));
  }
  Ok(Some(quantity))
}
