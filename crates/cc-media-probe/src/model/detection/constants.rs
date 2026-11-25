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
