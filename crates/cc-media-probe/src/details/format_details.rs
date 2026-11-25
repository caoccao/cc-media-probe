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

//! `FormatDetails`, the format's typed details (`api.md` section 11.1).

use super::enumeration::details_enumeration;

details_enumeration! {
  /// The format's typed details, from `MediaReport::format_details()`. Each probed format adds its variant in its
  /// phase, such as `Matroska(MatroskaDetails)`; the container's per-stream details live inside them, reached by
  /// stream index. Adding a variant to this non-exhaustive enum is a compatible change.
  ///
  /// The kind's stable name, such as `matroska`, and the conflicts, the observed lower bounds and the values beyond
  /// the exact range are reached the same way for every format, through the methods below, without matching on a
  /// variant.
  ///
  /// ```
  /// use cc_media_probe::MediaReport;
  ///
  /// fn kept_values(report: &MediaReport) -> Vec<String> {
  ///     let Some(details) = report.format_details() else {
  ///         return Vec::new();
  ///     };
  ///     let mut lines = vec![format!("details of the kind {}", details.as_str())];
  ///     lines.extend(details
  ///         .conflicts()
  ///         .iter()
  ///         .map(|conflict| format!("{}: {} values", conflict.field(), conflict.values().len())));
  ///     lines.extend(details.observed_lower_bounds().iter().map(|bound| format!("{} at least", bound.field())));
  ///     lines.extend(details.exact_values().iter().map(|exact| format!("{} = {}", exact.field(), exact.numerator())));
  ///     lines
  /// }
  /// ```
  FormatDetails {}
}

#[cfg(test)]
mod tests {
  use super::FormatDetails;
  use crate::details::UnrecognizedDetails;

  #[test]
  fn a_newer_kind_lists_no_values() {
    let details = FormatDetails::Unrecognized(UnrecognizedDetails::sample("matroska"));
    assert_eq!(details.as_str(), "matroska");
    assert_eq!(details.conflicts(), []);
    assert_eq!(details.observed_lower_bounds(), []);
    assert_eq!(details.exact_values(), []);
  }
}
