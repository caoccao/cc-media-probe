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

//! Why the path of a generated test file could not be given. The message leaves out the system's error, which
//! `source()` gives, as the library's errors do; it repeated it before the third review of plan Phase 3, task 4,
//! found that the fixtures' error should give its cause as its source. `Debug` writes the message and the cause, so
//! that a test that unwraps the path fails with both.

use std::error::Error;
use std::fmt;
use std::io;
use std::path::PathBuf;

use super::constants::{MALFORMED_NAME, MISSING, NOT_A_FILE, UNREADABLE};

/// Why the path of a generated test file could not be given.
///
/// ```
/// use cc_media_probe_testkit::generated::{GeneratedError, path};
///
/// let error = path("containers/Matroska/a.mkv").unwrap_err();
/// assert!(matches!(&error, GeneratedError::MalformedName(name) if name == "containers/Matroska/a.mkv"));
/// // `Debug` writes the message too, so that a test that unwraps the path fails with it.
/// assert_eq!(format!("{error:?}"), error.to_string());
/// ```
pub enum GeneratedError {
  /// A name that is not `<topic>/<format>/<file>`.
  MalformedName(String),
  /// A generated file that is missing, at its path: nothing stands there.
  Missing(PathBuf),
  /// A path whose metadata could not be read, with the error that the system gave.
  Unreadable(PathBuf, io::Error),
  /// A path that holds a folder or another entry that is no file.
  NotAFile(PathBuf),
}

impl fmt::Display for GeneratedError {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      GeneratedError::MalformedName(name) => write!(formatter, "{name}: {MALFORMED_NAME}"),
      GeneratedError::Missing(path) => write!(formatter, "{}: {MISSING}", path.display()),
      GeneratedError::Unreadable(path, _) => write!(formatter, "{}: {UNREADABLE}", path.display()),
      GeneratedError::NotAFile(path) => write!(formatter, "{}: {NOT_A_FILE}", path.display()),
    }
  }
}

impl fmt::Debug for GeneratedError {
  /// Writes the message, and the cause that `source()` gives after it.
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt::Display::fmt(self, formatter)?;
    match self.source() {
      Some(cause) => write!(formatter, ": {cause}"),
      None => Ok(()),
    }
  }
}

impl Error for GeneratedError {
  /// The system's error of a path whose metadata could not be read.
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match self {
      GeneratedError::Unreadable(_, error) => Some(error),
      _ => None,
    }
  }
}
