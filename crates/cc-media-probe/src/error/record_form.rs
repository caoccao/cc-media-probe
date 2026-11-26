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

//! How an error record is read and written (`json-output.md` section 4.4). Its kind decides the members that it
//! keeps: a kind of a newer version keeps every member that it does not read and writes it back after its own, as
//! the maintainer chose at the fifteenth review of `api.md`, held only until the kind is known (eighteenth review).
//! Reading keeps the record true to itself:
//!
//! - a partial report and observations are two stopping points, so a record holds one at most, and a record with a
//!   partial report has that report's record;
//! - only a source failure carries a partial report or the members of one, `source_failure_kind` always and
//!   `operating_system_error_code` when the error has a code, while a kind of a newer version may hold any of them;
//! - an operating system's code is a 64-bit integer, signed or unsigned, in canonical decimal, beside any kind;
//! - the error's own limit reason finds its counter among the record's exhausted limits;
//! - the candidates name each format once, fall in confidence, stand in the order of their checks, and give the
//!   record its first reason of detection (`candidates`, the third review of the third slice's first part);
//! - a known kind has the format, the hint, the candidates and the reason that it allows and at most 32 causes
//!   (`kind_rules`, the review of the third slice's first part and its later reviews);
//! - what the record names exists in it: the error's source and the sources of the first reasons in its source
//!   table, a failed source as the one that failed, no stream before an inventory, the partial report's format, the
//!   counters of the limits that its reasons name, the reasons of the requests that its entries keep, and no fewer
//!   bytes read than its partial report or its sources read (`references`, the same reviews).

use serde::Serialize;

use super::parts::ErrorParts;
use super::record::Description;
use super::{CandidateCheck, ErrorKind, ErrorRecord, Observations};
use crate::Format;
use crate::completeness::{LimitRecord, Reason};
use crate::document::SchemaVersion;
use crate::form::{KeptMembers, Outcome, Rooted, WithKept, canonical, unsigned_members, unsigned_value};
use crate::model::{Hint, MediaReport, Usage};
use crate::source::{AttemptedIdentity, SourceIdentifier};

/// An error record as written: its version first, then its members, then the members that a kind of a newer version
/// kept.
#[derive(Serialize)]
struct ErrorRecordMembers<'record> {
  schema_version: SchemaVersion,
  kind: &'record ErrorKind,
  message: &'record str,
  #[serde(skip_serializing_if = "Option::is_none")]
  format: Option<&'record Format>,
  candidates: &'record [CandidateCheck],
  #[serde(skip_serializing_if = "Option::is_none")]
  hint: Option<&'record Hint>,
  #[serde(skip_serializing_if = "Option::is_none")]
  reason: Option<&'record Reason>,
  #[serde(skip_serializing_if = "Option::is_none")]
  source_identifier: Option<SourceIdentifier>,
  #[serde(skip_serializing_if = "Option::is_none")]
  attempted_identity: Option<&'record AttemptedIdentity>,
  #[serde(skip_serializing_if = "Option::is_none")]
  offset: Option<u64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  offset_exact: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  logical_offset: Option<u64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  logical_offset_exact: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  source_failure_kind: Option<&'record str>,
  #[serde(skip_serializing_if = "Option::is_none")]
  operating_system_error_code: Option<&'record str>,
  cause_messages: &'record [String],
  #[serde(skip_serializing_if = "Option::is_none")]
  partial_report: Option<&'record MediaReport>,
  #[serde(skip_serializing_if = "Option::is_none")]
  observations: Option<&'record Observations>,
  usage: &'record Usage,
  record: &'record LimitRecord,
  #[serde(flatten)]
  kept: &'record KeptMembers,
}

impl Serialize for ErrorRecord {
  fn serialize<Output: serde::Serializer>(&self, serializer: Output) -> Result<Output::Ok, Output::Error> {
    let (parts, description) = (&self.parts, &self.description);
    let (offset, offset_exact) = unsigned_members(parts.offset);
    let (logical_offset, logical_offset_exact) = unsigned_members(parts.logical_offset);
    ErrorRecordMembers {
      schema_version: self.schema_version,
      kind: &parts.kind,
      message: &description.message,
      format: parts.format.as_ref(),
      candidates: &parts.candidates,
      hint: parts.hint.as_ref(),
      reason: parts.reason.as_ref(),
      source_identifier: parts.source_identifier,
      attempted_identity: parts.attempted_identity.as_ref(),
      offset,
      offset_exact,
      logical_offset,
      logical_offset_exact,
      source_failure_kind: description.source_failure_kind.as_deref(),
      operating_system_error_code: description.operating_system_error_code.as_deref(),
      cause_messages: &description.cause_messages,
      partial_report: parts.partial_report.as_deref(),
      observations: parts.observations.as_ref(),
      usage: &parts.usage,
      record: &parts.record,
      kept: &self.kept,
    }
    .serialize(serializer)
  }
}

