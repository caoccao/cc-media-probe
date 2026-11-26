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

//! `Guid`, a 16-byte GUID (`api.md` section 6.1).

use std::fmt;

use super::constants::{GUID_BYTES, GUID_FIRST_FIELD, GUID_SEPARATOR, GUID_SHORT_FIELD};

/// A 16-byte GUID, as a tag value of a format that stores one, such as an ASF attribute of the GUID type.
///
/// `bytes()` returns the 16 bytes as stored. `Display` writes the registry form with hyphens, in which the first
/// three fields are little-endian numbers, as the formats that store GUIDs lay them out:
/// `00000001-0000-0010-8000-00aa00389b71`.
///
/// ```
/// use cc_media_probe::model::Guid;
///
/// fn describe(guid: Guid) -> String {
///     format!("{guid} (first byte {:02x})", guid.bytes()[0])
/// }
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Guid {
  bytes: [u8; GUID_BYTES],
}

impl Guid {
  /// The 16 bytes as stored.
  ///
  /// ```
  /// use cc_media_probe::model::Guid;
  ///
  /// fn is_nil(guid: Guid) -> bool {
  ///     guid.bytes().iter().all(|byte| *byte == 0)
  /// }
  /// ```
  #[must_use]
  pub fn bytes(&self) -> [u8; GUID_BYTES] {
    self.bytes
  }

  /// The GUID of these stored bytes, as the serialized form gives them.
  #[cfg(feature = "serde")]
  pub(crate) fn from_bytes(bytes: [u8; GUID_BYTES]) -> Guid {
    Guid { bytes }
  }
}

impl fmt::Display for Guid {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    let (first, rest) = self.bytes.split_first_chunk::<GUID_FIRST_FIELD>().ok_or(fmt::Error)?;
    let (second, rest) = rest.split_first_chunk::<GUID_SHORT_FIELD>().ok_or(fmt::Error)?;
    let (third, rest) = rest.split_first_chunk::<GUID_SHORT_FIELD>().ok_or(fmt::Error)?;
    let (fourth, fifth) = rest.split_first_chunk::<GUID_SHORT_FIELD>().ok_or(fmt::Error)?;
    write!(
      formatter,
      "{:08x}-{:04x}-{:04x}-",
      u32::from_le_bytes(*first),
      u16::from_le_bytes(*second),
      u16::from_le_bytes(*third),
    )?;
    fourth.iter().try_for_each(|byte| write!(formatter, "{byte:02x}"))?;
    fmt::Write::write_char(formatter, GUID_SEPARATOR)?;
    fifth.iter().try_for_each(|byte| write!(formatter, "{byte:02x}"))
  }
}

#[cfg(test)]
mod tests {
  use super::Guid;

  impl Guid {
    /// A GUID of these bytes as stored.
    pub(crate) fn sample(bytes: [u8; 16]) -> Guid {
      Guid { bytes }
    }
  }

  #[test]
  fn displays_the_registry_form() {
    let pcm = Guid::sample([
      0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x80, 0x00, 0x00, 0xaa, 0x00, 0x38, 0x9b, 0x71,
    ]);
    assert_eq!(pcm.to_string(), "00000001-0000-0010-8000-00aa00389b71");
    assert_eq!(pcm.bytes()[0], 0x01);
  }
}
