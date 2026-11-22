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

//! `PortablePath`, `api.md` section 6.15, with the raw form of plan section 8.8. Serialized, a path is not a value
//! of its own: the map that holds it writes its display string under the path's name and, for a lossy path, two
//! members beside it (`api.md` section 12).

use std::path::{Path, PathBuf};

#[cfg(unix)]
use super::constants::BYTE_DIGITS;
#[cfg(any(unix, windows))]
use super::constants::HEXADECIMAL_RADIX;
#[cfg(windows)]
use super::constants::UNIT_DIGITS;
use super::constants::{UNIX_BYTES_NAME, WINDOWS_WIDE_NAME};
#[cfg(feature = "serde")]
use crate::constants::LOSSY_PATH_MESSAGE;
#[cfg(feature = "serde")]
use crate::document::InvalidDocument;
use crate::model::UnrecognizedValue;

/// A file-system path that any platform can read back.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PortablePath {
  display: String,
  lossy: bool,
  raw: Option<RawPath>,
}

impl PortablePath {
  /// The portable form of `path`, with its raw form when the display string loses part of it.
  pub fn from_path(path: &Path) -> PortablePath {
    let lossy = path.to_str().is_none();
    PortablePath {
      display: path.to_string_lossy().into_owned(),
      lossy,
      raw: if lossy { RawPath::of(path) } else { None },
    }
  }

  /// The path for people.
  pub fn display(&self) -> &str {
    &self.display
  }

  /// Whether the display string lost part of the path.
  pub fn is_lossy(&self) -> bool {
    self.lossy
  }

  /// The exact path, present exactly when the path is lossy.
  pub fn raw(&self) -> Option<&RawPath> {
    self.raw.as_ref()
  }

  /// The native path. A path that is not lossy is rebuilt from its display string on every platform; a lossy one
  /// from its raw form, when that encoding belongs to the running platform.
  pub fn to_native(&self) -> Option<PathBuf> {
    if self.lossy {
      self.raw.as_ref().and_then(RawPath::to_native)
    } else {
      Some(PathBuf::from(&self.display))
    }
  }

  /// A path read from its members: the display string, the flag and the raw form, which a lossy path needs and only
  /// a lossy path has.
  #[cfg(feature = "serde")]
  pub(crate) fn from_members(
    display: String,
    lossy: bool,
    raw: Option<RawPath>,
  ) -> Result<PortablePath, InvalidDocument> {
    if lossy != raw.is_some() {
      return Err(InvalidDocument::new(LOSSY_PATH_MESSAGE));
    }
    Ok(PortablePath { display, lossy, raw })
  }

  /// The members that a map writes for the path: the display string, the flag and the raw form.
  #[cfg(feature = "serde")]
  pub(crate) fn into_members(self) -> (String, bool, Option<RawPath>) {
    (self.display, self.lossy, self.raw)
  }
}

/// Whether a path's flag member is left out: it is written only for a lossy path.
#[cfg(feature = "serde")]
pub(crate) fn is_not_lossy(lossy: &bool) -> bool {
  !*lossy
}

/// A path exactly as the platform that wrote it holds it (plan section 8.8).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RawPath {
  encoding: RawPathEncoding,
  data: String,
}

impl RawPath {
  /// The encoding of the platform that wrote the path.
  pub fn encoding(&self) -> &RawPathEncoding {
    &self.encoding
  }

  /// The path's bytes, or its 16-bit units, in lowercase hexadecimal: two digits a byte or four a unit.
  pub fn data(&self) -> &str {
    &self.data
  }

  /// The raw form of a path on Unix: its bytes.
  #[cfg(unix)]
  fn of(path: &Path) -> Option<RawPath> {
    use std::os::unix::ffi::OsStrExt;
    let data = path
      .as_os_str()
      .as_bytes()
      .iter()
      .map(|byte| format!("{byte:02x}"))
      .collect();
    Some(RawPath {
      encoding: RawPathEncoding::UnixBytes,
      data,
    })
  }

  /// The raw form of a path on Windows: its 16-bit units.
  #[cfg(windows)]
  fn of(path: &Path) -> Option<RawPath> {
    use std::os::windows::ffi::OsStrExt;
    let data = path
      .as_os_str()
      .encode_wide()
      .map(|unit| format!("{unit:04x}"))
      .collect();
    Some(RawPath {
      encoding: RawPathEncoding::WindowsWide,
      data,
    })
  }

  /// No raw form on another platform.
  #[cfg(not(any(unix, windows)))]
  fn of(_path: &Path) -> Option<RawPath> {
    None
  }

  /// The native path, when the encoding is the running platform's.
  #[cfg(unix)]
  fn to_native(&self) -> Option<PathBuf> {
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;
    if !matches!(self.encoding, RawPathEncoding::UnixBytes) {
      return None;
    }
    let units = hexadecimal_units(&self.data, BYTE_DIGITS)?;
    let bytes = units
      .into_iter()
      .map(u8::try_from)
      .collect::<Result<Vec<u8>, _>>()
      .ok()?;
    Some(PathBuf::from(OsStr::from_bytes(&bytes)))
  }

  /// The native path, when the encoding is the running platform's.
  #[cfg(windows)]
  fn to_native(&self) -> Option<PathBuf> {
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;
    if !matches!(self.encoding, RawPathEncoding::WindowsWide) {
      return None;
    }
    Some(PathBuf::from(OsString::from_wide(&hexadecimal_units(
      &self.data,
      UNIT_DIGITS,
    )?)))
  }

  /// No native path on another platform.
  #[cfg(not(any(unix, windows)))]
  fn to_native(&self) -> Option<PathBuf> {
    None
  }
}

/// Hexadecimal text read as units of `digits` digits each; `None` for text that is not such units.
#[cfg(any(unix, windows))]
fn hexadecimal_units(data: &str, digits: usize) -> Option<Vec<u16>> {
  if !data.len().is_multiple_of(digits) {
    return None;
  }
  data
    .as_bytes()
    .chunks(digits)
    .map(|chunk| {
      let text = std::str::from_utf8(chunk).ok()?;
      u16::from_str_radix(text, HEXADECIMAL_RADIX).ok()
    })
    .collect()
}

/// The encoding of a raw path. Serialized as its stable name.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(into = "String", from = "String"))]
#[non_exhaustive]
pub enum RawPathEncoding {
  /// The bytes of a Unix path.
  UnixBytes,
  /// The 16-bit units of a Windows path.
  WindowsWide,
  /// A value that a newer version wrote.
  Unrecognized(UnrecognizedValue),
}

impl RawPathEncoding {
  /// The serialized name, or the text of an unrecognized value.
  pub fn as_str(&self) -> &str {
    match self {
      RawPathEncoding::UnixBytes => UNIX_BYTES_NAME,
      RawPathEncoding::WindowsWide => WINDOWS_WIDE_NAME,
      RawPathEncoding::Unrecognized(value) => value.as_str(),
    }
  }
}

#[cfg(feature = "serde")]
impl From<String> for RawPathEncoding {
  fn from(name: String) -> RawPathEncoding {
    match name.as_str() {
      UNIX_BYTES_NAME => RawPathEncoding::UnixBytes,
      WINDOWS_WIDE_NAME => RawPathEncoding::WindowsWide,
      _ => RawPathEncoding::Unrecognized(UnrecognizedValue::new(name)),
    }
  }
}

#[cfg(feature = "serde")]
impl From<RawPathEncoding> for String {
  fn from(encoding: RawPathEncoding) -> String {
    encoding.as_str().to_owned()
  }
}
