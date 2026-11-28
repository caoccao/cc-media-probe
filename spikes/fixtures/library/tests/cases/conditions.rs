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
//! one rule, a word that no rule reads is refused, and so are conditions that no adapter can run together. A
//! response takes exactly the arguments of the resolver's result that it stands for (`api.md` section 13).

use std::io::ErrorKind;

use cc_media_probe::{Limit, Limits};
use kit::syntax::{Case, Condition};

use crate::common::conditions::{Lookup, Naming, Outcomes, Response, conditions};

/// The file name of the fixture whose cases these tests read.
const FILE_NAME: &str = "movie.idx";

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
      ("pieces", "40,8"),
      ("companion.movie.sub", "found:vobsub/movie-data"),
      ("companion.movie.toyc", "found_with_identity:toy/companion:0a0B"),
      ("companion.same.toyc", "found_with_identity:toy/companion:"),
      ("companion.other.toyc", "limit_reached:sources_opened"),
      ("companion.denied.toyc", "failed:permission_denied"),
      ("companion.Movie%20%282026%29.sub", "declined:not%20today"),
      ("companion.quiet.toyc", "declined:"),
      ("lookup", "custom"),
    ])),
    &[],
    FILE_NAME,
  )
  .unwrap();
  assert_eq!(read.limits.header_bytes_per_stream(), 8);
  assert_eq!(read.pieces, [40, 8]);
  assert_eq!(read.lookup, Lookup::Custom);
  let named = |name: &str, response| (name.to_owned(), response);
  assert_eq!(
    read.companions,
    [
      named("movie.sub", Response::Found("vobsub/movie-data".to_owned())),
      named(
        "movie.toyc",
        Response::FoundWithIdentity("toy/companion".to_owned(), vec![0x0a, 0x0b])
      ),
      named(
        "same.toyc",
        Response::FoundWithIdentity("toy/companion".to_owned(), Vec::new())
      ),
      named("other.toyc", Response::LimitReached(Limit::SourcesOpened)),
      named("denied.toyc", Response::Failed(ErrorKind::PermissionDenied)),
      named("Movie (2026).sub", Response::Declined("not today".to_owned())),
      named("quiet.toyc", Response::Declined(String::new())),
    ]
  );
  let built_in = conditions(
    Some(&case(&[("lookup", "built_in"), ("companion.movie.sub", "not_found")])),
    &[],
    FILE_NAME,
  )
  .unwrap();
  assert_eq!(built_in.lookup, Lookup::BuiltIn);
}

#[test]
fn the_default_case_has_its_lookup_off_and_the_fixtures_file_name() {
  let read = conditions(None, &[], FILE_NAME).unwrap();
  assert_eq!(read.lookup, Lookup::Off);
  assert_eq!(
    (read.pieces, read.naming, read.companions),
    (Vec::new(), Naming::FileName, Vec::new())
  );
  let off = conditions(Some(&case(&[("lookup", "off")])), &[], FILE_NAME).unwrap();
  assert_eq!(off.lookup, Lookup::Off);
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
    let read = conditions(Some(&case(&[("hint", value)])), &[], FILE_NAME).unwrap();
    assert_eq!(read.naming, Naming::Hint(hint.to_owned()), "{value}");
    assert!(!read.has_file());
  }
  let nameless = conditions(Some(&case(&[("lookup", "custom"), ("hint", "none")])), &[], FILE_NAME).unwrap();
  assert_eq!((nameless.naming, nameless.lookup), (Naming::Absent, Lookup::Custom));
}

#[test]
fn a_word_that_no_rule_reads_is_refused() {
  for (name, value) in [
    ("limit.threads", "4"),
    ("limit.streams", "many"),
    ("pieces", "40,0"),
    ("companion.movie.sub", "maybe"),
    ("companion.movie.sub", "found_with_identity:toy/companion:abc"),
    ("companion.movie.sub", "found_with_identity:toy/companion:+a"),
    ("companion.movie.sub", "found:"),
    ("companion.movie.sub", "found:toy"),
    ("companion.movie.sub", "found:Toy/a"),
    ("companion.movie.sub", "found:toy/a/b"),
    ("companion.movie.sub", "declined"),
    ("companion.movie.sub", "declined:a%2"),
    ("companion.movie.sub", "not_found:why"),
    ("companion.movie.sub", "limit_reached:streams"),
    ("companion.movie.sub", "limit_reached:retained_data"),
    ("companion.movie.sub", "failed:"),
    ("companion.movie.sub", "failed:whatever"),
    ("companion.movie.sub", "failed:PermissionDenied"),
    ("companion.movie%2", "not_found"),
    ("companion.", "not_found"),
    ("lookup", "anywhere"),
    ("outcomes", "each"),
    ("hint", ""),
    ("hint", "a%2"),
    ("hint", "a%+1"),
    ("hint", "m%FFkv"),
    ("delay", "5"),
  ] {
    let refused = conditions(Some(&case(&[(name, value)])), &[], FILE_NAME).unwrap_err();
    assert_eq!(refused, format!("{name}={value}"));
  }
}

