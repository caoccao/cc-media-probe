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

/// The bytes of an ISO `scheme_type` (`json-output.md` section 7.3).
#[cfg(feature = "serde")]
pub const SCHEME_TYPE_BYTES: usize = 4;

/// The message of a protection scheme whose value is not in the spelling of its namespace.
#[cfg(feature = "serde")]
pub const SCHEME_MESSAGE: &str = "a protection scheme's value is not in the spelling of its namespace";

/// The message of a declaration without the members of its scope, or with another scope's (`json-output.md`
/// section 7.3).
#[cfg(feature = "serde")]
pub const DECLARATION_MESSAGE: &str =
  "a protection declaration lacks the members of its scope or holds another scope's";

/// The message of a protection that is not true to itself (`json-output.md` section 7.3).
#[cfg(feature = "serde")]
pub const PROTECTION_MESSAGE: &str = "a protection's sample descriptions disagree with its flag, its count or the \
                                      order of their indexes, or it declares nothing though examined in full";

/// The message of a sample description's entry whose index is 0 or whose state is `unknown` (`json-output.md`
/// section 7.3).
#[cfg(feature = "serde")]
pub const SAMPLE_DESCRIPTION_MESSAGE: &str = "a sample description's entry has the index 0 or the state unknown";
