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

//! `DeclarationRange`, where a stream's declaration lies (`api.md` section 6.3).

use crate::Seconds;
use crate::source::SourceOffset;

/// Where a stream's declaration lies: the first and the last byte offsets at which the file declares the stream,
/// and the time it spans, each optional.
///
/// ```
/// use cc_media_probe::Stream;
///
/// fn declared_between(stream: &Stream) -> Option<(u64, u64)> {
///     let range = stream.declaration_range()?;
///     Some((range.first_offset()?.offset(), range.last_offset()?.offset()))
/// }
///
/// fn declared_for(stream: &Stream) -> Option<String> {
///     let range = stream.declaration_range()?;
///     Some(format!("{} to {}", range.start()?, range.end()?))
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DeclarationRange {
  first_offset: Option<SourceOffset>,
  last_offset: Option<SourceOffset>,
  start: Option<Seconds>,
  end: Option<Seconds>,
}

impl DeclarationRange {
  /// The first byte offset at which the file declares the stream.
  #[must_use]
  pub fn first_offset(&self) -> Option<&SourceOffset> {
    self.first_offset.as_ref()
  }

  /// The last byte offset at which the file declares the stream.
  #[must_use]
  pub fn last_offset(&self) -> Option<&SourceOffset> {
    self.last_offset.as_ref()
  }

  /// The time at which the declaration starts to apply.
  #[must_use]
  pub fn start(&self) -> Option<Seconds> {
    self.start
  }

  /// The time at which the declaration stops applying.
  #[must_use]
  pub fn end(&self) -> Option<Seconds> {
    self.end
  }
}

#[cfg(test)]
mod tests {
  use super::DeclarationRange;
  use crate::Seconds;
  use crate::source::SourceOffset;

  impl DeclarationRange {
    /// A declaration between two offsets of the primary input, over the first ten seconds.
    pub(crate) fn sample() -> DeclarationRange {
      DeclarationRange {
        first_offset: Some(SourceOffset::sample(0, 188, None)),
        last_offset: Some(SourceOffset::sample(0, 376_000, None)),
        start: Some(Seconds::sample(0, 1)),
        end: Some(Seconds::sample(10, 1)),
      }
    }
  }

  #[test]
  fn keeps_its_offsets_and_times() {
    let range = DeclarationRange::sample();
    assert_eq!(range.first_offset().map(SourceOffset::offset), Some(188));
    assert_eq!(range.last_offset().map(SourceOffset::offset), Some(376_000));
    assert_eq!(range.start(), Some(Seconds::sample(0, 1)));
    assert_eq!(range.end(), Some(Seconds::sample(10, 1)));
  }
}
