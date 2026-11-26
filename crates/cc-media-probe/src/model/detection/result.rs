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

//! `Detection`, what `detect_*` returns, a root record (`api.md` section 8; plan sections 8.6 and 8.8).

use super::{Candidate, Confidence, DetectionOutcome, Hint};
use crate::Format;
use crate::completeness::Reason;
use crate::model::Usage;

/// What detection found: one of six outcomes, each with its own accessors, which are absent or empty for the
/// others. Detection never opens a companion, and on a forward-only source it reads only from the pinned prefix.
///
/// ```
/// use cc_media_probe::Detection;
/// use cc_media_probe::model::DetectionOutcome;
///
/// fn describe(detection: &Detection) -> String {
///     let used = format!("{} bytes read", detection.usage().bytes_read());
///     match detection.outcome() {
///         DetectionOutcome::Identified => format!(
///             "{} ({:?}), confidence {:?}; {used}",
///             detection.format().map_or("", |format| format.name()),
///             detection.format().and_then(|format| format.support()),
///             detection.confidence().map(|confidence| confidence.value()),
///         ),
///         DetectionOutcome::HintOnly => format!("hint {:?}; {used}", detection.hint().map(|hint| hint.format())),
///         DetectionOutcome::InsufficientData => format!(
///             "needs {:?} bytes ({:?}); {used}",
///             detection.bytes_needed(),
///             detection.insufficient_reason().map(|reason| reason.kind().as_str()),
///         ),
///         other => format!("{} with {} candidates; {used}", other.as_str(), detection.candidates().len()),
///     }
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
  feature = "serde",
  derive(serde::Deserialize),
  serde(try_from = "crate::form::Rooted<crate::form::WithKept<super::detection_form::DetectionBody>>")
)]
pub struct Detection {
  #[cfg(feature = "serde")]
  pub(super) schema_version: crate::document::SchemaVersion,
  pub(super) outcome: DetectionOutcome,
  pub(super) format: Option<Format>,
  pub(super) confidence: Option<Confidence>,
  pub(super) candidates: Vec<Candidate>,
  pub(super) hint: Option<Hint>,
  pub(super) bytes_needed: Option<u64>,
  pub(super) insufficient_reason: Option<Reason>,
  pub(super) usage: Usage,
  #[cfg(feature = "serde")]
  pub(super) kept: crate::form::KeptMembers,
}

impl Detection {
  /// What detection concluded.
  #[must_use]
  pub fn outcome(&self) -> &DetectionOutcome {
    &self.outcome
  }

  /// For `Identified`, the format, which is no family; for `FamilyOnly`, the family. A hinted format is under
  /// `hint()`, never here.
  #[must_use]
  pub fn format(&self) -> Option<&Format> {
    self.format.as_ref()
  }

  /// For `Identified`, how sure detection is.
  #[must_use]
  pub fn confidence(&self) -> Option<Confidence> {
    self.confidence
  }

  /// For `Ambiguous`, at least two candidates, each a different format, in rank.
  #[must_use]
  pub fn candidates(&self) -> &[Candidate] {
    &self.candidates
  }

  /// For `HintOnly`, the format that the hint suggests, not verified.
  #[must_use]
  pub fn hint(&self) -> Option<&Hint> {
    self.hint.as_ref()
  }

  /// For `InsufficientData`, when known, the least length of the input, counted from its start, that detection
  /// needs to decide: three bytes of an eight-byte signature give 8. It is a length, not a read budget.
  #[must_use]
  pub fn bytes_needed(&self) -> Option<u64> {
    self.bytes_needed
  }

  /// For `InsufficientData`, when known, whether the input ended (`TruncatedInput`) or a named limit was reached
  /// (`LimitExhausted`).
  #[must_use]
  pub fn insufficient_reason(&self) -> Option<&Reason> {
    self.insufficient_reason.as_ref()
  }

  /// What the budget accounted for.
  #[must_use]
  pub fn usage(&self) -> &Usage {
    &self.usage
  }

