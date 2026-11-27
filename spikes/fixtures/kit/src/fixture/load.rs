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

//! A fixture read from its file, and the seeds it gives: the whole fixture to the target of its header, unless that
//! is `none`, and each region to the target of its `@seed` (plan section 9.5).

use std::fmt;
use std::fs;
use std::io;
use std::path::PathBuf;

use super::constants::{MALFORMED_NAME, NAME_SEPARATOR, REGION_SEPARATOR, UNREADABLE};
use super::location::fixture_path;
use crate::syntax::{Body, Header, Route, SyntaxError, is_hyphenated_name, parse};

/// A fixture: its name, what its header states, and its assembled bytes and regions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fixture {
  /// The name, such as `flac/decoding-example-1`.
  pub name: String,
  /// What the header states.
  pub header: Header,
  /// The bytes and regions of the body.
  pub body: Body,
}

/// A seed: its name, where it goes and its bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Seed<'fixture> {
  /// The fixture's name for the whole fixture, and the name with `#` and the region's number, counted from 1, for
  /// a region: `toy/setup-and-parameters#1`.
  pub name: String,
  /// Where it goes.
  pub route: &'fixture Route,
  /// Its bytes.
  pub payload: &'fixture [u8],
}

/// Why a fixture, or the fixtures' folder, could not be read.
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
}

impl Fixture {
  /// Reads and assembles the fixture `name`.
  ///
  /// # Errors
  ///
  /// Returns the error of a name that the layout does not allow, or of a file that cannot be read or is not a
  /// fixture.
  pub fn load(name: &str) -> Result<Fixture, FixtureError> {
    let valid = name
      .split_once(NAME_SEPARATOR)
      .is_some_and(|(format, stem)| is_hyphenated_name(format) && is_hyphenated_name(stem));
    if !valid {
      return Err(FixtureError::MalformedName(name.to_owned()));
    }
    let text =
      fs::read_to_string(fixture_path(name)).map_err(|error| FixtureError::Unreadable(name.to_owned(), error))?;
    let parsed = parse(&text).map_err(|error| FixtureError::Syntax(name.to_owned(), error))?;
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

  /// The seeds of the fixture: the whole fixture first, unless its target is `none`, then each region in order.
  #[must_use]
  pub fn seeds(&self) -> Vec<Seed<'_>> {
    let whole = self.header.route.as_ref().map(|route| Seed {
      name: self.name.clone(),
      route,
      payload: &self.body.bytes,
    });
    let regions = self.body.regions.iter().zip(1_usize..).filter_map(|(region, number)| {
      Some(Seed {
        name: format!("{}{REGION_SEPARATOR}{number}", self.name),
        route: &region.route,
        payload: self.body.bytes.get(region.start..region.end)?,
      })
    });
    whole.into_iter().chain(regions).collect()
  }
}

impl fmt::Display for FixtureError {
  /// Writes the fixture's name or the path, and the message: `flac/decoding-example-1: 12: hex bytes are pairs of
  /// digits: 0a0`.
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      FixtureError::Unreadable(name, error) => write!(formatter, "{name}: {UNREADABLE}: {error}"),
      FixtureError::Syntax(name, error) => write!(formatter, "{name}: {error}"),
      FixtureError::MalformedName(name) => write!(formatter, "{name}: {MALFORMED_NAME}"),
      FixtureError::Unlisted(path, error) => write!(formatter, "{}: {error}", path.display()),
      FixtureError::Misplaced(path, message) => write!(formatter, "{}: {message}", path.display()),
    }
  }
}

impl std::error::Error for FixtureError {}
