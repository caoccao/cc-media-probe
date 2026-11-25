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

//! `RawPath` and `RawPathEncoding`, the exact form of a path that no JSON string represents (`api.md` section 6.15;
//! plan section 8.8).

#[cfg(target_os = "hermit")]
use std::os::hermit::ffi::{OsStrExt, OsStringExt};
#[cfg(target_os = "solid_asp3")]
use std::os::solid::ffi::{OsStrExt, OsStringExt};
#[cfg(unix)]
use std::os::unix::ffi::{OsStrExt, OsStringExt};
#[cfg(target_os = "wasi")]
use std::os::wasi::ffi::{OsStrExt, OsStringExt};
#[cfg(windows)]
use std::os::windows::ffi::{OsStrExt, OsStringExt};
#[cfg(target_os = "xous")]
use std::os::xous::ffi::{OsStrExt, OsStringExt};
use std::path::{Path, PathBuf};

use super::constants::{UNIX_BYTES_NAME, WINDOWS_WIDE_NAME};
use crate::string_enumeration::string_enumeration;

/// The exact form of a path whose display string lost information: its bytes or its 16-bit units in lowercase
/// hexadecimal, two digits a byte or four a unit. The platform that wrote the record chooses the encoding: Unix,
/// WASI, Hermit, SOLID and Xous write their bytes, and Windows its units. A platform whose standard library
/// specifies no exact form of a path writes none.
///
/// ```
/// use cc_media_probe::source::{PortablePath, RawPathEncoding};
///
/// fn describe(path: &PortablePath) -> String {
///     match path.raw() {
///         Some(raw) if matches!(raw.encoding(), RawPathEncoding::UnixBytes) => format!("bytes {}", raw.data()),
///         Some(raw) => format!("{} {}", raw.encoding().as_str(), raw.data()),
///         None => path.display().to_owned(),
///     }
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RawPath {
  encoding: RawPathEncoding,
  data: String,
}

impl RawPath {
  /// The encoding of the data.
  ///
  /// ```
  /// use cc_media_probe::source::{RawPath, RawPathEncoding};
  ///
  /// fn is_from_windows(raw: &RawPath) -> bool {
  ///     matches!(raw.encoding(), RawPathEncoding::WindowsWide)
  /// }
  /// ```
  #[must_use]
  pub fn encoding(&self) -> &RawPathEncoding {
    &self.encoding
  }

  /// The path's bytes or its 16-bit units in lowercase hexadecimal, and for an encoding that a newer version
  /// wrote, the data as written.
  ///
  /// ```
  /// use cc_media_probe::source::RawPath;
  ///
  /// fn length_in_digits(raw: &RawPath) -> usize {
  ///     raw.data().len()
  /// }
  /// ```
  #[must_use]
  pub fn data(&self) -> &str {
    &self.data
  }

  /// The raw form of the path `path` when it is `lossy`, in the running platform's exact form of a path: its bytes
  /// on Unix, WASI, Hermit, SOLID and Xous, whose standard library gives them as the operating system holds them,
  /// and its 16-bit units on Windows.
  #[cfg(any(
    unix,
    windows,
    target_os = "wasi",
    target_os = "hermit",
    target_os = "solid_asp3",
    target_os = "xous"
  ))]
  pub(super) fn of(path: &Path, lossy: bool) -> Option<RawPath> {
    lossy.then(|| RawPath {
      encoding: native_encoding(),
      data: native_data(path),
    })
  }

  /// No raw form on any other platform, whose standard library specifies no exact form of a path, or none that is
  /// stable: the form in which it holds one is unspecified and may change between versions, so it is never written
  /// (`api.md` section 6.15).
  #[cfg(not(any(
    unix,
    windows,
    target_os = "wasi",
    target_os = "hermit",
    target_os = "solid_asp3",
    target_os = "xous"
  )))]
  pub(super) fn of(_path: &Path, _lossy: bool) -> Option<RawPath> {
    None
  }

  /// The native path, when the encoding belongs to the running platform.
  pub(super) fn to_native(&self) -> Option<PathBuf> {
    native_path(&self.encoding, &self.data)
  }
}

/// The encoding of a path's exact form on a platform of bytes.
#[cfg(any(
  unix,
  target_os = "wasi",
  target_os = "hermit",
  target_os = "solid_asp3",
  target_os = "xous"
))]
fn native_encoding() -> RawPathEncoding {
  RawPathEncoding::UnixBytes
}

/// The encoding of a path's exact form on Windows.
#[cfg(windows)]
fn native_encoding() -> RawPathEncoding {
  RawPathEncoding::WindowsWide
}

/// The bytes of `path` as the operating system holds them, in hexadecimal, on a platform of bytes.
#[cfg(any(
  unix,
  target_os = "wasi",
  target_os = "hermit",
  target_os = "solid_asp3",
  target_os = "xous"
))]
fn native_data(path: &Path) -> String {
  crate::utilities::encode(
    path.as_os_str().as_bytes().iter().copied(),
    super::constants::BYTE_DIGITS,
  )
}

/// The 16-bit units of `path`, in hexadecimal, on Windows.
#[cfg(windows)]
fn native_data(path: &Path) -> String {
  crate::utilities::encode(path.as_os_str().encode_wide(), super::constants::UNIT_DIGITS)
}

