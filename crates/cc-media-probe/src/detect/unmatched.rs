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

//! The detection of an input whose bytes match nothing (`api.md` sections 4.3 and 8), which this build gives for
//! every input, since its detection inventory is empty until plan Phase 3.

use crate::Format;
use crate::model::{Detection, Hint, HintOrigin, Usage};

/// The detection of an input whose bytes match nothing, which reads nothing of it: `HintOnly` with a hint of the
/// format option where `format_hint` names a format, and `Unknown` otherwise. The file-name hint suggests no format
/// before the inventory maps names to formats.
pub(crate) fn detect(format_hint: Option<&Format>) -> Detection {
  let hint = format_hint.map(|format| Hint::new(format.clone(), HintOrigin::FormatOption));
  Detection::unmatched(hint, Usage::unused())
}

#[cfg(test)]
mod tests {
  use super::detect;
  use crate::model::{DetectionOutcome, Hint, HintOrigin, Usage};
  use crate::{Format, Limit};

  #[test]
  fn matches_nothing_without_a_hint() {
    let detection = detect(None);
    assert_eq!(detection.outcome(), &DetectionOutcome::Unknown);
    assert_eq!(detection.format(), None);
    assert_eq!(detection.confidence(), None);
    assert_eq!(detection.candidates(), []);
    assert_eq!(detection.hint(), None);
    assert_eq!(detection.bytes_needed(), None);
    assert_eq!(detection.insufficient_reason(), None);
    assert_eq!(detection.usage(), &Usage::unused());
    assert_eq!(detection.usage().get(&Limit::BytesRead), Some(0));
  }

  #[test]
  fn gives_the_format_hint_alone() {
    let detection = detect(Some(&Format::WebM));
    assert_eq!(detection.outcome(), &DetectionOutcome::HintOnly);
    assert_eq!(detection.format(), None);
    let hint = detection.hint();
    assert_eq!(hint.map(Hint::format), Some(&Format::WebM));
    assert_eq!(hint.map(Hint::origin), Some(&HintOrigin::FormatOption));
  }
}
