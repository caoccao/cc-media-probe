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

//! Tests that belong to the crate root, and the temporary directory that the tests of files share.

use std::fs;
use std::io;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

/// The number of the next temporary directory that this test process tries.
static NEXT_DIRECTORY: AtomicU64 = AtomicU64::new(0);

/// A directory that a test creates in the system's temporary directory and removes when it is dropped. The
/// directory did not exist before, so nothing that another run or another user left there can stand in it: a
/// name inside it names no file until the test writes one.
pub(crate) struct TemporaryDirectory {
  path: PathBuf,
}

impl TemporaryDirectory {
  /// Creates the directory exclusively, trying the next name while one already exists.
  pub(crate) fn new() -> io::Result<TemporaryDirectory> {
    let mut last_error = io::Error::from(io::ErrorKind::AlreadyExists);
    for _ in 0..1_000 {
      let number = NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed);
      let path = std::env::temp_dir().join(format!("cc-media-probe-test-{}-{number}", std::process::id()));
      match fs::create_dir(&path) {
        Ok(()) => return Ok(TemporaryDirectory { path }),
        Err(error) if error.kind() == io::ErrorKind::AlreadyExists => last_error = error,
        Err(error) => return Err(error),
      }
    }
    Err(last_error)
  }

  /// The path of `name` inside the directory.
  pub(crate) fn join(&self, name: &str) -> PathBuf {
    self.path.join(name)
  }
}

impl Drop for TemporaryDirectory {
  fn drop(&mut self) {
    // A directory that cannot be removed stays behind, and fails no test: the next run takes another name.
    let _ = fs::remove_dir_all(&self.path);
  }
}

#[test]
fn creates_a_new_directory_each_time() -> io::Result<()> {
  let first = TemporaryDirectory::new()?;
  let second = TemporaryDirectory::new()?;
  assert_ne!(first.path, second.path);
  assert!(first.path.is_dir());
  assert!(!first.join("movie.mkv").exists());
  let path = first.path.clone();
  drop(first);
  assert!(!path.exists());
  Ok(())
}

/// The guard of the repository marker (plan section 7.1, option A). It is
/// compiled only when the marker is absent, and fails when the test kit sits
/// beside the crate: then this is a repository build whose compiler flags
/// were replaced, typically by a `RUSTFLAGS` variable.
#[cfg(not(cc_media_probe_repository))]
#[test]
fn repository_builds_carry_the_marker() {
  let test_kit = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../cc-media-probe-testkit/Cargo.toml");
  assert!(
    !test_kit.exists(),
    "the repository marker cc_media_probe_repository is missing; unset RUSTFLAGS, which replaces the flags of .cargo/config.toml"
  );
}
