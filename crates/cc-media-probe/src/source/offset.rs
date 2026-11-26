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

//! `SourceOffset`, a byte offset that names its source (`api.md` section 6.3; plan section 7.7).

use super::SourceIdentifier;

/// A byte offset that names its source, as every byte offset of the report does (plan section 7.7).
///
/// ```
/// use cc_media_probe::Stream;
///
/// fn declared_at(stream: &Stream) -> Option<String> {
///     let offset = stream.declaration_range()?.first_offset()?;
///     Some(format!("byte {} of source {}", offset.offset(), offset.source_identifier()))
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SourceOffset {
  source_identifier: SourceIdentifier,
  offset: u64,
  logical_offset: Option<u64>,
}

impl SourceOffset {
  /// The source that the offset counts in.
  ///
  /// ```
  /// use cc_media_probe::source::SourceOffset;
  ///
  /// fn in_the_primary_input(offset: &SourceOffset) -> bool {
  ///     offset.source_identifier().value() == 0
  /// }
  /// ```
  #[must_use]
  pub fn source_identifier(&self) -> SourceIdentifier {
    self.source_identifier
  }

  /// The offset from that source's own start.
  ///
  /// ```
  /// use cc_media_probe::source::SourceOffset;
  ///
  /// fn kibibytes(offset: &SourceOffset) -> u64 {
  ///     offset.offset() / 1_024
  /// }
  /// ```
  #[must_use]
  pub fn offset(&self) -> u64 {
    self.offset
  }

  /// The offset in the stream that several files form, where they form one.
  ///
  /// ```
  /// use cc_media_probe::source::SourceOffset;
  ///
  /// fn position(offset: &SourceOffset) -> u64 {
  ///     offset.logical_offset().unwrap_or(offset.offset())
  /// }
  /// ```
  #[must_use]
  pub fn logical_offset(&self) -> Option<u64> {
    self.logical_offset
  }
}

/// A source offset as written: its source, and each offset as a number or beyond 2^53 - 1 its twin (`json-output.md`
/// section 5.5).
#[cfg(feature = "serde")]
#[derive(serde::Serialize, serde::Deserialize)]
struct SourceOffsetForm {
  source_identifier: SourceIdentifier,
  #[serde(skip_serializing_if = "Option::is_none")]
  offset: Option<u64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  offset_exact: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  logical_offset: Option<u64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  logical_offset_exact: Option<String>,
}

#[cfg(feature = "serde")]
impl serde::Serialize for SourceOffset {
  fn serialize<Output: serde::Serializer>(&self, serializer: Output) -> Result<Output::Ok, Output::Error> {
    let (offset, offset_exact) = crate::form::unsigned_members(Some(self.offset));
    let (logical_offset, logical_offset_exact) = crate::form::unsigned_members(self.logical_offset);
    let form = SourceOffsetForm {
      source_identifier: self.source_identifier,
      offset,
      offset_exact,
      logical_offset,
      logical_offset_exact,
    };
    serde::Serialize::serialize(&form, serializer)
  }
}

#[cfg(feature = "serde")]
impl TryFrom<SourceOffsetForm> for SourceOffset {
  type Error = &'static str;

  fn try_from(form: SourceOffsetForm) -> Result<SourceOffset, &'static str> {
    Ok(SourceOffset {
      source_identifier: form.source_identifier,
      offset: crate::form::required_unsigned(form.offset, form.offset_exact)?,
      logical_offset: crate::form::unsigned_value(form.logical_offset, form.logical_offset_exact)?,
    })
  }
}

#[cfg(feature = "serde")]
crate::form::checked_form!(SourceOffset from SourceOffsetForm);

#[cfg(test)]
mod tests {
  use super::SourceOffset;
  use crate::source::SourceIdentifier;

  impl SourceOffset {
    /// The offset `offset` in the source with the number `source`.
    pub(crate) fn sample(source: u64, offset: u64, logical_offset: Option<u64>) -> SourceOffset {
      SourceOffset {
        source_identifier: SourceIdentifier::sample(source),
        offset,
        logical_offset,
      }
    }
  }

  #[test]
  fn names_its_source() {
    let offset = SourceOffset::sample(1, 4_096, Some(1_052_672));
    assert_eq!(offset.source_identifier(), SourceIdentifier::sample(1));
    assert_eq!(offset.offset(), 4_096);
    assert_eq!(offset.logical_offset(), Some(1_052_672));
  }
}
