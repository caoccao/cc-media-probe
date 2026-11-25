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

//! `PartReason`, one shortfall and the part it stopped (`api.md` section 5.3).

use super::{Part, Reason};
use crate::source::SourceIdentifier;

/// One shortfall and the part it stopped, from `Completeness::reasons()` and `LimitRecord::first_reason()`.
///
/// ```
/// use cc_media_probe::completeness::Completeness;
///
/// fn list_reasons(completeness: &Completeness) {
///     for reason in completeness.reasons() {
///         let stream = reason.stream().map(|index| format!(" of stream {index}")).unwrap_or_default();
///         let source = reason.source_identifier().map(|source| format!(" of source {source}")).unwrap_or_default();
///         println!("{}{stream}{source}: {}", reason.part().as_str(), reason.reason().kind().as_str());
///     }
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PartReason {
  part: Part,
  stream: Option<usize>,
  source_identifier: Option<SourceIdentifier>,
  reason: Reason,
}

impl PartReason {
  /// The part that the reason stopped.
  #[must_use]
  pub fn part(&self) -> &Part {
    &self.part
  }

  /// The stream, for a stream's headers, and for discovery where a reported stream's declaration fell short.
  #[must_use]
  pub fn stream(&self) -> Option<usize> {
    self.stream
  }

  /// The source, for a companion's request or a source that failed, which names its entry in the source table.
  #[must_use]
  pub fn source_identifier(&self) -> Option<SourceIdentifier> {
    self.source_identifier
  }

  /// Why the part fell short.
  #[must_use]
  pub fn reason(&self) -> &Reason {
    &self.reason
  }
}

#[cfg(test)]
mod tests {
  use super::PartReason;
  use crate::Limit;
  use crate::completeness::{Part, Reason, ReasonKind};
  use crate::source::SourceIdentifier;

  impl PartReason {
    /// The reason `reason` of the part `part`, without a stream or a source.
    pub(crate) fn sample(part: Part, reason: Reason) -> PartReason {
      PartReason {
        part,
        stream: None,
        source_identifier: None,
        reason,
      }
    }

    /// The same reason, naming this stream.
    pub(crate) fn of_stream(self, stream: usize) -> PartReason {
      PartReason {
        stream: Some(stream),
        ..self
      }
    }

    /// The same reason, naming this source.
    pub(crate) fn of_source(self, source: u64) -> PartReason {
      PartReason {
        source_identifier: Some(SourceIdentifier::sample(source)),
        ..self
      }
    }
  }

  #[test]
  fn names_the_part_and_where_it_fell_short() {
    let reason = PartReason::sample(Part::Headers, Reason::exhausted(Limit::HeaderBytesPerStream)).of_stream(2);
    assert_eq!(reason.part(), &Part::Headers);
    assert_eq!(reason.stream(), Some(2));
    assert_eq!(reason.source_identifier(), None);
    assert_eq!(reason.reason().limit(), Some(&Limit::HeaderBytesPerStream));
    let companion = PartReason::sample(Part::Companions, Reason::sample(ReasonKind::CompanionNotFound)).of_source(1);
    assert_eq!(companion.source_identifier(), Some(SourceIdentifier::sample(1)));
  }
}
