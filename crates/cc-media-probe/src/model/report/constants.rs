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

//! The serialized names of `InventoryClass` (`json-output.md` section 6.1).

/// The serialized name of `InventoryClass::Declared`.
pub const DECLARED_NAME: &str = "declared";
/// The serialized name of `InventoryClass::SingleStream`.
pub const SINGLE_STREAM_NAME: &str = "single_stream";
/// The serialized name of `InventoryClass::Observed`.
pub const OBSERVED_NAME: &str = "observed";

/// The message of a stream whose index is not its position in the report (`json-output.md` section 7.3).
#[cfg(feature = "serde")]
pub const STREAM_INDEX_MESSAGE: &str = "a stream's index is not its position in the report";

/// The message of a headers status that disagrees with the header states of the streams (`json-output.md` section
/// 10).
#[cfg(feature = "serde")]
pub const HEADERS_DISAGREE_MESSAGE: &str = "a report's headers status disagrees with the header states of its streams";

/// The message of a protection whose flag of sample descriptions disagrees with the report's format.
#[cfg(feature = "serde")]
pub const SAMPLE_DESCRIPTIONS_MESSAGE: &str = "a protection's flag of sample descriptions disagrees with the report's \
                                               format";

/// The message of a stream index that names no stream of the report, or a stream embedded in itself
/// (`json-output.md` section 10).
#[cfg(feature = "serde")]
pub const UNKNOWN_STREAM_MESSAGE: &str = "a member names a stream that the report does not hold";

/// The message of a resolved location that names no entity of the report (`json-output.md` section 10).
#[cfg(feature = "serde")]
pub const UNKNOWN_ENTITY_MESSAGE: &str = "a resolved location names an entity that the report does not hold";

/// The message of a shortfall that a record of the report states and its completeness does not record
/// (`json-output.md` section 10).
#[cfg(feature = "serde")]
pub const SHORTFALL_MESSAGE: &str = "a record of the report states a shortfall that its completeness does not record";

/// The message of a section that was not requested and holds results (`api.md` section 4.3).
#[cfg(feature = "serde")]
pub const SKIPPED_SECTION_MESSAGE: &str = "a section that was not requested holds results";

/// The message of a report whose records state more reasons than it keeps, gives first or counts among the omitted
/// reasons (`api.md` sections 5.4 and 6.15).
#[cfg(feature = "serde")]
pub const UNCOUNTED_REASON_MESSAGE: &str = "a reason that a report's records state is neither kept, first nor counted \
                                            among the omitted reasons";
