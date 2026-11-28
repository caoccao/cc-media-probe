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
//!   read of the readers and of the custom source delivers bytes of two pieces;
//! - `hint=<name>`: a file-name hint in percent-encoded UTF-8 (RFC 3986 section 2.1), such as `hint=NUL%20.backup`,
//!   given to every adapter but the file adapters, whose file has a name that every platform can write; or
//!   `hint=none`, which gives those adapters no hint at all, so that no name can be built (a hint spelled `none` is
//!   written `%6Eone`);
//! - `lookup=off|custom|built_in`: the companion lookup, off by default as the API's is; built-in lookup runs
//!   through blocking adapters alone, since an async probe reads only the async option, which has no built-in
//!   resolver;
//! - `companion.<requested name>=<response>`: the answer to the request for that name, percent-encoded as a hint
//!   is: `found:<fixture>`, `found_with_identity:<fixture>:<hexadecimal token>`, `not_found`, `declined:<reason>`,
//!   `limit_reached:<limit>` or `failed:<error kind>` from the custom resolver, as its five results allow them, and
//!   `found:<fixture>` or `not_found` with built-in lookup, which writes each found companion beside the file under a
//!   name that every platform can write;
//! - `outcomes=shared|per_adapter`: whether the adapters of each class share one semantic part, the default, or
//!   each has its own, where a limit stops adapters differently, the default limits among them.
//!
//! The default case is the probe target's, so its `default:` line takes only `outcomes`. A word that no rule reads
//! fails the test, so a misspelled condition is never ignored, and so do conditions that no adapter can run
//! together.

use std::io::ErrorKind;

use cc_media_probe::{Limit, Limits};
use kit::syntax::{Case, Condition, is_hyphenated_name, is_portable_file_name};
use serde_json::Value;

use super::failures::failure_kind;

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
/// The condition of the outcomes.
const OUTCOMES: &str = "outcomes";
/// Separates the sizes of the pieces.
const SIZE_SEPARATOR: char = ',';
/// Separates a response from its argument.
const ARGUMENT_SEPARATOR: char = ':';
/// Separates a fixture's format from its name.
const FIXTURE_SEPARATOR: char = '/';
/// Opens a percent-encoded byte.
const PERCENT: u8 = b'%';
/// The limits that a custom resolver's `limit_reached` may name (`api.md` section 13).
const RESOLVER_LIMITS: [Limit; 3] = [Limit::CompanionRequests, Limit::SourcesOpened, Limit::BytesRead];
/// The message of a companion without a lookup that asks for it.
const NO_LOOKUP: &str = "a companion's answer needs `lookup=custom` or `lookup=built_in`";
/// The message of a companion's answer given twice.
const REPEATED_COMPANION: &str = "a companion's name, decoded, has one answer";
/// The message of a built-in companion's response that only a custom resolver gives.
const NOT_BUILT_IN: &str = "built-in lookup finds a companion or does not";
/// The message of a built-in companion's name that not every platform can write beside the file.
const NOT_PORTABLE: &str =
  "a built-in companion's name is one that every platform can write, apart from the file's and each other's in case";
/// The message of an answer beside built-in lookup without a path, which declines each request itself.
const PATHLESS_BUILT_IN: &str = "built-in lookup without a path declines each request itself, so it takes no answer";
/// The message of pieces beside built-in lookup, which only the file adapter runs.
const PIECES_WITH_BUILT_IN: &str = "built-in lookup runs the file adapter alone, whose reads are not cut";
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

/// The companion lookup of a case.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Lookup {
  /// No request is made, as the API's default; the companions are "not enabled".
  #[default]
  Off,
  /// Every adapter's custom resolver, blocking or async, answers as the case says.
  Custom,
  /// The built-in resolver: through the blocking `probe_file` with its folder, and through every other blocking
  /// adapter without one, where it declines each request. An async probe reads only the async option, which has
  /// no built-in resolver, so the async adapters never run it; a library test of Phase 14, task 1, checks that
  /// they make no request and report the companions "not enabled".
  BuiltIn,
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

/// The answer to one request, one of a resolver's five results (`api.md` section 13).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Response {
  /// The fixture's bytes, as a source without a token, or written beside the file.
  Found(String),
  /// The fixture's bytes, with a token of the resolver's choosing, which may be empty.
  FoundWithIdentity(String, Vec<u8>),
  /// No such companion.
  NotFound,
  /// The resolver declines, with its reason, which may be empty.
  Declined(String),
  /// A limit that a resolver may name does not allow the request.
  LimitReached(Limit),
  /// The resolver fails with an error of that kind.
  Failed(ErrorKind),
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
  /// The companion lookup.
  pub lookup: Lookup,
  /// The answer to each requested name, decoded.
  pub companions: Vec<(String, Response)>,
  /// Whether the adapters share their semantic part.
  pub outcomes: Outcomes,
}

impl Conditions {
  /// Whether the file adapters run the case: only under the fixture's file name.
  pub fn has_file(&self) -> bool {
    self.naming == Naming::FileName
  }
}

