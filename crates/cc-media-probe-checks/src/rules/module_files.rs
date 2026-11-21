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

//! Directory modules use `mod.rs` (R30): no `name.rs` beside a `name/`
//! directory that holds Rust files.

use std::collections::BTreeSet;

use crate::constants::paths::{BINARY_ROOT, DIRECTORY_MODULE_ROOT, LIBRARY_ROOT, PATH_SEPARATOR, RUST_EXTENSION};
use crate::constants::punctuation::FULL_STOP;
use crate::findings::{Finding, Rule};
use crate::rules::constants::MODULE_FILE_BESIDE_DIRECTORY;

/// Checks every Rust file path, relative and with `/`, against R30.
#[must_use]
pub fn check_module_files(paths: &[String]) -> Vec<Finding> {
  let directories: BTreeSet<&str> = paths
    .iter()
    .filter_map(|path| path.rsplit_once(PATH_SEPARATOR).map(|(directory, _)| directory))
    .collect();
  let extension = format!("{FULL_STOP}{RUST_EXTENSION}");
  let mut findings = Vec::new();
  for path in paths {
    let file_name = path.rsplit(PATH_SEPARATOR).next().unwrap_or_default();
    if [DIRECTORY_MODULE_ROOT, LIBRARY_ROOT, BINARY_ROOT].contains(&file_name) {
      continue;
    }
    let Some(module_directory) = path.strip_suffix(&extension) else {
      continue;
    };
    let nested = format!("{module_directory}{PATH_SEPARATOR}");
    let has_directory = directories
      .iter()
      .any(|directory| *directory == module_directory || directory.starts_with(&nested));
    if has_directory {
      findings.push(Finding::new(path, 0, Rule::ModuleFile, MODULE_FILE_BESIDE_DIRECTORY));
    }
  }
  findings
}

#[cfg(test)]
mod tests {
  use super::check_module_files;

  fn flagged(paths: &[&str]) -> Vec<String> {
    let owned: Vec<String> = paths.iter().map(|path| (*path).to_owned()).collect();
    check_module_files(&owned)
      .into_iter()
      .map(|finding| finding.path)
      .collect()
  }

  #[test]
  fn rejects_a_file_beside_its_directory() {
    let found = flagged(&["src/lib.rs", "src/segment.rs", "src/segment/tracks.rs"]);
    assert_eq!(found, vec!["src/segment.rs".to_owned()]);
    let deep = flagged(&["src/segment.rs", "src/segment/inner/tracks.rs"]);
    assert_eq!(deep, vec!["src/segment.rs".to_owned()]);
  }

  #[test]
  fn accepts_directory_modules_and_plain_files() {
    assert_eq!(
      flagged(&[
        "src/lib.rs",
        "src/segment/mod.rs",
        "src/segment/tracks.rs",
        "src/tags.rs",
        "src/tags_extra/mod.rs"
      ]),
      Vec::<String>::new()
    );
    assert_eq!(
      flagged(&["src/main.rs", "src/main/helper.rs", "notes.txt"]),
      Vec::<String>::new()
    );
  }
}
