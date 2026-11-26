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
//! takes exactly one of the two, and each only in its own range (`json-output.md` section 5.5).

use super::decimal::canonical;
use crate::constants::LARGEST_EXACT;
use crate::form::constants::TWIN_MESSAGE;

/// The number and the twin that an unsigned quantity is written as.
pub(crate) fn unsigned_members(value: Option<u64>) -> (Option<u64>, Option<String>) {
  match value {
    Some(value) if value > LARGEST_EXACT => (None, Some(value.to_string())),
    other => (other, None),
  }
}

/// An unsigned quantity read from its number or its twin, or absent with neither.
pub(crate) fn unsigned_value(number: Option<u64>, twin: Option<String>) -> Result<Option<u64>, &'static str> {
  match (number, twin) {
    (None, None) => Ok(None),
    (Some(number), None) if number <= LARGEST_EXACT => Ok(Some(number)),
    (None, Some(text)) => match canonical::<u64>(&text) {
      Some(value) if value > LARGEST_EXACT => Ok(Some(value)),
      _ => Err(TWIN_MESSAGE),
    },
    _ => Err(TWIN_MESSAGE),
  }
}

/// An unsigned quantity that a record cannot omit, read from its number or its twin.
pub(crate) fn required_unsigned(number: Option<u64>, twin: Option<String>) -> Result<u64, &'static str> {
  unsigned_value(number, twin)?.ok_or(TWIN_MESSAGE)
}

/// The number and the twin that a signed integer within -2^63 and 2^64 - 1 is written as.
pub(crate) fn signed_members(value: i128) -> (Option<i64>, Option<String>) {
  match i64::try_from(value) {
    Ok(number) if number.unsigned_abs() <= LARGEST_EXACT => (Some(number), None),
    _ => (None, Some(value.to_string())),
  }
}

/// A signed integer read from its number or its twin, within -2^63 and 2^64 - 1, the range of a tag's integer.
pub(crate) fn signed_value(number: Option<i64>, twin: Option<String>) -> Result<i128, &'static str> {
  match (number, twin) {
    (Some(number), None) if number.unsigned_abs() <= LARGEST_EXACT => Ok(i128::from(number)),
    (None, Some(text)) => match canonical::<i128>(&text) {
      Some(value)
        if value.unsigned_abs() > u128::from(LARGEST_EXACT)
          && value >= i128::from(i64::MIN)
          && value <= i128::from(u64::MAX) =>
      {
        Ok(value)
      }
      _ => Err(TWIN_MESSAGE),
    },
    _ => Err(TWIN_MESSAGE),
  }
}
