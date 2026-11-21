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

//! Discovery of Rust files and package directories, skipping build output
//! and hidden directories.

use std::fs;
use std::io;
use std::path::Path;

use crate::constants::paths::{
  HIDDEN_PREFIX, MANIFEST_FILE, NODE_MODULES_DIRECTORY, PATH_SEPARATOR, RUST_EXTENSION, TARGET_DIRECTORY,
};

/// The Rust files and the package directories of a repository, as relative
/// paths with `/`, sorted.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RepositoryFiles {
  /// Every `.rs` file.
  pub rust_files: Vec<String>,
  /// Every directory that holds a `Cargo.toml`; the root is the empty
  /// string.
  pub package_directories: Vec<String>,
}

/// Walks the tree below `root`, skipping `target`, `node_modules` and every
/// hidden directory.
///
/// # Errors
///
/// Returns an error when a directory cannot be read.
pub fn walk_repository(root: &Path) -> io::Result<RepositoryFiles> {
  let mut files = RepositoryFiles::default();
  let mut pending = vec![root.to_path_buf()];
  while let Some(directory) = pending.pop() {
    for entry in fs::read_dir(&directory)? {
      let entry = entry?;
      let name = entry.file_name().to_string_lossy().into_owned();
      let file_type = entry.file_type()?;
      let path = entry.path();
      if file_type.is_dir() {
        let is_skipped = name.starts_with(HIDDEN_PREFIX) || name == TARGET_DIRECTORY || name == NODE_MODULES_DIRECTORY;
        if !is_skipped {
          pending.push(path);
        }
      } else if file_type.is_file() {
        if name == MANIFEST_FILE {
          files.package_directories.push(relative_path(root, &directory));
        }
        if path.extension().is_some_and(|extension| extension == RUST_EXTENSION) {
          files.rust_files.push(relative_path(root, &path));
        }
      }
    }
  }
  files.rust_files.sort();
  files.package_directories.sort();
  Ok(files)
}

/// `path` relative to `root`, with `/` between components.
#[must_use]
pub fn relative_path(root: &Path, path: &Path) -> String {
  let relative = path.strip_prefix(root).unwrap_or(path);
  relative
    .components()
    .map(|component| component.as_os_str().to_string_lossy())
    .collect::<Vec<_>>()
    .join(&PATH_SEPARATOR.to_string())
}

#[cfg(test)]
mod tests {
  use super::{relative_path, walk_repository};
  use std::fs;
  use std::path::Path;

  #[test]
  fn finds_rust_files_and_packages() {
    let root = std::env::temp_dir().join(format!("checks-walk-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    for directory in [
      "crates/library/src/segment",
      "crates/library/target/debug",
      ".hidden",
      "node_modules/package",
    ] {
      fs::create_dir_all(root.join(directory)).unwrap();
    }
    for file in [
      "Cargo.toml",
      "crates/library/Cargo.toml",
      "crates/library/src/lib.rs",
      "crates/library/src/segment/mod.rs",
      "crates/library/src/notes.txt",
      "crates/library/target/debug/generated.rs",
      ".hidden/hidden.rs",
      "node_modules/package/script.rs",
    ] {
      fs::write(root.join(file), "").unwrap();
    }
    let files = walk_repository(&root).unwrap();
    fs::remove_dir_all(&root).unwrap();
    assert_eq!(
      files.rust_files,
      vec!["crates/library/src/lib.rs", "crates/library/src/segment/mod.rs"]
    );
    assert_eq!(files.package_directories, vec!["", "crates/library"]);
  }

  #[test]
  fn reports_a_missing_root() {
    assert!(walk_repository(Path::new("/nonexistent/repository/root")).is_err());
  }

  #[test]
  fn makes_paths_relative() {
    assert_eq!(
      relative_path(Path::new("/repository"), Path::new("/repository/src/lib.rs")),
      "src/lib.rs"
    );
    assert_eq!(
      relative_path(Path::new("/other"), Path::new("src/lib.rs")),
      "src/lib.rs"
    );
  }
}
