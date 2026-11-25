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

//! `SourceOutcome`, what became of a request for a source (`api.md` section 6.15).

use super::constants::{DECLINED_NAME, FAILED_NAME, LIMIT_REACHED_NAME, NOT_FOUND_NAME, OPENED_NAME};
use crate::string_enumeration::string_enumeration;

string_enumeration! {
  /// What became of a request for a source.
  ///
  /// ```
  /// use cc_media_probe::source::{SourceEntry, SourceOutcome};
  ///
  /// fn was_read(entry: &SourceEntry) -> bool {
  ///     matches!(entry.outcome(), SourceOutcome::Opened)
  /// }
  ///
  /// assert_eq!(SourceOutcome::LimitReached.as_str(), "limit_reached");
  /// ```
  SourceOutcome {
    /// The source was opened and read.
    Opened => OPENED_NAME,
    /// The resolver found no such source.
    NotFound => NOT_FOUND_NAME,
    /// The resolver declined the request.
    Declined => DECLINED_NAME,
    /// A limit refused the request before it was made.
    LimitReached => LIMIT_REACHED_NAME,
    /// The source failed with an I/O error, which the error keeps (R19).
    Failed => FAILED_NAME,
  }
}

#[cfg(test)]
mod tests {
  use super::SourceOutcome;
  use crate::model::UnrecognizedValue;

  #[test]
  fn names_each_outcome() {
    for (outcome, name) in [
      (SourceOutcome::Opened, "opened"),
      (SourceOutcome::NotFound, "not_found"),
      (SourceOutcome::Declined, "declined"),
      (SourceOutcome::LimitReached, "limit_reached"),
      (SourceOutcome::Failed, "failed"),
      (
        SourceOutcome::Unrecognized(UnrecognizedValue::sample("deferred")),
        "deferred",
      ),
    ] {
      assert_eq!(outcome.as_str(), name);
    }
  }
}