/// The members of an error record other than its version, as read.
#[derive(serde::Deserialize)]
pub(super) struct ErrorRecordBody {
  kind: ErrorKind,
  message: String,
  format: Option<Format>,
  candidates: Vec<CandidateCheck>,
  hint: Option<Hint>,
  reason: Option<Reason>,
  source_identifier: Option<SourceIdentifier>,
  attempted_identity: Option<AttemptedIdentity>,
  offset: Option<u64>,
  offset_exact: Option<String>,
  logical_offset: Option<u64>,
  logical_offset_exact: Option<String>,
  source_failure_kind: Option<String>,
  operating_system_error_code: Option<String>,
  cause_messages: Vec<String>,
  partial_report: Option<MediaReport>,
  observations: Option<Observations>,
  usage: Usage,
  record: LimitRecord,
}

impl Outcome for ErrorRecordBody {
  const MEMBER: &'static str = super::constants::KIND_MEMBER;

  fn is_newer(text: &str) -> bool {
    matches!(ErrorKind::from_text(text.to_owned()), ErrorKind::Unrecognized(_))
  }
}

impl TryFrom<Rooted<WithKept<ErrorRecordBody>>> for ErrorRecord {
  type Error = &'static str;

  fn try_from(rooted: Rooted<WithKept<ErrorRecordBody>>) -> Result<ErrorRecord, &'static str> {
    let (body, kept) = (rooted.body.data, rooted.body.kept);
    let parts = ErrorParts {
      kind: body.kind,
      format: body.format,
      candidates: body.candidates,
      hint: body.hint,
      reason: body.reason,
      source_identifier: body.source_identifier,
      attempted_identity: body.attempted_identity,
      offset: unsigned_value(body.offset, body.offset_exact)?,
      logical_offset: unsigned_value(body.logical_offset, body.logical_offset_exact)?,
      partial_report: body.partial_report.map(Box::new),
      observations: body.observations,
      usage: body.usage,
      record: body.record,
    };
    let description = Description {
      message: body.message,
      source_failure_kind: body.source_failure_kind,
      operating_system_error_code: body.operating_system_error_code,
      cause_messages: body.cause_messages,
    };
    check_record(&parts, &description)?;
    Ok(ErrorRecord {
      schema_version: rooted.version,
      parts,
      description,
      kept,
    })
  }
}

/// The checks of an error record that need more than one of its members.
fn check_record(parts: &ErrorParts, description: &Description) -> Result<(), &'static str> {
  use super::constants::{
    ERROR_REASON_MESSAGE, OPERATING_SYSTEM_CODE_MESSAGE, SOURCE_FAILURE_MESSAGE, STOPPING_POINTS_MESSAGE,
  };
  let failed = matches!(parts.kind, ErrorKind::SourceFailed);
  let known = !matches!(parts.kind, ErrorKind::Unrecognized(_));
  let report_record = parts
    .partial_report
    .as_ref()
    .map(|report| report.completeness().record());
  if (parts.partial_report.is_some() && parts.observations.is_some())
    || report_record.is_some_and(|record| *record != parts.record)
  {
    return Err(STOPPING_POINTS_MESSAGE);
  }
  let source_members = description.source_failure_kind.is_some() || description.operating_system_error_code.is_some();
  if (failed && description.source_failure_kind.is_none())
    || (known && !failed && (source_members || parts.partial_report.is_some()))
  {
    return Err(SOURCE_FAILURE_MESSAGE);
  }
  let code_canonical = description
    .operating_system_error_code
    .as_deref()
    .is_none_or(|code| canonical::<i64>(code).is_some() || canonical::<u64>(code).is_some());
  if !code_canonical {
    return Err(OPERATING_SYSTEM_CODE_MESSAGE);
  }
  let reason_found = parts
    .reason
    .as_ref()
    .is_none_or(|reason| reason.finds_its_counter(|limit| parts.record.exhausted(limit)));
  if !reason_found {
    return Err(ERROR_REASON_MESSAGE);
  }
  super::candidates::check_candidates(parts)?;
  super::detection_stops::check_detection_stops(parts)?;
  super::kind_rules::check_kind(parts, description)?;
  super::references::check_references(parts)
}
