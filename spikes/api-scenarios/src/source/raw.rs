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

//! `RawPath`, the exact form of a lossy path (plan section 8.8): the bytes of a Unix path or the 16-bit units of a
//! Windows path, in lowercase hexadecimal. Reading checks that the data of a known encoding is whole units of that
//! form, and keeps the data of an encoding that a newer version wrote as it was written (`api.md` section 12).

use std::path::{Path, PathBuf};

#[cfg(any(unix, feature = "serde"))]
use super::constants::BYTE_DIGITS;
#[cfg(any(unix, windows))]
use super::constants::HEXADECIMAL_RADIX;
#[cfg(any(windows, feature = "serde"))]
use super::constants::UNIT_DIGITS;
use super::constants::{UNIX_BYTES_NAME, WINDOWS_WIDE_NAME};
#[cfg(feature = "serde")]
use crate::constants::RAW_PATH_DATA_MESSAGE;
#[cfg(feature = "serde")]
use crate::document::InvalidDocument;
use crate::model::UnrecognizedValue;

/// A path exactly as the platform that wrote it holds it (plan section 8.8).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "RawPathData"))]
pub struct RawPath {
  encoding: RawPathEncoding,
  data: String,
}

impl RawPath {
  /// The encoding of the platform that wrote the path.
  pub fn encoding(&self) -> &RawPathEncoding {
    &self.encoding
  }

  /// The path's bytes, or its 16-bit units, in lowercase hexadecimal: two digits a byte or four a unit. For an
  /// encoding that a newer version wrote, the data as written.
  pub fn data(&self) -> &str {
    &self.data
  }

  /// The raw form of a path on Unix: its bytes.
  #[cfg(unix)]
  pub(super) fn of(path: &Path) -> Option<RawPath> {
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
  pub(super) fn of(path: &Path) -> Option<RawPath> {
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
  pub(super) fn of(_path: &Path) -> Option<RawPath> {
    None
  }

  /// The native path, when the encoding is the running platform's.
  #[cfg(unix)]
  pub(super) fn to_native(&self) -> Option<PathBuf> {
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
  pub(super) fn to_native(&self) -> Option<PathBuf> {
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
  pub(super) fn to_native(&self) -> Option<PathBuf> {
    None
  }
}

/// A raw path as read, before its data is checked.
#[cfg(feature = "serde")]
#[derive(serde::Deserialize)]
struct RawPathData {
  encoding: RawPathEncoding,
  data: String,
}

#[cfg(feature = "serde")]
impl TryFrom<RawPathData> for RawPath {
  type Error = InvalidDocument;

  /// The data of a known encoding is whole units of lowercase hexadecimal digits, as the writer writes it, so a path
  /// of the running platform's encoding always rebuilds. A foreign path of a known encoding is checked the same way
  /// and kept. The data of an encoding that a newer version wrote is kept as written.
  fn try_from(read: RawPathData) -> Result<RawPath, InvalidDocument> {
    if let Some(digits) = read.encoding.unit_digits()
      && !is_whole_units(&read.data, digits)
    {
      return Err(InvalidDocument::new(RAW_PATH_DATA_MESSAGE));
    }
    Ok(RawPath {
      encoding: read.encoding,
      data: read.data,
    })
  }
}

/// Whether the text is whole units of `digits` lowercase hexadecimal digits each.
#[cfg(any(unix, windows, feature = "serde"))]
fn is_whole_units(data: &str, digits: usize) -> bool {
  data.len().is_multiple_of(digits)
    && data
      .bytes()
      .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
}

/// Hexadecimal text read as units of `digits` digits each; `None` for text that is not such units.
#[cfg(any(unix, windows))]
fn hexadecimal_units(data: &str, digits: usize) -> Option<Vec<u16>> {
  if !is_whole_units(data, digits) {
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

  /// The hexadecimal digits of one unit of a known encoding; `None` for an encoding that a newer version wrote.
  #[cfg(feature = "serde")]
  fn unit_digits(&self) -> Option<usize> {
    match self {
      RawPathEncoding::UnixBytes => Some(BYTE_DIGITS),
      RawPathEncoding::WindowsWide => Some(UNIT_DIGITS),
      RawPathEncoding::Unrecognized(_) => None,
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
