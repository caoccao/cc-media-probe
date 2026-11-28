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

//! The outcomes of a case's adapters (`docs/design/test-kit.md` section 4.1): shared by each class by default, or
//! each adapter's own where a limit stops adapters differently, as the fourth review of the proposal found in the
//! I/O spike under a working memory of 0 and the fifth under the default read limit, and only where a limit
//! explains each difference, as the fifth asked. In Phase 2 every adapter gives the same error, so no fixture's
//! case can have outcomes of each adapter yet, and these tests give the parts themselves.

use serde_json::{Value, json};

use crate::common::outcomes::{Outcome, per_adapter, shared};

/// The seekable adapters, in the order of a run.
const SEEKABLE: [&str; 4] = ["bytes", "file", "reader", "source"];

/// Detection decides without memory: bytes in memory are read in place.
fn decided() -> Value {
  json!({ "kind": "unknown_format", "record": { "exhausted": [] } })
}

/// Detection stops at the working memory: a reader's first cache block needs memory.
fn stopped() -> Value {
  json!({
    "kind": "format_not_determined",
    "reason": { "kind": "limit_exhausted", "limit": "working_memory" },
    "record": { "exhausted": ["working_memory"] },
  })
}

/// A report of `titles` titles whose probe exhausted `exhausted`.
fn report(titles: u64, exhausted: &[&str]) -> Value {
  json!({ "titles": titles, "completeness": { "record": { "exhausted": exhausted } } })
}

/// The outcomes of a run whose adapters give the parts of `parts`, in the order of `SEEKABLE`, then `stream`.
fn each(parts: [&Value; 5]) -> Vec<Outcome> {
  SEEKABLE
    .iter()
    .chain(&["stream"])
    .zip(parts)
    .map(|(adapter, semantic)| Outcome {
      adapter,
      forward_only: *adapter == "stream",
      semantic: semantic.clone(),
    })
    .collect()
}

/// The outcomes of a run whose bytes adapter gives `bytes` and every other adapter `others`.
fn run(bytes: &Value, others: &Value) -> Vec<Outcome> {
  each([bytes, others, others, others, others])
}

#[test]
fn each_class_shares_its_outcome_by_default() {
  assert_eq!(
    shared(&run(&stopped(), &stopped())).unwrap(),
    (stopped(), Some(stopped()))
  );
  let mut forward = run(&decided(), &decided());
  forward.last_mut().unwrap().semantic = stopped();
  assert_eq!(shared(&forward).unwrap(), (decided(), Some(stopped())));
  forward.pop();
  assert_eq!(shared(&forward).unwrap(), (decided(), None));
}

#[test]
fn an_adapter_that_stops_apart_from_its_class_fails_a_shared_case() {
  let error = shared(&run(&decided(), &stopped())).unwrap_err();
  assert!(error.ends_with(" file"), "{error}");
}

#[test]
fn outcomes_of_each_adapter_keep_the_part_of_each() {
  let each = per_adapter(&run(&decided(), &stopped())).unwrap();
  assert_eq!(each["bytes"], decided());
  for adapter in ["file", "reader", "source", "stream"] {
    assert_eq!(each[adapter], stopped(), "{adapter}");
  }
  assert_eq!(each.as_object().unwrap().len(), 5);
}

#[test]
fn a_read_limit_that_cuts_the_readers_beside_bytes_that_complete_keeps_each_part() {
  let complete = report(1_269, &[]);
  let cut = report(1_024, &["bytes_read"]);
  let parts = per_adapter(&each([&complete, &cut, &cut, &cut, &cut])).unwrap();
  assert_eq!((&parts["bytes"], &parts["reader"]), (&complete, &cut));
  let unequal = report(1_100, &["bytes_read"]);
  assert!(
    per_adapter(&each([&complete, &cut, &unequal, &cut, &cut])).is_ok(),
    "adapters that each exhausted a limit may stop at different points"
  );
}

#[test]
fn outcomes_of_each_adapter_where_every_class_agrees_are_refused() {
  let error = per_adapter(&run(&stopped(), &stopped())).unwrap_err();
  assert!(error.contains("outcomes=per_adapter"), "{error}");
  let mut forward = run(&decided(), &decided());
  forward.last_mut().unwrap().semantic = stopped();
  let error = per_adapter(&forward).unwrap_err();
  assert!(
    error.contains("outcomes=per_adapter"),
    "a forward-only part of its own is `forward`: {error}"
  );
}

#[test]
fn a_difference_that_no_limit_explains_is_refused() {
  let toy = json!({ "format": "toy", "record": { "exhausted": [] } });
  let flac = json!({ "format": "flac", "record": { "exhausted": [] } });
  let two = [
    Outcome {
      adapter: "bytes",
      forward_only: false,
      semantic: toy.clone(),
    },
    Outcome {
      adapter: "reader",
      forward_only: false,
      semantic: flac.clone(),
    },
  ];
  let error = per_adapter(&two).unwrap_err();
  assert!(
    error.contains("neither exhausted a limit") && error.ends_with(" reader"),
    "{error}"
  );
  let error = per_adapter(&each([&toy, &flac, &stopped(), &stopped(), &toy])).unwrap_err();
  assert!(
    error.ends_with(" file"),
    "a third adapter's limit explains no difference between two that exhausted none: {error}"
  );
  let mut forward = run(&decided(), &stopped());
  forward.push(Outcome {
    adapter: "async_stream",
    forward_only: true,
    semantic: decided(),
  });
  forward[4].semantic = toy;
  let error = per_adapter(&forward).unwrap_err();
  assert!(
    error.ends_with(" async_stream"),
    "the forward-only class is held to it too: {error}"
  );
}

#[test]
fn a_case_that_no_seekable_adapter_ran_fails_apart_from_its_outcomes() {
  for outcomes in [Vec::new(), run(&decided(), &stopped()).split_off(4)] {
    assert_eq!(per_adapter(&outcomes).unwrap_err(), "no seekable adapter ran the case");
    assert_eq!(shared(&outcomes).unwrap_err(), "no seekable adapter ran the case");
  }
}