/// The conditions of `case` of a fixture whose file name is `file_name`, or for `None` those of its default case,
/// which the header's `default:` line gives as `default`, only `outcomes` among them.
///
/// # Errors
///
/// Returns the word that no rule reads, or whose value does not fit its rule, or why conditions cannot run together.
pub fn conditions(case: Option<&Case>, default: &[Condition], file_name: &str) -> Result<Conditions, String> {
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
    } else if let Some(requested) = name.strip_prefix(COMPANION) {
      let requested = percent_decoded(requested)
        .filter(|requested| !requested.is_empty())
        .ok_or_else(refused)?;
      if conditions.companions.iter().any(|(known, _)| *known == requested) {
        return Err(format!("{name}: {REPEATED_COMPANION}"));
      }
      conditions
        .companions
        .push((requested, response(value).ok_or_else(refused)?));
    } else if name == LOOKUP {
      conditions.lookup = lookup_named(value).ok_or_else(refused)?;
    } else if name == OUTCOMES {
      conditions.outcomes = outcomes_named(value).ok_or_else(refused)?;
    } else {
      return Err(refused());
    }
  }
  together(&conditions, file_name)?;
  Ok(conditions)
}

/// Checks that the conditions can run together: an answer beside a lookup that asks for it, and with built-in
/// lookup, only the answers that it gives, under names that every platform can write beside the file, none
/// without the file, which declines each request itself, and no pieces, since it runs the file adapter alone.
fn together(conditions: &Conditions, file_name: &str) -> Result<(), String> {
  let mut names = vec![file_name];
  for (requested, response) in &conditions.companions {
    let refused = |message: &str| Err(format!("{COMPANION}{requested}: {message}"));
    match conditions.lookup {
      Lookup::Off => return refused(NO_LOOKUP),
      Lookup::Custom => {}
      Lookup::BuiltIn => {
        if !conditions.has_file() {
          return refused(PATHLESS_BUILT_IN);
        }
        if !matches!(response, Response::Found(_) | Response::NotFound) {
          return refused(NOT_BUILT_IN);
        }
        if !is_portable_file_name(requested) || names.iter().any(|name| name.eq_ignore_ascii_case(requested)) {
          return refused(NOT_PORTABLE);
        }
        names.push(requested);
      }
    }
  }
  if conditions.lookup == Lookup::BuiltIn && conditions.has_file() && !conditions.pieces.is_empty() {
    return Err(format!("{PIECES}: {PIECES_WITH_BUILT_IN}"));
  }
  Ok(())
}

/// The lookup of a serialized name.
fn lookup_named(name: &str) -> Option<Lookup> {
  match name {
    "off" => Some(Lookup::Off),
    "custom" => Some(Lookup::Custom),
    "built_in" => Some(Lookup::BuiltIn),
    _ => None,
  }
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

/// The response that `value` writes, with the arguments that the resolver's result takes: a fixture by its name of
/// the layout, a token that may be empty, a reason in percent-encoded UTF-8 that may be empty, a limit that a
/// resolver may name, and a stable I/O error kind.
fn response(value: &str) -> Option<Response> {
  let (kind, argument) = match value.split_once(ARGUMENT_SEPARATOR) {
    Some((kind, argument)) => (kind, Some(argument)),
    None => (value, None),
  };
  match (kind, argument) {
    ("found", Some(fixture)) => fixture_name(fixture).map(Response::Found),
    ("found_with_identity", Some(argument)) => {
      let (fixture, token) = argument.split_once(ARGUMENT_SEPARATOR)?;
      Some(Response::FoundWithIdentity(fixture_name(fixture)?, token_bytes(token)?))
    }
    ("not_found", None) => Some(Response::NotFound),
    ("declined", Some(reason)) => percent_decoded(reason).map(Response::Declined),
    ("limit_reached", Some(limit)) => limit_named(limit)
      .filter(|limit| RESOLVER_LIMITS.contains(limit))
      .map(Response::LimitReached),
    ("failed", Some(kind)) => failure_kind(kind).map(Response::Failed),
    _ => None,
  }
}

/// A fixture's name of the layout, `<format>/<name>`, or `None` for anything else.
fn fixture_name(name: &str) -> Option<String> {
  let (format, fixture) = name.split_once(FIXTURE_SEPARATOR)?;
  (is_hyphenated_name(format) && is_hyphenated_name(fixture)).then(|| name.to_owned())
}

/// The bytes of a token written in hexadecimal digits, two to a byte, which may be none.
fn token_bytes(token: &str) -> Option<Vec<u8>> {
  let (pairs, rest) = token.as_bytes().as_chunks::<2>();
  if !rest.is_empty() {
    return None;
  }
  pairs.iter().map(|pair| hexadecimal_byte(pair)).collect()
}

/// The byte that two hexadecimal digits write, in either case, or `None` for anything else, a sign among it.
fn hexadecimal_byte(digits: &[u8]) -> Option<u8> {
  if digits.len() != 2 || !digits.iter().all(u8::is_ascii_hexdigit) {
    return None;
  }
  u8::from_str_radix(std::str::from_utf8(digits).ok()?, 16).ok()
}
