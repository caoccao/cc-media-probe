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

//! `Candidate`, one possible format of an ambiguous detection (`api.md` section 8).

use super::Confidence;
use crate::Format;
use crate::form_record::form_record;

form_record! {
  /// One possible format of an ambiguous detection, in the rank of `api.md` section 5.1, which falls with the
  /// confidence; each candidate is a different format.
  ///
  /// ```
  /// use cc_media_probe::Detection;
  ///
  /// fn candidates(detection: &Detection) -> Vec<String> {
  ///     detection
  ///         .candidates()
  ///         .iter()
  ///         .map(|candidate| format!("{}, confidence {}", candidate.format(), candidate.confidence()))
  ///         .collect()
  /// }
  /// ```
  #[derive(Clone, Debug, PartialEq, Eq, Hash)]
  pub struct Candidate {
    format: Format,
    confidence: Confidence,
  }
}

impl Candidate {
  /// The candidate's format.
  #[must_use]
  pub fn format(&self) -> &Format {
    &self.format
  }

  /// How sure detection is of the candidate.
  #[must_use]
  pub fn confidence(&self) -> Confidence {
    self.confidence
  }
}

#[cfg(test)]
mod tests {
  use super::Candidate;
  use crate::Format;
  use crate::model::Confidence;

  impl Candidate {
    /// A candidate of this format and confidence.
    pub(crate) fn sample(format: Format, confidence: u8) -> Candidate {
      Candidate {
        format,
        confidence: Confidence::new(confidence).unwrap(),
      }
    }
  }

  #[test]
  fn keeps_the_format_and_its_confidence() {
    let candidate = Candidate::sample(Format::Mp4, 60);
    assert_eq!(candidate.format(), &Format::Mp4);
    assert_eq!(candidate.confidence().value(), 60);
  }
}
