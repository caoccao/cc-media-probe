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

//! `BuiltInResolver`, the built-in resolver's three steps of plan section 7.7: resolve the path following every
//! link, check that it lies inside the resolved root, open the resolved path, and then resolve it once more and
//! decline the companion if the answer changed or the object is not a regular file. Before the open, the resolved
//! path must name a regular file too, since opening a named pipe for reading waits for a writer (POSIX `open`), so
//! a decline could never be returned. Containment holds, and the open does not wait, on the condition that nobody
//! else changes the folder tree while the probe runs.
//!
//! An input without a folder declines every request, while a folder or a path that cannot be resolved fails the
//! request with the error that the file system gave, which ends the probe with its cause (R19): a failure is never
//! taken for an absent folder or for a path that changed.

use std::fs::{self, File};
use std::io::{self, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use cc_media_probe::source::PositionalSource;

use super::constants::{CHANGED, CURRENT_FOLDER, FILE_NUMBERS_LENGTH, NO_FOLDER, NOT_A_FILE, OUTSIDE_ROOT};
use super::{Answer, CompanionRequest, CompanionResolver, Resolution};

/// The built-in resolver of one probe: the folder of the probed file, and that folder resolved once to its real
/// location, the trusted root, when the first request needs it, so the error of a root that cannot be resolved
/// reaches that request.
#[derive(Clone, Debug)]
pub struct BuiltInResolver {
  folder: Option<PathBuf>,
  root: OnceLock<PathBuf>,
}

impl BuiltInResolver {
  /// The resolver for the file at `primary`, or one without a folder, which declines every request.
  #[must_use]
  pub fn new(primary: Option<&Path>) -> BuiltInResolver {
    let folder = primary.and_then(Path::parent).map(|folder| {
      if folder.as_os_str().is_empty() {
        PathBuf::from(CURRENT_FOLDER)
      } else {
        folder.to_path_buf()
      }
    });
    BuiltInResolver {
      folder,
      root: OnceLock::new(),
    }
  }

  /// The trusted root: `folder` resolved to its real location the first time a request needs it.
  fn root(&self, folder: &Path) -> io::Result<&PathBuf> {
    if let Some(root) = self.root.get() {
      return Ok(root);
    }
    let resolved = fs::canonicalize(folder)?;
    Ok(self.root.get_or_init(|| resolved))
  }
}

impl CompanionResolver for BuiltInResolver {
  fn resolve(&self, request: &CompanionRequest) -> Resolution {
    let Some(folder) = &self.folder else {
      return Answer::Declined(NO_FOLDER.to_owned());
    };
    let root = match self.root(folder) {
      Ok(root) => root,
      Err(error) => return Answer::Failed(error),
    };
    let candidate = folder.join(&request.name);
    let resolved = match fs::canonicalize(&candidate) {
      Ok(resolved) => resolved,
      Err(error) if error.kind() == io::ErrorKind::NotFound => return Answer::NotFound,
      Err(error) => return Answer::Failed(error),
    };
    if !resolved.starts_with(root) {
      return Answer::Declined(OUTSIDE_ROOT.to_owned());
    }
    match fs::metadata(&resolved) {
      Ok(metadata) if metadata.is_file() => {}
      Ok(_) => return Answer::Declined(NOT_A_FILE.to_owned()),
      Err(error) => return Answer::Failed(error),
    }
    let file = match File::open(&resolved) {
      Ok(file) => file,
      Err(error) => return Answer::Failed(error),
    };
    match fs::canonicalize(&candidate) {
      Ok(again) if again == resolved => {}
      Ok(_) => return Answer::Declined(CHANGED.to_owned()),
      Err(error) if error.kind() == io::ErrorKind::NotFound => return Answer::Declined(CHANGED.to_owned()),
      Err(error) => return Answer::Failed(error),
    }
    match file.metadata() {
      Ok(metadata) if metadata.is_file() => {}
      Ok(_) => return Answer::Declined(NOT_A_FILE.to_owned()),
      Err(error) => return Answer::Failed(error),
    }
    let identity = identity_of(&file, &resolved);
    Answer::Found {
      source: Box::new(FileSource(file)),
      identity,
    }
  }
}

/// The identity of an opened file in the built-in resolver's form: its device and file numbers where the standard
/// library gives them, as on Unix, and its resolved real path otherwise (`api.md` section 13).
pub(crate) fn identity_of(file: &File, path: &Path) -> Option<Vec<u8>> {
  if cfg!(unix) {
    file_numbers(file).map(|numbers| numbers.to_vec())
  } else {
    resolved_identity(path)
  }
}

/// The device and file numbers of an opened file, a fixed-size identity, where the standard library gives them.
pub(crate) fn file_numbers(file: &File) -> Option<[u8; FILE_NUMBERS_LENGTH]> {
  #[cfg(unix)]
  {
    use std::os::unix::fs::MetadataExt;

    let metadata = file.metadata().ok()?;
    let mut numbers = [0_u8; FILE_NUMBERS_LENGTH];
    let (device, inode) = numbers.split_at_mut(size_of::<u64>());
    device.copy_from_slice(&metadata.dev().to_be_bytes());
    inode.copy_from_slice(&metadata.ino().to_be_bytes());
    Some(numbers)
  }
  #[cfg(not(unix))]
  {
    let _ = file;
    None
  }
}

/// The identity of the file at `path` where no file numbers are known: its resolved real path.
pub(crate) fn resolved_identity(path: &Path) -> Option<Vec<u8>> {
  Some(fs::canonicalize(path).ok()?.into_os_string().into_encoded_bytes())
}

/// An opened companion file as a positional source.
struct FileSource(File);

impl PositionalSource for FileSource {
  fn read_at(&mut self, position: u64, buffer: &mut [u8]) -> io::Result<usize> {
    self.0.seek(SeekFrom::Start(position))?;
    self.0.read(buffer)
  }

  fn length(&mut self) -> io::Result<Option<u64>> {
    self.0.metadata().map(|metadata| Some(metadata.len()))
  }
}
