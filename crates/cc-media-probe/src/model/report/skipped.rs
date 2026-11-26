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

//! The sections that a caller skipped hold nothing (`api.md` section 4.3; `json-output.md` section 10), as the
//! maintainer chose at the third review of the third slice's first part, which read a tag set beside tags "not
//! requested", and an edition and an attachment beside chapters and attachments alike. Each section is held to its
//! own list: tags "not requested" leave no tag set, chapters no edition, and attachments, which are "not requested"
//! only when both of their options skip, no attachment. A picture that a tag structure carries is an attachment, so
//! cover art requested on its own still stands beside tags that were skipped. A section that does not apply to the
//! format is left to the format phases, and a status of a newer version is not checked.

use super::MediaReport;
use crate::completeness::SectionStatus;

/// Whether each section that was not requested holds nothing.
pub(super) fn check_skipped(report: &MediaReport) -> Result<(), &'static str> {
  let completeness = &report.completeness;
  let holds_nothing = |status: &SectionStatus, empty: bool| !matches!(status, SectionStatus::NotRequested) || empty;
  if holds_nothing(completeness.tags(), report.tag_sets.is_empty())
    && holds_nothing(completeness.chapters(), report.chapter_editions.is_empty())
    && holds_nothing(completeness.attachments(), report.attachments.is_empty())
  {
    Ok(())
  } else {
    Err(super::constants::SKIPPED_SECTION_MESSAGE)
  }
}
