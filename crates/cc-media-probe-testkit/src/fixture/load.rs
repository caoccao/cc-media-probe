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

//! A fixture read from its file: its header, its assembled bytes and its regions, and the name of the input as a
//! file. Plan Phase 3, task 19, adds the seeds that it gives by name: the whole fixture to the target of its
//! header, unless that is `none`, and each region to the target of its `@seed` (plan section 9.5).

use std::error::Error;
use std::ffi::OsStr;
use std::fmt;
use std::fs;
use std::io;
use std::iter;
use std::path::{Path, PathBuf};

use super::constants::{
  DEFAULT_CASE, LONG_EXPECTATION_NAME, MALFORMED_NAME, NAME_SEPARATOR, NOT_A_FIXTURE, PARTS, UNLISTED, UNREADABLE,
};
use super::location::{expectation_path, folder, listed_fixture_path};
use crate::syntax::{Body, Header, MAXIMUM_FILE_NAME_LENGTH, SyntaxError, is_hyphenated_name, parse};

/// A fixture: its name, what its header states, and its assembled bytes and regions.
///
/// ```
/// # fn main() -> Result<(), cc_media_probe_testkit::fixture::FixtureError> {
/// use cc_media_probe_testkit::fixture::Fixture;
///
/// let fixture = Fixture::load("toy/setup-and-parameters")?;
/// assert_eq!(fixture.file_name(), "setup-and-parameters");
/// assert_eq!(Some(fixture.body.bytes.len()), fixture.header.length);
/// assert_eq!(fixture.header.cases[0].name, "short-reads");
///
/// let text = "description: two fields\nsource: a section\ntarget: toy\noutcome: report\n---\n## A section\n\
///             00 01 # the fields\n";
/// assert_eq!(Fixture::from_text("toy/two-fields", text)?.body.bytes, [0, 1]);
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fixture {
  /// The name, such as `flac/decoding-example-1`.
  pub name: String,
  /// What the header states.
  pub header: Header,
  /// The bytes and regions of the body.
  pub body: Body,
}

/// Why a fixture, or the fixtures' folder, could not be read. An error that has a cause, a file that cannot be read,
/// a text that is not a fixture or a folder that cannot be listed, gives it as its `source()` and leaves it out of
/// its message, as the library's errors do.
///
/// ```
/// use std::error::Error;
///
/// use cc_media_probe_testkit::fixture::{Fixture, FixtureError};
///
/// let error = Fixture::load("toy/Setup").unwrap_err();
/// assert!(matches!(&error, FixtureError::MalformedName(name) if name == "toy/Setup"));
/// assert!(error.to_string().starts_with("toy/Setup: a fixture's name is `<format>/<name>`"));
/// assert!(error.source().is_none());
///
/// let text = "description: two fields\nsource: a section\ntarget: toy\noutcome: report\n---\n## A section\n\
///             00 0 # half a byte\n";
/// let error = Fixture::from_text("toy/two-fields", text).unwrap_err();
/// assert!(matches!(&error, FixtureError::Syntax(name, _) if name == "toy/two-fields"));
/// assert_eq!(error.to_string(), "toy/two-fields: the text is not a fixture");
/// let cause = error.source().map(ToString::to_string);
/// assert_eq!(cause.as_deref(), Some("7: hex bytes are pairs of digits: 0"));
/// ```
#[derive(Debug)]
pub enum FixtureError {
  /// The file could not be read.
  Unreadable(String, io::Error),
  /// The text is not a fixture.
  Syntax(String, SyntaxError),
  /// A name that is not `<format>/<name>` of lowercase words joined by `-`.
  MalformedName(String),
  /// A folder that could not be listed.
  Unlisted(PathBuf, io::Error),
  /// A file or folder that the layout does not allow, with why.
  Misplaced(PathBuf, &'static str),
  /// A fixture, by its name, with a case whose expectation files would have a name longer than a file system
  /// allows, or `None` for the default case.
  LongExpectationName(String, Option<String>),
}

impl Fixture {
  /// Reads and assembles the fixture `name` from the fixtures' folder.
  ///
  /// # Errors
  ///
  /// Returns the error of a name that the layout does not allow; of a symbolic link on the way to the file, or of
  /// an entry there that is not the layout's folder or regular file; of a fixture that no entry names by its exact
  /// spelling, as not found; of a file that cannot be read or is not a fixture; or of a case whose expectation
  /// files no file system can hold.
  pub fn load(name: &str) -> Result<Fixture, FixtureError> {
    Fixture::load_from(&folder(), name)
  }

