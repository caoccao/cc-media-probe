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
/// The proposed default of the retained-data limit.
pub const DEFAULT_RETAINED_DATA: u64 = 16 * 1024 * 1024;
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
/// The serialized names of `Part`.
pub const ATTACHMENTS_NAME: &str = "attachments";
/// The serialized name of `Feature::Zlib`.
pub const ZLIB_NAME: &str = "zlib";

/// The serialized names of `Limit`.
pub const BYTES_READ_NAME: &str = "bytes_read";
/// The serialized names of `Limit`.
pub const WORKING_MEMORY_NAME: &str = "working_memory";
/// The serialized names of `Limit`.
pub const RETAINED_DATA_NAME: &str = "retained_data";
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
pub const SCHEMA_VERSION_TEXT_MESSAGE: &str = "a schema version is two decimal numbers separated by a period";
/// The messages of a document that breaks the rules of `api.md` section 12.
#[cfg(feature = "serde")]
pub const LOSSY_PATH_MESSAGE: &str = "a raw form stands only beside a lossy path's flag, and both only beside a display \
                                      string";
/// The messages of a document that breaks the rules of `api.md` section 12.
#[cfg(feature = "serde")]
pub const LOSSY_DISPLAY_MESSAGE: &str = "a lossy path's display string holds U+FFFD";
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
pub const TREE_ORDER_MESSAGE: &str =
  "a tree's flat form lists its nodes in depth-first order, each after its parent and within its parent's subtree";
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

/// The messages of a document that breaks the rules of `api.md` section 12.
#[cfg(feature = "serde")]
pub const DECIMAL_MESSAGE: &str = "a count or source identifier that is always a string is canonical decimal: \
                                   digits with no sign and no leading zero";
/// The messages of a document that breaks the rules of `api.md` section 12.
#[cfg(feature = "serde")]
pub const DUPLICATE_MEMBER_MESSAGE: &str = "a map names each member once";
/// The messages of a document that breaks the rules of `api.md` section 12.
#[cfg(feature = "serde")]
pub const KEPT_VALUE_EXPECTING: &str = "any value but null, which the form never writes";
/// The messages of a document that breaks the rules of `api.md` section 12.
#[cfg(feature = "serde")]
pub const SUBTREE_MESSAGE: &str = "a standalone node is the flat form of its subtree, with exactly one root";
/// The messages of a document that breaks the rules of `api.md` section 12.
#[cfg(feature = "serde")]
pub const SIZE_MESSAGE: &str = "a size's width and height are each at most 2^53 - 1";

/// The messages of a document that breaks the rules of `api.md` section 12.
#[cfg(feature = "serde")]
pub const CHROMA_FORMAT_MESSAGE: &str = "a chroma format of a known kind holds the member named after its kind when \
                                         it has a value, and no member of another known kind";
/// The messages of a document that breaks the rules of `api.md` section 12.
#[cfg(feature = "serde")]
pub const HEADERS_DISAGREE_MESSAGE: &str = "a report's headers status disagrees with the header states of its streams";
/// A report whose reasons, first reasons and statuses disagree part by part (twelfth review).
#[cfg(feature = "serde")]
pub const REASONS_DISAGREE_MESSAGE: &str = "the reasons of a completeness disagree with its statuses or its record";

/// The members of an enum that carries data: the kind, which names the member that holds the value.
#[cfg(feature = "serde")]
pub const KIND_MEMBER: &str = "kind";
/// The member of an entry and of a detection that holds its outcome, which decides whether the members that the
/// record does not read are kept (eighteenth review).
#[cfg(feature = "serde")]
pub const OUTCOME_MEMBER: &str = "outcome";
/// The member of a node of a flat tree that holds the position of its parent (`api.md` section 12).
#[cfg(feature = "serde")]
pub const PARENT_MEMBER: &str = "parent";

/// The largest magnitude that every JSON consumer reads exactly, 2^53 - 1 (R24, RFC 7493 section 2.2).
pub const LARGEST_EXACT: u64 = 9_007_199_254_740_991;

/// Why a document written as it goes stopped: an index beyond the exact range.
#[cfg(feature = "serde")]
pub const POSITION_RANGE_MESSAGE: &str = "a document's last index is 2^53 - 1";

