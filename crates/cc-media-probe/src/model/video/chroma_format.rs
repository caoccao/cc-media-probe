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

//! `ChromaFormat` and `ChromaSubsampling`, how a video subsamples its chroma (`api.md` section 6.5).

use super::constants::{
  CHROMA_411_KIND, CHROMA_420_KIND, CHROMA_422_KIND, CHROMA_444_KIND, MONOCHROME_KIND, OTHER_KIND,
};
use crate::data_enumeration::data_enumeration;
use crate::form_record::form_record;

data_enumeration! {
  /// How a video subsamples its chroma. An enum that carries data: serialized, it is a map of its `kind` and, for
  /// `Other`, the member named after it (`json-output.md` section 6.2).
  ///
  /// ```
  /// use cc_media_probe::model::ChromaFormat;
  ///
  /// fn describe(format: &ChromaFormat) -> String {
  ///     match format {
  ///         ChromaFormat::Chroma420 => "4:2:0".to_owned(),
  ///         ChromaFormat::Other(subsampling) => {
  ///             format!("{} by {}", subsampling.horizontal(), subsampling.vertical())
  ///         }
  ///         other => other.as_str().to_owned(),
  ///     }
  /// }
  ///
  /// assert_eq!(ChromaFormat::Monochrome.as_str(), "monochrome");
  /// ```
  ChromaFormat {
    /// No chroma.
    Monochrome => MONOCHROME_KIND,
    /// Chroma at half the horizontal and half the vertical resolution.
    Chroma420 => CHROMA_420_KIND,
    /// Chroma at half the horizontal resolution.
    Chroma422 => CHROMA_422_KIND,
    /// Chroma at full resolution.
    Chroma444 => CHROMA_444_KIND,
    /// Chroma at a quarter of the horizontal resolution.
    Chroma411 => CHROMA_411_KIND,
    /// A subsampling kept as written.
    Other(ChromaSubsampling) => OTHER_KIND,
  }
}

form_record! {
  /// A chroma subsampling kept as written: the horizontal and the vertical factors, such as Matroska's
  /// `ChromaSubsamplingHorz` and `ChromaSubsamplingVert`. `ChromaFormat` shows an example.
  #[derive(Clone, Debug, PartialEq, Eq, Hash)]
  pub struct ChromaSubsampling {
    horizontal: u64,
    vertical: u64,
  }
}

impl ChromaSubsampling {
  /// The horizontal factor, as written.
  #[must_use]
  pub fn horizontal(&self) -> u64 {
    self.horizontal
  }

  /// The vertical factor, as written.
  #[must_use]
  pub fn vertical(&self) -> u64 {
    self.vertical
  }
}

#[cfg(test)]
mod tests {
  use super::{ChromaFormat, ChromaSubsampling};
  use crate::model::UnrecognizedKind;

  impl ChromaSubsampling {
    /// A subsampling by these factors.
    pub(crate) fn sample(horizontal: u64, vertical: u64) -> ChromaSubsampling {
      ChromaSubsampling { horizontal, vertical }
    }
  }

  #[test]
  fn names_each_kind() {
    for (format, name) in [
      (ChromaFormat::Monochrome, "monochrome"),
      (ChromaFormat::Chroma420, "chroma_420"),
      (ChromaFormat::Chroma422, "chroma_422"),
      (ChromaFormat::Chroma444, "chroma_444"),
      (ChromaFormat::Chroma411, "chroma_411"),
      (ChromaFormat::Other(ChromaSubsampling::sample(4, 1)), "other"),
      (
        ChromaFormat::Unrecognized(UnrecognizedKind::sample("chroma_440")),
        "chroma_440",
      ),
    ] {
      assert_eq!(format.as_str(), name);
    }
  }

  #[test]
  fn keeps_a_subsampling_as_written() {
    let subsampling = ChromaSubsampling::sample(4, 1);
    assert_eq!((subsampling.horizontal(), subsampling.vertical()), (4, 1));
  }
}
