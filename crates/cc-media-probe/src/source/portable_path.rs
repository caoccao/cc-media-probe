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

//! `PortablePath`, a file-system path that any platform can read back (`api.md` section 6.15; plan section 8.8).

use std::path::{Path, PathBuf};

use super::RawPath;

/// A file-system path that any platform can read back, though no JSON string may represent it exactly.
///
/// A path is lossy exactly when its native form is not valid Unicode: bytes that are not UTF-8 on Unix, WASI,
/// Hermit, SOLID and Xous, or 16-bit units with an unpaired surrogate on Windows. Its display string is then that
/// form decoded with U+FFFD in place of what is not valid, and its raw form keeps it exactly. A platform whose
/// standard library specifies no exact form of a path, such as UEFI or a WebAssembly target without WASI, keeps a
/// lossy path's display string and flag only, and so does every reader of what it wrote.
///
/// ```
/// use std::path::Path;
/// use cc_media_probe::source::PortablePath;
///
/// let path = PortablePath::from_path(Path::new("movie.mkv"));
/// assert_eq!(path.display(), "movie.mkv");
/// assert!(!path.is_lossy());
/// assert_eq!(path.raw(), None);
/// assert_eq!(path.to_native().as_deref(), Some(Path::new("movie.mkv")));
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PortablePath {
  display: String,
  lossy: bool,
  raw: Option<RawPath>,
}

impl PortablePath {
  /// The portable form of `path`, with its raw form when the display string loses part of it.
  ///
  /// ```
  /// use std::path::Path;
  /// use cc_media_probe::source::PortablePath;
  ///
  /// let input = PortablePath::from_path(Path::new("notes.txt"));
  /// assert_eq!(input.display(), "notes.txt");
  /// ```
  #[must_use]
  pub fn from_path(path: &Path) -> PortablePath {
    let lossy = path.to_str().is_none();
    PortablePath {
      display: path.to_string_lossy().into_owned(),
      lossy,
      raw: RawPath::of(path, lossy),
    }
  }

  /// The path for people: the path itself when it is valid Unicode, and otherwise its native form decoded with
  /// U+FFFD in place of what is not valid.
  ///
  /// ```
  /// use cc_media_probe::source::PortablePath;
  ///
  /// fn name(path: &PortablePath) -> String {
  ///     if path.is_lossy() {
  ///         format!("{} (the name is not valid Unicode)", path.display())
  ///     } else {
  ///         path.display().to_owned()
  ///     }
  /// }
  /// ```
  #[must_use]
  pub fn display(&self) -> &str {
    &self.display
  }

  /// Whether `display()` lost information.
  ///
  /// ```
  /// use std::path::Path;
  /// use cc_media_probe::source::PortablePath;
  ///
  /// assert!(!PortablePath::from_path(Path::new("movie.mkv")).is_lossy());
  /// ```
  #[must_use]
  pub fn is_lossy(&self) -> bool {
    self.lossy
  }

  /// The exact form of the path: present only when it is lossy, and then always where the platform that wrote the
  /// path has an exact form of a path: Unix, WASI, Hermit, SOLID, Xous and Windows. Availability depends on the
  /// writer, not on the reader. A lossy path that a platform without an exact form wrote has none, wherever it is
  /// read: an archive from UEFI read on Unix gives `is_lossy()` true and `raw()` absent.
  ///
  /// ```
  /// use std::path::Path;
  /// use cc_media_probe::source::PortablePath;
  ///
  /// assert_eq!(PortablePath::from_path(Path::new("movie.mkv")).raw(), None);
  /// ```
  #[must_use]
  pub fn raw(&self) -> Option<&RawPath> {
    self.raw.as_ref()
  }

  /// The native path. A path that is not lossy is rebuilt from its display string on every platform, since
  /// Unicode text converts to a native path without loss. A lossy path is rebuilt from its raw form only when that
  /// encoding belongs to the running platform, and gives `None` otherwise, as does a lossy path without a raw
  /// form.
  ///
  /// ```
  /// use std::path::Path;
  /// use cc_media_probe::source::PortablePath;
  ///
  /// let path = PortablePath::from_path(Path::new("folder/movie.mkv"));
  /// assert_eq!(path.to_native().as_deref(), Some(Path::new("folder/movie.mkv")));
  /// ```
  #[must_use]
  pub fn to_native(&self) -> Option<PathBuf> {
    if self.lossy {
      self.raw.as_ref().and_then(RawPath::to_native)
    } else {
      Some(PathBuf::from(&self.display))
    }
  }
}

#[cfg(test)]
mod tests {
  use std::path::Path;

  use super::PortablePath;
  use crate::source::{RawPath, RawPathEncoding};

  #[test]
  fn keeps_a_unicode_path_as_its_display_string() {
    let path = PortablePath::from_path(Path::new("film.mkv"));
    assert_eq!(path.display(), "film.mkv");
    assert!(!path.is_lossy());
    assert_eq!(path.raw(), None);
    assert_eq!(path.to_native().as_deref(), Some(Path::new("film.mkv")));
  }

  #[test]
  fn rebuilds_no_foreign_lossy_path() {
    // `�.bin` as the other platform writes it: a byte 0xff or an unpaired surrogate 0xd800 before `.bin`.
    let (encoding, data) = if cfg!(windows) {
      (RawPathEncoding::UnixBytes, "ff2e62696e")
    } else {
      (RawPathEncoding::WindowsWide, "d800002e00620069006e")
    };
    let foreign = PortablePath {
      display: "\u{fffd}.bin".to_owned(),
      lossy: true,
      raw: Some(RawPath::sample(encoding, data)),
    };
    assert!(foreign.is_lossy());
    assert_eq!(foreign.to_native(), None);
    let without_raw_form = PortablePath { raw: None, ..foreign };
    assert_eq!(without_raw_form.to_native(), None);
  }

  #[cfg(unix)]
  #[test]
  fn keeps_a_unix_path_that_is_not_unicode() {
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;
    let native = Path::new(OsStr::from_bytes(b"\xff.bin"));
    let path = PortablePath::from_path(native);
    assert_eq!(path.display(), "\u{fffd}.bin");
    assert!(path.is_lossy());
    assert_eq!(path.raw().map(RawPath::data), Some("ff2e62696e"));
    assert_eq!(path.to_native().as_deref(), Some(native));
  }
}
