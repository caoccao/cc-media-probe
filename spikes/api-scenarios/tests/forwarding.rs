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

//! The eighth review's case of an entry whose outcome a newer version wrote, `api.md` section 12.

use cc_media_probe::document::{Document, DocumentEntry, Generator};
use serde_json::{Value, json};

/// A document of version 1.1 whose one entry was skipped, with the payload that its outcome needs.
fn skipped_document() -> Value {
  json!({
    "schema_version": "1.1",
    "generator": { "name": "catalog", "version": "2" },
    "results": [{
      "index": 0,
      "input": "notes.txt",
      "outcome": "skipped",
      "skip_reason": { "kind": "excluded", "pattern": "*.txt" },
    }],
  })
}

#[test]
fn an_entry_of_a_newer_outcome_writes_its_payload_back() {
  let read: Document = serde_json::from_value(skipped_document()).unwrap();
  let written = serde_json::to_value(&read).unwrap();
  assert_eq!(
    written["results"][0]["skip_reason"],
    skipped_document()["results"][0]["skip_reason"]
  );
  let rebuilt = Document::from_results(Generator::new("catalog", "1"), read.results().to_vec());
  let rewritten = serde_json::to_value(&rebuilt).unwrap();
  assert_eq!(rewritten["results"][0]["outcome"], json!("skipped"));
  assert_eq!(
    rewritten["results"][0]["skip_reason"],
    skipped_document()["results"][0]["skip_reason"]
  );
}

#[test]
fn an_outcome_of_a_newer_version_is_forwarded_with_its_entry() {
  // The ninth review's case. The skipped entry holds a detection whose outcome a newer version wrote, so this
  // version cannot derive the entry's outcome from it. `with_outcome` refuses the recorded `skipped`, which it could
  // only build without `skip_reason`, and the entry as read forwards both.
  let mut text = skipped_document();
  text["results"][0]["detection"] =
    json!({ "schema_version": "1.1", "outcome": "identified_by_content", "candidates": [] });
  let read: Document = serde_json::from_value(text).unwrap();
  let entry = &read.results()[0];
  let refused = DocumentEntry::from_detection(entry.input().clone(), entry.detection().unwrap().clone()).unwrap_err();
  assert!(refused.with_outcome(entry.outcome().clone()).is_err());
  let written = serde_json::to_value(Document::from_results(
    Generator::new("catalog", "1"),
    vec![entry.clone()],
  ))
  .unwrap();
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
  // reader reads it, under `report`, `detection` or `error`, so the rebuilt document is one version throughout.
  let mut record = serde_json::to_value(cc_media_probe::probe_bytes(b"").unwrap_err().to_record()).unwrap();
  record["schema_version"] = json!("1.1");
  let mut text = skipped_document();
  text["results"][0]["error"] = record.clone();
  // A member that this version does not read is data to it, and comes back unchanged, whatever it holds; that is why
  // section 14 keeps records out of such members.
  text["results"][0]["skip_detail"] = record;
  let read: Document = serde_json::from_value(text).unwrap();
  let rebuilt = Document::from_results(Generator::new("catalog", "1"), read.results().to_vec());
  let written = serde_json::to_value(&rebuilt).unwrap();
  assert_eq!(written["schema_version"], json!("1.0"));
  assert_eq!(written["results"][0]["error"]["schema_version"], json!("1.0"));
  assert_eq!(written["results"][0]["skip_detail"]["schema_version"], json!("1.1"));
}

#[test]
fn only_an_entry_of_a_newer_outcome_keeps_the_members_it_does_not_read() {
  // A known outcome ignores an unknown member and does not write it back (section 14); a newer one keeps its members
  // in the order read, refuses null among them, and still has its records checked against the document's version.
  let mut known = skipped_document();
  let mut error = serde_json::to_value(cc_media_probe::probe_bytes(b"").unwrap_err().to_record()).unwrap();
  error["schema_version"] = json!("1.1");
  known["results"][0]["outcome"] = json!("error");
  known["results"][0]["error"] = error.clone();
  let read: Document = serde_json::from_value(known).unwrap();
  assert!(
    serde_json::to_value(&read).unwrap()["results"][0]
      .get("skip_reason")
      .is_none()
  );
  // Written as text, since `serde_json::Value` orders a map's members by name.
  let text = r#"{"schema_version":"1.1","generator":{"name":"catalog","version":"2"},"results":[{"index":0,"input":"a","outcome":"skipped","skip_reason":{"kind":"excluded"},"retry_after":"PT1H"}]}"#;
  let rewritten = serde_json::to_string(&serde_json::from_str::<Document>(text).unwrap()).unwrap();
  assert!(
    rewritten.ends_with(r#""outcome":"skipped","skip_reason":{"kind":"excluded"},"retry_after":"PT1H"}]}"#),
    "{rewritten}"
  );
  let mut with_null = skipped_document();
  with_null["results"][0]["retry_after"] = Value::Null;
  assert!(serde_json::from_value::<Document>(with_null).is_err());
  let mut with_record = skipped_document();
  let mut other_version = error;
  other_version["schema_version"] = json!("1.0");
  with_record["results"][0]["error"] = other_version;
  assert!(
    serde_json::from_value::<Document>(with_record).is_err(),
    "a nested record of another version"
  );
}
