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

//! `PortablePath`, `api.md` section 6.15, with the raw form of plan section 8.8 in `raw.rs`. Serialized, a path is
//! not a value of its own: the map that holds it writes its display string under the path's name and, for a lossy
//! path, two members beside it (`api.md` section 12).

use std::path::{Path, PathBuf};

use super::raw::RawPath;
#[cfg(feature = "serde")]
use crate::constants::{LOSSY_DISPLAY_MESSAGE, LOSSY_FLAG_MESSAGE, LOSSY_PATH_MESSAGE, PATH_DISAGREES_MESSAGE};
#[cfg(feature = "serde")]
use crate::document::InvalidDocument;

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

  /// The exact path: present only when the path is lossy, and then always where the platform that wrote it has an
  /// exact form of a path. A lossy path that a platform without one wrote has none, wherever it is read.
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

  /// A path read from its members, which describe one path. The flag is written only as `true`, and the raw form
  /// stands only beside it. Every lossy display string holds U+FFFD, beside a raw form of any encoding and beside
  /// none, as a platform without an exact form of a path writes it (`json-output.md` section 5.8, after the second
  /// and third reviews of the skeleton's first slice). For a known encoding, the raw data is not valid Unicode, since
  /// a path that is valid Unicode is never lossy, and the display string is that data decoded with U+FFFD. The data
  /// of a newer encoding is kept as written.
  #[cfg(feature = "serde")]
  pub(crate) fn from_members(
    display: String,
    lossy: Option<bool>,
    raw: Option<RawPath>,
  ) -> Result<PortablePath, InvalidDocument> {
    match (lossy, raw) {
      (None, None) => Ok(PortablePath {
        display,
        lossy: false,
        raw: None,
      }),
      (None, Some(_)) => Err(InvalidDocument::new(LOSSY_PATH_MESSAGE)),
      (Some(false), _) => Err(InvalidDocument::new(LOSSY_FLAG_MESSAGE)),
      (Some(true), _) if !display.contains(char::REPLACEMENT_CHARACTER) => {
        Err(InvalidDocument::new(LOSSY_DISPLAY_MESSAGE))
      }
      (Some(true), raw) => match raw.as_ref().and_then(RawPath::decoded) {
        Some((is_unicode, decoded)) if is_unicode || decoded != display => {
          Err(InvalidDocument::new(PATH_DISAGREES_MESSAGE))
        }
        _ => Ok(PortablePath {
          display,
          lossy: true,
          raw,
        }),
      },
    }
  }

  /// The members that a map writes for the path: the display string, the flag, written only as `true`, and the raw
  /// form.
  #[cfg(feature = "serde")]
  pub(crate) fn into_members(self) -> (String, Option<bool>, Option<RawPath>) {
    (self.display, self.lossy.then_some(true), self.raw)
  }
}
