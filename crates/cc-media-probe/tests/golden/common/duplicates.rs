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

//! JSON text whose objects each name a member once, as the library's strict reader requires of a record
//! (`json-output.md` section 2, RFC 7493 section 2.3). A value that `serde_json` has built keeps the last member of a
//! name and drops the others, so the check needs the text, as that section says. An expectation with a member named
//! twice was compared as if the first were not there: the second review of plan Phase 3, task 4, put a `kind` of
//! `source_failed` before a toy expectation's `unknown_format`, and the golden run passed. So an expectation's text is
//! read here first, every object at every depth, and the first repeated name fails with its line and column.

use std::collections::BTreeSet;
use std::fmt;

use serde::de::{Deserialize, Deserializer, Error, MapAccess, SeqAccess, Visitor};

/// Whether `text` is JSON whose objects each name a member once, or the error of the first name repeated in one
/// object, with its line and column.
pub fn distinct_members(text: &str) -> Result<(), serde_json::Error> {
  serde_json::from_str::<Distinct>(text).map(|_| ())
}

/// A JSON value whose objects each name a member once, read and dropped.
struct Distinct;

impl<'input> Deserialize<'input> for Distinct {
  fn deserialize<Input: Deserializer<'input>>(deserializer: Input) -> Result<Distinct, Input::Error> {
    deserializer.deserialize_any(DistinctVisitor)
  }
}

/// Reads a `Distinct`: any value, each object's names checked and each member's value read the same way.
struct DistinctVisitor;

impl<'input> Visitor<'input> for DistinctVisitor {
  type Value = Distinct;

  fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str("a JSON value whose objects each name a member once")
  }

  fn visit_bool<Failure: Error>(self, _value: bool) -> Result<Distinct, Failure> {
    Ok(Distinct)
  }

  fn visit_i64<Failure: Error>(self, _value: i64) -> Result<Distinct, Failure> {
    Ok(Distinct)
  }

  fn visit_u64<Failure: Error>(self, _value: u64) -> Result<Distinct, Failure> {
    Ok(Distinct)
  }

  fn visit_f64<Failure: Error>(self, _value: f64) -> Result<Distinct, Failure> {
    Ok(Distinct)
  }

  fn visit_str<Failure: Error>(self, _value: &str) -> Result<Distinct, Failure> {
    Ok(Distinct)
  }

  fn visit_unit<Failure: Error>(self) -> Result<Distinct, Failure> {
    Ok(Distinct)
  }

  fn visit_seq<Entries: SeqAccess<'input>>(self, mut entries: Entries) -> Result<Distinct, Entries::Error> {
    while entries.next_element::<Distinct>()?.is_some() {}
    Ok(Distinct)
  }

  /// Also reads the object by which the `arbitrary_precision` feature of `serde_json` gives a number, whose one
  /// member has a private name.
  fn visit_map<Members: MapAccess<'input>>(self, mut members: Members) -> Result<Distinct, Members::Error> {
    let mut names = BTreeSet::new();
    while let Some(name) = members.next_key::<String>()? {
      if names.contains(&name) {
        return Err(Error::custom(format!(
          "the member {name:?} is named twice in one object"
        )));
      }
      members.next_value::<Distinct>()?;
      names.insert(name);
    }
    Ok(Distinct)
  }
}
