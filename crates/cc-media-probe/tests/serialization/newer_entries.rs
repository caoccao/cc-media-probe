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

//! An entry of an outcome that a newer version wrote, such as a hypothetical `skipped` with its `skip_reason`
//! (`api.md` section 12; `json-output.md` section 3; plan section 8.8). It keeps every member that it does not read,
//! as written in the sense of `api.md` section 12, and writes them back after its own members in the order read,
//! from the document read and from one that either writer rebuilt, while `with_outcome` refuses that outcome. What
//! it does read is checked: its input, and each record that it holds against the document's version. An entry of a
//! known outcome passes such members by as they come, and holds a member before its outcome only until the outcome
//! is known (eighteenth review).

use cc_media_probe::document::{Document, DocumentEntry, Generator};
use serde_json::{Value, json};

use crate::fixtures::{detection, document, entry, error_record, generator, reads_document, usage, with};

/// A document of version 1.1 whose one entry was skipped, with the payload that its outcome needs.
fn skipped_document() -> Value {
  let skipped = json!({
    "index": 0,
    "input": "notes.txt",
    "outcome": "skipped",
    "skip_reason": { "kind": "excluded", "pattern": "*.txt" },
  });
  document("1.1", json!([skipped]))
}

/// A detection of version 1.1 whose outcome only that version knows.
fn verified() -> Value {
  json!({ "schema_version": "1.1", "outcome": "verified", "candidates": [], "usage": usage() })
}

#[test]
fn an_entry_of_a_newer_outcome_writes_its_payload_back() {
  let payload = skipped_document()["results"][0]["skip_reason"].clone();
  let read: Document = serde_json::from_value(skipped_document()).unwrap();
  assert_eq!(read.results()[0].outcome().as_str(), "skipped");
  assert!(read.results()[0].report().is_none() && read.results()[0].error().is_none());
  assert_eq!(
    serde_json::to_value(&read).unwrap()["results"][0]["skip_reason"],
    payload
  );
  // Rebuilt by either writer, at this version.
  let rebuilt = Document::from_results(Generator::new("catalog", "1"), read.results().to_vec());
  let rewritten = serde_json::to_value(&rebuilt).unwrap();
  assert_eq!(rewritten["schema_version"], json!("1.0"));
  assert_eq!(rewritten["results"][0]["outcome"], json!("skipped"));
  assert_eq!(rewritten["results"][0]["skip_reason"], payload);
  let mut streamed = Vec::new();
  let mut serializer = serde_json::Serializer::new(&mut streamed);
  Document::serialize_results(
    &Generator::new("catalog", "1"),
    read.results().to_vec(),
    &mut serializer,
  )
  .unwrap();
  assert_eq!(serde_json::from_slice::<Value>(&streamed).unwrap(), rewritten);
}

#[test]
fn an_outcome_of_a_newer_version_is_forwarded_with_its_entry() {
  // The ninth review's case: the skipped entry holds a detection whose outcome a newer version wrote, so this
  // version cannot derive the entry's outcome from it. `with_outcome` refuses the recorded `skipped`, which it could
  // only build without `skip_reason`, and the entry as read forwards both.
  let mut text = skipped_document();
  text["results"][0]["detection"] = verified();
  let read: Document = serde_json::from_value(text).unwrap();
  let entry = &read.results()[0];
  let refused = DocumentEntry::from_detection(entry.input().clone(), entry.detection().unwrap().clone()).unwrap_err();
  assert_eq!(
    refused.clone().with_outcome(entry.outcome().clone()).unwrap_err(),
    refused
  );
  let written = serde_json::to_value(Document::from_results(
    Generator::new("catalog", "1"),
    vec![entry.clone()],
  ));
  let written = written.unwrap();
  assert_eq!(written["results"][0]["outcome"], json!("skipped"));
  assert_eq!(
    written["results"][0]["skip_reason"],
    skipped_document()["results"][0]["skip_reason"]
  );
  assert_eq!(written["results"][0]["detection"]["schema_version"], json!("1.0"));
}

