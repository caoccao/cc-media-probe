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

//! The folder of one case's files (`docs/design/test-kit.md` section 4.2): below Cargo's folder for integration
//! tests, `expectations`, then a component for the run, the process's identifier, and one for the case, its number
//! in the process, each of 8 hexadecimal digits, so that concurrent runs and cases never share a folder and no name
//! of a fixture or a case counts. Components of `_` pad it to 512 bytes, at most 255 each, so that each path below
//! it has the same charge on every machine. A file name of 255 bytes below it stays under the 1,024 bytes of a path
//! on macOS, and the standard library gives Windows the long form of a path beyond 260 characters by itself.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU32, Ordering};

/// The length of every case's folder, in bytes of its display string.
pub const FOLDER_LENGTH: usize = 512;
/// The most bytes of one component of the folder.
pub const MAXIMUM_COMPONENT: usize = 255;
/// The width, in hexadecimal digits, of the folder's components for the run and for the case.
const COMPONENT_WIDTH: usize = 8;

/// The cases that this process has given a folder, so that each has its own.
static CASES: AtomicU32 = AtomicU32::new(0);

/// The folder of one case: the case's own component, removed when the case is done, and the padded folder below it.
pub struct CaseFolder {
  /// The case's component below the run's.
  pub root: PathBuf,
  /// The padded folder that the case writes its files into.
  pub path: PathBuf,
}

impl Drop for CaseFolder {
  fn drop(&mut self) {
    let _ = fs::remove_dir_all(&self.root);
  }
}

/// A folder of its own for the files of one case. The folder must be valid Unicode, so that no path below it is
/// lossy, and short enough to pad; otherwise the test fails with a message that names `CARGO_TARGET_DIR`.
pub fn case_folder() -> CaseFolder {
  let width = COMPONENT_WIDTH;
  let run = format!("{:0width$x}", std::process::id());
  let mut component = format!("{:0width$x}", CASES.fetch_add(1, Ordering::Relaxed));
  let mut path = Path::new(env!("CARGO_TARGET_TMPDIR")).join("expectations").join(run);
  let mut root = None;
  let length = path
    .to_str()
    .unwrap_or_else(|| panic!("{}: the folder for tests is valid Unicode", path.display()))
    .len();
  let mut remaining = FOLDER_LENGTH
    .checked_sub(length)
    .filter(|remaining| *remaining > component.len())
    .unwrap_or_else(|| {
      panic!(
        "{}: no room below it for a folder of {FOLDER_LENGTH} bytes; set CARGO_TARGET_DIR to a shorter folder",
        path.display()
      )
    });
  while remaining > 0 {
    let available = remaining.saturating_sub(1);
    let mut taken = available.min(MAXIMUM_COMPONENT);
    if available.saturating_sub(taken) == 1 {
      taken = taken.saturating_sub(1);
    }
    component.push_str(&"_".repeat(taken.saturating_sub(component.len())));
    path.push(&component);
    // The case's own component is the first, padded as the folder holds it.
    root.get_or_insert_with(|| path.clone());
    remaining = available.saturating_sub(taken);
    component = String::new();
  }
  let root = root.unwrap();
  let _ = fs::remove_dir_all(&root);
  fs::create_dir_all(&path).unwrap();
  CaseFolder { root, path }
}

#[test]
fn every_case_folder_has_the_same_length_and_one_of_its_own() {
  let folders = [case_folder(), case_folder()];
  assert_ne!(folders[0].path, folders[1].path);
  for folder in &folders {
    assert_eq!(folder.path.to_str().unwrap().len(), FOLDER_LENGTH);
    assert!(folder.path.is_dir());
    let below = folder
      .path
      .strip_prefix(Path::new(env!("CARGO_TARGET_TMPDIR")))
      .unwrap();
    assert!(below.iter().all(|component| component.len() <= MAXIMUM_COMPONENT));
    assert!(folder.path.starts_with(&folder.root));
    fs::write(folder.path.join("written.bin"), [0]).unwrap();
  }
  // The review of plan Phase 3, task 4, found the root kept before its padding, so that the populated folder stayed.
  let written: Vec<_> = folders
    .iter()
    .map(|folder| (folder.root.clone(), folder.path.clone()))
    .collect();
  drop(folders);
  for (root, path) in written {
    assert!(!path.exists() && !root.exists(), "{}", path.display());
  }
}
