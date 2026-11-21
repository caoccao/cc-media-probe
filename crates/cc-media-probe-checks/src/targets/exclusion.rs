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

//! The excluded workspaces stay outside the root workspace: in a temporary
//! copy of the workspace, a package is created under each excluded directory,
//! and `cargo metadata` must succeed both at the root, without the probes as
//! members, and inside each probe.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::constants::metadata::WORKSPACE_MEMBERS_MEMBER;
use crate::constants::paths::LIBRARY_ROOT;
use crate::constants::paths::{CRATES_DIRECTORY, MANIFEST_FILE, SOURCE_DIRECTORY, TARGET_DIRECTORY};
use crate::findings::{Finding, Rule};
use crate::json::JsonValue;
use crate::targets::cargo::run_cargo_metadata;
use crate::targets::constants::{
  EXCLUDED_DIRECTORIES, METADATA_FAILED, PROBE_IS_MEMBER, PROBE_LIBRARY, PROBE_MANIFEST, PROBE_PACKAGE, ROOT_FILES,
  TEMPORARY_PREFIX,
};

/// Numbers the probes of one process, so that probes running at the same
/// time use different directories.
static NEXT_PROBE: AtomicUsize = AtomicUsize::new(0);

/// Runs the exclusion probe for the workspace at `root`.
///
/// # Errors
///
/// Returns an error when the temporary copy cannot be written.
pub fn check_exclusion(root: &Path) -> io::Result<Vec<Finding>> {
  let sequence = NEXT_PROBE.fetch_add(1, Ordering::Relaxed);
  let process = std::process::id();
  let copy = std::env::temp_dir().join(format!("{TEMPORARY_PREFIX}{process}-{sequence}"));
  let _ = fs::remove_dir_all(&copy);
  let result = probe(root, &copy);
  let _ = fs::remove_dir_all(&copy);
  result
}

fn probe(root: &Path, copy: &Path) -> io::Result<Vec<Finding>> {
  fs::create_dir_all(copy)?;
  for file in ROOT_FILES {
    let source = root.join(file);
    if source.is_file() {
      fs::copy(&source, copy.join(file))?;
    }
  }
  copy_tree(&root.join(CRATES_DIRECTORY), &copy.join(CRATES_DIRECTORY))?;
  let mut probes = Vec::new();
  for directory in EXCLUDED_DIRECTORIES {
    let package = copy.join(directory).join(PROBE_PACKAGE);
    fs::create_dir_all(package.join(SOURCE_DIRECTORY))?;
    fs::write(package.join(MANIFEST_FILE), PROBE_MANIFEST)?;
    fs::write(package.join(SOURCE_DIRECTORY).join(LIBRARY_ROOT), PROBE_LIBRARY)?;
    probes.push(package);
  }
  let mut findings = Vec::new();
  match run_cargo_metadata(&copy.join(MANIFEST_FILE), None) {
    Ok(metadata) => {
      let members = metadata
        .get(WORKSPACE_MEMBERS_MEMBER)
        .map_or_default(JsonValue::elements);
      let probe_is_member = members
        .iter()
        .filter_map(JsonValue::as_text)
        .any(|member| member.contains(PROBE_PACKAGE));
      if probe_is_member {
        findings.push(Finding::new(MANIFEST_FILE, 0, Rule::Targets, PROBE_IS_MEMBER));
      }
    }
    Err(error) => findings.push(metadata_failure(MANIFEST_FILE, &error)),
  }
  for package in probes {
    if let Err(error) = run_cargo_metadata(&package.join(MANIFEST_FILE), None) {
      let label = package
        .strip_prefix(copy)
        .unwrap_or(&package)
        .to_string_lossy()
        .into_owned();
      findings.push(metadata_failure(&label, &error));
    }
  }
  Ok(findings)
}

fn metadata_failure(path: &str, error: &io::Error) -> Finding {
  let detail = error.to_string();
  let first_line = detail.lines().next().unwrap_or_default();
  Finding::new(path, 0, Rule::Targets, format!("{METADATA_FAILED}: {first_line}"))
}

/// Copies a directory tree without its `target` directories.
fn copy_tree(source: &Path, destination: &Path) -> io::Result<()> {
  let mut pending: Vec<(PathBuf, PathBuf)> = vec![(source.to_path_buf(), destination.to_path_buf())];
  while let Some((from, to)) = pending.pop() {
    fs::create_dir_all(&to)?;
    for entry in fs::read_dir(&from)? {
      let entry = entry?;
      let file_type = entry.file_type()?;
      let target = to.join(entry.file_name());
      if file_type.is_dir() && entry.file_name() != TARGET_DIRECTORY {
        pending.push((entry.path(), target));
      } else if file_type.is_file() {
        fs::copy(entry.path(), target)?;
      }
    }
  }
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::check_exclusion;
  use std::path::Path;

  #[test]
  fn keeps_spikes_and_readers_outside_this_workspace() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    assert_eq!(check_exclusion(&root).unwrap(), Vec::new());
  }

  #[test]
  fn reports_a_workspace_that_cannot_be_read() {
    let findings = check_exclusion(Path::new("/nonexistent/repository"));
    assert!(findings.is_err());
  }
}