  /// The version of the serialized form that the detection was read with, or `SchemaVersion::CURRENT` for one that
  /// this version built, with the `serde` feature (`api.md` section 12).
  ///
  /// ```
  /// # fn main() -> Result<(), cc_media_probe::Error> {
  /// use cc_media_probe::document::SchemaVersion;
  ///
  /// let detection = cc_media_probe::detect_bytes(b"not a media file")?;
  /// assert_eq!(detection.schema_version(), SchemaVersion::CURRENT);
  /// # Ok(())
  /// # }
  /// ```
  #[cfg(feature = "serde")]
  #[must_use]
  pub fn schema_version(&self) -> crate::document::SchemaVersion {
    self.schema_version
  }

  /// The detection of bytes that matched nothing: `HintOnly` with the hint where there is one, and `Unknown`
  /// otherwise, since a hint never selects a format for such bytes (`api.md` section 4.3).
  pub(crate) fn unmatched(hint: Option<Hint>, usage: Usage) -> Detection {
    Detection {
      #[cfg(feature = "serde")]
      schema_version: crate::document::SchemaVersion::CURRENT,
      outcome: if hint.is_some() {
        DetectionOutcome::HintOnly
      } else {
        DetectionOutcome::Unknown
      },
      format: None,
      confidence: None,
      candidates: Vec::new(),
      hint,
      bytes_needed: None,
      insufficient_reason: None,
      usage,
      #[cfg(feature = "serde")]
      kept: crate::form::KeptMembers::default(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::Detection;
  use crate::Format;
  use crate::completeness::{Reason, ReasonKind};
  use crate::model::{Candidate, Confidence, DetectionOutcome, Hint, Usage};

  impl Detection {
    /// A detection that identified this format with full confidence.
    pub(crate) fn identified(format: Format) -> Detection {
      Detection {
        #[cfg(feature = "serde")]
        schema_version: crate::document::SchemaVersion::CURRENT,
        outcome: DetectionOutcome::Identified,
        format: Some(format),
        confidence: Confidence::new(100),
        candidates: Vec::new(),
        hint: None,
        bytes_needed: None,
        insufficient_reason: None,
        usage: Usage::sample(),
        #[cfg(feature = "serde")]
        kept: crate::form::KeptMembers::default(),
      }
    }
  }

  #[test]
  fn gives_the_members_of_each_outcome() {
    let identified = Detection::identified(Format::Matroska);
    assert_eq!(identified.outcome(), &DetectionOutcome::Identified);
    assert_eq!(identified.format(), Some(&Format::Matroska));
    assert_eq!(identified.confidence().map(|confidence| confidence.value()), Some(100));
    assert_eq!(identified.candidates(), []);
    assert_eq!(identified.hint(), None);
    assert_eq!(identified.usage(), &Usage::sample());
    let ambiguous = Detection {
      outcome: DetectionOutcome::Ambiguous,
      format: None,
      confidence: None,
      candidates: vec![
        Candidate::sample(Format::Mp4, 60),
        Candidate::sample(Format::QuickTime, 40),
      ],
      ..Detection::identified(Format::Matroska)
    };
    assert_eq!(ambiguous.candidates().len(), 2);
    let hinted = Detection {
      outcome: DetectionOutcome::HintOnly,
      format: None,
      confidence: None,
      hint: Some(Hint::sample(Format::Avi)),
      ..Detection::identified(Format::Matroska)
    };
    assert_eq!(hinted.hint().map(Hint::format), Some(&Format::Avi));
    let insufficient = Detection {
      outcome: DetectionOutcome::InsufficientData,
      format: None,
      confidence: None,
      bytes_needed: Some(8),
      insufficient_reason: Some(Reason::sample(ReasonKind::TruncatedInput)),
      ..Detection::identified(Format::Matroska)
    };
    assert_eq!(insufficient.bytes_needed(), Some(8));
    assert_eq!(
      insufficient.insufficient_reason().map(Reason::kind),
      Some(&ReasonKind::TruncatedInput)
    );
  }
}