/// Why an entry cannot be built from a record alone (`api.md` section 12).
#[cfg(feature = "serde")]
pub const DERIVE_OUTCOME_MESSAGE: &str =
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

/// The version-first reading of a root record (plan section 8.8, task 5): a root record without its version.
#[cfg(feature = "serde")]
pub const MISSING_VERSION_MESSAGE: &str = "a root record has no schema_version";
/// The nineteenth review: a number that is not finite, named in the error of a root that is that number.
#[cfg(feature = "serde")]
pub const NUMBER_UNEXPECTED: &str = "a number";
/// The version-first reading of a root record: what the version error says after the version it found.
#[cfg(feature = "serde")]
pub const READER_VERSION_LABEL: &str = "this version reads";
/// The version-first reading of a root record: a member that holds null, at any depth.
#[cfg(feature = "serde")]
pub const NULL_MEMBER_MESSAGE: &str = "a member holds null, which the form never writes";
/// The version-first reading of a root record: what its reader expects.
#[cfg(feature = "serde")]
pub const ROOT_RECORD_EXPECTING: &str = "a root record, a map with its schema_version";
/// The version-first reading of a root record: what a member held before the version may be.
#[cfg(feature = "serde")]
pub const HELD_VALUE_EXPECTING: &str = "any value, held until the version is read";

/// The thirteenth review: a nested root record whose version is not that of the record that holds it, which the
/// reader compares before it decodes the nested record's body (`json-output.md` section 4.5).
#[cfg(feature = "serde")]
pub const NESTED_ROOT_VERSION_MESSAGE: &str =
  "a nested root record has a schema version other than that of the record that holds it";
/// The thirteenth review: what the nested version error says after the version it found.
#[cfg(feature = "serde")]
pub const HOLDER_VERSION_LABEL: &str = "the record that holds it has";
/// The thirteenth review: a record written as a positional array, which the form never writes.
#[cfg(feature = "serde")]
pub const POSITIONAL_RECORD_MESSAGE: &str = "a record is written as an object of named members, never as an array";
/// The thirteenth review: binary data whose length is not the size it states, or whose text is not hexadecimal.
#[cfg(feature = "serde")]
pub const BINARY_DATA_MESSAGE: &str =
  "binary data is whole bytes of lowercase hexadecimal digits, exactly as many as its size";
/// The thirteenth review: a raw codec identifier not in the spelling of its namespace.
#[cfg(feature = "serde")]
pub const RAW_CODE_MESSAGE: &str = "a raw codec identifier is not in the spelling of its namespace";
/// The thirteenth review: a fixed-size record that names a limit or a part twice in one array, or whose
/// first-stopped pairs do not name exactly its exhausted limits, in their order.
#[cfg(feature = "serde")]
pub const LIMIT_RECORD_MESSAGE: &str = "the record names each limit and part once in each array, and the first-stopped \
                                        pairs name exactly the exhausted limits, in their order";
