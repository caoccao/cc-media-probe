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

//! What a repository build compiles: the repository marker without `cfg(fuzzing)`, both hidden modules, and the
//! walker that integration tests reach through the repository-only entry points.

use kit::fixture::Fixture;
use library::fuzzing_entry_points::{Configuration, configuration};
use library::repository_entry_points::{toy_outcome, toy_stream_outcomes};

#[test]
fn a_repository_build_has_the_marker_and_not_the_fuzzing_name() {
  assert_eq!(
    configuration(),
    Configuration {
      fuzzing: false,
      repository: true
    }
  );
}

#[test]
fn integration_tests_reach_the_walker_through_the_repository_entry_points() {
  let fixture = Fixture::load("toy/setup-and-parameters").unwrap();
  assert_eq!(toy_outcome(&fixture.body.bytes), "report");
  assert_eq!(toy_stream_outcomes(&fixture.body.bytes), ["parsed"]);
  assert_eq!(toy_outcome(&fixture.body.bytes[..40]), "truncated");
  assert_eq!(toy_outcome(b"fLaC"), "unknown_format");
  assert_eq!(toy_outcome(&fixture.body.bytes[..12]), "insufficient_data");
}
