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

/// The limits in the order of `api.md` section 4.5, which the record keeps (`json-output.md` section 7.2).
#[cfg(feature = "serde")]
pub const LIMIT_ORDER: [crate::Limit; 12] = [
  crate::Limit::BytesRead,
  crate::Limit::WorkingMemory,
  crate::Limit::HeaderBytesPerStream,
  crate::Limit::RetainedData,
  crate::Limit::RetainedPayloads,
  crate::Limit::DecompressedOutput,
  crate::Limit::WorkUnits,
  crate::Limit::NestingDepth,
  crate::Limit::Streams,
  crate::Limit::Diagnostics,
  crate::Limit::CompanionRequests,
  crate::Limit::SourcesOpened,
];

/// The parts in the order of `api.md` section 5.3, which the record keeps.
#[cfg(feature = "serde")]
pub const PART_ORDER: [super::Part; 8] = [
  super::Part::Detection,
  super::Part::Discovery,
  super::Part::Headers,
  super::Part::Tags,
  super::Part::Chapters,
  super::Part::Attachments,
  super::Part::Timing,
  super::Part::Companions,
];

/// The message of a reason of a known kind without the members of its kind, or with another kind's
/// (`json-output.md` section 7.2).
#[cfg(feature = "serde")]
pub const REASON_MESSAGE: &str = "a reason of a known kind lacks the members of its kind or holds another kind's";

/// The message of a record that is not true to itself (`json-output.md` section 7.2).
#[cfg(feature = "serde")]
pub const LIMIT_RECORD_MESSAGE: &str = "the limit record names a limit or a part twice or out of order, a stopped part \
                                        without its first reason or a limit not exhausted, or holds a first reason \
                                        with a detail";

/// The message of a reason that names a limit which the record does not pair with a part that it stopped first
/// (`json-output.md` section 7.2; fifth review of the third slice's first part).
#[cfg(feature = "serde")]
pub const UNSTOPPED_LIMIT_MESSAGE: &str = "a reason names a limit that its record does not say stopped a part";

/// The message of a record that counts an omitted reason without a first reason or the exhausted retained
/// data, which alone keeps a reason from being kept.
#[cfg(feature = "serde")]
pub const OMITTED_REASONS_MESSAGE: &str = "a limit record that counts an omitted reason has no first reason or has not \
                                           exhausted the retained data";

/// The message of a record that counts an omitted reference without the first reason of a part that finds
/// references and the retained data paired with the part that it stopped first (fifth review of the third slice's
/// first part).
#[cfg(feature = "serde")]
pub const OMITTED_REFERENCES_MESSAGE: &str = "a limit record that counts an omitted reference does not say that the \
                                              retained data stopped a part, or a part that finds references";

/// The message of a record that counts an omitted declaration without the first reason of discovery and the stream
/// limit or retained data paired with the part that it stopped first, or an omitted diagnostic without the exhausted
/// diagnostics or retained data (fourth and fifth reviews of the third slice's first part).
#[cfg(feature = "serde")]
pub const OMITTED_STORAGE_MESSAGE: &str = "a limit record counts an omitted declaration or diagnostic without the \
                                           limit that kept it from being stored";

/// The message of a record whose limit that stopped detection is recorded as stopping another part first (fourth
/// review of the third slice's first part).
#[cfg(feature = "serde")]
pub const DETECTION_LIMIT_MESSAGE: &str = "a limit that stopped detection is recorded as stopping another part first";

/// The message of a completeness whose reasons disagree with its statuses or its record.
#[cfg(feature = "serde")]
pub const REASONS_DISAGREE_MESSAGE: &str = "the reasons of a completeness disagree with its statuses or its record";

/// The message of a header status whose identification or reason its state does not allow (`api.md` section 5.4).
#[cfg(feature = "serde")]
pub const HEADER_STATUS_MESSAGE: &str = "a header status holds an identification or a reason that its state does not \
                                         allow";

/// The message of a companion's reason against another part, or of a companion not found or declined without its
/// source (`api.md` sections 6.15 and 13).
#[cfg(feature = "serde")]
pub const COMPANION_REASON_MESSAGE: &str = "a companion's reason stands against another part, or a request's reason \
                                            names no source";

/// The message of a completeness whose companions never ran beside an exhausted companion counter (sixth review of
/// the third slice's first part).
#[cfg(feature = "serde")]
pub const COMPANION_COUNTER_MESSAGE: &str = "a completeness whose companions never ran exhausts a companion counter";

/// The message of a reason that names the diagnostics limit as its cause, which only bounds the diagnostics stored
/// and stops no part (sixth review of the third slice's first part).
#[cfg(feature = "serde")]
pub const DIAGNOSTICS_CAUSE_MESSAGE: &str = "a reason names the diagnostics limit, which stops no part";

/// The message of a record that pairs the diagnostics limit with a part, or holds it as exhausted beside no omitted
/// diagnostic (sixth review of the third slice's first part).
#[cfg(feature = "serde")]
pub const DIAGNOSTICS_LIMIT_MESSAGE: &str = "a limit record pairs the diagnostics limit with a part, or holds it as \
                                             exhausted without an omitted diagnostic";

/// The limits that can refuse a companion before its request, in the order of the checks that a request needs (plan
/// section 7.7): the request limit, the source limit and the retained data that holds the request's entry (`api.md`
/// section 13; seventh review of the third slice's first part).
#[cfg(feature = "serde")]
pub const BEFORE_REQUEST_LIMITS: [crate::Limit; 3] = [
  crate::Limit::CompanionRequests,
  crate::Limit::SourcesOpened,
  crate::Limit::RetainedData,
];

/// The limits that can stop a companion once its request was made: the request limit, the source limit and the read
/// limit, which a resolver's `limit_reached` names, and the working memory that cannot hold the source's identity
/// token (`api.md` section 13; seventh review of the third slice's first part).
#[cfg(feature = "serde")]
pub const AFTER_REQUEST_LIMITS: [crate::Limit; 4] = [
  crate::Limit::CompanionRequests,
  crate::Limit::SourcesOpened,
  crate::Limit::BytesRead,
  crate::Limit::WorkingMemory,
];

/// The message of a `companion_limited` reason whose limit cannot stop a companion where its source places it, before
/// or after its request (`api.md` section 13; seventh review of the third slice's first part).
#[cfg(feature = "serde")]
pub const COMPANION_LIMIT_MESSAGE: &str = "a companion_limited reason names a limit that cannot stop a companion \
                                           before its request, or after it, as its source tells";
