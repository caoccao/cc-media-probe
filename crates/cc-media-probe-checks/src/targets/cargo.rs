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

//! Running `cargo metadata`.

use std::ffi::OsString;
use std::io;
use std::path::Path;
use std::process::Command;

use crate::json::{JsonValue, parse_json};
use crate::targets::constants::{CARGO_EXECUTABLE, CARGO_VARIABLE, MANIFEST_PATH_ARGUMENT, METADATA_ARGUMENTS};

/// The Cargo executable: the one that runs this program, if any.
#[must_use]
pub fn cargo_executable() -> OsString {
  std::env::var_os(CARGO_VARIABLE).unwrap_or_else(|| OsString::from(CARGO_EXECUTABLE))
}

/// Runs `cargo metadata` for the manifest and parses its output.
/// `extra_arguments` replace the default ones after `metadata` when given.
///
/// # Errors
///
/// Returns an error when Cargo cannot be started, fails, or prints
/// something that is not JSON.
pub fn run_cargo_metadata(manifest: &Path, extra_arguments: Option<&[&str]>) -> io::Result<JsonValue> {
  let arguments = extra_arguments.unwrap_or(METADATA_ARGUMENTS);
  let output = Command::new(cargo_executable())
    .args(arguments)
    .arg(MANIFEST_PATH_ARGUMENT)
    .arg(manifest)
    .output()?;
  if !output.status.success() {
    return Err(io::Error::other(String::from_utf8_lossy(&output.stderr).into_owned()));
  }
  let text = String::from_utf8_lossy(&output.stdout);
  parse_json(&text).map_err(io::Error::other)
}

#[cfg(test)]
mod tests {
  use super::{cargo_executable, run_cargo_metadata};
  use std::path::Path;

  #[test]
  fn reads_the_metadata_of_this_workspace() {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../Cargo.toml");
    let metadata = run_cargo_metadata(&manifest, None).unwrap();
    assert!(metadata.get("packages").is_some());
    assert!(!cargo_executable().is_empty());
  }

  #[test]
  fn reports_failures() {
    assert!(run_cargo_metadata(Path::new("/nonexistent/Cargo.toml"), None).is_err());
  }
}
