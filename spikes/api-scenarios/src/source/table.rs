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

//! The source table, `api.md` section 6.15: one entry for each source the probe requested.

use std::num::ParseIntError;

use super::path::PortablePath;
#[cfg(feature = "serde")]
use super::raw::RawPath;
use crate::completeness::Reason;
#[cfg(feature = "serde")]
use crate::constants::LOSSY_PATH_MESSAGE;
#[cfg(feature = "serde")]
use crate::document::InvalidDocument;
#[cfg(feature = "serde")]
use crate::twin::{unsigned_members, unsigned_value};

/// Which source of a probe something concerns; 0 is the primary input. Serialized as a decimal string, like every
/// identifier, although it is 32 bits wide.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(into = "String", try_from = "String"))]
pub struct SourceIdentifier(u32);

impl From<SourceIdentifier> for String {
  fn from(identifier: SourceIdentifier) -> String {
    identifier.0.to_string()
  }
}

impl TryFrom<String> for SourceIdentifier {
  type Error = ParseIntError;

  fn try_from(text: String) -> Result<SourceIdentifier, ParseIntError> {
    text.parse().map(SourceIdentifier)
  }
}

/// One source the probe requested.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(into = "SourceEntryData", try_from = "SourceEntryData"))]
pub struct SourceEntry {
  identifier: SourceIdentifier,
  reason: Option<Reason>,
  bytes_read: Option<u64>,
  path: Option<PortablePath>,
}

impl SourceEntry {
  /// The source's identifier.
  pub fn identifier(&self) -> SourceIdentifier {
    self.identifier
  }

  /// Why a requested companion was not opened: not found, declined with the resolver's reason, or a limit by name.
  pub fn reason(&self) -> Option<&Reason> {
    self.reason.as_ref()
  }

  /// The bytes read from it.
  pub fn bytes_read(&self) -> Option<u64> {
    self.bytes_read
  }

  /// Its path, for a file.
  pub fn path(&self) -> Option<&PortablePath> {
    self.path.as_ref()
  }
}

/// An entry's members: its path is the display string, and a lossy path adds its flag and its raw form beside it.
/// The bytes read beyond the exact range are the twin `bytes_read_exact` (`api.md` section 12).
#[cfg(feature = "serde")]
#[derive(serde::Serialize, serde::Deserialize)]
struct SourceEntryData {
  identifier: SourceIdentifier,
  #[serde(skip_serializing_if = "Option::is_none")]
  reason: Option<Reason>,
  #[serde(skip_serializing_if = "Option::is_none")]
  bytes_read: Option<u64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  bytes_read_exact: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  path: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  path_lossy: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  path_raw: Option<RawPath>,
}

#[cfg(feature = "serde")]
impl From<SourceEntry> for SourceEntryData {
  fn from(entry: SourceEntry) -> SourceEntryData {
    let (path, path_lossy, path_raw) = match entry.path.map(PortablePath::into_members) {
      Some((display, lossy, raw)) => (Some(display), lossy, raw),
      None => (None, None, None),
    };
    let (bytes_read, bytes_read_exact) = unsigned_members(entry.bytes_read);
    SourceEntryData {
      identifier: entry.identifier,
      reason: entry.reason,
      bytes_read,
      bytes_read_exact,
      path,
      path_lossy,
      path_raw,
    }
  }
}

#[cfg(feature = "serde")]
impl TryFrom<SourceEntryData> for SourceEntry {
  type Error = InvalidDocument;

  /// A path's flag and raw form stand only beside its display string, and the bytes read come from their number or
  /// their twin.
  fn try_from(data: SourceEntryData) -> Result<SourceEntry, InvalidDocument> {
    if data.path.is_none() && (data.path_lossy.is_some() || data.path_raw.is_some()) {
      return Err(InvalidDocument::new(LOSSY_PATH_MESSAGE));
    }
    let path = data
      .path
      .map(|display| PortablePath::from_members(display, data.path_lossy, data.path_raw))
      .transpose()?;
    Ok(SourceEntry {
      identifier: data.identifier,
      reason: data.reason,
      bytes_read: unsigned_value(data.bytes_read, data.bytes_read_exact)?,
      path,
    })
  }
}
