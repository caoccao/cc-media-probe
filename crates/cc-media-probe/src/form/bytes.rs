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

//! Bytes in the form's one encoding of bytes, lowercase hexadecimal with two digits a byte (`json-output.md` section
//! 5.6): binary tag data, attachment data and a tag name's stored bytes. A field names these functions in its
//! `serialize_with` and `deserialize_with`.

use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::constants::HEXADECIMAL_MESSAGE;
use crate::utilities::{decode_bytes, encode_bytes};

/// Bytes that a field may hold, as Serde hands the field to `serialize_with`.
pub(crate) trait StoredBytes {
  /// The bytes, when the field holds them.
  fn stored(&self) -> Option<&[u8]>;
}

impl StoredBytes for Option<Vec<u8>> {
  fn stored(&self) -> Option<&[u8]> {
    self.as_deref()
  }
}

/// Bytes that a writer borrows, written as lowercase hexadecimal, so that writing them copies no payload.
pub(crate) struct BorrowedBytes<'bytes>(pub(crate) &'bytes [u8]);

impl Serialize for BorrowedBytes<'_> {
  fn serialize<Output: Serializer>(&self, serializer: Output) -> Result<Output::Ok, Output::Error> {
    serializer.serialize_str(&encode_bytes(self.0))
  }
}

/// Writes present bytes as lowercase hexadecimal; the field skips absent ones.
pub(crate) fn write_optional_bytes<Bytes: StoredBytes, Output: Serializer>(
  bytes: &Bytes,
  serializer: Output,
) -> Result<Output::Ok, Output::Error> {
  match bytes.stored() {
    Some(bytes) => serializer.serialize_str(&encode_bytes(bytes)),
    None => serializer.serialize_none(),
  }
}

/// Reads present bytes from whole bytes of lowercase hexadecimal; the field gives absent ones `None` by its
/// `default`.
pub(crate) fn read_optional_bytes<'input, Input: Deserializer<'input>>(
  deserializer: Input,
) -> Result<Option<Vec<u8>>, Input::Error> {
  let text = String::deserialize(deserializer)?;
  decode_bytes(&text)
    .map(Some)
    .ok_or_else(|| Input::Error::custom(HEXADECIMAL_MESSAGE))
}
