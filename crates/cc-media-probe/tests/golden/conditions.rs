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

//! The conditions of a case (`docs/design/test-kit.md` section 2.3): each word that the test kit gives is read by
//! one rule, and a word that no rule reads is refused. The library has no companion lookup before plan Phase 14,
//! task 1, so a lookup other than `off` and a companion's answer are refused until then by a message that says so.

use cc_media_probe::Limits;
use cc_media_probe_testkit::syntax::{Case, Condition};

use crate::common::conditions::{Naming, Outcomes, conditions};

/// A case of `words`, each `name=value`.
fn case(words: &[(&str, &str)]) -> Case {
  Case {
    name: "a-case".to_owned(),
    conditions: words
      .iter()
      .map(|(name, value)| Condition {
        name: (*name).to_owned(),
        value: (*value).to_owned(),
      })
      .collect(),
  }
}

#[test]
fn each_condition_is_read_by_its_rule() {
  let read = conditions(
    Some(&case(&[
      ("limit.header_bytes_per_stream", "8"),
      ("limit.bytes_read", "18446744073709551615"),
      ("pieces", "40,8"),
      ("lookup", "off"),
    ])),
    &[],
  )
  .unwrap();
  assert_eq!(read.limits.header_bytes_per_stream(), 8);
  assert_eq!(read.limits.bytes_read(), u64::MAX);
  assert_eq!(read.limits.streams(), Limits::new().streams());
  assert_eq!(read.pieces, [40, 8]);
  assert_eq!((read.naming, read.outcomes), (Naming::FileName, Outcomes::Shared));
}

#[test]
fn the_default_case_has_the_default_limits_and_the_fixtures_file_name() {
  let read = conditions(None, &[]).unwrap();
  assert_eq!(read.limits, Limits::default());
  assert!(read.has_file());
  assert_eq!((read.pieces, read.naming), (Vec::new(), Naming::FileName));
}

#[test]
fn a_hint_is_percent_encoded_utf_8_for_the_names_that_no_file_can_have_or_none() {
  for (value, hint) in [
    ("NUL%20.backup", "NUL .backup"),
    ("Movie%3a%20Part%202", "Movie: Part 2"),
    ("Film%20%E2%80%93%20Teil%202.idx", "Film \u{2013} Teil 2.idx"),
    ("movie.", "movie."),
    ("%6Eone", "none"),
  ] {
    let read = conditions(Some(&case(&[("hint", value)])), &[]).unwrap();
    assert_eq!(read.naming, Naming::Hint(hint.to_owned()), "{value}");
    assert!(!read.has_file());
  }
  let nameless = conditions(Some(&case(&[("hint", "none")])), &[]).unwrap();
  assert_eq!(nameless.naming, Naming::Absent);
  assert!(!nameless.has_file());
}

#[test]
fn a_word_that_no_rule_reads_is_refused() {
  for (name, value) in [
    ("limit.threads", "4"),
    ("limit.streams", "many"),
    ("limit.streams", "-1"),
    ("pieces", "40,0"),
    ("pieces", "40,"),
    ("hint", ""),
    ("hint", "a%2"),
    ("hint", "a%+1"),
    ("hint", "m%FFkv"),
    ("lookup", "on"),
    ("outcomes", "each"),
    ("delay", "5"),
  ] {
    let refused = conditions(Some(&case(&[(name, value)])), &[]).unwrap_err();
    assert_eq!(refused, format!("{name}={value}"));
  }
}

#[test]
fn a_companion_lookup_is_refused_until_the_library_has_one() {
  for (name, value) in [
    ("lookup", "custom"),
    ("lookup", "built_in"),
    ("companion.movie.sub", "not_found"),
    ("companion.Movie%20%282026%29.sub", "found:toy/movie-companion"),
  ] {
    let refused = conditions(Some(&case(&[(name, value)])), &[]).unwrap_err();
    assert_eq!(
      refused,
      format!("{name}={value}: the library has no companion lookup before plan Phase 14, task 1, which adds its cases")
    );
  }
}

#[test]
fn outcomes_of_each_adapter_need_no_limit_and_are_shared_by_default() {
  let read = conditions(
    Some(&case(&[("limit.working_memory", "0"), ("outcomes", "per_adapter")])),
    &[],
  )
  .unwrap();
  assert_eq!((read.outcomes, read.limits.working_memory()), (Outcomes::PerAdapter, 0));
  let pieces = conditions(Some(&case(&[("outcomes", "per_adapter"), ("pieces", "4")])), &[]).unwrap();
  assert_eq!((pieces.outcomes, pieces.pieces), (Outcomes::PerAdapter, vec![4]));
  let shared = conditions(Some(&case(&[("outcomes", "shared")])), &[]).unwrap();
  assert_eq!(shared.outcomes, Outcomes::Shared);
  assert_eq!(conditions(None, &[]).unwrap().outcomes, Outcomes::Shared);
}

#[test]
fn the_default_line_gives_the_default_case_its_outcomes_alone() {
  let default = case(&[("outcomes", "per_adapter")]).conditions;
  let read = conditions(None, &default).unwrap();
  assert_eq!((read.outcomes, read.limits), (Outcomes::PerAdapter, Limits::default()));
  let named = conditions(Some(&case(&[("pieces", "4")])), &default).unwrap();
  assert_eq!(named.outcomes, Outcomes::Shared, "a named case has its own conditions");
  for (name, value) in [("limit.working_memory", "0"), ("pieces", "4"), ("lookup", "off")] {
    let default = case(&[("outcomes", "per_adapter"), (name, value)]).conditions;
    let error = conditions(None, &default).unwrap_err();
    assert!(error.starts_with(&format!("{name}={value}: ")), "{error}");
  }
}