/// The thirteenth review: an integer identifier that is not canonical decimal within its width.
#[cfg(feature = "serde")]
pub const IDENTIFIER_MESSAGE: &str = "an integer identifier is canonical decimal within its width";
/// The thirteenth review: the hexadecimal digits of one byte, and their base (`json-output.md` section 5.6).
#[cfg(feature = "serde")]
pub const BYTE_HEXADECIMAL_DIGITS: usize = 2;
/// The thirteenth review: the base of hexadecimal digits.
#[cfg(feature = "serde")]
pub const HEXADECIMAL_BASE: u32 = 16;
/// The thirteenth review: the bytes of a four-character code, such as an ISO sample entry's.
#[cfg(feature = "serde")]
pub const FOUR_CHARACTER_CODE_BYTES: usize = 4;
/// The fourteenth review: the reserved name of the newtype request with which a record of the form asks the
/// deserializer for the scope of the read. Only the library's strict deserializer answers it; a format gives the
/// plain newtype.
#[cfg(feature = "serde")]
pub const READ_SCOPE_NAME: &str = "$cc_media_probe::read_scope";
/// The fourteenth review: what a request for the scope expects.
#[cfg(feature = "serde")]
pub const READ_SCOPE_EXPECTING: &str = "a record of the form";
/// The fourteenth review: an answer to the request for the scope that lacks one of its two elements.
#[cfg(feature = "serde")]
pub const READ_SCOPE_MESSAGE: &str = "the scope of a read was answered without its version and its reader";
/// The nineteenth review: the name of the strict deserializer's answer to a request for the scope, an enum's variant
/// whose identifier is these bytes. Only their address counts: the answer is the strict deserializer's when the
/// identifier is this static itself, which no input and no other deserializer can give, while a sequence or a map
/// of the same content, such as one that Serde's `SeqDeserializer` gives, is a value read alone.
#[cfg(feature = "serde")]
pub static READ_SCOPE_TOKEN: [u8; 27] = *b"$cc_media_probe::read_scope";
/// The nineteenth review: the elements of the variant that answers a request for the scope, the version and the
/// strict deserializer.
#[cfg(feature = "serde")]
pub const READ_SCOPE_LENGTH: usize = 2;
/// The nineteenth review: what the identifier of an enum given in answer to a request for the scope expects.
#[cfg(feature = "serde")]
pub const READ_SCOPE_IDENTIFIER_EXPECTING: &str = "a variant's name";
/// The fourteenth review: a stream whose index is not its position in the report.
#[cfg(feature = "serde")]
pub const STREAM_INDEX_MESSAGE: &str = "a stream's index is its position in the report";
/// The fourteenth review: a normalized quantity beyond 2^53 - 1, which no reader holds exactly.
#[cfg(feature = "serde")]
pub const QUANTITY_MESSAGE: &str = "a quantity is a number of at most 2^53 - 1";
/// The nineteenth review: a value without its provenance, or a provenance without its value.
#[cfg(feature = "serde")]
pub const PROVENANCE_PAIR_MESSAGE: &str = "a value and its provenance are present together";
/// The fourteenth review: an error record with both a partial report and observations, two stopping points.
#[cfg(feature = "serde")]
pub const STOPPING_POINTS_MESSAGE: &str =
  "an error holds a partial report, from after the inventory, or observations, from before it, never both";
/// The fourteenth review: an error record whose record is not that of its partial report.
#[cfg(feature = "serde")]
pub const ERROR_RECORD_MESSAGE: &str = "an error that carries a partial report has that report's record";
/// The fourteenth review: a detection whose members do not fit its outcome, or whose confidences do not hold; since
/// the fifteenth review also candidates that repeat a format.
#[cfg(feature = "serde")]
pub const DETECTION_MESSAGE: &str = "a detection holds the members of its outcome and no other, with confidences from \
                                     0 to 100 that fall with the candidates' rank, which name each format once";
/// The fourteenth review: the largest confidence (`api.md` section 8).
#[cfg(feature = "serde")]
pub const MAXIMUM_CONFIDENCE: u8 = 100;
/// The fourteenth review: the fewest candidates of an ambiguous detection, which has several.
#[cfg(feature = "serde")]
pub const MINIMUM_CANDIDATES: usize = 2;
/// The fourteenth review: a value that JSON cannot carry faithfully, which the form refuses from any format; since the
/// fifteenth review also a member name that is not a string, and since the sixteenth a tagged value and an integer
/// beyond 64 bits, as ciborium gives them.
#[cfg(feature = "serde")]
pub const NOT_JSON_VALUE_MESSAGE: &str = "a value is one that JSON carries as read: no non-finite number, no byte \
                                         string, no tagged value, no integer beyond 64 bits and no member name that \
                                         is not a string";
