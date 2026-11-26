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

//! `Hint` and `HintOrigin`, a format that a hint suggests (`api.md` section 8).

use super::constants::{FILE_NAME_NAME, FORMAT_OPTION_NAME};
use crate::Format;
use crate::form_record::form_record;
use crate::string_enumeration::string_enumeration;

form_record! {
  /// A format that a hint suggests while the bytes matched nothing. It is not verified, and never presented as an
  /// identification.
  ///
  /// ```
  /// use cc_media_probe::Detection;
  ///
  /// fn hint(detection: &Detection) -> Option<String> {
  ///     let hint = detection.hint()?;
  ///     Some(format!("maybe {} (from the {}), not verified", hint.format(), hint.origin().as_str()))
  /// }
  /// ```
  #[derive(Clone, Debug, PartialEq, Eq, Hash)]
  pub struct Hint {
    format: Format,
    origin: HintOrigin,
  }
}

impl Hint {
  /// The format that the hint suggests.
  #[must_use]
  pub fn format(&self) -> &Format {
    &self.format
  }

  /// Where the hint came from.
  #[must_use]
  pub fn origin(&self) -> &HintOrigin {
    &self.origin
  }

  /// The hint of `format`, from `origin`.
  pub(crate) fn new(format: Format, origin: HintOrigin) -> Hint {
    Hint { format, origin }
  }
}

string_enumeration! {
  /// Where a hint came from. `Hint` shows an example.
  ///
  /// ```
  /// use cc_media_probe::model::HintOrigin;
  ///
  /// assert_eq!(HintOrigin::FileName.as_str(), "file_name");
  /// ```
  HintOrigin {
    /// The file-name hint: the path's final component, or the one that the builder sets.
    FileName => FILE_NAME_NAME,
    /// The format hint that the builder sets.
    FormatOption => FORMAT_OPTION_NAME,
  }
}

#[cfg(test)]
mod tests {
  use super::{Hint, HintOrigin};
  use crate::Format;

  impl Hint {
    /// A hint of this format from the file name.
    pub(crate) fn sample(format: Format) -> Hint {
      Hint::new(format, HintOrigin::FileName)
    }
  }

  #[test]
  fn keeps_the_format_and_its_origin() {
    let hint = Hint::sample(Format::Matroska);
    assert_eq!(hint.format(), &Format::Matroska);
    assert_eq!(hint.origin(), &HintOrigin::FileName);
    assert_eq!(HintOrigin::FormatOption.as_str(), "format_option");
  }
}
