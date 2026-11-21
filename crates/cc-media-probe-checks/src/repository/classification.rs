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

//! What kind of file a Rust source file is.

use crate::constants::paths::{
  BENCHMARKS_DIRECTORY, BINARY_ROOT, DIRECTORY_MODULE_ROOT, EXAMPLES_DIRECTORY, LIBRARY_ROOT, PATH_SEPARATOR,
  TESTS_DIRECTORY, TESTS_FILE,
};

/// Whether a file is a module root (R30).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RootKind {
  /// An ordinary file.
  NotARoot,
  /// `lib.rs` or a `mod.rs`.
  ModuleRoot,
  /// `main.rs`, which may also hold `main`.
  BinaryRoot,
}

/// What a file is for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Purpose {
  /// Library, binary or tool code.
  Code,
  /// A `tests.rs`, or a file below a `tests` or `benches` directory.
  Test,
  /// A file below a package's `examples` directory.
  Example,
}

/// A Rust source file and the rules that apply to it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceFile {
  /// The path relative to the repository root, with `/`.
  pub relative_path: String,
  /// Whether it is a module root.
  pub root: RootKind,
  /// What it is for.
  pub purpose: Purpose,
}

impl SourceFile {
  /// `lib.rs`, `main.rs` or a `mod.rs` (R30).
  #[must_use]
  pub fn is_module_root(&self) -> bool {
    self.root != RootKind::NotARoot
  }

  /// `main.rs`, which may also hold `main`.
  #[must_use]
  pub fn is_binary_root(&self) -> bool {
    self.root == RootKind::BinaryRoot
  }

  /// Test code, benchmarks and examples, where literals may stay inline
  /// (R33).
  #[must_use]
  pub fn is_exempt_from_constants(&self) -> bool {
    self.purpose != Purpose::Code
  }
}

/// Classifies `relative_path`; `package_directories` are the directories
/// that hold a `Cargo.toml`, relative and with `/`.
#[must_use]
pub fn classify(relative_path: &str, package_directories: &[String]) -> SourceFile {
  let components: Vec<&str> = relative_path.split(PATH_SEPARATOR).collect();
  let file_name = components.last().copied().unwrap_or_default();
  let directories = components.split_last().map_or_default(|(_, rest)| rest);
  let package_relative = package_relative_components(relative_path, package_directories);
  let root = if file_name == BINARY_ROOT {
    RootKind::BinaryRoot
  } else if [LIBRARY_ROOT, DIRECTORY_MODULE_ROOT].contains(&file_name) {
    RootKind::ModuleRoot
  } else {
    RootKind::NotARoot
  };
  let is_test = file_name == TESTS_FILE
    || directories
      .iter()
      .any(|directory| *directory == TESTS_DIRECTORY || *directory == BENCHMARKS_DIRECTORY);
  let purpose = if is_test {
    Purpose::Test
  } else if package_relative.first() == Some(&EXAMPLES_DIRECTORY) {
    Purpose::Example
  } else {
    Purpose::Code
  };
  SourceFile {
    relative_path: relative_path.to_owned(),
    root,
    purpose,
  }
}

/// The components of `relative_path` below the deepest package directory
/// that contains it.
fn package_relative_components<'path>(relative_path: &'path str, package_directories: &[String]) -> Vec<&'path str> {
  let package = package_directories
    .iter()
    .filter(|directory| {
      directory.is_empty()
        || relative_path
          .strip_prefix(directory.as_str())
          .is_some_and(|rest| rest.starts_with(PATH_SEPARATOR))
    })
    .max_by_key(|directory| directory.len());
  let rest = match package {
    Some(directory) if !directory.is_empty() => relative_path
      .get(directory.len().saturating_add(1)..)
      .unwrap_or(relative_path),
    _ => relative_path,
  };
  rest.split(PATH_SEPARATOR).collect()
}

#[cfg(test)]
mod tests {
  use super::{Purpose, classify};

  fn packages() -> Vec<String> {
    vec![
      String::new(),
      "crates/library".to_owned(),
      "crates/library/examples/nested".to_owned(),
    ]
  }

  #[test]
  fn recognizes_module_roots() {
    let library = classify("crates/library/src/lib.rs", &packages());
    assert!(library.is_module_root() && !library.is_binary_root());
    let binary = classify("crates/library/src/main.rs", &packages());
    assert!(binary.is_module_root() && binary.is_binary_root());
    assert!(classify("crates/library/src/segment/mod.rs", &packages()).is_module_root());
    assert!(!classify("crates/library/src/segment.rs", &packages()).is_module_root());
  }

  #[test]
  fn recognizes_tests_benchmarks_and_examples() {
    let purpose = |path: &str| classify(path, &packages()).purpose;
    assert_eq!(purpose("crates/library/src/tests.rs"), Purpose::Test);
    assert_eq!(purpose("crates/library/tests/golden.rs"), Purpose::Test);
    assert_eq!(purpose("crates/library/tests/common/mod.rs"), Purpose::Test);
    assert_eq!(purpose("crates/library/benches/probe.rs"), Purpose::Test);
    assert_eq!(purpose("crates/library/src/testing.rs"), Purpose::Code);
    assert_eq!(purpose("crates/library/examples/list.rs"), Purpose::Example);
    assert_eq!(purpose("crates/library/examples/nested/src/lib.rs"), Purpose::Code);
    assert_eq!(purpose("examples/top.rs"), Purpose::Example);
    assert_eq!(purpose("crates/library/src/examples/list.rs"), Purpose::Code);
    assert!(classify("tests/golden.rs", &packages()).is_exempt_from_constants());
    assert!(!classify("src/golden.rs", &packages()).is_exempt_from_constants());
  }
}
