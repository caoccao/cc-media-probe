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

//! The hidden entry points of fuzz targets (plan section 9.5, D19, `docs/design/test-kit.md` section 6.3),
//! compiled under `cfg(fuzzing)`, which cargo-fuzz passes to every crate that it builds, and in repository builds,
//! so that the seed check is an ordinary test. A published build has neither configuration name, so the module is
//! absent there and is no part of the public API.
//!
//! This task holds the one prober of the default case alone, from which the golden tests start each case's prober
//! (plan Phase 3, task 4). Plan Phase 3, task 5, adds the toy format to it through the private function that the
//! repository-only `with_toy_format` calls too, and task 20 adds the rest of the module: the targets, their input
//! encodings and what an input reached.

use std::ffi::OsString;

use crate::Prober;

/// The prober of a fixture's default case, which the targets of detection and of the full public probe run, and
/// from which the golden tests start each case's prober: `Prober::new()`, with its companion lookup off, and
/// `hint` as its file-name hint where one is given. A default case's expectation and the probe target's outcome
/// therefore come from one setup, in the repository build and in the fuzz build.
#[must_use]
pub fn prober(hint: Option<OsString>) -> Prober {
  let prober = Prober::new();
  match hint {
    Some(hint) => prober.with_file_name_hint(hint),
    None => prober,
  }
}

#[cfg(test)]
mod tests {
  use std::ffi::OsStr;

  use super::prober;
  use crate::Prober;

  #[test]
  fn the_default_cases_prober_has_the_default_options_and_the_hint_where_one_is_given() {
    let named = prober(Some("movie.idx".into()));
    assert_eq!(named.file_name_hint(), Some(OsStr::new("movie.idx")));
    assert_eq!(named.limits(), Prober::new().limits());
    assert_eq!(named.format_hint(), None);
    let nameless = prober(None);
    assert_eq!(nameless.file_name_hint(), None);
    assert_eq!(nameless.limits(), Prober::new().limits());
  }
}
