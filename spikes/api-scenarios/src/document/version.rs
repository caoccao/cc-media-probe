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

//! `SchemaVersion`, `api.md` section 12, serialized as text such as `"1.0"`.

use super::invalid::InvalidDocument;
use crate::constants::{SCHEMA_MAJOR, SCHEMA_MINOR, SCHEMA_VERSION_TEXT_MESSAGE, VERSION_SEPARATOR};
use crate::decimal::canonical;

/// A schema version, `major.minor`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(into = "String", try_from = "String")]
pub struct SchemaVersion {
  major: u32,
  minor: u32,
}

impl SchemaVersion {
  /// The version this release writes.
  pub const CURRENT: SchemaVersion = SchemaVersion {
    major: SCHEMA_MAJOR,
    minor: SCHEMA_MINOR,
  };

  /// The major version.
  pub fn major(&self) -> u32 {
    self.major
  }

  /// The minor version.
  pub fn minor(&self) -> u32 {
    self.minor
  }
}

impl From<SchemaVersion> for String {
  fn from(version: SchemaVersion) -> String {
    format!("{}{VERSION_SEPARATOR}{}", version.major, version.minor)
  }
}

impl TryFrom<String> for SchemaVersion {
  type Error = InvalidDocument;

  /// Two canonical decimal numbers of 32 bits separated by a period: ASCII digits, no sign and no leading zero, so
  /// `"+1.+0"`, `"01.0"` and `"1.00"` are no version (thirteenth review). A version that this release does not read
  /// is still a version; the record decides what to do with it.
  fn try_from(text: String) -> Result<SchemaVersion, InvalidDocument> {
    let invalid = || InvalidDocument::new(SCHEMA_VERSION_TEXT_MESSAGE);
    let (major, minor) = text.split_once(VERSION_SEPARATOR).ok_or_else(invalid)?;
    Ok(SchemaVersion {
      major: canonical(major).ok_or_else(invalid)?,
      minor: canonical(minor).ok_or_else(invalid)?,
    })
  }
}
