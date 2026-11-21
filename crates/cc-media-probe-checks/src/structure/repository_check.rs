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

//! The structure check over the whole repository.

use std::fs;
use std::io;
use std::path::Path;

use crate::findings::{Finding, Rule};
use crate::repository::{classify, repository_file, walk_repository};
use crate::rules::check_module_files;
use crate::structure::constants::NOT_UTF8;
use crate::structure::source_check::check_source;

/// Checks every Rust file below `root` and the module files of R30. The
/// findings are sorted by path, line and rule.
///
/// # Errors
///
/// Returns an error when a directory or file cannot be read.
pub fn check_repository(root: &Path) -> io::Result<Vec<Finding>> {
  let files = walk_repository(root)?;
  let mut findings = check_module_files(&files.rust_files);
  for relative in &files.rust_files {
    let file = classify(relative, &files.package_directories);
    let bytes = fs::read(repository_file(root, relative))?;
    match String::from_utf8(bytes) {
      Ok(source) => findings.extend(check_source(&file, &source)),
      Err(_) => findings.push(Finding::new(relative, 0, Rule::Syntax, NOT_UTF8)),
    }
  }
  findings.sort();
  Ok(findings)
}

#[cfg(test)]
mod tests {
  use super::check_repository;
  use crate::findings::Rule;
  use std::fs;

  #[test]
  fn checks_every_file_and_the_module_files() {
    let root = std::env::temp_dir().join(format!("checks-repository-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(root.join("src/segment")).unwrap();
    fs::write(root.join("Cargo.toml"), "").unwrap();
    fs::write(root.join("src/segment.rs"), "").unwrap();
    fs::write(root.join("src/segment/tracks.rs"), [0xff, 0xfe]).unwrap();
    let findings = check_repository(&root).unwrap();
    fs::remove_dir_all(&root).unwrap();
    let rules: Vec<(String, Rule)> = findings
      .into_iter()
      .map(|finding| (finding.path, finding.rule))
      .collect();
    assert_eq!(
      rules,
      vec![
        ("src/segment.rs".to_owned(), Rule::ModuleFile),
        ("src/segment.rs".to_owned(), Rule::Header),
        ("src/segment/tracks.rs".to_owned(), Rule::Syntax),
      ]
    );
  }
}
