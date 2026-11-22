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
use crate::constants::LOSSY_PATH_MESSAGE;
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