#[test]
fn conditions_that_no_adapter_can_run_together_are_refused() {
  for (words, refused) in [
    (&[("companion.movie.sub", "not_found")][..], "companion.movie.sub"),
    (
      &[("lookup", "built_in"), ("companion.movie.sub", "declined:")][..],
      "companion.movie.sub",
    ),
    (
      &[("lookup", "built_in"), ("companion.NUL.sub", "not_found")][..],
      "companion.NUL.sub",
    ),
    (
      &[("lookup", "built_in"), ("companion.Movie.IDX", "not_found")][..],
      "companion.Movie.IDX",
    ),
    (
      &[
        ("lookup", "built_in"),
        ("companion.a.sub", "not_found"),
        ("companion.A.SUB", "not_found"),
      ][..],
      "companion.A.SUB",
    ),
    (
      &[
        ("lookup", "custom"),
        ("companion.a.sub", "not_found"),
        ("companion.%61.sub", "declined:"),
      ][..],
      "companion.%61.sub",
    ),
    (
      &[
        ("lookup", "built_in"),
        ("hint", "a.idx"),
        ("companion.a.sub", "not_found"),
      ][..],
      "companion.a.sub",
    ),
    (&[("lookup", "built_in"), ("pieces", "4")][..], "pieces"),
  ] {
    let error = conditions(Some(&case(words)), &[], FILE_NAME).unwrap_err();
    assert!(error.starts_with(&format!("{refused}: ")), "{error}");
  }
}

#[test]
fn built_in_lookup_without_a_path_runs_through_every_other_blocking_adapter() {
  for hint in ["a.idx", "none"] {
    let read = conditions(
      Some(&case(&[("lookup", "built_in"), ("hint", hint), ("pieces", "4")])),
      &[],
      FILE_NAME,
    )
    .unwrap();
    assert_eq!((read.lookup, read.has_file()), (Lookup::BuiltIn, false), "{hint}");
  }
}

#[test]
fn outcomes_of_each_adapter_need_no_limit_and_are_shared_by_default() {
  let read = conditions(
    Some(&case(&[("limit.working_memory", "0"), ("outcomes", "per_adapter")])),
    &[],
    FILE_NAME,
  )
  .unwrap();
  assert_eq!((read.outcomes, read.limits.working_memory()), (Outcomes::PerAdapter, 0));
  let pieces = conditions(
    Some(&case(&[("outcomes", "per_adapter"), ("pieces", "4")])),
    &[],
    FILE_NAME,
  )
  .unwrap();
  assert_eq!((pieces.outcomes, pieces.pieces), (Outcomes::PerAdapter, vec![4]));
  let shared = conditions(Some(&case(&[("outcomes", "shared")])), &[], FILE_NAME).unwrap();
  assert_eq!(shared.outcomes, Outcomes::Shared);
  assert_eq!(conditions(None, &[], FILE_NAME).unwrap().outcomes, Outcomes::Shared);
}

#[test]
fn the_default_line_gives_the_default_case_its_outcomes_alone() {
  let default = case(&[("outcomes", "per_adapter")]).conditions;
  let read = conditions(None, &default, FILE_NAME).unwrap();
  assert_eq!((read.outcomes, read.limits), (Outcomes::PerAdapter, Limits::default()));
  let named = conditions(Some(&case(&[("pieces", "4")])), &default, FILE_NAME).unwrap();
  assert_eq!(named.outcomes, Outcomes::Shared, "a named case has its own conditions");
  for (name, value) in [("limit.working_memory", "0"), ("pieces", "4"), ("lookup", "custom")] {
    let default = case(&[("outcomes", "per_adapter"), (name, value)]).conditions;
    let error = conditions(None, &default, FILE_NAME).unwrap_err();
    assert!(error.starts_with(&format!("{name}={value}: ")), "{error}");
  }
}
