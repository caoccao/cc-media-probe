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

//! The constants of the stub's root modules.

/// The bytes a stub probe reads before it stops.
pub const PREFIX_LENGTH: usize = 16;

/// The proposed default of the bytes-read limit (plan section 7.5).
pub const DEFAULT_BYTES_READ: u64 = 64 * 1024 * 1024;
/// The proposed default of the working-memory limit.
pub const DEFAULT_WORKING_MEMORY: u64 = 32 * 1024 * 1024;
/// The proposed default of the stream limit.
pub const DEFAULT_STREAMS: u64 = 4096;

/// The display of a complete report.
pub const COMPLETE_TEXT: &str = "complete";
/// The display of an incomplete report.
pub const INCOMPLETE_TEXT: &str = "incomplete";

/// The serialized names of `SectionStatus` beyond the two above.
pub const NOT_REQUESTED_NAME: &str = "not_requested";
/// The serialized names of `SectionStatus` beyond the two above.
pub const NOT_APPLICABLE_NAME: &str = "not_applicable";

/// The serialized names of `ReasonKind`.
pub const TRUNCATED_INPUT_NAME: &str = "truncated_input";
/// The serialized names of `ReasonKind`.
pub const LIMIT_EXHAUSTED_NAME: &str = "limit_exhausted";

/// The serialized names of `HeaderState`.
pub const PARSED_NAME: &str = "parsed";
/// The serialized names of `HeaderState`.
pub const NOT_FOUND_WITHIN_BUDGET_NAME: &str = "not_found_within_budget";
/// The serialized names of `HeaderState`.
pub const NOT_ATTEMPTED_NAME: &str = "not_attempted";

/// The serialized names of `ErrorKind`.
pub const SOURCE_FAILED_NAME: &str = "source_failed";
/// The serialized names of `ErrorKind`.
pub const UNKNOWN_FORMAT_NAME: &str = "unknown_format";
/// The serialized names of `ErrorKind`.
pub const FORMAT_NOT_DETERMINED_NAME: &str = "format_not_determined";
/// The serialized names of `ErrorKind`.
pub const UNSUPPORTED_FORMAT_NAME: &str = "unsupported_format";
/// The serialized names of `ErrorKind`.
pub const FEATURE_DISABLED_NAME: &str = "feature_disabled";
/// The serialized names of `ErrorKind`.
pub const NO_STREAM_INVENTORY_NAME: &str = "no_stream_inventory";

/// The messages of `Error`, by kind.
pub const SOURCE_FAILED_MESSAGE: &str = "the source failed";
/// The messages of `Error`, by kind.
pub const UNKNOWN_FORMAT_MESSAGE: &str = "unknown format";
/// The messages of `Error`, by kind.
pub const FORMAT_NOT_DETERMINED_MESSAGE: &str = "the format could not be determined";
/// The messages of `Error`, by kind.
pub const UNSUPPORTED_FORMAT_MESSAGE: &str = "unsupported format";
/// The messages of `Error`, by kind.
pub const FEATURE_DISABLED_MESSAGE: &str = "the stream information needs a disabled feature";
/// The messages of `Error`, by kind.
pub const NO_STREAM_INVENTORY_MESSAGE: &str = "the probe stopped before the stream inventory";

/// The schema version that the stub writes.
#[cfg(feature = "serde")]
pub const SCHEMA_MAJOR: u32 = 1;
/// The schema version that the stub writes.
#[cfg(feature = "serde")]
pub const SCHEMA_MINOR: u32 = 0;

/// The serialized names of `EntryOutcome`.
#[cfg(feature = "serde")]
pub const OK_NAME: &str = "ok";
/// The serialized names of `EntryOutcome`.
#[cfg(feature = "serde")]
pub const PARTIAL_NAME: &str = "partial";
/// The serialized names of `EntryOutcome`.
#[cfg(feature = "serde")]
pub const ERROR_NAME: &str = "error";

/// The messages of a document that breaks the rules of `api.md` section 12.
#[cfg(feature = "serde")]
pub const USAGE_ERROR_WITH_RESULTS_MESSAGE: &str = "a document with a usage error holds no results";
/// The messages of a document that breaks the rules of `api.md` section 12.
#[cfg(feature = "serde")]
pub const ENTRY_WITHOUT_ONE_RECORD_MESSAGE: &str =
  "an entry of a known outcome holds exactly one of a report, a detection and an error record";
