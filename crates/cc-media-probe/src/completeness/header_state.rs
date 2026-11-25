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

//! `HeaderState` and `Identification`, the outcome of header parsing for one stream (`api.md` section 5.4).

use super::constants::{
  COMPRESSION_UNAVAILABLE_NAME, COMPRESSION_UNAVAILABLE_TEXT, IDENTIFICATION_ONLY_NAME, IDENTIFICATION_ONLY_TEXT,
  IDENTIFIED_NAME, MALFORMED_NAME, NOT_ATTEMPTED_NAME, NOT_ATTEMPTED_TEXT, NOT_FOUND_WITHIN_BUDGET_NAME,
  NOT_FOUND_WITHIN_BUDGET_TEXT, PARSED_NAME, PARTIALLY_PARSED_BY_DESIGN_NAME, PARTIALLY_PARSED_BY_DESIGN_TEXT,
  PARTIALLY_PARSED_FOR_LACK_OF_BYTES_NAME, PARTIALLY_PARSED_FOR_LACK_OF_BYTES_TEXT, PENDING_PARSER_NAME,
  PENDING_PARSER_TEXT, PROTECTED_NAME, UNKNOWN_CODEC_NAME, UNKNOWN_CODEC_TEXT, UNSUPPORTED_COMPRESSION_NAME,
  UNSUPPORTED_COMPRESSION_TEXT,
};
use crate::string_enumeration::string_enumeration;

string_enumeration! {
  /// The outcome of header parsing for one stream. The first four states are final: the probe did everything
  /// that this version can do. The others are not, and a state that a newer version wrote is not final either.
  ///
  /// ```
  /// use cc_media_probe::Stream;
  /// use cc_media_probe::completeness::HeaderState;
  ///
  /// fn header_is_cut_short(stream: &Stream) -> bool {
  ///     matches!(
  ///         stream.header_status().state(),
  ///         HeaderState::NotFoundWithinBudget | HeaderState::PartiallyParsedForLackOfBytes
  ///     )
  /// }
  ///
  /// assert_eq!(HeaderState::PartiallyParsedByDesign.as_str(), "partially_parsed_by_design");
  /// ```
  HeaderState (
    /// The state's text for people, such as "not found within the budget".
    pub(crate) fn text
  ) {
    /// Every header field was read. Final.
    Parsed => PARSED_NAME, PARSED_NAME,
    /// Some fields lie behind decoding, as implicit spectral-band replication does. Final.
    PartiallyParsedByDesign => PARTIALLY_PARSED_BY_DESIGN_NAME, PARTIALLY_PARSED_BY_DESIGN_TEXT,
    /// The codec was identified and its header not parsed; `HeaderStatus::identification()` says why. Final.
    Identified => IDENTIFIED_NAME, IDENTIFIED_NAME,
    /// The bytes that hold the codec header are encrypted. Final.
    Protected => PROTECTED_NAME, PROTECTED_NAME,
    /// No header field was read before the input, a limit, the reach of a forward-only source or a source failure
    /// ended the search; the reason says which.
    NotFoundWithinBudget => NOT_FOUND_WITHIN_BUDGET_NAME, NOT_FOUND_WITHIN_BUDGET_TEXT,
    /// Some fields were read before the input, a limit, the reach of a forward-only source or a source failure
    /// ended; the reason says which.
    PartiallyParsedForLackOfBytes => PARTIALLY_PARSED_FOR_LACK_OF_BYTES_NAME, PARTIALLY_PARSED_FOR_LACK_OF_BYTES_TEXT,
    /// The bytes contradict the syntax, or a reference that the specification requires to resolve has nothing to
    /// resolve to.
    Malformed => MALFORMED_NAME, MALFORMED_NAME,
    /// The header is compressed with zlib, and the `zlib` feature is off.
    CompressionUnavailable => COMPRESSION_UNAVAILABLE_NAME, COMPRESSION_UNAVAILABLE_TEXT,
    /// The probe stopped before the stream's turn.
    NotAttempted => NOT_ATTEMPTED_NAME, NOT_ATTEMPTED_TEXT,
  }
}

