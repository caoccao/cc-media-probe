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

//! The ninth review's cases of a catch-all whose payload leaves the enum that read it, `api.md` sections 3.3 and 12.
//! Each payload names its enum as its type parameter, as the maintainer chose, so moving one into another enum does
//! not compile: the doc tests of `UnrecognizedValue`, `UnrecognizedKind` and `UnrecognizedDetails` show it. What
//! remains to check at run time is that a payload put back into its own enum reads back, and that an outcome of a
//! newer version is never given to an entry that this version builds.

use std::path::Path;

use cc_media_probe::document::{Document, DocumentEntry, EntryOutcome, Generator};
use cc_media_probe::model::{ChromaFormat, TagValue};
use cc_media_probe::source::PortablePath;
use serde_json::{Value, json};

#[test]
fn a_kind_put_back_into_its_own_enum_reads_back() {
  // `text` is no kind of `ChromaFormat`, so it reads into that enum's catch-all, and only that enum takes it back.
  let chroma: ChromaFormat = serde_json::from_value(json!({ "kind": "text", "other": 7 })).unwrap();
  let ChromaFormat::Unrecognized(kind) = chroma else {
    panic!("a kind that this enum does not know");
  };
  let again = ChromaFormat::Unrecognized(kind);
  let written = serde_json::to_value(&again).unwrap();
  assert_eq!(serde_json::from_value::<ChromaFormat>(written.clone()).unwrap(), again);
  // The tag value's own catch-all keeps its own kinds' members, which the chroma format's never held.
  let value: TagValue = serde_json::from_value(json!({ "kind": "geolocation", "geolocation": "1" })).unwrap();
  let TagValue::Unrecognized(kind) = value else {
    panic!("a kind that this enum does not know");
  };
  assert_eq!(
    serde_json::to_value(TagValue::Unrecognized(kind)).unwrap()["geolocation"],
    json!("1")
  );
}

/// A report beside a status of a newer version, which leaves its outcome to the caller.
fn report_beside_a_newer_status() -> Value {
  json!({
    "schema_version": "1.0",
    "format": "Matroska",
    "streams": [],
    "chapter_editions": [],
    "tag_sets": [],
    "attachments": [],
    "diagnostics": [],
    "completeness": {
      "detection": "Complete",
      "discovery": "Complete",
      "headers": "Complete",
      "tags": { "Unrecognized": "partly_skipped" },
      "chapters": "Complete",
      "attachments": "Complete",
      "reasons": [],
      "record": { "exhausted": [], "first_stopped": [], "first_reasons": [], "omitted_reasons": "0", "omitted_references": "0" },
    },
    "usage": { "bytes_read": 0, "peak_working_memory": 0 },
    "sources": [],
  })
}

#[test]
fn an_entry_takes_only_an_outcome_that_this_version_knows() {
  let input = || PortablePath::from_path(Path::new("movie.mkv"));
  let report = || serde_json::from_value(report_beside_a_newer_status()).unwrap();
  // The only catch-all of an outcome comes from reading one, here `skipped` of a newer version, which can need
  // members that only its version builds.
  let skipped = json!({
    "schema_version": "1.0",
    "generator": { "name": "catalog", "version": "2" },
    "results": [{ "index": 0, "input": "a", "outcome": "skipped" }],
  });
  let read: Document = serde_json::from_value(skipped).unwrap();
  let newer = read.results()[0].outcome().clone();
  assert!(matches!(newer, EntryOutcome::Unrecognized(_)));
  let refused = DocumentEntry::from_report(input(), report()).unwrap_err();
  // The refusal gives its parts back (C-INTERMEDIATE), the report without a copy.
  assert_eq!(refused.input(), &input());
  assert_eq!(refused.report(), Some(&report()));
  assert!(refused.detection().is_none());
  assert!(refused.clone().into_detection().is_none());
  assert_eq!(refused.clone().into_report(), Some(report()));
  let refused_again = refused.clone().with_outcome(newer).unwrap_err();
  assert_eq!(refused_again, refused, "the refusal comes back unchanged");
  // A known outcome that the record allows builds an entry, and the document reads back.
  for outcome in [EntryOutcome::Ok, EntryOutcome::Partial] {
    let entry = DocumentEntry::from_report(input(), report())
      .unwrap_err()
      .with_outcome(outcome)
      .unwrap();
    let written = serde_json::to_value(Document::from_results(Generator::new("catalog", "1"), vec![entry])).unwrap();
    assert!(serde_json::from_value::<Document>(written.clone()).is_ok(), "{written}");
  }
  assert!(
    refused.with_outcome(EntryOutcome::Error).is_err(),
    "a report never warrants error"
  );
}
