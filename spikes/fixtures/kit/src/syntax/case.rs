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

//! A case of a fixture: the conditions, beyond its bytes, under which its tests probe it, such as a limit, the
//! pieces that a reader delivers, or a companion's response. A `case:` line of the header gives its name and its
//! conditions as `name=value` words, as `@seed` gives a route; the test kit reads the words and never interprets
//! them, and the library's tests do (`docs/design/test-kit.md` section 2.1). A `default:` line gives the default
//! case's conditions the same way, without a name.

use super::constants::ASSIGNMENT;
use super::error::SyntaxErrorKind;
use super::names::is_hyphenated_name;

/// A case: its name, which names its expectation files, and its conditions in the order given.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Case {
  /// The name, lowercase words joined by `-`, such as `split-header`.
  pub name: String,
  /// The conditions, each a name and a value, such as `pieces` and `40,8`.
  pub conditions: Vec<Condition>,
}

/// One condition of a case.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Condition {
  /// The name, such as `limit.header_bytes_per_stream`.
  pub name: String,
  /// The value, such as `8`.
  pub value: String,
}

/// Reads the value of a `case:` line: the case's name, then at least one `name=value` word, each name once.
pub(crate) fn parse_case(value: &str) -> Result<Case, SyntaxErrorKind> {
  let mut words = value.split_whitespace();
  let name = words
    .next()
    .filter(|name| is_hyphenated_name(name))
    .ok_or_else(|| SyntaxErrorKind::MalformedCase(value.to_owned()))?;
  Ok(Case {
    name: name.to_owned(),
    conditions: parse_conditions(words, value)?,
  })
}

/// Reads the value of a `default:` line: at least one `name=value` word, each name once.
pub(crate) fn parse_default(value: &str) -> Result<Vec<Condition>, SyntaxErrorKind> {
  parse_conditions(value.split_whitespace(), value)
}

/// Reads `words`, the conditions of the line `value`: at least one `name=value` word, each name once.
fn parse_conditions<'value>(
  words: impl Iterator<Item = &'value str>,
  value: &str,
) -> Result<Vec<Condition>, SyntaxErrorKind> {
  let mut conditions: Vec<Condition> = Vec::new();
  for word in words {
    let (condition, setting) = word
      .split_once(ASSIGNMENT)
      .filter(|(condition, setting)| !condition.is_empty() && !setting.is_empty())
      .ok_or_else(|| SyntaxErrorKind::MalformedCondition(word.to_owned()))?;
    if conditions.iter().any(|known| known.name == condition) {
      return Err(SyntaxErrorKind::RepeatedMember(condition.to_owned()));
    }
    conditions.push(Condition {
      name: condition.to_owned(),
      value: setting.to_owned(),
    });
  }
  if conditions.is_empty() {
    return Err(SyntaxErrorKind::MalformedCase(value.to_owned()));
  }
  Ok(conditions)
}
