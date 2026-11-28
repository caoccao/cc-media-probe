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

//! The conditions of a case (`docs/design/test-kit.md` section 2.3), which the test kit reads as words and the
//! library's tests interpret here:
//! - `limit.<limit>=<decimal>`: a limit by its serialized name, such as `limit.header_bytes_per_stream=8`;
//! - `pieces=<size>,<size>,…`: the input cut into pieces of these sizes from byte 0, the last repeating, so that no
//!   read of the readers and of the custom sources delivers bytes of two pieces;
//! - `hint=<name>`: a file-name hint in percent-encoded UTF-8 (RFC 3986 section 2.1), such as `hint=NUL%20.backup`,
//!   given to every adapter but the file adapters, whose file has a name that every platform can write; or
//!   `hint=none`, which gives those adapters no hint at all, so that no name can be built (a hint spelled `none` is
//!   written `%6Eone`);
//! - `lookup=off`: the companion lookup, off by default as the API's is. The library has no companion lookup
//!   before plan Phase 14, task 1, which adds `custom` and `built_in` with the answers of `companion.<name>`, so
//!   until then each of those is refused by a message that says so;
//! - `outcomes=shared|per_adapter`: whether the adapters of each class share one semantic part, the default, or
//!   each has its own, where a limit stops adapters differently, the default limits among them.
//!
//! The default case is the probe target's, so its `default:` line takes only `outcomes`. A word that no rule reads
//! fails the test, so a misspelled condition is never ignored.

use cc_media_probe::{Limit, Limits};
use cc_media_probe_testkit::syntax::{Case, Condition};
use serde_json::Value;

/// The prefix of a limit's condition.
const LIMIT: &str = "limit.";
/// The condition of the pieces.
const PIECES: &str = "pieces";
/// The condition of a hint-only or nameless case.
const HINT: &str = "hint";
/// The value of `hint` that gives no hint.
const NO_HINT: &str = "none";
/// The prefix of a companion's condition.
const COMPANION: &str = "companion.";
/// The condition of the lookup.
const LOOKUP: &str = "lookup";
/// The only lookup before plan Phase 14, task 1.
const LOOKUP_OFF: &str = "off";
/// The lookups that plan Phase 14, task 1, adds.
const LATER_LOOKUPS: [&str; 2] = ["custom", "built_in"];
/// The condition of the outcomes.
const OUTCOMES: &str = "outcomes";
/// Separates the sizes of the pieces.
const SIZE_SEPARATOR: char = ',';
/// Opens a percent-encoded byte.
const PERCENT: u8 = b'%';
/// The message of a lookup or an answer that the library does not have yet.
const NO_LOOKUP_YET: &str = "the library has no companion lookup before plan Phase 14, task 1, which adds its cases";
/// The message of a condition of the default case other than its outcomes.
const DEFAULT_OUTCOMES_ONLY: &str =
  "the default case is the probe target's, so its `default:` line takes only `outcomes`";

/// The name that the adapters of a case are given.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum Naming {
  /// The fixture's file name: the file adapters' file has it, and every other adapter gets it as the hint.
  #[default]
  FileName,
  /// A hint-only case: every adapter but the file adapters gets this hint.
  Hint(String),
  /// A nameless case: every adapter but the file adapters gets no hint.
  Absent,
}

/// Whether the adapters of a case share their semantic part.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Outcomes {
  /// The adapters of each class share one semantic part: the seekable class's, and the forward-only class's where
  /// it differs.
  #[default]
  Shared,
  /// Each adapter has a semantic part of its own, since a limit stops adapters differently.
  PerAdapter,
}

/// What a case asks of the probes.
#[derive(Debug, Clone, Default)]
pub struct Conditions {
  /// The limits.
  pub limits: Limits,
  /// The sizes of the pieces of the input; empty for reads that are not cut.
  pub pieces: Vec<usize>,
  /// The name that the adapters are given.
  pub naming: Naming,
  /// Whether the adapters share their semantic part.
  pub outcomes: Outcomes,
}

