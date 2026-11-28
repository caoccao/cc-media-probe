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

//! The usage of each adapter's record against the limits that the adapter was given (`docs/design/test-kit.md`
//! section 4.1), checked as the library wrote it, before its paths are netted and before anything is compared or
//! written, so that an outcome of its own never hides a limit that an adapter passed:
//! - the usage is read by the library's strict reader, so each counter has exactly one valid form, its number or
//!   beyond 2^53 − 1 its twin, and a missing usage or counter fails, as the fifth review of the proposal found;
//! - the record's usage and its partial report's each keep every counter within its limit, since the partial
//!   report's peak working memory is no part of the error's bytes read, as the fifth review found too, and a partial
//!   report needs its usage, as the sixth found;
//! - the bytes that the test's own counters measured stay within the read limit, and are the usage's bytes read,
//!   which counts every byte that a source delivers (plan sections 7.5 and 9.4).

use cc_media_probe::model::Usage;
use cc_media_probe::{Limit, Limits};
use cc_media_probe_testkit::readers::Counts;
use serde_json::Value;

/// The limits that bound a counter of the usage, each with the counter's member (`json-output.md` section 7.13).
const BOUNDED: [(&str, Limit); 8] = [
  ("bytes_read", Limit::BytesRead),
  ("peak_working_memory", Limit::WorkingMemory),
  ("work_units", Limit::WorkUnits),
  ("retained_data", Limit::RetainedData),
  ("retained_payloads", Limit::RetainedPayloads),
  ("decompressed_output", Limit::DecompressedOutput),
  ("companion_requests", Limit::CompanionRequests),
  ("sources_opened", Limit::SourcesOpened),
];
/// The place of a record's own usage.
const OWN: &str = "/usage";
/// The place of an error's partial report.
const PARTIAL_REPORT: &str = "/partial_report";
/// The place of its partial report's usage.
const PARTIAL: &str = "/partial_report/usage";
/// The message of a record without a usage.
const NO_USAGE: &str = "the record has no usage at";
/// The message of a usage that is not the form's.
const MALFORMED: &str = "the usage is not the form's at";
/// The message of a counter beyond its limit.
const BEYOND: &str = "the usage passes its limit through";
/// The message of a delivery beyond the read limit.
const OVERREAD: &str = "the bytes delivered pass the read limit through";
/// The message of bytes read that are not the bytes delivered.
const UNCOUNTED: &str = "the usage's bytes read are not the bytes delivered through";

/// The usage at `place` of `record`, read strictly.
fn usage_at(record: &Value, place: &str, adapter: &str) -> Result<Usage, String> {
  let usage = record
    .pointer(place)
    .ok_or_else(|| format!("{NO_USAGE} {place} of {adapter}"))?;
  serde_json::from_value::<Usage>(usage.clone()).map_err(|error| format!("{MALFORMED} {place} of {adapter}: {error}"))
}

/// Checks that `record` has a usage of the form, and its partial report too wherever the record has one, and that
/// each counter of each stays within its limit in `limits`, the limits that the adapter was given. A partial report
/// without a usage fails, as the sixth review of the proposal found it passed as no partial report at all.
///
/// # Errors
///
/// Returns the usage that is missing or not the form's, or the counter beyond its limit, with `adapter`.
pub fn within_limits(record: &Value, limits: &Limits, adapter: &str) -> Result<(), String> {
  let own = usage_at(record, OWN, adapter)?;
  let partial = record
    .pointer(PARTIAL_REPORT)
    .map(|_| usage_at(record, PARTIAL, adapter))
    .transpose()?;
  for (place, usage) in [(OWN, Some(own)), (PARTIAL, partial)] {
    let Some(usage) = usage else {
      continue;
    };
    for (member, limit) in &BOUNDED {
      if let (Some(counted), Some(bound)) = (usage.get(limit), limits.get(limit))
        && counted > bound
      {
        return Err(format!("{BEYOND} {adapter}: {place}/{member} {counted} > {bound}"));
      }
    }
  }
  Ok(())
}

/// Checks that the bytes that the test's counters measured under `adapter`, where it counts them, stay within the
/// read limit of `limits`, and are the bytes read of `record`'s usage. The counters cover every source of a probe
/// that the test hands the library, the primary input alone before Phase 14 adds companions.
///
/// # Errors
///
/// Returns the delivery beyond the limit, or the bytes read that differ from it, with `adapter`.
pub fn within_reads(record: &Value, counts: Option<Counts>, limits: &Limits, adapter: &str) -> Result<(), String> {
  let Some(counts) = counts else {
    return Ok(());
  };
  let bound = limits.bytes_read();
  if counts.delivered > bound {
    return Err(format!("{OVERREAD} {adapter}: {} > {bound}", counts.delivered));
  }
  let reported = usage_at(record, OWN, adapter)?.bytes_read();
  if reported != counts.delivered {
    return Err(format!("{UNCOUNTED} {adapter}: {reported} != {}", counts.delivered));
  }
  Ok(())
}