/// The fourteenth review: the bytes of a GUID, such as the `SubFormat` of an extensible WAVE format.
#[cfg(feature = "serde")]
pub const GUID_BYTES: usize = 16;
/// The fourteenth review: the first and the last printable ASCII character, the text of an EBML string (RFC 8794
/// section 7.4).
#[cfg(feature = "serde")]
pub const PRINTABLE_FIRST: char = ' ';
/// The fourteenth review: the last printable ASCII character.
#[cfg(feature = "serde")]
pub const PRINTABLE_LAST: char = '~';
/// The fifteenth review: the name of the one member of the map that serde_json's `arbitrary_precision` feature gives
/// for a number that is not an integer within 64 bits, whose value is the number's text.
#[cfg(feature = "serde")]
pub const ARBITRARY_PRECISION_NAME: &str = "$serde_json::private::Number";
/// The fifteenth review: the decimal point of a JSON number (RFC 8259 section 6).
#[cfg(feature = "serde")]
pub const DECIMAL_POINT: char = '.';
/// The fifteenth review: the two letters that start the exponent of a JSON number (RFC 8259 section 6).
#[cfg(feature = "serde")]
pub const EXPONENT_MARKERS: [char; 2] = ['e', 'E'];
/// The fifteenth review: the plus sign of a JSON exponent (RFC 8259 section 6).
#[cfg(feature = "serde")]
pub const PLUS_SIGN: char = '+';
/// The fifteenth review: the digit that no integer part of more than one digit starts with (RFC 8259 section 6).
#[cfg(feature = "serde")]
pub const ZERO_DIGIT: char = '0';
/// The fifteenth review: negative zero, which a reader without the feature reads as a floating-point number.
#[cfg(feature = "serde")]
pub const NEGATIVE_ZERO: &str = "-0";
/// The fifteenth review: a confidence beyond 100, which a candidate read alone refuses as a detection does.
#[cfg(feature = "serde")]
pub const CONFIDENCE_MESSAGE: &str = "a confidence lies from 0 to 100";
/// The fifteenth review: a reason of a known kind without the members of its kind, or with those of another.
#[cfg(feature = "serde")]
pub const REASON_MESSAGE: &str = "a reason of a known kind holds the members of its kind and no other";
/// The fifteenth review: a partial report beside an error kind other than a source failure.
#[cfg(feature = "serde")]
pub const PARTIAL_REPORT_KIND_MESSAGE: &str = "only a source failure carries a partial report";
/// The fifteenth review: an error's limit reason whose limit the record does not hold as exhausted.
#[cfg(feature = "serde")]
pub const REASON_COUNTER_MESSAGE: &str = "an error's limit reason names a limit that its record holds as exhausted";
/// The sixteenth review: an error record whose members of a source failure disagree with its kind.
#[cfg(feature = "serde")]
pub const SOURCE_FAILURE_MEMBERS_MESSAGE: &str = "a source failure holds the kind of its failure, and no other known error kind holds it or an operating system's \
   error code";
/// The second review of the skeleton's first slice: an operating system's error code that is not a 64-bit integer,
/// signed or unsigned, in canonical decimal.
#[cfg(feature = "serde")]
pub const OPERATING_SYSTEM_CODE_MESSAGE: &str = "an operating system's error code is a 64-bit integer, signed or \
                                                 unsigned, in canonical decimal";
/// The seventeenth review: a member of an object of a known type with the name of serde_json's private map, which
/// the Serde data model cannot tell from a number of arbitrary precision (`json-output.md` section 2).
#[cfg(feature = "serde")]
pub const RESERVED_NAME_MESSAGE: &str = "no object of a known type has a member named `$serde_json::private::Number`, \
                                         the name that serde_json gives a number of arbitrary precision";
/// The seventeenth review: an enum written as an object of other than one member.
#[cfg(feature = "serde")]
pub const ENUM_MEMBER_MESSAGE: &str = "an enum written as an object has one member, named after its variant";
/// The seventeenth review: the name by which a variant's record is requested, which `Strict` does not read.
#[cfg(feature = "serde")]
pub const VARIANT_RECORD_NAME: &str = "variant record";
/// The seventeenth review: a limit record that counts an omitted reason without a first reason for it to belong to.
#[cfg(feature = "serde")]
pub const OMITTED_REASONS_MESSAGE: &str = "a limit record that counts an omitted reason has a first reason";
/// The eighteenth review: a limit record that counts an omitted reference without the retained data exhausted, or
/// without a first reason of a part that finds references (`api.md` section 6.12).
#[cfg(feature = "serde")]
pub const OMITTED_REFERENCES_MESSAGE: &str = "a limit record that counts an omitted reference has exhausted the \
                                              retained data and has the first reason of a part that finds references";
/// The eighteenth review: what a record whose outcome decides whether its unknown members are kept is read from.
#[cfg(feature = "serde")]
pub const KEPT_OUTCOME_EXPECTING: &str = "a record of the form whose outcome decides what it keeps";
/// The eighteenth review: the name by which such a record is requested, which `Strict` does not read.
#[cfg(feature = "serde")]
pub const KEPT_OUTCOME_RECORD: &str = "record of an outcome";
