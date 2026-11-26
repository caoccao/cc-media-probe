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

//! `SectionRequest`, whether a probe reads a section of the report (`api.md` section 4.3).

/// Whether a probe reads the tags or the chapters. The default of both options is `Read`; `Skip` leaves the
/// section "not requested" in the completeness record (`api.md` section 5.3).
///
/// ```
/// use cc_media_probe::Prober;
/// use cc_media_probe::model::SectionRequest;
///
/// let prober = Prober::new().with_tags(SectionRequest::Skip);
/// assert_eq!(prober.tags(), &SectionRequest::Skip);
/// assert_eq!(prober.chapters(), &SectionRequest::Read);
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum SectionRequest {
  /// The probe reads the section.
  Read,
  /// The probe leaves the section out, and its status is "not requested".
  Skip,
}

#[cfg(test)]
mod tests {
  use super::SectionRequest;

  #[test]
  fn tells_the_two_requests_apart() {
    assert_ne!(SectionRequest::Read, SectionRequest::Skip);
    assert_eq!(format!("{:?}", SectionRequest::Skip), "Skip");
  }
}
