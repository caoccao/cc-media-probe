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

//! `ErrorKind`, the ending of the ordered procedure (`api.md` sections 5.1 and 10.1).

use super::constants::{
  FEATURE_DISABLED_MESSAGE, FEATURE_DISABLED_NAME, FORMAT_NOT_DETERMINED_MESSAGE, FORMAT_NOT_DETERMINED_NAME,
  NO_STREAM_INVENTORY_MESSAGE, NO_STREAM_INVENTORY_NAME, SOURCE_FAILED_MESSAGE, SOURCE_FAILED_NAME,
  UNKNOWN_FORMAT_MESSAGE, UNKNOWN_FORMAT_NAME, UNSUPPORTED_FORMAT_MESSAGE, UNSUPPORTED_FORMAT_NAME,
};
use crate::string_enumeration::string_enumeration;

string_enumeration! {
  /// Which ending of the ordered procedure an error is (`api.md` section 5.1); the first rule that applies wins.
  ///
  /// ```
  /// use cc_media_probe::{Error, ErrorKind};
  ///
  /// fn is_worth_retrying(error: &Error) -> bool {
  ///     matches!(error.kind(), ErrorKind::SourceFailed)
  /// }
  ///
  /// assert_eq!(ErrorKind::FormatNotDetermined.as_str(), "format_not_determined");
  /// ```
  ErrorKind (
    /// The kind's message for people, or for a kind that a newer version wrote, its serialized name.
    pub(crate) fn message
  ) {
    /// A source failed with an I/O error, which the error keeps as its cause (R19).
    SourceFailed => SOURCE_FAILED_NAME, SOURCE_FAILED_MESSAGE,
    /// Nothing matched, a hint only, or every candidate rejected.
    UnknownFormat => UNKNOWN_FORMAT_NAME, UNKNOWN_FORMAT_MESSAGE,
    /// Too few bytes to decide, or no candidate validated and at least one could not be checked.
    FormatNotDetermined => FORMAT_NOT_DETERMINED_NAME, FORMAT_NOT_DETERMINED_MESSAGE,
    /// A detect-only format, or a family whose walker depends on the specific type (R20).
    UnsupportedFormat => UNSUPPORTED_FORMAT_NAME, UNSUPPORTED_FORMAT_MESSAGE,
    /// All stream information lies behind a disabled feature.
    FeatureDisabled => FEATURE_DISABLED_NAME, FEATURE_DISABLED_MESSAGE,
    /// The probe stopped before the stream inventory.
    NoStreamInventory => NO_STREAM_INVENTORY_NAME, NO_STREAM_INVENTORY_MESSAGE,
  }
}

#[cfg(test)]
mod tests {
  use super::ErrorKind;
  use crate::model::UnrecognizedValue;

  #[test]
  fn names_each_kind_and_its_message() {
    for (kind, name, message) in [
      (ErrorKind::SourceFailed, "source_failed", "the source failed"),
      (ErrorKind::UnknownFormat, "unknown_format", "the format is unknown"),
      (
        ErrorKind::FormatNotDetermined,
        "format_not_determined",
        "the format could not be determined",
      ),
      (
        ErrorKind::UnsupportedFormat,
        "unsupported_format",
        "the format is not supported",
      ),
      (
        ErrorKind::FeatureDisabled,
        "feature_disabled",
        "the stream information needs a disabled feature",
      ),
      (
        ErrorKind::NoStreamInventory,
        "no_stream_inventory",
        "the probe stopped before the stream inventory",
      ),
      (
        ErrorKind::Unrecognized(UnrecognizedValue::sample("interrupted")),
        "interrupted",
        "interrupted",
      ),
    ] {
      assert_eq!((kind.as_str(), kind.message()), (name, message));
    }
  }
}
