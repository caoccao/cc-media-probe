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

//! The header of a fixture: `name: value` lines up to the line `---`, with blank lines and `#` comments between
//! them.

use super::case::{Case, Condition, parse_case, parse_default};
use super::constants::{
  CASE, COMMENT, DECIMAL_RADIX, DEFAULT, DESCRIPTION, ERRATUM, FILE_NAME, LENGTH, MEMBER_SEPARATOR, SOURCE,
};
use super::error::{SyntaxError, SyntaxErrorKind};
use super::names::file_name;
use super::route::{Route, RouteMembers, once};

/// What a fixture's header states.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header {
  /// What the fixture holds, in one line.
  pub description: String,
  /// The specification and section that the fixture comes from as a whole.
  pub source: String,
  /// The errata that the fixture depends on, in the order given.
  pub errata: Vec<String>,
  /// Where the whole fixture goes as a seed, or `None` for the target `none`.
  pub route: Option<Route>,
  /// The name of the input as a file, when the header states one.
  pub file_name: Option<String>,
  /// The conditions of the default case, which a `default:` line gives; empty without one.
  pub default_conditions: Vec<Condition>,
  /// The cases beyond the default conditions, in the order given.
  pub cases: Vec<Case>,
  /// How many bytes the body assembles to, when the header states it.
  pub length: Option<usize>,
}

/// The members of a header while they are read.
#[derive(Default)]
struct Members {
  /// The description, once read.
  description: Option<String>,
  /// The source, once read.
  source: Option<String>,
  /// The errata so far.
  errata: Vec<String>,
  /// The route's members so far.
  route: RouteMembers,
  /// The file name, once read.
  file_name: Option<String>,
  /// The default case's conditions, once read.
  default_conditions: Option<Vec<Condition>>,
  /// The cases so far.
  cases: Vec<Case>,
  /// The length, once read.
  length: Option<usize>,
}

impl Members {
  /// Takes the member `name` with `value`.
  fn take(&mut self, name: &str, value: &str) -> Result<(), SyntaxErrorKind> {
    match name {
      DESCRIPTION => once(&mut self.description, name, value.to_owned()),
      SOURCE => once(&mut self.source, name, value.to_owned()),
      ERRATUM => {
        self.errata.push(value.to_owned());
        Ok(())
      }
      FILE_NAME => file_name(value).and_then(|value| once(&mut self.file_name, name, value)),
      DEFAULT => parse_default(value).and_then(|value| once(&mut self.default_conditions, name, value)),
      CASE => {
        let case = parse_case(value)?;
        if self.cases.iter().any(|known| known.name == case.name) {
          return Err(SyntaxErrorKind::RepeatedCase(case.name));
        }
        self.cases.push(case);
        Ok(())
      }
      LENGTH => decimal(value).and_then(|value| once(&mut self.length, name, value)),
      _ => match self.route.take(name, value) {
        Ok(true) => Ok(()),
        Ok(false) => Err(SyntaxErrorKind::UnknownMember(name.to_owned())),
        Err(error) => Err(error),
      },
    }
  }
}

/// Reads a header from its lines, each with its number.
pub(crate) fn parse_header<'text>(
  lines: impl IntoIterator<Item = (usize, &'text str)>,
  end_line: usize,
) -> Result<Header, SyntaxError> {
  let mut members = Members::default();
  for (number, line) in lines {
    let trimmed = line.trim();
    if trimmed.is_empty() || trimmed.starts_with(COMMENT) {
      continue;
    }
    let (name, value) = trimmed
      .split_once(MEMBER_SEPARATOR)
      .map(|(name, value)| (name.trim(), value.trim()))
      .filter(|(name, value)| !name.is_empty() && !value.is_empty())
      .ok_or_else(|| SyntaxErrorKind::MalformedMember.at(number))?;
    members.take(name, value).map_err(|kind| kind.at(number))?;
  }
  let missing = |name| SyntaxErrorKind::MissingMember(name).at(end_line);
  Ok(Header {
    description: members.description.ok_or_else(|| missing(DESCRIPTION))?,
    source: members.source.ok_or_else(|| missing(SOURCE))?,
    errata: members.errata,
    route: members.route.finish_header().map_err(|kind| kind.at(end_line))?,
    file_name: members.file_name,
    default_conditions: members.default_conditions.unwrap_or_default(),
    cases: members.cases,
    length: members.length,
  })
}

/// A decimal number, such as a length.
fn decimal(value: &str) -> Result<usize, SyntaxErrorKind> {
  let digits = value.chars().all(|character| character.is_digit(DECIMAL_RADIX));
  digits
    .then(|| value.parse::<usize>().ok())
    .flatten()
    .ok_or(SyntaxErrorKind::MalformedLength)
}
