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
//! misnamed fixture is never left out unseen. Neither the folder nor anything in it is a symbolic link, each taken
//! by its own type rather than what it links to, since writing expectations would follow a link out of the folder,
//! as the review of plan Phase 3, task 4, found. A fixture loaded by its name meets the same rules, from the folder
//! to its file: its second review found that a direct load followed a link, and its third that it opened
//! `toy/Sample.hex` for `toy/sample` on a file system that ignores case, and a FIFO, which blocked it. So each entry
//! on the way is looked up in its folder's listing, by its exact spelling, and must be a folder or a regular file of
//! the layout's before the file is opened.

use std::fs::{self, FileType};
use std::io::{self, ErrorKind};
use std::path::{Path, PathBuf};

use super::constants::{
  ABSENT, EXPECTATION_EXTENSION, FIXTURE_EXTENSION, FOLDER, LINKED, MISPLACED, NAME_SEPARATOR, ORPHAN, PART_SEPARATOR,
  PARTS,
};
use super::load::FixtureError;
use crate::syntax::is_hyphenated_name;

/// The fixtures' folder.
///
/// ```
/// use cc_media_probe_testkit::fixture::{fixture_path, folder};
///
/// assert!(folder().ends_with("fixtures"));
/// assert!(fixture_path("toy/setup-and-parameters").starts_with(folder()));
/// ```
#[must_use]
pub fn folder() -> PathBuf {
  Path::new(env!("CARGO_MANIFEST_DIR")).join(FOLDER)
}

/// The path of the fixture `name`, such as `flac/decoding-example-1`.
///
/// ```
/// use cc_media_probe_testkit::fixture::{fixture_path, folder};
///
/// let path = fixture_path("flac/decoding-example-1");
/// assert_eq!(path, folder().join("flac").join("decoding-example-1.hex"));
/// assert!(path.is_file());
/// ```
#[must_use]
pub fn fixture_path(name: &str) -> PathBuf {
  file_path(&folder(), name, &[FIXTURE_EXTENSION])
}

/// The path below `base` of the fixture `name`, whose format has been checked, once `layout_of` would list it, from
/// the top: `base` is no symbolic link, and the format's folder and the fixture's file are entries of their folders
/// by the exact spelling of `name`, a folder and a regular file, neither a link. A fixture that no entry names so
/// is unreadable as not found.
pub(super) fn listed_fixture_path(base: &Path, name: &str) -> Result<PathBuf, FixtureError> {
  refuse_link(base, |error| FixtureError::Unlisted(base.to_path_buf(), error))?;
  let path = file_path(base, name, &[FIXTURE_EXTENSION]);
  let format_folder = path.parent().unwrap_or(base);
  for (place, folder, parent) in [(format_folder, true, base), (path.as_path(), false, format_folder)] {
    let (entry, kind) = entries(parent)?
      .into_iter()
      .find(|(entry, _)| entry.file_name() == place.file_name())
      .ok_or_else(|| FixtureError::Unreadable(name.to_owned(), io::Error::new(ErrorKind::NotFound, ABSENT)))?;
    valid_name(&entry, kind, folder)?;
  }
  Ok(path)
}

/// Refuses `path` when it is a symbolic link, taken by its own type; `unreadable` makes the error of a path whose
/// type cannot be read.
fn refuse_link(path: &Path, unreadable: impl FnOnce(io::Error) -> FixtureError) -> Result<(), FixtureError> {
  if fs::symlink_metadata(path).map_err(unreadable)?.file_type().is_symlink() {
    Err(FixtureError::Misplaced(path.to_path_buf(), LINKED))
  } else {
    Ok(())
  }
}

/// The path of the expectation `part` of the fixture `name` in `case`, or in the default case for `None`:
/// `toy/setup-and-parameters.expected.json`, `toy/setup-and-parameters.short-reads.adapters.json`.
///
/// ```
/// use cc_media_probe_testkit::fixture::{expectation_path, folder};
///
/// let toy = folder().join("toy");
/// assert_eq!(
///     expectation_path("toy/setup-and-parameters", None, "expected"),
///     toy.join("setup-and-parameters.expected.json"),
/// );
/// assert_eq!(
///     expectation_path("toy/setup-and-parameters", Some("short-reads"), "adapters"),
///     toy.join("setup-and-parameters.short-reads.adapters.json"),
/// );
/// ```
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
///
/// ```
/// # fn main() -> Result<(), cc_media_probe_testkit::fixture::FixtureError> {
/// use cc_media_probe_testkit::fixture::{Expectation, expectation_path, layout};
///
/// let short_reads = Expectation {
///     fixture: "toy/setup-and-parameters".to_owned(),
///     case: Some("short-reads".to_owned()),
///     part: "adapters".to_owned(),
/// };
/// assert!(layout()?.expectations.contains(&short_reads));
/// let path = expectation_path(&short_reads.fixture, short_reads.case.as_deref(), &short_reads.part);
/// assert!(path.is_file());
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Expectation {
  /// The fixture's name.
  pub fixture: String,
  /// The case, or `None` for the default one.
  pub case: Option<String>,
  /// The part: `expected`, `forward`, `outcomes` or `adapters`.
  pub part: String,
}

