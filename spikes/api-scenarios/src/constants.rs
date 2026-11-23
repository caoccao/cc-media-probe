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

/// The serialized names of `Part`.
pub const DETECTION_NAME: &str = "detection";
/// The serialized names of `Part`.
pub const DISCOVERY_NAME: &str = "discovery";
/// The serialized names of `Part`.
pub const HEADERS_NAME: &str = "headers";
/// The serialized names of `Part`.
pub const TAGS_NAME: &str = "tags";
/// The serialized names of `Part`.
pub const CHAPTERS_NAME: &str = "chapters";

/// The serialized names of `Limit`.
pub const BYTES_READ_NAME: &str = "bytes_read";
/// The serialized names of `Limit`.
pub const WORKING_MEMORY_NAME: &str = "working_memory";
/// The serialized names of `Limit`.
pub const STREAMS_NAME: &str = "streams";

/// The serialized names of `ReasonKind`.
pub const RANGE_UNAVAILABLE_NAME: &str = "range_unavailable";
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

/// Between the words of a stable I/O error kind, as in `permission_denied`.
pub const WORD_SEPARATOR: char = '_';

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
/// The messages of a document that breaks the rules of `api.md` section 12.
#[cfg(feature = "serde")]
pub const OUTCOME_DISAGREES_MESSAGE: &str = "an entry's outcome disagrees with the record it holds";
/// The messages of a document that breaks the rules of `api.md` section 12.
#[cfg(feature = "serde")]
pub const OTHER_MAJOR_VERSION_MESSAGE: &str =
  "a root record has a schema major version that this version does not read";
/// The messages of a document that breaks the rules of `api.md` section 12.
#[cfg(feature = "serde")]
pub const NESTED_VERSION_MESSAGE: &str =
  "a nested record's schema version differs from that of the document or record that holds it";
/// The messages of a document that breaks the rules of `api.md` section 12.
#[cfg(feature = "serde")]
pub const SCHEMA_VERSION_TEXT_MESSAGE: &str = "a schema version is two decimal numbers separated by a period";
/// The messages of a document that breaks the rules of `api.md` section 12.
#[cfg(feature = "serde")]
pub const LOSSY_PATH_MESSAGE: &str = "a lossy path has a raw form beside its display string, and only a lossy path";
/// The messages of a document that breaks the rules of `api.md` section 12.
#[cfg(feature = "serde")]
pub const RAW_PATH_DATA_MESSAGE: &str = "the data of a raw path is lowercase hexadecimal, two digits a byte for \
                                         unix_bytes and four a unit for windows_wide";
/// The messages of a document that breaks the rules of `api.md` section 12.
#[cfg(feature = "serde")]
pub const LOSSY_FLAG_MESSAGE: &str = "a path's lossy flag is written only as true";
/// The messages of a document that breaks the rules of `api.md` section 12.
#[cfg(feature = "serde")]
pub const PATH_DISAGREES_MESSAGE: &str = "a lossy path's raw data is not valid Unicode, and its display string is \
                                          that data decoded with U+FFFD";
/// The messages of a document that breaks the rules of `api.md` section 12.
#[cfg(feature = "serde")]
pub const POSITION_MESSAGE: &str = "an entry's index is its position in the results";
/// The messages of a document that breaks the rules of `api.md` section 12.
#[cfg(feature = "serde")]
pub const TREE_ORDER_MESSAGE: &str = "a node of a tree's flat form comes after its parent";
/// The messages of a document that breaks the rules of `api.md` section 12.
#[cfg(feature = "serde")]
pub const RATIONAL_MESSAGE: &str =
  "a rational is reduced, with a denominator of at least 1 and both parts at most 2^53 - 1 in magnitude";
/// The messages of a document that breaks the rules of `api.md` section 12.
#[cfg(feature = "serde")]
pub const TWIN_MESSAGE: &str = "a quantity is a number within 2^53 - 1, or else its _exact twin, the canonical \
                                decimal text of a larger value, never both";
/// The messages of a document that breaks the rules of `api.md` section 12.
#[cfg(feature = "serde")]
pub const TAG_VALUE_MESSAGE: &str = "a tag value of a known kind holds the member named after its kind, and no \
                                     member of another known kind";

/// The largest magnitude that every JSON consumer reads exactly, 2^53 - 1 (R24, RFC 7493 section 2.2).
pub const LARGEST_EXACT: u64 = 9_007_199_254_740_991;

/// Why a document written as it goes stopped: an index beyond the exact range.
#[cfg(feature = "serde")]
pub const POSITION_RANGE_MESSAGE: &str = "a document's last index is 2^53 - 1";

/// Why an entry cannot be built from a record alone (`api.md` section 12).
#[cfg(feature = "serde")]
pub const UNDERIVABLE_OUTCOME_MESSAGE: &str =
  "the record holds a value of a newer version that decides the entry's outcome, so the outcome must be given";
/// Why the writer's entries can be taken only once.
#[cfg(feature = "serde")]
pub const ENTRIES_TAKEN_MESSAGE: &str = "the entries of a document written as it goes were already written";

/// The members of a document, in the order written.
#[cfg(feature = "serde")]
pub const DOCUMENT_NAME: &str = "Document";
/// The members of a document, in the order written.
#[cfg(feature = "serde")]
pub const SCHEMA_VERSION_MEMBER: &str = "schema_version";
/// The members of a document, in the order written.
#[cfg(feature = "serde")]
pub const GENERATOR_MEMBER: &str = "generator";
/// The members of a document, in the order written.
#[cfg(feature = "serde")]
pub const RESULTS_MEMBER: &str = "results";
/// The members that a document of results writes.
#[cfg(feature = "serde")]
pub const RESULTS_DOCUMENT_MEMBER_COUNT: usize = 3;

/// Between the major and the minor number of a schema version, as in `1.0`.
#[cfg(feature = "serde")]
pub const VERSION_SEPARATOR: char = '.';

/// The sign of a negative quantity's twin.
#[cfg(feature = "serde")]
pub const MINUS_SIGN: char = '-';
