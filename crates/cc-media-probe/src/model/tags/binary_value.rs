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

//! `BinaryValue`, a binary tag value (`api.md` section 6.9).

/// A binary tag value: its size, and its bytes where they were kept within the retained-data limit.
///
/// ```
/// use cc_media_probe::model::BinaryValue;
///
/// fn describe(binary: &BinaryValue) -> String {
///     match binary.data() {
///         Some(data) => format!("{} bytes, starting {:02x?}", binary.size(), data.iter().take(4).collect::<Vec<_>>()),
///         None => format!("{} bytes, not kept", binary.size()),
///     }
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BinaryValue {
  size: u64,
  data: Option<Vec<u8>>,
}

impl BinaryValue {
  /// The size of the value, in bytes.
  #[must_use]
  pub fn size(&self) -> u64 {
    self.size
  }

  /// The bytes, when they were kept within the retained-data limit.
  #[must_use]
  pub fn data(&self) -> Option<&[u8]> {
    self.data.as_deref()
  }
}

#[cfg(test)]
mod tests {
  use super::BinaryValue;

  impl BinaryValue {
    /// A value of these bytes, kept.
    pub(crate) fn sample(data: &[u8]) -> BinaryValue {
      BinaryValue {
        size: u64::try_from(data.len()).unwrap(),
        data: Some(data.to_vec()),
      }
    }
  }

  #[test]
  fn keeps_the_size_and_the_bytes() {
    let binary = BinaryValue::sample(&[0x89, 0x50, 0x4e, 0x47]);
    assert_eq!(binary.size(), 4);
    assert_eq!(binary.data(), Some(&[0x89, 0x50, 0x4e, 0x47][..]));
    let dropped = BinaryValue { size: 9, data: None };
    assert_eq!(dropped.data(), None);
  }
}