/// The fixtures and the expectation files of the fixtures' folder.
///
/// ```
/// # fn main() -> Result<(), cc_media_probe_testkit::fixture::FixtureError> {
/// use cc_media_probe_testkit::fixture::layout;
///
/// let layout = layout()?;
/// assert!(layout.fixtures.iter().any(|name| name == "flac/decoding-example-1"));
/// assert!(layout.expectations.iter().all(|file| layout.fixtures.contains(&file.fixture)));
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Layout {
  /// The fixtures' names, sorted.
  pub fixtures: Vec<String>,
  /// The expectation files, sorted.
  pub expectations: Vec<Expectation>,
}

/// The names of every fixture, sorted.
///
/// ```
/// # fn main() -> Result<(), cc_media_probe_testkit::fixture::FixtureError> {
/// use cc_media_probe_testkit::fixture::{Fixture, names};
///
/// for name in names()? {
///     assert!(!Fixture::load(&name)?.body.bytes.is_empty(), "{name}");
/// }
/// # Ok(())
/// # }
/// ```
///
/// # Errors
///
/// Returns the error of a folder that cannot be read, or of a file or folder that the layout does not allow.
pub fn names() -> Result<Vec<String>, FixtureError> {
  layout().map(|layout| layout.fixtures)
}

/// The fixtures and expectation files of the fixtures' folder, checked against the layout.
///
/// ```
/// # fn main() -> Result<(), cc_media_probe_testkit::fixture::FixtureError> {
/// use cc_media_probe_testkit::fixture::{layout, names};
///
/// let layout = layout()?;
/// assert_eq!(layout.fixtures, names()?);
/// assert!(layout.expectations.iter().any(|file| file.case.is_none() && file.part == "expected"));
/// # Ok(())
/// # }
/// ```
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
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use std::fs;
///
/// use cc_media_probe_testkit::fixture::{FixtureError, folder, layout, layout_of};
///
/// assert_eq!(layout_of(&folder())?, layout()?);
///
/// // A folder of the example's own, which nothing else writes, so that it holds what the example puts there.
/// let base = std::env::temp_dir().join(format!("cc-media-probe-testkit-layout-of-{}", std::process::id()));
/// let _ = fs::remove_dir_all(&base);
/// assert!(matches!(layout_of(&base), Err(FixtureError::Unlisted(path, _)) if path == base));
/// fs::create_dir_all(base.join("toy"))?;
/// fs::write(base.join("toy").join("sample.hex"), "")?;
/// fs::write(base.join("toy").join("sample.expected.json"), "")?;
/// let listed = layout_of(&base)?;
/// assert_eq!(listed.fixtures, ["toy/sample"]);
/// assert_eq!(listed.expectations.len(), 1);
/// fs::remove_dir_all(&base)?;
/// # Ok(())
/// # }
/// ```
///
/// # Errors
///
/// Returns the error of a folder that cannot be read, or of a file or folder that the layout does not allow, a
/// symbolic link among them.
pub fn layout_of(base: &Path) -> Result<Layout, FixtureError> {
  refuse_link(base, |error| FixtureError::Unlisted(base.to_path_buf(), error))?;
  let mut layout = Layout::default();
  for (format, format_type) in entries(base)? {
    let format_name = valid_name(&format, format_type, true)?;
    for (file, file_type) in entries(&format)? {
      let file_name = valid_name(&file, file_type, false)?;
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

/// The entries of `folder`, each with its own type, which for a symbolic link is the link's and never its target's.
fn entries(folder: &Path) -> Result<Vec<(PathBuf, FileType)>, FixtureError> {
  let unreadable = |error| FixtureError::Unlisted(folder.to_path_buf(), error);
  fs::read_dir(folder)
    .map_err(unreadable)?
    .map(|entry| {
      let entry = entry.map_err(unreadable)?;
      Ok((entry.path(), entry.file_type().map_err(unreadable)?))
    })
    .collect()
}

/// The UTF-8 file name of `path`, of type `kind`, which is a folder named as a format when `folder` says so, and a
/// file otherwise, and never a symbolic link.
fn valid_name(path: &Path, kind: FileType, folder: bool) -> Result<String, FixtureError> {
  if kind.is_symlink() {
    return Err(FixtureError::Misplaced(path.to_path_buf(), LINKED));
  }
  let misplaced = || FixtureError::Misplaced(path.to_path_buf(), MISPLACED);
  let name = path.file_name().and_then(|name| name.to_str()).ok_or_else(misplaced)?;
  let fits = if folder {
    kind.is_dir() && is_hyphenated_name(name)
  } else {
    kind.is_file()
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
