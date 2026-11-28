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

//! A folder of its own for one test's files, below Cargo's folder for integration tests: named after the test, the
//! process's identifier and a number in the process, so that neither concurrent runs nor concurrent tests share
//! one, and removed when the test is done. The layout tests shared two fixed folders, and concurrent runs removed
//! each other's files, before the review of plan Phase 3, task 4, found it.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU32, Ordering};

/// The folders that this process has given out.
static FOLDERS: AtomicU32 = AtomicU32::new(0);

/// A test's own folder, removed when dropped.
pub struct Scratch {
  /// The folder.
  pub path: PathBuf,
}

impl Drop for Scratch {
  fn drop(&mut self) {
    let _ = fs::remove_dir_all(&self.path);
  }
}

/// A new, empty folder for the test `name`.
pub fn scratch(name: &str) -> Scratch {
  let number = FOLDERS.fetch_add(1, Ordering::Relaxed);
  let path = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!("{name}-{:08x}-{number:08x}", std::process::id()));
  let _ = fs::remove_dir_all(&path);
  fs::create_dir_all(&path).unwrap();
  Scratch { path }
}
