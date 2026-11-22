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

//! The attachment option of `api.md` section 4.3, and an attachment of section 6.10 as far as the scenarios read
//! it. An option is never serialized, so it has no catch-all.

/// Whether a probe reads attachments, and whether it keeps their data.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum AttachmentHandling {
  /// Leave the section "not requested".
  Skip,
  /// Read the metadata only.
  #[default]
  MetadataOnly,
  /// Keep the payloads too, within the retained-payload limit.
  WithData,
}

/// One attachment of a requested role.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Attachment {
  name: Option<String>,
}

impl Attachment {
  /// The attachment's name, such as a Matroska `FileName`.
  pub fn name(&self) -> Option<&str> {
    self.name.as_deref()
  }
}
