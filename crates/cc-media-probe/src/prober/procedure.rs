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

//! The ordered procedure of `api.md` section 5.1 as far as this build reaches it: detection, and the ending that
//! its outcome gives. Every entry point, blocking and async, ends here once it has its input.

use super::Prober;
use crate::detect::detect;
use crate::{Detection, Error, MediaReport};

impl Prober {
  /// The detection of `input` with these options. This build has no detection inventory, so it reads nothing of
  /// the input and lets it go.
  pub(crate) fn detect_input<Input>(&self, input: Input) -> Detection {
    drop(input);
    detect(self.format_hint())
  }

  /// The probe of `input` with these options, from detection on. Its bytes match nothing in this build, with or
  /// without a hint, so the probe ends with `UnknownFormat` and the detection's hint and usage.
  pub(crate) fn probe_input<Input>(&self, input: Input) -> Result<MediaReport, Error> {
    Err(Error::unknown_format(&self.detect_input(input)))
  }
}

#[cfg(test)]
mod tests {
  use crate::model::{DetectionOutcome, Hint, Usage};
  use crate::{Error, ErrorKind, Format, Prober};

  #[test]
  fn ends_with_an_unknown_format() {
    let error = Prober::new().probe_input(()).err();
    assert_eq!(error.as_ref().map(Error::kind), Some(&ErrorKind::UnknownFormat));
    assert_eq!(error.as_ref().and_then(|error| error.hint()), None);
    assert_eq!(error.as_ref().map(Error::usage), Some(&Usage::unused()));
    assert_eq!(
      error.as_ref().map(ToString::to_string).as_deref(),
      Some("the format is unknown")
    );
  }

  #[test]
  fn keeps_the_format_hint_in_the_error() {
    let prober = Prober::new().with_format_hint(Format::Mp4);
    assert_eq!(prober.detect_input(()).outcome(), &DetectionOutcome::HintOnly);
    let error = prober.probe_input(()).err();
    let hinted = error.as_ref().and_then(|error| error.hint()).map(Hint::format);
    assert_eq!(hinted, Some(&Format::Mp4));
    assert!(error.is_some_and(|error| error.source_failure().is_none() && error.partial_report().is_none()));
  }
}
