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

//! The twelfth review's case, `api.md` section 5.3, as the maintainer chose: `is_complete()` reads the statuses and
//! the fixed-size record only, so `reasons()` never decides it. A report whose reasons, first reasons and statuses
//! disagree part by part is malformed, read on its own or in a document.

use std::path::Path;

use cc_media_probe::MediaReport;
use cc_media_probe::document::{Document, DocumentEntry, EntryOutcome, Generator};
use cc_media_probe::source::PortablePath;
use serde_json::{Value, json};

/// A report without streams whose tags have that status, with those reasons and those first reasons in the record,
/// and every other part complete.
fn report_text(tags: Value, reasons: Value, first_reasons: Value) -> Value {
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
      "tags": tags,
      "chapters": "Complete",
      "attachments": "Complete",
      "reasons": reasons,
      "record": { "exhausted": [], "first_stopped": [], "first_reasons": first_reasons },
    },
    "usage": { "bytes_read": 0, "peak_working_memory": 0 },
    "sources": [],
  })
}

fn read(tags: Value, reasons: Value, first_reasons: Value) -> Result<MediaReport, serde_json::Error> {
  serde_json::from_value(report_text(tags, reasons, first_reasons))
}

fn outcome(report: MediaReport) -> Option<EntryOutcome> {
  DocumentEntry::from_report(PortablePath::from_path(Path::new("live.mkv")), report)
    .ok()
    .map(|entry| entry.outcome().clone())
}

fn tags_reason() -> Value {
  json!({ "part": "Tags", "reason": { "kind": "TruncatedInput" } })
}

#[test]
fn a_reason_that_the_record_and_the_status_do_not_carry_is_malformed() {
  // The review's two records: the same complete statuses and the same empty first reasons, and one of them lists a
  // reason of the tags. The first is complete; the second contradicts itself, so it is not read.
  let complete = read(json!("Complete"), json!([]), json!([])).unwrap();
  assert!(complete.completeness().is_complete());
  assert_eq!(outcome(complete), Some(EntryOutcome::Ok));
  assert!(read(json!("Complete"), json!([tags_reason()]), json!([])).is_err());
}

#[test]
fn reasons_first_reasons_and_statuses_agree_part_by_part() {
  // A kept reason needs the first reason of its part; a first reason of a known part needs a status that records a
  // shortfall; and such a status needs a first reason, since the record keeps one whatever the limits.
  for (tags, reasons, first_reasons) in [
    (json!("Incomplete"), json!([tags_reason()]), json!([])),
    (json!("Complete"), json!([tags_reason()]), json!([tags_reason()])),
    (json!("Complete"), json!([]), json!([tags_reason()])),
    (json!("Incomplete"), json!([]), json!([])),
  ] {
    let case = format!("{tags} {reasons} {first_reasons}");
    assert!(read(tags, reasons, first_reasons).is_err(), "{case}");
  }
  // Where they agree, the report reads, with or without the kept reason, and the verdict is the same.
  for reasons in [json!([tags_reason()]), json!([])] {
    let short = read(json!("Incomplete"), reasons, json!([tags_reason()])).unwrap();
    assert!(!short.completeness().is_complete());
    assert_eq!(outcome(short), Some(EntryOutcome::Partial));
  }
}

#[test]
fn a_part_or_a_status_of_a_newer_version_agrees_through_the_record() {
  // A part that only a newer version knows has no status here, so its first reason alone says that it fell short.
  let newer_part = json!({ "part": { "Unrecognized": "programs" }, "reason": { "kind": "TruncatedInput" } });
  let newer = read(json!("Complete"), json!([newer_part]), json!([newer_part])).unwrap();
  assert!(!newer.completeness().is_complete());
  assert_eq!(outcome(newer), Some(EntryOutcome::Partial));
  assert!(read(json!("Complete"), json!([newer_part]), json!([])).is_err());
  // Beside a status of a newer version, only the version that wrote it knows whether it records a shortfall.
  let newer_status = json!({ "Unrecognized": "partly_read" });
  assert!(read(newer_status.clone(), json!([]), json!([tags_reason()])).is_ok());
  assert!(read(newer_status, json!([]), json!([])).is_ok());
}

#[test]
fn a_document_entry_that_holds_a_contradicting_report_fails_with_its_document() {
  let complete = read(json!("Complete"), json!([]), json!([])).unwrap();
  let entry = DocumentEntry::from_report(PortablePath::from_path(Path::new("live.mkv")), complete).unwrap();
  let mut text = serde_json::to_value(Document::from_results(Generator::new("catalog", "1"), vec![entry])).unwrap();
  assert!(serde_json::from_value::<Document>(text.clone()).is_ok());
  text["results"][0]["report"]["completeness"]["reasons"] = json!([tags_reason()]);
  for outcome in ["ok", "partial"] {
    text["results"][0]["outcome"] = json!(outcome);
    assert!(serde_json::from_value::<Document>(text.clone()).is_err(), "{outcome}");
  }
}
