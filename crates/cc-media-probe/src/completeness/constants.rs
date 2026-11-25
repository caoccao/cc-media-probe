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

//! The serialized names and the texts for people of completeness (`json-output.md` section 6.1).

/// The serialized name of a complete status, and the display of a complete report.
pub const COMPLETE_NAME: &str = "complete";
/// The serialized name of an incomplete status, and the start of the display of an incomplete report.
pub const INCOMPLETE_NAME: &str = "incomplete";
/// The serialized name of `SectionStatus::NotRequested`.
pub const NOT_REQUESTED_NAME: &str = "not_requested";
/// The serialized name of the statuses `NotApplicable`.
pub const NOT_APPLICABLE_NAME: &str = "not_applicable";
/// The serialized name of `TimingStatus::Established`.
pub const ESTABLISHED_NAME: &str = "established";
/// The serialized name of `TimingStatus::NotAvailableByDesign`.
pub const NOT_AVAILABLE_BY_DESIGN_NAME: &str = "not_available_by_design";
/// The serialized name of the statuses `CutShort`.
pub const CUT_SHORT_NAME: &str = "cut_short";
/// The serialized name of `CompanionStatus::NotEnabled`.
pub const NOT_ENABLED_NAME: &str = "not_enabled";

/// The serialized name of `Part::Detection`.
pub const DETECTION_NAME: &str = "detection";
/// The serialized name of `Part::Discovery`.
pub const DISCOVERY_NAME: &str = "discovery";
/// The serialized name of `Part::Headers`.
pub const HEADERS_NAME: &str = "headers";
/// The serialized name of `Part::Tags`.
pub const TAGS_NAME: &str = "tags";
/// The serialized name of `Part::Chapters`.
pub const CHAPTERS_NAME: &str = "chapters";
/// The serialized name of `Part::Attachments`.
pub const ATTACHMENTS_NAME: &str = "attachments";
/// The serialized name of `Part::Timing`.
pub const TIMING_NAME: &str = "timing";
/// The serialized name of `Part::Companions`.
pub const COMPANIONS_NAME: &str = "companions";

/// The serialized name of `Feature::Zlib`.
pub const ZLIB_NAME: &str = "zlib";

/// The serialized name of `ReasonKind::RangeUnavailable`.
pub const RANGE_UNAVAILABLE_NAME: &str = "range_unavailable";
/// The serialized name of `ReasonKind::TruncatedInput`.
pub const TRUNCATED_INPUT_NAME: &str = "truncated_input";
/// The serialized name of `ReasonKind::LimitExhausted`.
pub const LIMIT_EXHAUSTED_NAME: &str = "limit_exhausted";
/// The serialized name of `ReasonKind::EndOfInputNotObserved`.
pub const END_OF_INPUT_NOT_OBSERVED_NAME: &str = "end_of_input_not_observed";
/// The serialized name of `ReasonKind::FeatureDisabled`.
pub const FEATURE_DISABLED_NAME: &str = "feature_disabled";
/// The serialized name of `ReasonKind::CompanionsNotEnabled`.
pub const COMPANIONS_NOT_ENABLED_NAME: &str = "companions_not_enabled";
/// The serialized name of `ReasonKind::CompanionNotFound`.
pub const COMPANION_NOT_FOUND_NAME: &str = "companion_not_found";
/// The serialized name of `ReasonKind::CompanionDeclined`.
pub const COMPANION_DECLINED_NAME: &str = "companion_declined";
/// The serialized name of `ReasonKind::CompanionLimited`.
pub const COMPANION_LIMITED_NAME: &str = "companion_limited";
/// The serialized name of `ReasonKind::CompanionNameUnavailable`.
pub const COMPANION_NAME_UNAVAILABLE_NAME: &str = "companion_name_unavailable";
/// The serialized name of `ReasonKind::UnsupportedStructure`.
pub const UNSUPPORTED_STRUCTURE_NAME: &str = "unsupported_structure";
/// The serialized name of `ReasonKind::DamagedStructure`.
pub const DAMAGED_STRUCTURE_NAME: &str = "damaged_structure";
/// The serialized name of `ReasonKind::FurtherSegmentsNotExamined`.
pub const FURTHER_SEGMENTS_NOT_EXAMINED_NAME: &str = "further_segments_not_examined";
/// The serialized name of `ReasonKind::MediaSegmentWithoutInitializationData`.
pub const MEDIA_SEGMENT_WITHOUT_INITIALIZATION_DATA_NAME: &str = "media_segment_without_initialization_data";
/// The serialized name of `ReasonKind::SourceFailed`.
pub const SOURCE_FAILED_NAME: &str = "source_failed";