/// The native path of the raw form with this encoding and data, on a platform of bytes, when the encoding is
/// `UnixBytes`.
#[cfg(any(
  unix,
  target_os = "wasi",
  target_os = "hermit",
  target_os = "solid_asp3",
  target_os = "xous"
))]
fn native_path(encoding: &RawPathEncoding, data: &str) -> Option<PathBuf> {
  use std::ffi::OsString;
  if !matches!(encoding, RawPathEncoding::UnixBytes) {
    return None;
  }
  let bytes = crate::utilities::decode_units(data, super::constants::BYTE_DIGITS)?
    .into_iter()
    .map(u8::try_from)
    .collect::<Result<Vec<u8>, _>>()
    .ok()?;
  Some(PathBuf::from(OsString::from_vec(bytes)))
}

/// The native path of the raw form with this encoding and data, on Windows, when the encoding is `WindowsWide`.
#[cfg(windows)]
fn native_path(encoding: &RawPathEncoding, data: &str) -> Option<PathBuf> {
  use std::ffi::OsString;
  if !matches!(encoding, RawPathEncoding::WindowsWide) {
    return None;
  }
  let units = crate::utilities::decode_units(data, super::constants::UNIT_DIGITS)?;
  Some(PathBuf::from(OsString::from_wide(&units)))
}

/// No native path from a raw form on any other platform, since none of its paths has one.
#[cfg(not(any(
  unix,
  windows,
  target_os = "wasi",
  target_os = "hermit",
  target_os = "solid_asp3",
  target_os = "xous"
)))]
fn native_path(_encoding: &RawPathEncoding, _data: &str) -> Option<PathBuf> {
  None
}

string_enumeration! {
  /// The encoding of a raw path.
  ///
  /// ```
  /// use cc_media_probe::source::RawPathEncoding;
  ///
  /// assert_eq!(RawPathEncoding::UnixBytes.as_str(), "unix_bytes");
  /// ```
  RawPathEncoding {
    /// The bytes of a Unix path, two lowercase hexadecimal digits a byte.
    UnixBytes => UNIX_BYTES_NAME,
    /// The 16-bit units of a Windows path, four lowercase hexadecimal digits a unit.
    WindowsWide => WINDOWS_WIDE_NAME,
  }
}

#[cfg(test)]
mod tests {
  use super::{RawPath, RawPathEncoding};
  use crate::model::UnrecognizedValue;

  impl RawPath {
    /// The raw form `data` in `encoding`.
    pub(crate) fn sample(encoding: RawPathEncoding, data: &str) -> RawPath {
      RawPath {
        encoding,
        data: data.to_owned(),
      }
    }
  }

  #[test]
  fn names_each_encoding() {
    assert_eq!(RawPathEncoding::UnixBytes.as_str(), "unix_bytes");
    assert_eq!(RawPathEncoding::WindowsWide.as_str(), "windows_wide");
    assert_eq!(
      RawPathEncoding::Unrecognized(UnrecognizedValue::sample("wasi_bytes")).as_str(),
      "wasi_bytes"
    );
  }

  #[test]
  fn keeps_the_encoding_and_the_data() {
    let raw = RawPath::sample(RawPathEncoding::WindowsWide, "d800002e");
    assert_eq!(raw.encoding(), &RawPathEncoding::WindowsWide);
    assert_eq!(raw.data(), "d800002e");
  }

  #[cfg(any(
    unix,
    target_os = "wasi",
    target_os = "hermit",
    target_os = "solid_asp3",
    target_os = "xous"
  ))]
  #[test]
  fn rebuilds_only_a_path_of_the_running_platform() {
    use std::ffi::OsStr;
    use std::path::Path;

    use super::OsStrExt;
    let path = Path::new(OsStr::from_bytes(b"\xff.bin"));
    let raw = RawPath::of(path, true);
    assert_eq!(raw.as_ref().map(RawPath::data), Some("ff2e62696e"));
    assert_eq!(raw.as_ref().and_then(RawPath::to_native).as_deref(), Some(path));
    assert_eq!(RawPath::of(path, false), None);
    assert_eq!(RawPath::sample(RawPathEncoding::WindowsWide, "d800").to_native(), None);
    assert_eq!(RawPath::sample(RawPathEncoding::UnixBytes, "f").to_native(), None);
  }

  #[cfg(windows)]
  #[test]
  fn rebuilds_only_a_path_of_the_running_platform() {
    use std::ffi::OsString;
    use std::path::PathBuf;

    use super::OsStringExt;
    let path = PathBuf::from(OsString::from_wide(&[0xd800, 0x002e]));
    let raw = RawPath::of(&path, true);
    assert_eq!(raw.as_ref().map(RawPath::data), Some("d800002e"));
    assert_eq!(RawPath::of(&path, false), None);
    assert_eq!(raw.as_ref().and_then(RawPath::to_native), Some(path));
    assert_eq!(RawPath::sample(RawPathEncoding::UnixBytes, "ff").to_native(), None);
    assert_eq!(RawPath::sample(RawPathEncoding::WindowsWide, "d80").to_native(), None);
  }
}
