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

//! `Limit`, the name of one counter of the budget (`api.md` section 4.5; plan section 7.5).

use super::constants::{
  BYTES_READ_NAME, BYTES_READ_TEXT, COMPANION_REQUESTS_NAME, COMPANION_REQUESTS_TEXT, DECOMPRESSED_OUTPUT_NAME,
  DECOMPRESSED_OUTPUT_TEXT, DIAGNOSTICS_NAME, DIAGNOSTICS_TEXT, HEADER_BYTES_PER_STREAM_NAME,
  HEADER_BYTES_PER_STREAM_TEXT, NESTING_DEPTH_NAME, NESTING_DEPTH_TEXT, RETAINED_DATA_NAME, RETAINED_DATA_TEXT,
  RETAINED_PAYLOADS_NAME, RETAINED_PAYLOADS_TEXT, SOURCES_OPENED_NAME, SOURCES_OPENED_TEXT, STREAMS_NAME, STREAMS_TEXT,
  WORK_UNITS_NAME, WORK_UNITS_TEXT, WORKING_MEMORY_NAME, WORKING_MEMORY_TEXT,
};
use crate::string_enumeration::string_enumeration;

string_enumeration! {
  /// The name of one counter of the budget (plan section 7.5). The same names serve the limits, the usage
  /// summary, the fixed-size record and the reasons, in this order, which is the order that the record keeps.
  ///
  /// ```
  /// use cc_media_probe::Limit;
  /// use cc_media_probe::completeness::LimitRecord;
  ///
  /// fn exhausted(record: &LimitRecord) -> Vec<&'static str> {
  ///     [Limit::BytesRead, Limit::WorkingMemory, Limit::RetainedData]
  ///         .iter()
  ///         .filter(|limit| record.exhausted(limit))
  ///         .map(|limit| match limit {
  ///             Limit::BytesRead => "read",
  ///             Limit::WorkingMemory => "memory",
  ///             _ => "retained data",
  ///         })
  ///         .collect()
  /// }
  ///
  /// assert_eq!(Limit::HeaderBytesPerStream.as_str(), "header_bytes_per_stream");
  /// ```
  Limit (
    /// The limit's name for people, such as "per-stream header limit", or for a limit that a newer version wrote,
    /// its serialized name.
    pub(crate) fn text
  ) {
    /// The bytes read from every source together.
    BytesRead => BYTES_READ_NAME, BYTES_READ_TEXT,
    /// The working memory that the probe's buffers hold at any moment.
    WorkingMemory => WORKING_MEMORY_NAME, WORKING_MEMORY_TEXT,
    /// The bytes held for one header: a stream's codec header, or the image header of one attachment.
    HeaderBytesPerStream => HEADER_BYTES_PER_STREAM_NAME, HEADER_BYTES_PER_STREAM_TEXT,
    /// The data that the report keeps, other than attachment payloads.
    RetainedData => RETAINED_DATA_NAME, RETAINED_DATA_TEXT,
    /// The attachment payloads that the report keeps.
    RetainedPayloads => RETAINED_PAYLOADS_NAME, RETAINED_PAYLOADS_TEXT,
    /// The output of decompression.
    DecompressedOutput => DECOMPRESSED_OUTPUT_NAME, DECOMPRESSED_OUTPUT_TEXT,
    /// The units of work, a bound on the loops over the file's structure.
    WorkUnits => WORK_UNITS_NAME, WORK_UNITS_TEXT,
    /// The depth of nested structures.
    NestingDepth => NESTING_DEPTH_NAME, NESTING_DEPTH_TEXT,
    /// The stream declarations.
    Streams => STREAMS_NAME, STREAMS_TEXT,
    /// The diagnostics that the report keeps.
    Diagnostics => DIAGNOSTICS_NAME, DIAGNOSTICS_TEXT,
    /// The requests for companions.
    CompanionRequests => COMPANION_REQUESTS_NAME, COMPANION_REQUESTS_TEXT,
    /// The sources opened.
    SourcesOpened => SOURCES_OPENED_NAME, SOURCES_OPENED_TEXT,
  }
}

#[cfg(feature = "serde")]
impl Limit {
  /// Whether the limit can stop `part` (`super::constants::LIMIT_SCOPES`): a limit that only one activity charges
  /// stops only the parts of that activity, and every other limit any part. A limit or a part of a newer version is
  /// not checked.
  pub(crate) fn can_stop(&self, part: &crate::completeness::Part) -> bool {
    matches!(part, crate::completeness::Part::Unrecognized(_))
      || super::constants::LIMIT_SCOPES
        .iter()
        .find(|(limit, _)| limit == self)
        .is_none_or(|(_, parts)| parts.contains(part))
  }
}

#[cfg(test)]
mod tests {
  use super::Limit;
  use crate::model::UnrecognizedValue;

  #[test]
  fn names_each_limit() {
    let limits = [
      (Limit::BytesRead, "bytes_read", "read limit"),
      (Limit::WorkingMemory, "working_memory", "working-memory limit"),
      (
        Limit::HeaderBytesPerStream,
        "header_bytes_per_stream",
        "per-stream header limit",
      ),
      (Limit::RetainedData, "retained_data", "retained-data limit"),
      (Limit::RetainedPayloads, "retained_payloads", "retained-payload limit"),
      (Limit::DecompressedOutput, "decompressed_output", "decompression limit"),
      (Limit::WorkUnits, "work_units", "work limit"),
      (Limit::NestingDepth, "nesting_depth", "nesting-depth limit"),
      (Limit::Streams, "streams", "stream limit"),
      (Limit::Diagnostics, "diagnostics", "diagnostics limit"),
      (
        Limit::CompanionRequests,
        "companion_requests",
        "companion-request limit",
      ),
      (Limit::SourcesOpened, "sources_opened", "source limit"),
      (
        Limit::Unrecognized(UnrecognizedValue::sample("threads")),
        "threads",
        "threads",
      ),
    ];
    for (limit, name, text) in limits {
      assert_eq!(limit.as_str(), name);
      assert_eq!(limit.text(), text);
    }
  }
}
