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

//! The comparison of a value with its expectation file, exactly, or its writing when the environment variable
//! `CC_MEDIA_PROBE_WRITE_EXPECTATIONS` asks for it: the explicit action that regenerates expectations, whose diff is
//! then reviewed (plan section 9.1). The file is parsed and compared with the value as a value, so the order of its
//! members and its white space do not count, whichever order `serde_json::Map` keeps; a written file has its
//! members sorted, by an explicit sort.

use std::fs;
use std::path::Path;

use kit::fixture::expectation_path;
use serde_json::{Map, Value};

/// The variable that makes the comparison write the expectation instead.
pub const WRITE: &str = "CC_MEDIA_PROBE_WRITE_EXPECTATIONS";

/// Whether expectations are written in this run.
pub fn writing() -> bool {
  std::env::var_os(WRITE).is_some()
}

/// `value` with the members of each object in sorted order.
pub fn sorted(value: &Value) -> Value {
  match value {
    Value::Object(members) => {
      let mut names: Vec<&String> = members.keys().collect();
      names.sort();
      let mut ordered = Map::new();
      for name in names {
        ordered.insert(name.clone(), sorted(&members[name]));
      }
      Value::Object(ordered)
    }
    Value::Array(entries) => Value::Array(entries.iter().map(sorted).collect()),
    other => other.clone(),
  }
}

/// The text of an expectation: the value with sorted members, in pretty JSON with a final line feed.
fn text(value: &Value) -> String {
  format!("{}\n", serde_json::to_string_pretty(&sorted(value)).unwrap())
}

/// The value stored at `path`, or the failure that names the variable.
pub fn stored(path: &Path) -> Value {
  let text =
    fs::read_to_string(path).unwrap_or_else(|error| panic!("{}: {error}; set {WRITE} to write it", path.display()));
  serde_json::from_str(&text).unwrap()
}

/// Checks `value` against the expectation `part` of fixture `name` in `case`, or writes it.
pub fn check(name: &str, case: Option<&str>, part: &str, value: &Value) {
  let path = expectation_path(name, case, part);
  if writing() {
    fs::write(&path, text(value)).unwrap();
    return;
  }
  assert_eq!(
    value,
    &stored(&path),
    "{name}: the {part} part differs from {}; set {WRITE} to write it",
    path.display()
  );
}

/// Checks that fixture `name` has no expectation `part` in `case`, or removes it when expectations are written.
pub fn check_absent(name: &str, case: Option<&str>, part: &str) {
  let path = expectation_path(name, case, part);
  if !path.exists() {
    return;
  }
  if writing() {
    fs::remove_file(&path).unwrap();
    return;
  }
  panic!(
    "{name}: {} is not needed any more; set {WRITE} to remove it",
    path.display()
  );
}