impl Conditions {
  /// Whether the file adapters run the case: only under the fixture's file name.
  pub fn has_file(&self) -> bool {
    self.naming == Naming::FileName
  }
}

/// The conditions of `case`, or for `None` those of the default case, which the header's `default:` line gives as
/// `default`, only `outcomes` among them.
///
/// # Errors
///
/// Returns the word that no rule reads, or whose value does not fit its rule, or that asks for a companion lookup
/// before the library has one.
pub fn conditions(case: Option<&Case>, default: &[Condition]) -> Result<Conditions, String> {
  let words = if let Some(case) = case {
    case.conditions.as_slice()
  } else {
    if let Some(word) = default.iter().find(|word| word.name != OUTCOMES) {
      return Err(format!("{}={}: {DEFAULT_OUTCOMES_ONLY}", word.name, word.value));
    }
    default
  };
  let mut conditions = Conditions::default();
  for condition in words {
    let (name, value) = (condition.name.as_str(), condition.value.as_str());
    let refused = || format!("{name}={value}");
    if let Some(limit) = name.strip_prefix(LIMIT) {
      let value = value.parse::<u64>().map_err(|_| refused())?;
      conditions.limits = conditions.limits.with(&limit_named(limit).ok_or_else(refused)?, value);
    } else if name == PIECES {
      conditions.pieces = value
        .split(SIZE_SEPARATOR)
        .map(|size| size.parse::<usize>().ok().filter(|size| *size > 0))
        .collect::<Option<Vec<usize>>>()
        .ok_or_else(refused)?;
    } else if name == HINT {
      conditions.naming = if value == NO_HINT {
        Naming::Absent
      } else {
        Naming::Hint(
          percent_decoded(value)
            .filter(|hint| !hint.is_empty())
            .ok_or_else(refused)?,
        )
      };
    } else if (name == LOOKUP && LATER_LOOKUPS.contains(&value)) || name.starts_with(COMPANION) {
      return Err(format!("{}: {NO_LOOKUP_YET}", refused()));
    } else if name == LOOKUP {
      if value != LOOKUP_OFF {
        return Err(refused());
      }
    } else if name == OUTCOMES {
      conditions.outcomes = outcomes_named(value).ok_or_else(refused)?;
    } else {
      return Err(refused());
    }
  }
  Ok(conditions)
}

/// The text of a percent-encoded value, which may be empty, or `None` for a `%` without two hexadecimal digits, or
/// bytes that are not UTF-8.
fn percent_decoded(value: &str) -> Option<String> {
  let mut bytes = Vec::new();
  let mut rest = value.as_bytes();
  while let Some((&byte, after)) = rest.split_first() {
    rest = after;
    if byte == PERCENT {
      let (digits, after) = rest.split_at_checked(2)?;
      bytes.push(hexadecimal_byte(digits)?);
      rest = after;
    } else {
      bytes.push(byte);
    }
  }
  String::from_utf8(bytes).ok()
}

/// The byte that two hexadecimal digits write, in either case, or `None` for anything else, a sign among it.
fn hexadecimal_byte(digits: &[u8]) -> Option<u8> {
  if digits.len() != 2 || !digits.iter().all(u8::is_ascii_hexdigit) {
    return None;
  }
  u8::from_str_radix(std::str::from_utf8(digits).ok()?, 16).ok()
}

/// The outcomes of a serialized name.
fn outcomes_named(name: &str) -> Option<Outcomes> {
  match name {
    "shared" => Some(Outcomes::Shared),
    "per_adapter" => Some(Outcomes::PerAdapter),
    _ => None,
  }
}

/// The limit of a serialized name, or `None` for a name that this version does not know.
fn limit_named(name: &str) -> Option<Limit> {
  match serde_json::from_value::<Limit>(Value::from(name)).ok()? {
    Limit::Unrecognized(_) => None,
    limit => Some(limit),
  }
}