/// The text for people of `ReasonKind::RangeUnavailable`.
pub const RANGE_UNAVAILABLE_TEXT: &str = "bytes outside the readable range";
/// The text for people of `ReasonKind::TruncatedInput`.
pub const TRUNCATED_INPUT_TEXT: &str = "truncated input";
/// The text for people of `ReasonKind::LimitExhausted` without its limit.
pub const LIMIT_EXHAUSTED_TEXT: &str = "an exhausted limit";
/// The text for people of `ReasonKind::EndOfInputNotObserved`.
pub const END_OF_INPUT_NOT_OBSERVED_TEXT: &str = "an end of input that was not observed";
/// The text for people of `ReasonKind::FeatureDisabled`.
pub const FEATURE_DISABLED_TEXT: &str = "a disabled feature";
/// The text for people of `ReasonKind::CompanionsNotEnabled`.
pub const COMPANIONS_NOT_ENABLED_TEXT: &str = "companion lookup, which is off";
/// The text for people of `ReasonKind::CompanionNotFound`.
pub const COMPANION_NOT_FOUND_TEXT: &str = "a companion that was not found";
/// The text for people of `ReasonKind::CompanionDeclined`.
pub const COMPANION_DECLINED_TEXT: &str = "a companion that the resolver declined";
/// The text for people of `ReasonKind::CompanionLimited` without its limit.
pub const COMPANION_LIMITED_TEXT: &str = "a companion that a limit refused";
/// The text for people of `ReasonKind::CompanionNameUnavailable`.
pub const COMPANION_NAME_UNAVAILABLE_TEXT: &str = "a companion name that could not be built";
/// The text for people of `ReasonKind::UnsupportedStructure`.
pub const UNSUPPORTED_STRUCTURE_TEXT: &str = "a structure that this version does not support";
/// The text for people of `ReasonKind::DamagedStructure`.
pub const DAMAGED_STRUCTURE_TEXT: &str = "a damaged structure";
/// The text for people of `ReasonKind::FurtherSegmentsNotExamined`.
pub const FURTHER_SEGMENTS_NOT_EXAMINED_TEXT: &str = "further segments that were not examined";
/// The text for people of `ReasonKind::MediaSegmentWithoutInitializationData`.
pub const MEDIA_SEGMENT_WITHOUT_INITIALIZATION_DATA_TEXT: &str = "a media segment without its initialization data";
/// The text for people of `ReasonKind::SourceFailed`.
pub const SOURCE_FAILED_TEXT: &str = "a source that failed";

/// The words between a part and its reason in the display of an incomplete report.
pub const STOPPED_BY_TEXT: &str = "stopped by";
/// The article before a limit in the display of an incomplete report.
pub const DEFINITE_ARTICLE: &str = "the";

/// The serialized name of `HeaderState::Parsed`.
pub const PARSED_NAME: &str = "parsed";
/// The serialized name of `HeaderState::PartiallyParsedByDesign`.
pub const PARTIALLY_PARSED_BY_DESIGN_NAME: &str = "partially_parsed_by_design";
/// The serialized name of `HeaderState::Identified`.
pub const IDENTIFIED_NAME: &str = "identified";
/// The serialized name of `HeaderState::Protected`.
pub const PROTECTED_NAME: &str = "protected";
/// The serialized name of `HeaderState::NotFoundWithinBudget`.
pub const NOT_FOUND_WITHIN_BUDGET_NAME: &str = "not_found_within_budget";
/// The serialized name of `HeaderState::PartiallyParsedForLackOfBytes`.
pub const PARTIALLY_PARSED_FOR_LACK_OF_BYTES_NAME: &str = "partially_parsed_for_lack_of_bytes";
/// The serialized name of `HeaderState::Malformed`.
pub const MALFORMED_NAME: &str = "malformed";
/// The serialized name of `HeaderState::CompressionUnavailable`.
pub const COMPRESSION_UNAVAILABLE_NAME: &str = "compression_unavailable";
/// The serialized name of `HeaderState::NotAttempted`.
pub const NOT_ATTEMPTED_NAME: &str = "not_attempted";

/// The text for people of `HeaderState::PartiallyParsedByDesign`.
pub const PARTIALLY_PARSED_BY_DESIGN_TEXT: &str = "partially parsed by design";
/// The text for people of `HeaderState::NotFoundWithinBudget`.
pub const NOT_FOUND_WITHIN_BUDGET_TEXT: &str = "not found within the budget";
/// The text for people of `HeaderState::PartiallyParsedForLackOfBytes`.
pub const PARTIALLY_PARSED_FOR_LACK_OF_BYTES_TEXT: &str = "partially parsed for lack of bytes";
/// The text for people of `HeaderState::CompressionUnavailable`.
pub const COMPRESSION_UNAVAILABLE_TEXT: &str = "compression unavailable";
/// The text for people of `HeaderState::NotAttempted`.
pub const NOT_ATTEMPTED_TEXT: &str = "not attempted";

/// The serialized name of `Identification::IdentificationOnly`.
pub const IDENTIFICATION_ONLY_NAME: &str = "identification_only";
/// The serialized name of `Identification::UnknownCodec`.
pub const UNKNOWN_CODEC_NAME: &str = "unknown_codec";
/// The serialized name of `Identification::PendingParser`.
pub const PENDING_PARSER_NAME: &str = "pending_parser";
/// The serialized name of `Identification::UnsupportedCompression`.
pub const UNSUPPORTED_COMPRESSION_NAME: &str = "unsupported_compression";

/// The text for people of `Identification::IdentificationOnly`.
pub const IDENTIFICATION_ONLY_TEXT: &str = "identification only";
/// The text for people of `Identification::UnknownCodec`.
pub const UNKNOWN_CODEC_TEXT: &str = "unknown codec";
/// The text for people of `Identification::PendingParser`.
pub const PENDING_PARSER_TEXT: &str = "parser pending";
/// The text for people of `Identification::UnsupportedCompression`.
pub const UNSUPPORTED_COMPRESSION_TEXT: &str = "unsupported compression";
