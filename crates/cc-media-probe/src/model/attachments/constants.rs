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

//! The serialized names of `AttachmentRole` (`json-output.md` section 6.1), and the message of an attachment that
//! contradicts itself.

/// The serialized name of `AttachmentRole::CoverArt`.
pub const COVER_ART_NAME: &str = "cover_art";
/// The serialized name of `AttachmentRole::Other`.
pub const OTHER_NAME: &str = "other";

/// The message of an attachment whose data are not as many bytes as its size, or of a linked picture with a size or
/// data (`api.md` section 6.10).
#[cfg(feature = "serde")]
pub const ATTACHMENT_MESSAGE: &str =
  "an attachment's data are not as many bytes as its size, or a linked picture embeds data";
