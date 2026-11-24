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

//! How a detection is read and written, `json-output.md` section 4.3. Each outcome that this version knows has the
//! members it needs and none that another outcome needs, as the fourteenth review asked, so an entry's outcome is
//! decided from a detection that describes it. An outcome of a newer version reads whatever members it holds.
//! `bytes_needed` beyond 2^53 - 1 is its twin `bytes_needed_exact`, as the maintainer chose at the same review.

use super::detection::{Candidate, Detection, DetectionOutcome, Hint};
use super::format::Format;
use crate::completeness::Reason;
use crate::constants::{DETECTION_MESSAGE, MAXIMUM_CONFIDENCE, MINIMUM_CANDIDATES};
use crate::document::{InvalidDocument, SchemaVersion};
use crate::neutral::Rooted;
use crate::twin::{unsigned_members, unsigned_value};

/// A detection's members as written, the version first.
#[derive(serde::Serialize)]
pub(super) struct DetectionMembers {
  schema_version: SchemaVersion,
  outcome: DetectionOutcome,
  #[serde(skip_serializing_if = "Option::is_none")]
  format: Option<Format>,
  #[serde(skip_serializing_if = "Option::is_none")]
  confidence: Option<u8>,
  candidates: Vec<Candidate>,
  #[serde(skip_serializing_if = "Option::is_none")]
  hint: Option<Hint>,
  #[serde(skip_serializing_if = "Option::is_none")]
  bytes_needed: Option<u64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  bytes_needed_exact: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  insufficient_reason: Option<Reason>,
}

impl From<Detection> for DetectionMembers {
  fn from(detection: Detection) -> DetectionMembers {
    let (bytes_needed, bytes_needed_exact) = unsigned_members(detection.bytes_needed);
    DetectionMembers {
      schema_version: detection.schema_version,
      outcome: detection.outcome,
      format: detection.format,
      confidence: detection.confidence,
      candidates: detection.candidates,
      hint: detection.hint,
      bytes_needed,
      bytes_needed_exact,
      insufficient_reason: detection.insufficient_reason,
    }
  }
}

/// A detection's members as read, other than its version, which is read first (`neutral`).
#[derive(serde::Deserialize)]
pub(super) struct DetectionData {
  outcome: DetectionOutcome,
  format: Option<Format>,
  confidence: Option<u8>,
  candidates: Vec<Candidate>,
  hint: Option<Hint>,
  bytes_needed: Option<u64>,
  bytes_needed_exact: Option<String>,
  insufficient_reason: Option<Reason>,
}

impl TryFrom<Rooted<DetectionData>> for Detection {
  type Error = InvalidDocument;

  fn try_from(rooted: Rooted<DetectionData>) -> Result<Detection, InvalidDocument> {
    let data = rooted.body;
    let bytes_needed = unsigned_value(data.bytes_needed, data.bytes_needed_exact)?;
    let detection = Detection {
      schema_version: rooted.version,
      outcome: data.outcome,
      format: data.format,
      confidence: data.confidence,
      candidates: data.candidates,
      hint: data.hint,
      bytes_needed,
      insufficient_reason: data.insufficient_reason,
    };
    if !(confidences_hold(&detection) && members_fit_the_outcome(&detection)) {
      return Err(InvalidDocument::new(DETECTION_MESSAGE));
    }
    Ok(detection)
  }
}

/// Every confidence lies from 0 to 100, and the candidates' fall with their rank.
fn confidences_hold(detection: &Detection) -> bool {
  let candidates: Vec<u8> = detection.candidates.iter().map(Candidate::confidence).collect();
  detection
    .confidence
    .into_iter()
    .chain(candidates.iter().copied())
    .all(|value| value <= MAXIMUM_CONFIDENCE)
    && candidates.is_sorted_by(|earlier, later| earlier >= later)
}

/// Whether the members are those of the outcome: what it needs present, what another outcome needs absent. A format
/// that this version knows is a family exactly for a family only; a format of a newer version may be either.
fn members_fit_the_outcome(detection: &Detection) -> bool {
  let family = |wanted: bool| {
    detection
      .format
      .as_ref()
      .is_some_and(|format| matches!(format, Format::Unrecognized(_)) || format.is_family() == wanted)
  };
  let no_candidates = detection.candidates.is_empty();
  let no_shortage = detection.bytes_needed.is_none() && detection.insufficient_reason.is_none();
  let no_hint = detection.hint.is_none();
  match detection.outcome {
    DetectionOutcome::Identified => {
      family(false) && detection.confidence.is_some() && no_candidates && no_hint && no_shortage
    }
    DetectionOutcome::FamilyOnly => {
      family(true) && detection.confidence.is_none() && no_candidates && no_hint && no_shortage
    }
    DetectionOutcome::Ambiguous => {
      detection.candidates.len() >= MINIMUM_CANDIDATES
        && detection.format.is_none()
        && detection.confidence.is_none()
        && no_hint
        && no_shortage
    }
    DetectionOutcome::HintOnly => {
      detection.hint.is_some()
        && detection.format.is_none()
        && detection.confidence.is_none()
        && no_candidates
        && no_shortage
    }
    DetectionOutcome::InsufficientData => {
      detection.format.is_none() && detection.confidence.is_none() && no_candidates && no_hint
    }
    DetectionOutcome::Unknown => {
      detection.format.is_none() && detection.confidence.is_none() && no_candidates && no_hint && no_shortage
    }
    DetectionOutcome::Unrecognized(_) => true,
  }
}
