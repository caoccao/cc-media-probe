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

//! The serialized names and the messages of the errors (`json-output.md` section 6.1; `cli.md` section 8.6).

/// The serialized name of `ErrorKind::SourceFailed`.
pub const SOURCE_FAILED_NAME: &str = "source_failed";
/// The serialized name of `ErrorKind::UnknownFormat`.
pub const UNKNOWN_FORMAT_NAME: &str = "unknown_format";
/// The serialized name of `ErrorKind::FormatNotDetermined`.
pub const FORMAT_NOT_DETERMINED_NAME: &str = "format_not_determined";
/// The serialized name of `ErrorKind::UnsupportedFormat`.
pub const UNSUPPORTED_FORMAT_NAME: &str = "unsupported_format";
/// The serialized name of `ErrorKind::FeatureDisabled`.
pub const FEATURE_DISABLED_NAME: &str = "feature_disabled";
/// The serialized name of `ErrorKind::NoStreamInventory`.
pub const NO_STREAM_INVENTORY_NAME: &str = "no_stream_inventory";

/// The message of `ErrorKind::SourceFailed`.
pub const SOURCE_FAILED_MESSAGE: &str = "the source failed";
/// The message of `ErrorKind::UnknownFormat`.
pub const UNKNOWN_FORMAT_MESSAGE: &str = "the format is unknown";
/// The message of `ErrorKind::FormatNotDetermined`.
pub const FORMAT_NOT_DETERMINED_MESSAGE: &str = "the format could not be determined";
/// The message of `ErrorKind::UnsupportedFormat`.
pub const UNSUPPORTED_FORMAT_MESSAGE: &str = "the format is not supported";
/// The message of `ErrorKind::FeatureDisabled`.
pub const FEATURE_DISABLED_MESSAGE: &str = "the stream information needs a disabled feature";
/// The message of `ErrorKind::NoStreamInventory`.
pub const NO_STREAM_INVENTORY_MESSAGE: &str = "the probe stopped before the stream inventory";
/// The words before the byte at which a source failed.
pub const AT_BYTE_TEXT: &str = "at byte";
/// The message of a source that failed while it was being opened, before any read (`cli.md` section 8.6).
pub const NOT_OPENED_MESSAGE: &str = "the source could not be opened";
/// The most causes of an error's chain that its cause messages follow, so that a cyclic chain ends (`api.md`
/// section 10.2).
pub const MAXIMUM_CAUSES: usize = 32;

/// The serialized name of `CheckState::Validated`.
pub const VALIDATED_NAME: &str = "validated";
/// The serialized name of `CheckState::Rejected`.
pub const REJECTED_NAME: &str = "rejected";
/// The serialized name of `CheckState::NotCheckable`.
pub const NOT_CHECKABLE_NAME: &str = "not_checkable";
/// The serialized name of `CheckState::Outranked`.
pub const OUTRANKED_NAME: &str = "outranked";
