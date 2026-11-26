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

//! The serialized names of the detection types (`json-output.md` section 6.1).

/// The serialized name of `DetectionOutcome::Identified`.
pub const IDENTIFIED_NAME: &str = "identified";
/// The serialized name of `DetectionOutcome::FamilyOnly`.
pub const FAMILY_ONLY_NAME: &str = "family_only";
/// The serialized name of `DetectionOutcome::Ambiguous`.
pub const AMBIGUOUS_NAME: &str = "ambiguous";
/// The serialized name of `DetectionOutcome::HintOnly`.
pub const HINT_ONLY_NAME: &str = "hint_only";
/// The serialized name of `DetectionOutcome::InsufficientData`.
pub const INSUFFICIENT_DATA_NAME: &str = "insufficient_data";
/// The serialized name of `DetectionOutcome::Unknown`.
pub const UNKNOWN_NAME: &str = "unknown";

/// The serialized name of `HintOrigin::FileName`.
pub const FILE_NAME_NAME: &str = "file_name";
/// The serialized name of `HintOrigin::FormatOption`.
pub const FORMAT_OPTION_NAME: &str = "format_option";

/// The largest confidence (`api.md` section 8).
#[cfg(feature = "serde")]
pub const LARGEST_CONFIDENCE: u8 = 100;

/// The message of a confidence beyond 100 (`json-output.md` section 4.3).
#[cfg(feature = "serde")]
pub const CONFIDENCE_MESSAGE: &str = "a confidence lies beyond 100";

/// The member of a detection that holds its outcome, which decides the members that it keeps (`json-output.md`
/// section 4.3).
#[cfg(feature = "serde")]
pub const OUTCOME_MEMBER: &str = "outcome";

/// The fewest candidates of an ambiguous detection.
#[cfg(feature = "serde")]
pub const FEWEST_AMBIGUOUS_CANDIDATES: usize = 2;

/// The message of a detection whose members do not fit its outcome (`json-output.md` section 4.3).
#[cfg(feature = "serde")]
pub const DETECTION_MESSAGE: &str = "a detection lacks a member of its outcome, holds another outcome's, or ranks \
                                     candidates that name a format twice or rise in confidence";
