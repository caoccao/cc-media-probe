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

//! `SchemaVersion`, the version of a root record of the serialized form (`api.md` section 12; `json-output.md`
//! section 4.1).

use super::constants::{SCHEMA_MAJOR, SCHEMA_MINOR, VERSION_SEPARATOR};
use crate::form::{VERSION_FORM_MESSAGE, canonical};

/// The version of the serialized form that a root record carries, `major.minor`, serialized as text such as
/// `"1.0"`: two canonical decimal numbers of 32 bits, ASCII digits with no sign and no leading zero. A reader takes a
/// root record of its own major version and any minor version, and refuses every other with the version error, as
/// `json-output.md` section 4.1 says. Versions order by their major and then their minor number.
///
/// ```
/// use cc_media_probe::document::SchemaVersion;
///
/// let version = SchemaVersion::CURRENT;
/// assert_eq!((version.major(), version.minor()), (1, 0));
/// assert_eq!(serde_json::to_string(&version).ok().as_deref(), Some("\"1.0\""));
/// let read: Option<SchemaVersion> = serde_json::from_str("\"1.4294967295\"").ok();
/// assert!(read.is_some_and(|read| read > version));
/// assert!(serde_json::from_str::<SchemaVersion>("\"01.0\"").is_err());
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SchemaVersion {
  major: u32,
  minor: u32,
}

impl SchemaVersion {
  /// The version that this release writes.
  pub const CURRENT: SchemaVersion = SchemaVersion {
    major: SCHEMA_MAJOR,
    minor: SCHEMA_MINOR,
  };

  /// The major version, which changes only with an incompatible change of the form.
  #[must_use]
  pub fn major(&self) -> u32 {
    self.major
  }

  /// The minor version, which an additive change raises.
  #[must_use]
  pub fn minor(&self) -> u32 {
    self.minor
  }

  /// The version as the form writes it.
  pub(crate) fn to_text(self) -> String {
    format!("{}{VERSION_SEPARATOR}{}", self.major, self.minor)
  }

  /// The version that the text spells, or the message of the version error: two canonical decimal numbers of 32 bits
  /// separated by a period, so `"+1.+0"`, `"01.0"`, `"1.00"` and `"1.4294967296"` are no version (thirteenth review of
  /// `api.md`). A version that this release does not read is still a version; the record decides.
  pub(crate) fn from_text(text: &str) -> Result<SchemaVersion, &'static str> {
    let (major, minor) = text.split_once(VERSION_SEPARATOR).ok_or(VERSION_FORM_MESSAGE)?;
    Ok(SchemaVersion {
      major: canonical(major).ok_or(VERSION_FORM_MESSAGE)?,
      minor: canonical(minor).ok_or(VERSION_FORM_MESSAGE)?,
    })
  }
}

impl serde::Serialize for SchemaVersion {
  fn serialize<Output: serde::Serializer>(&self, serializer: Output) -> Result<Output::Ok, Output::Error> {
    serializer.serialize_str(&self.to_text())
  }
}

impl<'input> crate::form::ReadAlone<'input> for SchemaVersion {
  fn read_members<Input: serde::Deserializer<'input>>(deserializer: Input) -> Result<SchemaVersion, Input::Error> {
    let text = <String as serde::Deserialize>::deserialize(deserializer)?;
    SchemaVersion::from_text(&text).map_err(<Input::Error as serde::de::Error>::custom)
  }
}

impl<'input> serde::Deserialize<'input> for SchemaVersion {
  fn deserialize<Input: serde::Deserializer<'input>>(deserializer: Input) -> Result<SchemaVersion, Input::Error> {
    crate::form::read_alone(deserializer)
  }
}
