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

//! The match of the values that a recipe states with a report in the library's JSON form (`docs/design/test-kit.md`
//! section 4.4): every member that the recipe names must hold its value, a member that it names as null must be
//! absent, an array must have as many entries as the recipe lists, and a member that the recipe does not name is
//! not compared. Each difference is named by its JSON pointer, with each member's name escaped (RFC 6901), and the
//! differences come in the order of their pointers: an object's members by name, and an array's entries by index,
//! so `/1` before `/2` and `/10`. Sorting the messages instead would compare the text after each pointer too, and
//! put `/10` before `/1`, as the third review of the G3.2 proposal found.

use serde_json::Value;

use super::pointer::{entry, member};

/// The places where `actual` differs from what `expected` states, each with what was expected and found, in the
/// order of their pointers, whichever order `serde_json::Map` keeps.
pub fn mismatches(expected: &Value, actual: &Value) -> Vec<String> {
  let mut found = Vec::new();
  compare(expected, Some(actual), "", &mut found);
  found
}

/// Compares the value at `pointer`.
fn compare(expected: &Value, actual: Option<&Value>, pointer: &str, found: &mut Vec<String>) {
  match (expected, actual) {
    (Value::Null, None) => {}
    (Value::Null, Some(actual)) => found.push(format!("{pointer}: expected no value, found {actual}")),
    (_, None) => found.push(format!("{pointer}: expected {expected}, found no value")),
    (Value::Object(members), Some(Value::Object(actual))) => {
      let mut names: Vec<&String> = members.keys().collect();
      names.sort();
      for name in names {
        compare(&members[name], actual.get(name), &member(pointer, name), found);
      }
    }
    (Value::Array(entries), Some(Value::Array(actual))) => {
      if entries.len() == actual.len() {
        for (index, (expected, actual)) in entries.iter().zip(actual).enumerate() {
          compare(expected, Some(actual), &entry(pointer, index), found);
        }
      } else {
        found.push(format!(
          "{pointer}: expected {} entries, found {}",
          entries.len(),
          actual.len()
        ));
      }
    }
    (_, Some(actual)) => {
      if expected != actual {
        found.push(format!("{pointer}: expected {expected}, found {actual}"));
      }
    }
  }
}