impl HeaderState {
  /// Whether the state is final; a state that a newer version wrote is not.
  pub(crate) fn is_final(&self) -> bool {
    matches!(
      self,
      HeaderState::Parsed | HeaderState::PartiallyParsedByDesign | HeaderState::Identified | HeaderState::Protected
    )
  }
}

string_enumeration! {
  /// Why a stream whose state is `Identified` has no parsed header.
  ///
  /// ```
  /// use cc_media_probe::Stream;
  /// use cc_media_probe::completeness::Identification;
  ///
  /// fn parser_pending(stream: &Stream) -> bool {
  ///     matches!(stream.header_status().identification(), Some(Identification::PendingParser))
  /// }
  ///
  /// assert_eq!(Identification::UnknownCodec.as_str(), "unknown_codec");
  /// ```
  Identification (
    /// The identification's text for people, such as "parser pending".
    pub(crate) fn text
  ) {
    /// The codec is identified by design only; this version parses no header of it.
    IdentificationOnly => IDENTIFICATION_ONLY_NAME, IDENTIFICATION_ONLY_TEXT,
    /// The codec is one that this version does not know.
    UnknownCodec => UNKNOWN_CODEC_NAME, UNKNOWN_CODEC_TEXT,
    /// A later phase adds the parser of this pair of container and codec.
    PendingParser => PENDING_PARSER_NAME, PENDING_PARSER_TEXT,
    /// A compression that this version cannot undo covers the bytes that hold the header.
    UnsupportedCompression => UNSUPPORTED_COMPRESSION_NAME, UNSUPPORTED_COMPRESSION_TEXT,
  }
}

#[cfg(test)]
mod tests {
  use super::{HeaderState, Identification};
  use crate::model::UnrecognizedValue;

  #[test]
  fn names_each_state_and_whether_it_is_final() {
    let states = [
      (HeaderState::Parsed, "parsed", "parsed", true),
      (
        HeaderState::PartiallyParsedByDesign,
        "partially_parsed_by_design",
        "partially parsed by design",
        true,
      ),
      (HeaderState::Identified, "identified", "identified", true),
      (HeaderState::Protected, "protected", "protected", true),
      (
        HeaderState::NotFoundWithinBudget,
        "not_found_within_budget",
        "not found within the budget",
        false,
      ),
      (
        HeaderState::PartiallyParsedForLackOfBytes,
        "partially_parsed_for_lack_of_bytes",
        "partially parsed for lack of bytes",
        false,
      ),
      (HeaderState::Malformed, "malformed", "malformed", false),
      (
        HeaderState::CompressionUnavailable,
        "compression_unavailable",
        "compression unavailable",
        false,
      ),
      (HeaderState::NotAttempted, "not_attempted", "not attempted", false),
      (
        HeaderState::Unrecognized(UnrecognizedValue::sample("deferred")),
        "deferred",
        "deferred",
        false,
      ),
    ];
    for (state, name, text, is_final) in states {
      assert_eq!((state.as_str(), state.text(), state.is_final()), (name, text, is_final));
    }
  }

  #[test]
  fn names_each_identification() {
    let identifications = [
      (
        Identification::IdentificationOnly,
        "identification_only",
        "identification only",
      ),
      (Identification::UnknownCodec, "unknown_codec", "unknown codec"),
      (Identification::PendingParser, "pending_parser", "parser pending"),
      (
        Identification::UnsupportedCompression,
        "unsupported_compression",
        "unsupported compression",
      ),
      (
        Identification::Unrecognized(UnrecognizedValue::sample("licensed")),
        "licensed",
        "licensed",
      ),
    ];
    for (identification, name, text) in identifications {
      assert_eq!((identification.as_str(), identification.text()), (name, text));
    }
  }
}
