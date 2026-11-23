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

//! Two cases of a report that `api.md` sections 5.3 and 6.4 define, read from JSON since the stub builds no report:
//! a limit that stopped one candidate's validation check while another validated, and the established description
//! of a track's first sample that the probe could not keep.

use std::path::Path;

use cc_media_probe::completeness::{DetectionStatus, Part};
use cc_media_probe::details::FormatDetails;
use cc_media_probe::document::{DocumentEntry, EntryOutcome};
use cc_media_probe::model::CodecInformation;
use cc_media_probe::source::PortablePath;
use cc_media_probe::{Limit, MediaReport};
use serde_json::{Value, json};

/// A report without streams, whose completeness has that detection status, those reasons and that record. The stub
/// keeps the default form of Serde's derive for the statuses, so a catch-all is written as a tagged value.
fn report(detection: Value, reasons: Value, record: Value) -> MediaReport {
  serde_json::from_value(json!({
    "schema_version": "1.0",
    "format": "Mp4",
    "streams": [],
    "chapter_editions": [],
    "tag_sets": [],
    "attachments": [],
    "diagnostics": [],
    "completeness": {
      "detection": detection,
      "discovery": "Complete",
      "tags": "Complete",
      "chapters": "Complete",
      "attachments": "Complete",
      "reasons": reasons,
      "record": record,
    },
    "usage": { "bytes_read": 0, "peak_working_memory": 0 },
    "sources": [],
  }))
  .unwrap()
}

#[test]
fn a_report_whose_detection_was_cut_short_is_incomplete() {
  // The working-memory limit stopped the check of one candidate, and the other candidate validated.
  let stopped = report(
    json!("Incomplete"),
    json!([{
      "part": "Detection",
      "stream": null,
      "source_identifier": null,
      "reason": { "kind": "LimitExhausted", "limit": "WorkingMemory", "detail": null },
    }]),
    json!({ "exhausted": ["WorkingMemory"], "first_stopped": [["WorkingMemory", "Detection"]] }),
  );
  let completeness = stopped.completeness();
  assert_eq!(completeness.detection(), &DetectionStatus::Incomplete);
  assert!(!completeness.is_complete());
  assert_eq!(completeness.reasons()[0].part(), &Part::Detection);
  assert!(completeness.record().exhausted(&Limit::WorkingMemory));
  assert_eq!(
    completeness.record().first_stopped(&Limit::WorkingMemory),
    Some(&Part::Detection)
  );
  let input = || PortablePath::from_path(Path::new("clip.mp4"));
  let entry = DocumentEntry::from_report(input(), stopped).unwrap();
  assert_eq!(entry.outcome(), &EntryOutcome::Partial);
  let empty_record = json!({ "exhausted": [], "first_stopped": [] });
  let checked = report(json!("Complete"), json!([]), empty_record.clone());
  assert!(checked.completeness().is_complete());
  assert_eq!(
    DocumentEntry::from_report(input(), checked).unwrap().outcome(),
    &EntryOutcome::Ok
  );
  // A detection status that a newer version wrote decides an outcome that this version cannot derive.
  let newer = report(json!({ "Unrecognized": "checked_by_content" }), json!([]), empty_record);
  assert!(DocumentEntry::from_report(input(), newer).is_err());
}

fn codec(candidates: Value, has_every_candidate: bool) -> CodecInformation {
  serde_json::from_value(json!({
    "codec": null,
    "candidates": candidates,
    "has_every_candidate": has_every_candidate,
    "raw_identifiers": [],
    "profile": null,
    "level": null,
    "codec_string": null,
  }))
  .unwrap()
}

#[test]
fn a_known_description_that_was_not_kept_leaves_the_codec_absent() {
  // The first fragment selects description 2, which the retained-data limit kept the probe from keeping.
  let lost = codec(json!([]), false);
  assert_eq!(lost.codec(), None);
  assert_eq!(lost.name(), None);
  assert!(lost.candidates().is_empty());
  assert!(!lost.has_every_candidate());
  assert_eq!(lost.to_string(), "a description that was not read");
  let details: FormatDetails = serde_json::from_value(json!({
    "IsoBaseMedia": {
      "fragment_track_identifiers": ["1"],
      "tracks": [{ "stream_index": 0, "first_sample_description_index": 2 }],
    },
  }))
  .unwrap();
  let FormatDetails::IsoBaseMedia(iso) = details else {
    panic!("the ISO details");
  };
  assert_eq!(iso.track(0).unwrap().first_sample_description_index(), Some(2));
  // Before the selection is established, the kept description 1 is a candidate, and the details hold no index.
  let unresolved = codec(
    json!([{ "sample_description_index": 1, "codec": "H264", "raw_identifiers": [] }]),
    false,
  );
  assert_eq!(unresolved.to_string(), "H.264 or a description that was not read");
}

#[test]
fn a_full_budget_keeps_the_first_reason_of_each_part_in_the_record() {
  // The retained-data limit was full when discovery stopped, so `reasons()` holds nothing; the record still says why.
  let reason = json!({
    "part": "Discovery",
    "stream": null,
    "source_identifier": null,
    "reason": { "kind": "TruncatedInput", "limit": null, "detail": null },
  });
  // The stub's `Limit` has three counters, and the retained data is not among them, so the record leaves the
  // exhausted counter out.
  let record = json!({
    "exhausted": [],
    "first_stopped": [],
    "first_reasons": [reason],
    "omitted_reasons": 3,
  });
  let full = report(json!("Complete"), json!([]), record);
  let completeness = full.completeness();
  assert!(completeness.reasons().is_empty());
  let first = completeness.record().first_reason(&Part::Discovery).unwrap();
  assert_eq!(first.reason().kind().as_str(), "truncated_input");
  assert_eq!(first.reason().detail(), None, "the record keeps no detail text");
  assert_eq!(completeness.record().first_reason(&Part::Detection), None);
  assert_eq!(completeness.record().omitted_reasons(), 3);
  // A first reason is a known shortfall, so the entry is `partial`, never `ok`.
  let entry = DocumentEntry::from_report(PortablePath::from_path(Path::new("clip.mp4")), full).unwrap();
  assert_eq!(entry.outcome(), &EntryOutcome::Partial);
}
