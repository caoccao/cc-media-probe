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

//! The reasons of a companion's requests and the source entries that keep them (`api.md` sections 6.15 and 13), as
//! the third review of the third slice's first part asked, which read an entry not found beside the reason
//! `companion_declined` of the same source, kept and first, with no reason omitted:
//!
//! - **A reason that names a request,** `companion_not_found`, `companion_declined` or `companion_limited` with a
//!   source, is that source's entry's reason, apart from its detail, among the kept reasons and the first ones. A
//!   `companion_limited` without a source is a companion refused before its request, which has no entry.
//! - **An entry not opened,** not found, declined or refused by a limit, gives the companions a reason with its
//!   source, kept within the retained-data limit and counted among the omitted reasons beyond it. Its reason is
//!   then a first reason or among the omitted ones, so the first reason of the companions counts once.
//!
//! An outcome or a reason of a newer version is not checked.

use std::collections::HashMap;

use super::{SourceEntry, SourceIdentifier, SourceOutcome};
use crate::completeness::{Completeness, Part, PartReason, stated_reasons_not_kept};

/// Whether each reason that names a request is the reason of the entry that it names.
pub(crate) fn requests_agree<'reason>(
  sources: &[SourceEntry],
  reasons: impl IntoIterator<Item = &'reason PartReason>,
) -> bool {
  let entries: HashMap<SourceIdentifier, &SourceEntry> =
    sources.iter().map(|entry| (entry.identifier(), entry)).collect();
  reasons.into_iter().all(
    |reason| match (reason.reason().is_a_requests(), reason.source_identifier()) {
      (true, Some(source)) => entries.get(&source).is_none_or(|entry| {
        matches!(entry.outcome(), SourceOutcome::Unrecognized(_))
          || entry
            .reason()
            .is_some_and(|own| own.same_apart_from_detail(reason.reason()))
      }),
      _ => true,
    },
  )
}

/// The reasons of the entries not opened that the completeness neither keeps nor gives as the first reason of the
/// companions, which it counts among the first reasons that it lacks.
pub(crate) fn request_reasons_not_kept(sources: &[SourceEntry], completeness: &Completeness) -> u64 {
  let refused = sources
    .iter()
    .filter(|entry| entry.was_refused())
    .filter_map(|entry| entry.reason().map(|reason| (entry.identifier(), reason)));
  stated_reasons_not_kept(completeness, &Part::Companions, PartReason::source_identifier, refused)
}
