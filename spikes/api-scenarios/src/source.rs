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

//! The blocking positional source, `api.md` section 4.4, and the source table with its portable paths,
//! section 6.15.

use std::io;
use std::path::{Path, PathBuf};

use crate::completeness::Reason;

/// A source that is neither a file nor a reader, such as ranges of a remote object.
pub trait PositionalSource {
  /// Reads at most `buffer.len()` bytes starting at `position`. Returns 0 only at or past the end.
  fn read_at(&mut self, position: u64, buffer: &mut [u8]) -> io::Result<usize>;

  /// The total length, or `None` when the source does not know it.
  fn length(&mut self) -> io::Result<Option<u64>>;
}

impl<Source: PositionalSource + ?Sized> PositionalSource for &mut Source {
  fn read_at(&mut self, position: u64, buffer: &mut [u8]) -> io::Result<usize> {
    (**self).read_at(position, buffer)
  }

  fn length(&mut self) -> io::Result<Option<u64>> {
    (**self).length()
  }
}

impl<Source: PositionalSource + ?Sized> PositionalSource for Box<Source> {
  fn read_at(&mut self, position: u64, buffer: &mut [u8]) -> io::Result<usize> {
    (**self).read_at(position, buffer)
  }

  fn length(&mut self) -> io::Result<Option<u64>> {
    (**self).length()
  }
}

/// Which source of a probe something concerns; 0 is the primary input.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SourceIdentifier(u32);

/// One source the probe requested.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

/// A file-system path that any platform can read back. The stub keeps the display string only.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PortablePath {
  display: String,
  lossy: bool,
}

impl PortablePath {
  /// The portable form of `path`.
  pub fn from_path(path: &Path) -> PortablePath {
    PortablePath {
      display: path.to_string_lossy().into_owned(),
      lossy: path.to_str().is_none(),
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

  /// The native path. A path that is not lossy is rebuilt from its display string on every platform. A lossy one
  /// needs its raw form in the running platform's encoding, which the stub does not keep.
  pub fn to_native(&self) -> Option<PathBuf> {
    (!self.lossy).then(|| PathBuf::from(&self.display))
  }
}
