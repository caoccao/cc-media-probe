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

//! The corpus that the seed check writes when the environment variable `CC_MEDIA_PROBE_WRITE_CORPUS` asks for it,
//! after every seed passed (plan section 9.5): each seed in its target's input encoding, in
//! `fuzz/corpus/<target>/`, as `seed.<format>.<name>` for a whole fixture and `seed.<format>.<name>.<region>` for a
//! region. Fixture names never hold `.`, so no two seeds share a file, which the writer checks before it writes. A
//! file that starts with `seed.` is the writer's: one that no seed writes any more is removed, and every other file,
//! such as the inputs that the fuzzer found, is kept. A corpus that does not exist yet is empty; any other error of
//! the listing fails the writer with its path, so that an unreadable folder never keeps stale seeds unseen.

use std::collections::BTreeSet;
use std::fs::{self, DirEntry};
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

/// The variable that makes the seed check write the corpus.
pub const WRITE_CORPUS: &str = "CC_MEDIA_PROBE_WRITE_CORPUS";
/// The prefix of the files that the writer owns.
const PREFIX: &str = "seed.";

/// A checked seed: its target's name, its own name and its input.
pub struct Checked {
  /// The target's name, the folder of the seed.
  pub target: &'static str,
  /// The seed's name, such as `toy/setup-and-parameters#1`.
  pub name: String,
  /// The input, in the target's encoding.
  pub input: Vec<u8>,
}

/// Whether the corpus is written in this run.
pub fn writing() -> bool {
  std::env::var_os(WRITE_CORPUS).is_some()
}

/// The corpus folder of the fuzz workspace.
pub fn folder() -> PathBuf {
  Path::new(env!("CARGO_MANIFEST_DIR"))
    .join("..")
    .join("fuzz")
    .join("corpus")
}

/// The file name of the seed `name`: `seed.` and the name with `.` in place of `/` and `#`.
pub fn file_name(name: &str) -> String {
  format!("{PREFIX}{}", name.replace(['/', '#'], "."))
}

/// Writes `seeds` below `corpus`, and removes each file of the writer that no seed writes.
pub fn write(corpus: &Path, seeds: &[Checked]) {
  let mut paths = BTreeSet::new();
  for seed in seeds {
    let path = corpus.join(seed.target).join(file_name(&seed.name));
    assert!(paths.insert(path.clone()), "{}: two seeds write it", path.display());
  }
  for target in entries(corpus) {
    let is_folder = target
      .file_type()
      .unwrap_or_else(|error| panic!("{}: {error}", target.path().display()))
      .is_dir();
    if !is_folder {
      continue;
    }
    for file in entries(&target.path()) {
      let owned = file.file_name().to_str().is_some_and(|name| name.starts_with(PREFIX));
      if owned && !paths.contains(&file.path()) {
        fs::remove_file(file.path()).unwrap_or_else(|error| panic!("{}: {error}", file.path().display()));
      }
    }
  }
  for seed in seeds {
    let target = corpus.join(seed.target);
    fs::create_dir_all(&target).unwrap();
    fs::write(target.join(file_name(&seed.name)), &seed.input).unwrap();
  }
}

/// The entries of `folder`, none for a folder that does not exist; any other error fails with the path.
fn entries(folder: &Path) -> Vec<DirEntry> {
  let listing = match fs::read_dir(folder) {
    Ok(listing) => listing,
    Err(error) if error.kind() == ErrorKind::NotFound => return Vec::new(),
    Err(error) => panic!("{}: {error}", folder.display()),
  };
  listing
    .map(|entry| entry.unwrap_or_else(|error| panic!("{}: {error}", folder.display())))
    .collect()
}
