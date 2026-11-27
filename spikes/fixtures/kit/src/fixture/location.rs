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

//! Where fixtures and their expectations are: below the fixtures' folder, which is found from the kit's own manifest
//! folder (plan section 7.1), by names that use `/` on every platform. The layout is checked whole: the folder holds
//! format folders only, and each holds fixtures `<name>.hex` and their expectations `<name>.<part>.json` and
//! `<name>.<case>.<part>.json`, with names of lowercase words joined by `-`. Anything else is an error, so a
//! misnamed fixture is never left out unseen.

use std::fs;
use std::path::{Path, PathBuf};

use super::constants::{
  EXPECTATION_EXTENSION, FIXTURE_EXTENSION, FOLDER, MISPLACED, NAME_SEPARATOR, ORPHAN, PARENT, PART_SEPARATOR, PARTS,
};
use super::load::FixtureError;
use crate::syntax::is_hyphenated_name;

/// The fixtures' folder.
#[must_use]
pub fn folder() -> PathBuf {
  Path::new(env!("CARGO_MANIFEST_DIR")).join(PARENT).join(FOLDER)
}

/// The path of the fixture `name`, such as `flac/decoding-example-1`.
#[must_use]
pub fn fixture_path(name: &str) -> PathBuf {
  file_path(&folder(), name, &[FIXTURE_EXTENSION])
}

/// The path of the expectation `part` of the fixture `name` in `case`, or in the default case for `None`:
/// `toy/setup-and-parameters.expected.json`, `toy/setup-and-parameters.split-header.adapters.json`.
#[must_use]
pub fn expectation_path(name: &str, case: Option<&str>, part: &str) -> PathBuf {
  expectation_path_in(&folder(), name, case, part)
}

/// The path of an expectation below `base`.
fn expectation_path_in(base: &Path, name: &str, case: Option<&str>, part: &str) -> PathBuf {
  let suffixes: Vec<&str> = case.into_iter().chain([part, EXPECTATION_EXTENSION]).collect();
  file_path(base, name, &suffixes)
}

/// An expectation file: its fixture, its case, `None` for the default one, and its part.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Expectation {
  /// The fixture's name.
  pub fixture: String,
  /// The case, or `None` for the default one.
  pub case: Option<String>,
  /// The part: `expected`, `forward` or `adapters`.
  pub part: String,
}

/// The fixtures and the expectation files of the fixtures' folder.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Layout {
  /// The fixtures' names, sorted.
  pub fixtures: Vec<String>,
  /// The expectation files, sorted.
  pub expectations: Vec<Expectation>,
}

/// The names of every fixture, sorted.
///
/// # Errors
///
/// Returns the error of a folder that cannot be read, or of a file or folder that the layout does not allow.
pub fn names() -> Result<Vec<String>, FixtureError> {
  layout().map(|layout| layout.fixtures)
}

/// The fixtures and expectation files of the fixtures' folder, checked against the layout.
///
/// # Errors
///
/// Returns the error of a folder that cannot be read, or of a file or folder that the layout does not allow.
pub fn layout() -> Result<Layout, FixtureError> {
  layout_of(&folder())
}

/// The fixtures and expectation files below `base`, checked against the layout, as `layout` checks the fixtures'
/// folder.
///
/// # Errors
///
/// Returns the error of a folder that cannot be read, or of a file or folder that the layout does not allow.
pub fn layout_of(base: &Path) -> Result<Layout, FixtureError> {
  let mut layout = Layout::default();
  for format in entries(base)? {
    let format_name = valid_name(&format, true)?;
    for file in entries(&format)? {
      let file_name = valid_name(&file, false)?;
      let parts: Vec<&str> = file_name.split(PART_SEPARATOR).collect();
      let name = |stem: &str| format!("{format_name}{NAME_SEPARATOR}{stem}");
      match parts.as_slice() {
        [stem, FIXTURE_EXTENSION] if is_hyphenated_name(stem) => layout.fixtures.push(name(stem)),
        [stem, case @ .., part, EXPECTATION_EXTENSION]
          if case.len() <= 1
            && PARTS.contains(part)
            && is_hyphenated_name(stem)
            && case.iter().all(|case| is_hyphenated_name(case)) =>
        {
          layout.expectations.push(Expectation {
            fixture: name(stem),
            case: case.first().map(|case| (*case).to_owned()),
            part: (*part).to_owned(),
          });
        }
        _ => return Err(FixtureError::Misplaced(file, MISPLACED)),
      }
    }
  }
  layout.fixtures.sort();
  layout.expectations.sort();
  if let Some(orphan) = layout
    .expectations
    .iter()
    .find(|expectation| layout.fixtures.binary_search(&expectation.fixture).is_err())
  {
    let path = expectation_path_in(base, &orphan.fixture, orphan.case.as_deref(), &orphan.part);
    return Err(FixtureError::Misplaced(path, ORPHAN));
  }
  Ok(layout)
}

/// The entries of `folder`.
fn entries(folder: &Path) -> Result<Vec<PathBuf>, FixtureError> {
  let unreadable = |error| FixtureError::Unlisted(folder.to_path_buf(), error);
  fs::read_dir(folder)
    .map_err(unreadable)?
    .map(|entry| entry.map(|entry| entry.path()).map_err(unreadable))
    .collect()
}

/// The UTF-8 file name of `path`, which is a folder named as a format when `folder` says so, and a file otherwise.
fn valid_name(path: &Path, folder: bool) -> Result<String, FixtureError> {
  let misplaced = || FixtureError::Misplaced(path.to_path_buf(), MISPLACED);
  let name = path.file_name().and_then(|name| name.to_str()).ok_or_else(misplaced)?;
  let fits = if folder {
    path.is_dir() && is_hyphenated_name(name)
  } else {
    path.is_file()
  };
  fits.then(|| name.to_owned()).ok_or_else(misplaced)
}

/// The path below `base` of `name` with `suffixes` after its last part, each after a `.`.
fn file_path(base: &Path, name: &str, suffixes: &[&str]) -> PathBuf {
  let mut path = base.to_path_buf();
  let mut parts = name.split(NAME_SEPARATOR).peekable();
  while let Some(component) = parts.next() {
    if parts.peek().is_some() {
      path.push(component);
    } else {
      let mut file = component.to_owned();
      for suffix in suffixes {
        file.push(PART_SEPARATOR);
        file.push_str(suffix);
      }
      path.push(file);
    }
  }
  path
}
