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

//! A section that was not requested holds nothing (`api.md` section 4.3), as the maintainer chose at the third
//! review of the third slice's first part, which read a tag set beside tags "not requested", and an edition and an
//! attachment beside chapters and attachments alike. Each section is held to its own list, so cover art requested on
//! its own stands beside tags that were skipped, and a section that does not apply to the format or whose status is
//! of a newer version is not checked.

use cc_media_probe::MediaReport;
use serde_json::{Value, json};

use crate::fixtures::report;

fn reads(report: &Value) -> bool {
  serde_json::from_value::<MediaReport>(report.clone()).is_ok()
}

#[test]
fn a_section_that_was_not_requested_holds_nothing() {
  let tag_set = json!({
    "stream_targets": [],
    "edition_targets": [],
    "chapter_targets": [],
    "attachment_targets": [],
    "target_scope": "whole_file",
    "superseded": false,
    "tags": [],
  });
  let edition = json!({ "origin": "matroska_edition", "chapters": [] });
  for (section, list, item) in [
    ("tags", "tag_sets", tag_set),
    ("chapters", "chapter_editions", edition),
    ("attachments", "attachments", json!({})),
  ] {
    let mut holding = report("1.0");
    holding[list] = json!([item]);
    assert!(reads(&holding), "{section}");
    let with_status = |report: &Value, status: &str| {
      let mut report = report.clone();
      report["completeness"][section] = json!(status);
      report
    };
    // The review's cases.
    assert!(!reads(&with_status(&holding, "not_requested")), "{section}");
    assert!(reads(&with_status(&report("1.0"), "not_requested")), "{section}");
    assert!(reads(&with_status(&holding, "not_applicable")), "{section}");
    assert!(reads(&with_status(&holding, "future_status")), "{section}");
  }
  let mut cover_art = report("1.0");
  cover_art["attachments"] = json!([{}]);
  cover_art["completeness"]["tags"] = json!("not_requested");
  assert!(reads(&cover_art));
}