#[test]
fn a_rebuilt_document_gives_its_version_to_every_record_that_it_reads() {
  // The ninth review's case, as the maintainer settled it: a minor version puts a root record only where every
  // reader reads it, under `report`, `detection` or `error`, so the rebuilt document is one version throughout. A
  // member that this version does not read is data to it, and comes back unchanged, whatever it holds.
  let record = error_record("1.1");
  let mut text = skipped_document();
  text["results"][0]["error"] = record.clone();
  text["results"][0]["skip_detail"] = record;
  let read: Document = serde_json::from_value(text).unwrap();
  let rebuilt = Document::from_results(Generator::new("catalog", "1"), read.results().to_vec());
  let written = serde_json::to_value(&rebuilt).unwrap();
  assert_eq!(written["results"][0]["error"]["schema_version"], json!("1.0"));
  assert_eq!(written["results"][0]["skip_detail"]["schema_version"], json!("1.1"));
}

#[test]
fn only_an_entry_of_a_newer_outcome_keeps_the_members_that_it_does_not_read() {
  // A known outcome ignores an unknown member and does not write it back.
  let mut known = skipped_document();
  known["results"][0]["outcome"] = json!("error");
  known["results"][0]["error"] = error_record("1.1");
  let read: Document = serde_json::from_value(known).unwrap();
  assert!(
    serde_json::to_value(&read).unwrap()["results"][0]
      .get("skip_reason")
      .is_none()
  );
  // A newer one writes them back after its own members, in the order read, which only the text shows, since
  // `serde_json::Value` orders a map's members by name.
  let skipped =
    r#"{"index": 0, "input": "a", "outcome": "skipped", "skip_reason": {"kind": "excluded"}, "retry_after": "PT1H"}"#;
  let text = format!(
    r#"{{"schema_version": "1.1", "generator": {}, "results": [{skipped}]}}"#,
    generator()
  );
  let rewritten = serde_json::to_string(&serde_json::from_str::<Document>(&text).unwrap()).unwrap();
  assert!(
    rewritten.ends_with(r#""outcome":"skipped","skip_reason":{"kind":"excluded"},"retry_after":"PT1H"}]}"#),
    "{rewritten}"
  );
  // A kept member refuses null, and a record that the entry holds still has the document's version.
  let mut with_null = skipped_document();
  with_null["results"][0]["retry_after"] = Value::Null;
  assert!(!reads_document(&with_null));
  let mut with_record = skipped_document();
  with_record["results"][0]["error"] = error_record("1.0");
  assert!(!reads_document(&with_record), "a nested record of another version");
}

#[test]
fn an_entry_holds_what_it_does_not_read_only_until_its_outcome_is_known() {
  // An entry decides by its outcome: a newer one keeps the members before and after it, in the order written, and a
  // known one passes them by, as a detection and an error record do.
  let document = |entry: &str| {
    let text = format!(
      r#"{{"schema_version": "1.1", "generator": {}, "results": [{entry}]}}"#,
      generator()
    );
    serde_json::to_string(&serde_json::from_str::<Document>(&text).unwrap()).unwrap()
  };
  let written = document(r#"{"index": 0, "input": "a", "before": 1, "outcome": "skipped", "after": 2}"#);
  assert!(
    written.contains(r#""outcome":"skipped","before":1,"after":2}"#),
    "{written}"
  );
  let detection = verified().to_string();
  let written = document(&format!(
    r#"{{"index": 0, "input": "a", "before": 1, "outcome": "partial", "detection": {detection}, "after": 2}}"#
  ));
  assert!(
    !written.contains(r#""before""#) && !written.contains(r#""after""#),
    "{written}"
  );
  // A member passed by is read strictly all the same.
  let passed_by = format!(
    r#"{{"index": 0, "input": "a", "outcome": "error", "error": {}, "after": {{"x": null}}}}"#,
    error_record("1.1")
  );
  let text = format!(
    r#"{{"schema_version": "1.1", "generator": {}, "results": [{passed_by}]}}"#,
    generator()
  );
  assert!(serde_json::from_str::<Document>(&text).is_err());
}

#[test]
fn a_detection_of_a_newer_outcome_keeps_its_members_in_a_document() {
  // Inside a document of version 1.1, rewritten by this version, the member that only that version knows stays.
  let deferred = with(verified(), "retry_after", json!(3));
  let read: Document =
    serde_json::from_value(document("1.1", json!([entry(0, "partial", "detection", deferred)]))).unwrap();
  let written = serde_json::to_value(&read).unwrap();
  assert_eq!(written["results"][0]["detection"]["retry_after"], json!(3));
  // A detection of a known outcome drops it.
  let known = with(detection("1.1"), "retry_after", json!(3));
  let read: Document = serde_json::from_value(document("1.1", json!([entry(0, "ok", "detection", known)]))).unwrap();
  let written = serde_json::to_value(&read).unwrap();
  assert!(written["results"][0]["detection"].get("retry_after").is_none());
}

/// A document of version 1.0 with one entry of a newer outcome that keeps `ratio`, and a member `future` that no type
/// reads, with the document's version before or after them.
fn keeping(ratio: &Value, future: &Value, version_first: bool) -> String {
  let results = json!([{ "index": 0, "input": "a", "outcome": "skipped", "ratio": ratio }]);
  let head = format!(r#""schema_version": "1.0", "generator": {}"#, generator());
  let rest = format!(r#""results": {results}, "future": {future}"#);
  if version_first {
    format!("{{{head}, {rest}}}")
  } else {
    format!("{{{rest}, {head}}}")
  }
}

fn private(number: &str) -> Value {
  json!({ "$serde_json::private::Number": number })
}

fn written_ratio(text: &str) -> Value {
  let read: Document = serde_json::from_str(text).unwrap();
  serde_json::to_value(&read).unwrap()["results"][0]["ratio"].clone()
}

#[test]
fn a_kept_member_takes_the_private_map_of_a_number_as_that_number() {
  // `json-output.md` section 2: an object of that shape where no type is known is the number that it spells, before
  // the version or after it, and `1e400`, which is no finite number, fails as a value that JSON does not carry.
  for version_first in [false, true] {
    for (number, expected) in [
      ("60", json!(60)),
      ("1.25", json!(1.25)),
      ("18446744073709551616", json!(18_446_744_073_709_551_616.0_f64)),
      ("-0", json!(-0.0)),
    ] {
      assert_eq!(
        written_ratio(&keeping(&private(number), &json!(1), version_first)),
        expected,
        "{number}"
      );
    }
    let failure = serde_json::from_str::<Document>(&keeping(&private("1e400"), &json!(1), version_first));
    assert!(
      failure
        .unwrap_err()
        .to_string()
        .starts_with("a value that JSON does not carry")
    );
    // A map of that name that holds no number, or that holds another member beside it, stays a map.
    for map in [
      private("not a number"),
      json!({ "$serde_json::private::Number": "1e400", "other": 1 }),
    ] {
      assert_eq!(written_ratio(&keeping(&map, &json!(1), version_first)), map);
    }
  }
}

#[test]
fn a_member_that_no_type_reads_checks_the_private_map_in_either_order() {
  for version_first in [false, true] {
    let failure = serde_json::from_str::<Document>(&keeping(&json!(1), &private("1e400"), version_first));
    assert!(
      failure
        .unwrap_err()
        .to_string()
        .starts_with("a value that JSON does not carry"),
      "{version_first}"
    );
    assert!(serde_json::from_str::<Document>(&keeping(&json!(1), &private("1.25"), version_first)).is_ok());
    let deep = json!({ "values": [1, private("-1e400")] });
    assert!(serde_json::from_str::<Document>(&keeping(&json!(1), &deep, version_first)).is_err());
  }
}

#[test]
fn an_integer_beyond_64_bits_where_no_type_is_known_is_the_nearest_double() {
  // The seventeenth review's case: JSON `18446744073709551617` in a member of a future entry comes back as the
  // nearest double, without the feature from serde_json's own parser and with it through the private map.
  for version_first in [true, false] {
    for ratio in [json!(18_446_744_073_709_551_617.0_f64), private("18446744073709551617")] {
      let text = keeping(&ratio, &json!(1), version_first).replace("1.8446744073709552e19", "18446744073709551617");
      assert_eq!(written_ratio(&text).to_string(), "1.8446744073709552e+19");
    }
  }
  // Where a type is known, the member refuses it: an entry's index is an integer within 2^53 - 1.
  let text = keeping(&json!(1), &json!(1), true).replacen(r#""index":0"#, r#""index":18446744073709551617"#, 1);
  assert!(serde_json::from_str::<Document>(&text).is_err());
}
