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

//! `EntryOutcome`, what a document's entry holds (`api.md` section 12; `json-output.md` section 3).

use super::constants::{ERROR_NAME, OK_NAME, PARTIAL_NAME};
use crate::string_enumeration::string_enumeration;

string_enumeration! {
  /// What an entry holds, by the table of plan section 8.9: `ok` for a complete report or an identified format,
  /// `partial` for an incomplete report or a detection of a family only, an ambiguous result or a hint only, and
  /// `error` for an error record or a detection of insufficient data or an unknown format. An outcome that a newer
  /// version wrote, such as a hypothetical `skipped`, reads into the catch-all, and its entry keeps the members that
  /// this version does not read.
  ///
  /// ```
  /// use cc_media_probe::document::EntryOutcome;
  ///
  /// fn fails_under_strict(outcome: &EntryOutcome) -> bool {
  ///     !matches!(outcome, EntryOutcome::Ok)
  /// }
  ///
  /// assert_eq!(EntryOutcome::Partial.as_str(), "partial");
  /// assert!(fails_under_strict(&EntryOutcome::Error));
  /// let newer: Option<EntryOutcome> = serde_json::from_str("\"skipped\"").ok();
  /// assert!(matches!(newer, Some(EntryOutcome::Unrecognized(_))));
  /// ```
  EntryOutcome {
    /// A complete report, or an identified format.
    Ok => OK_NAME,
    /// An incomplete report, or a detection of a family only, an ambiguous result or a hint only.
    Partial => PARTIAL_NAME,
    /// An error record, or a detection of insufficient data or an unknown format.
    Error => ERROR_NAME,
  }
}

#[cfg(test)]
mod tests {
  use super::EntryOutcome;
  use crate::model::UnrecognizedValue;

  #[test]
  fn names_each_outcome() {
    for (outcome, name) in [
      (EntryOutcome::Ok, "ok"),
      (EntryOutcome::Partial, "partial"),
      (EntryOutcome::Error, "error"),
      (
        EntryOutcome::Unrecognized(UnrecognizedValue::sample("skipped")),
        "skipped",
      ),
    ] {
      assert_eq!(outcome.as_str(), name);
      assert_eq!(EntryOutcome::from_text(name.to_owned()), outcome);
    }
  }

  #[test]
  fn a_name_of_this_version_is_found_from_borrowed_text() {
    // An entry, a detection and an error record ask whether their outcome is newer with this lookup, which copies
    // no text, as the second review of the third slice's second part asked.
    assert_eq!(EntryOutcome::from_known_text("partial"), Some(EntryOutcome::Partial));
    assert_eq!(EntryOutcome::from_known_text("skipped"), None);
    // An enum whose only value so far is its catch-all knows no name.
    assert_eq!(crate::model::AmbisonicChannelOrdering::from_known_text("acn"), None);
  }
}
