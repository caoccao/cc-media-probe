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

//! The serialized names of protection (`json-output.md` sections 6.1 and 7.3).

use super::SampleDescriptionState;

/// The serialized name of `ProtectionScope::Packets`.
pub const PACKETS_NAME: &str = "packets";
/// The serialized name of `ProtectionScope::CodecPrivateData`.
pub const CODEC_PRIVATE_DATA_NAME: &str = "codec_private_data";
/// The serialized name of `ProtectionScope::SampleDescription`.
pub const SAMPLE_DESCRIPTION_NAME: &str = "sample_description";
/// The serialized name of `ProtectionScope::EncodingSettings`.
pub const ENCODING_SETTINGS_NAME: &str = "encoding_settings";
/// The serialized name of `ProtectionScope::Other`.
pub const OTHER_NAME: &str = "other";
/// The serialized name of `ProtectionScope::Unread`.
pub const UNREAD_NAME: &str = "unread";

/// The serialized name of `SampleDescriptionState::Clear`.
pub const CLEAR_NAME: &str = "clear";
/// The serialized name of `SampleDescriptionState::Declared`.
pub const DECLARED_NAME: &str = "declared";
/// The name of `SampleDescriptionState::Unknown`, which `Protection::sample_description_state()` gives for a
/// description that the list does not hold and which no record writes.
pub const UNKNOWN_NAME: &str = "unknown";

/// The serialized name of `ProtectionSchemeNamespace::IsoSchemeType`.
pub const ISO_SCHEME_TYPE_NAME: &str = "iso_scheme_type";
/// The serialized name of `ProtectionSchemeNamespace::MatroskaContentEncryptionAlgorithm`.
pub const MATROSKA_CONTENT_ENCRYPTION_ALGORITHM_NAME: &str = "matroska_content_encryption_algorithm";

/// The state of a description that the list does not hold, which `sample_description_state()` returns by reference.
pub static UNKNOWN_STATE: SampleDescriptionState = SampleDescriptionState::Unknown;
