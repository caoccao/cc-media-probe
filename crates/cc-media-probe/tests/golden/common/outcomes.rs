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

//! The semantic parts of one case's adapters (`docs/design/test-kit.md` section 4.1). By default each class shares
//! one: the seekable adapters' in `expected`, and the forward-only adapters' in `forward` where it differs. A case
//! with `outcomes=per_adapter`, or a default case whose `default:` line says so, keeps each adapter's own in
//! `outcomes` instead, where a limit stops adapters differently while each honors it: under a working memory of 0,
//! bytes in memory read in place and may decide while a reader's first cache block needs memory, as the fourth
//! review of the proposal found, and under the default limits, a reader that follows title references backward
//! reads a cache block for each and can exhaust the read limit, while bytes in memory read in place, as the fifth
//! found. A limit must explain each difference: in each class, the adapters that exhausted no limit share one
//! part, and some class has two adapters that differ, or the shared parts would do.

use serde_json::{Map, Value};

/// The places of the limits that a record holds as exhausted: a report's, and an error record's.
const EXHAUSTED: [&str; 2] = ["/completeness/record/exhausted", "/record/exhausted"];
/// The message of a case that no seekable adapter ran.
const NO_SEEKABLE: &str = "no seekable adapter ran the case";
/// The message of an adapter whose semantic part differs from its class's.
const DIFFERS: &str = "the semantic part differs from its class's through";
/// The message of an adapter that differs from its class's without a limit that explains it.
const UNEXPLAINED: &str =
  "the semantic part differs from another of its class while neither exhausted a limit, through";
/// The message of outcomes of each adapter where every class agrees.
const AGREEING: &str = "every adapter agrees with its class, so the case needs no `outcomes=per_adapter`";

/// One adapter's semantic part.
pub struct Outcome {
  /// The adapter's name.
  pub adapter: &'static str,
  /// Whether the adapter is forward only.
  pub forward_only: bool,
  /// The semantic part of its record.
  pub semantic: Value,
}

impl Outcome {
  /// Whether the record holds a limit as exhausted.
  fn exhausted(&self) -> bool {
    EXHAUSTED.iter().any(|place| {
      self
        .semantic
        .pointer(place)
        .and_then(Value::as_array)
        .is_some_and(|limits| !limits.is_empty())
    })
  }
}

/// The adapters of one class.
fn class(outcomes: &[Outcome], forward_only: bool) -> impl Iterator<Item = &Outcome> {
  outcomes
    .iter()
    .filter(move |outcome| outcome.forward_only == forward_only)
}

/// The part that `members` share, `None` without a member, or the first adapter whose part differs from the first
/// member's.
fn agreed<'outcome>(
  mut members: impl Iterator<Item = &'outcome Outcome>,
) -> Result<Option<&'outcome Value>, &'static str> {
  let Some(first) = members.next() else {
    return Ok(None);
  };
  match members.find(|outcome| outcome.semantic != first.semantic) {
    Some(other) => Err(other.adapter),
    None => Ok(Some(&first.semantic)),
  }
}

/// The semantic part that each class shares: the seekable class's, and the forward-only class's where an adapter
/// of that class ran.
///
/// # Errors
///
/// Returns that no seekable adapter ran, or the adapter whose part differs from the first of its class.
pub fn shared(outcomes: &[Outcome]) -> Result<(Value, Option<Value>), String> {
  let part = |forward_only| agreed(class(outcomes, forward_only)).map_err(|adapter| format!("{DIFFERS} {adapter}"));
  let seekable = part(false)?.ok_or_else(|| NO_SEEKABLE.to_owned())?;
  Ok((seekable.clone(), part(true)?.cloned()))
}

/// Each adapter's semantic part, by the adapter's name, where a limit explains each difference.
///
/// # Errors
///
/// Returns that no seekable adapter ran, the adapter that differs from its class's adapters that exhausted no
/// limit, or that every adapter agrees with its class, so the case needs no outcomes of each adapter.
pub fn per_adapter(outcomes: &[Outcome]) -> Result<Value, String> {
  if class(outcomes, false).next().is_none() {
    return Err(NO_SEEKABLE.to_owned());
  }
  let mut differs = false;
  for forward_only in [false, true] {
    let unlimited = class(outcomes, forward_only).filter(|outcome| !outcome.exhausted());
    agreed(unlimited).map_err(|adapter| format!("{UNEXPLAINED} {adapter}"))?;
    differs |= agreed(class(outcomes, forward_only)).is_err();
  }
  if !differs {
    return Err(AGREEING.to_owned());
  }
  let each: Map<String, Value> = outcomes
    .iter()
    .map(|outcome| (outcome.adapter.to_owned(), outcome.semantic.clone()))
    .collect();
  Ok(Value::Object(each))
}