  /// Reads and assembles the fixture `name` below `base`, as `load` reads it from the fixtures' folder. The file is
  /// opened only once `layout_of` would list it: `base` is no symbolic link, and the format's folder and the
  /// fixture's file are entries of their folders spelled exactly as `name` spells them, a folder and a regular
  /// file, neither a link.
  ///
  /// # Errors
  ///
  /// Returns the errors of `load`, and the error of a folder on the way that cannot be listed.
  pub fn load_from(base: &Path, name: &str) -> Result<Fixture, FixtureError> {
    let path = listed_fixture_path(base, valid_name(name)?)?;
    let text = fs::read_to_string(path).map_err(|error| FixtureError::Unreadable(name.to_owned(), error))?;
    Fixture::from_text(name, &text)
  }

  /// Assembles the fixture `name` from `text`, as `load` assembles its file. Each case's expectation files, the
  /// default case's among them, must have names of at most 255 bytes, which most file systems allow, so that a
  /// case name that the syntax accepts never fails only when expectations are written, as the review of plan Phase
  /// 3, task 4, found of a case name of 240 bytes.
  ///
  /// # Errors
  ///
  /// Returns the error of a name that the layout does not allow, of a text that is not a fixture, or of a case
  /// whose expectation files no file system can hold.
  pub fn from_text(name: &str, text: &str) -> Result<Fixture, FixtureError> {
    let name = valid_name(name)?;
    let parsed = parse(text).map_err(|error| FixtureError::Syntax(name.to_owned(), error))?;
    let cases = iter::once(None).chain(parsed.header.cases.iter().map(|case| Some(case.name.as_str())));
    for case in cases {
      let longest = PARTS
        .iter()
        .filter_map(|part| expectation_path(name, case, part).file_name().map(OsStr::len))
        .max();
      if longest.is_some_and(|length| length > MAXIMUM_FILE_NAME_LENGTH) {
        return Err(FixtureError::LongExpectationName(
          name.to_owned(),
          case.map(str::to_owned),
        ));
      }
    }
    Ok(Fixture {
      name: name.to_owned(),
      header: parsed.header,
      body: parsed.body,
    })
  }

  /// The name of the input as a file: the header's `file_name`, or else the fixture's own name after its format,
  /// without an extension, so that it suggests no format: `setup-and-parameters`.
  #[must_use]
  pub fn file_name(&self) -> &str {
    self.header.file_name.as_deref().unwrap_or_else(|| {
      self
        .name
        .rsplit_once(NAME_SEPARATOR)
        .map_or(self.name.as_str(), |(_, stem)| stem)
    })
  }
}

impl fmt::Display for FixtureError {
  /// Writes the fixture's name or the path, and the message, without the cause that `source()` gives:
  /// `flac/decoding-example-1: the text is not a fixture`. The message repeated the cause before the third review
  /// of plan Phase 3, task 4, asked for the source, which an error that gives it leaves out of its message.
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      FixtureError::Unreadable(name, _) => write!(formatter, "{name}: {UNREADABLE}"),
      FixtureError::Syntax(name, _) => write!(formatter, "{name}: {NOT_A_FIXTURE}"),
      FixtureError::MalformedName(name) => write!(formatter, "{name}: {MALFORMED_NAME}"),
      FixtureError::Unlisted(path, _) => write!(formatter, "{}: {UNLISTED}", path.display()),
      FixtureError::Misplaced(path, message) => write!(formatter, "{}: {message}", path.display()),
      FixtureError::LongExpectationName(name, Some(case)) => {
        write!(formatter, "{name}: {case}: {LONG_EXPECTATION_NAME}")
      }
      FixtureError::LongExpectationName(name, None) => {
        write!(formatter, "{name}: {DEFAULT_CASE}: {LONG_EXPECTATION_NAME}")
      }
    }
  }
}

/// `name` when it is `<format>/<name>` of lowercase words joined by `-`, which the layout allows.
fn valid_name(name: &str) -> Result<&str, FixtureError> {
  let valid = name
    .split_once(NAME_SEPARATOR)
    .is_some_and(|(format, stem)| is_hyphenated_name(format) && is_hyphenated_name(stem));
  if valid {
    Ok(name)
  } else {
    Err(FixtureError::MalformedName(name.to_owned()))
  }
}

impl Error for FixtureError {
  /// The cause of a file that cannot be read, of a text that is not a fixture and of a folder that cannot be
  /// listed, which the third review of plan Phase 3, task 4, found hidden.
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match self {
      FixtureError::Unreadable(_, error) | FixtureError::Unlisted(_, error) => Some(error),
      FixtureError::Syntax(_, error) => Some(error),
      _ => None,
    }
  }
}
