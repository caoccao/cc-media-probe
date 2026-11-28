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

//! The round trip of each probe's result (plan section 8.8): its text reads back through the strict reader as the
//! record that it is and is written again as the same text, or the golden pass fails before it compares or writes
//! anything; and the text of an expectation names each member of an object once and is the text that the writer
//! writes for its value.

use std::panic;

use cc_media_probe::MediaReport;
use cc_media_probe::fuzzing_entry_points::prober;
use serde::{Deserialize, Serialize};
use serde_json::{Map, json};

use crate::common::expectation::{parsed, stored, written};
use crate::common::round_trip::read_back;
use crate::folder::case_folder;
use crate::records::{entry, report};

#[test]
fn a_result_whose_text_reads_back_as_itself_gives_its_json_value() {
  let record = prober(None).probe_bytes(&[0; 16]).unwrap_err().to_record();
  assert_eq!(read_back(&record).unwrap(), serde_json::to_value(&record).unwrap());
  let built = report(5, &[entry("0", "opened", &Map::new())]);
  let read: MediaReport = serde_json::from_value(built.clone()).unwrap();
  assert_eq!(read_back(&read).unwrap(), built);
}

/// A record whose reader drops a member that its writer writes, while its equality ignores that member.
#[derive(Debug, Serialize, Deserialize)]
struct Stamped {
  /// Written, and never read back.
  #[serde(skip_deserializing)]
  fresh: bool,
  /// Written and read back.
  value: u8,
}

impl PartialEq for Stamped {
  fn eq(&self, other: &Stamped) -> bool {
    self.value == other.value
  }
}

#[test]
fn a_text_that_the_reader_refuses_reads_as_another_record_or_writes_again_otherwise_fails() {
  // The second review of plan Phase 3, task 4: the pass took each result's value without reading its text back.
  let refused = read_back(&f64::NAN).unwrap_err();
  assert!(refused.starts_with("the strict reader refuses the text"), "{refused}");
  let other = read_back(&Some(None::<u8>)).unwrap_err();
  assert_eq!(other, "the text reads back as another record: None: null");
  let rewritten = read_back(&Stamped { fresh: true, value: 7 }).unwrap_err();
  assert_eq!(
    rewritten,
    r#"the record read back is written as other text: {"fresh":false,"value":7}: {"fresh":true,"value":7}"#
  );
}

#[test]
fn an_expectation_that_names_a_member_twice_in_one_object_fails_with_its_place() {
  // The review's case: a `kind` of `source_failed` before the toy expectation's own `kind`.
  let repeated = parsed("{\n  \"kind\": \"source_failed\",\n  \"kind\": \"unknown_format\"\n}").unwrap_err();
  assert_eq!(
    repeated,
    "the member \"kind\" is named twice in one object at line 3 column 8"
  );
  let nested = parsed(r#"{"record": [{"part": "tags", "part": "chapters"}]}"#).unwrap_err();
  assert!(nested.starts_with("the member \"part\" is named twice"), "{nested}");
  let apart = json!({"a": {"part": 1}, "b": [{"part": 2}, {"part": 3}], "part": 4});
  assert_eq!(parsed(&written(&apart)).unwrap(), apart);
}

#[test]
fn an_expectation_is_the_text_that_the_writer_writes_for_its_value() {
  let value = json!({"b": {"kind": "A"}, "a": [1, 2]});
  let text = written(&value);
  assert_eq!(
    text,
    "{\n  \"a\": [\n    1,\n    2\n  ],\n  \"b\": {\n    \"kind\": \"A\"\n  }\n}\n"
  );
  assert_eq!(parsed(&text).unwrap(), value);
  for (other, line) in [
    (text.replace("\"A\"", "\"\\u0041\""), 7),
    (
      "{\n  \"b\": {\n    \"kind\": \"A\"\n  },\n  \"a\": [\n    1,\n    2\n  ]\n}\n".to_owned(),
      2,
    ),
    (text.replace("    1,\n    2", "    1, 2"), 3),
    (text.trim_end().to_owned(), 10),
    (format!("{text}\n"), 11),
  ] {
    let error = parsed(&other).unwrap_err();
    assert!(error.starts_with(&format!("line {line} is ")), "{other}: {error}");
  }
  assert_eq!(
    parsed(text.trim_end()).unwrap_err(),
    "line 10 is the end of the text, where the writer writes \"\" for the value that the file holds"
  );
}

#[test]
fn an_object_that_spells_a_private_number_fails_its_expectation_in_every_build() {
  // The third review of plan Phase 3, task 4: with serde_json's `arbitrary_precision` on, this object parses as the
  // number 0, so a file that held it where the report has the number 0 passed in that build.
  let forged = written(&json!({"retained_data": {"$serde_json::private::Number": "0"}}));
  let expected = json!({"retained_data": 0});
  let private_numbers = serde_json::from_str::<serde_json::Value>(r#"{"$serde_json::private::Number": "0"}"#)
    .unwrap()
    .is_number();
  match parsed(&forged) {
    Ok(value) => {
      assert!(!private_numbers);
      assert_ne!(value, expected);
    }
    Err(error) => {
      assert!(private_numbers);
      assert_eq!(
        error,
        "line 2 is \"  \\\"retained_data\\\": {\", where the writer writes \"  \\\"retained_data\\\": 0\" for the value \
         that the file holds"
      );
    }
  }
}

#[test]
fn a_stored_expectation_with_a_member_named_twice_names_its_file() {
  let folder = case_folder();
  let path = folder.path.join("repeated.expected.json");
  std::fs::write(&path, r#"{"kind": "source_failed", "kind": "unknown_format"}"#).unwrap();
  let failure = panic::catch_unwind(|| stored(&path)).unwrap_err();
  let message = failure.downcast_ref::<String>().unwrap();
  assert!(
    message.starts_with(&format!("{}: the member \"kind\" is named twice", path.display())),
    "{message}"
  );
  std::fs::write(&path, r#"{"kind": "unknown_format"}"#).unwrap();
  let failure = panic::catch_unwind(|| stored(&path)).unwrap_err();
  let message = failure.downcast_ref::<String>().unwrap();
  assert!(
    message.starts_with(&format!("{}: line 1 is ", path.display())),
    "{message}"
  );
  assert!(
    message.ends_with("; set CC_MEDIA_PROBE_WRITE_EXPECTATIONS to write it"),
    "{message}"
  );
}
