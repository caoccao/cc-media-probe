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

//! The source table as a whole (`api.md` section 6.15; `json-output.md` section 10), which a report and the
//! observations of an error hold: as the maintainer chose at the review of the third slice's first part, it names
//! each source once and holds the primary input, which every probe opens first, and an alias names an entry of the
//! table. Each entry checks its own identifier against its role, so the identifier 0 is the primary input's.
//!
//! An alias names the first source that a later request led to again (section 13): an entry of a lower identifier,
//! since requests take their identifiers in order, which is no alias itself. So following an alias ends at once, as
//! the second review of the third slice's first part asked, which read two entries each the alias of the other. An
//! alias is a source that was found and opened, and then closed again because its identity was that of a source
//! already open, so, as the third review of that part asked, which read the alias of a source not found, the alias
//! was opened and the source it names was opened, or failed later, in the partial data of an error. As its fourth
//! review asked, an entry checks alone what it can know alone: that it was opened and names an earlier source. An
//! outcome of a newer version is not checked.
//!
//! Every source of the probe charges the one budget (plan section 7.5), so the bytes that the entries read add up
//! to at most the usage's bytes read, as that review asked, which read a primary input of 150 bytes beside a usage
//! of 64.

use std::collections::{HashMap, HashSet};

use super::{SourceEntry, SourceIdentifier, SourceOutcome};

/// The identifiers of the table, or the message of a table that names a source twice, lacks the primary input or
/// holds an alias of a source that it does not name, that is not earlier, that is an alias itself or whose outcome
/// no alias has.
pub(crate) fn table_identifiers(sources: &[SourceEntry]) -> Result<HashSet<SourceIdentifier>, &'static str> {
  let mut entries: HashMap<SourceIdentifier, &SourceEntry> = HashMap::with_capacity(sources.len());
  let once = sources
    .iter()
    .all(|entry| entries.insert(entry.identifier(), entry).is_none());
  let primary = entries.contains_key(&SourceIdentifier::primary());
  // An entry checks on its own that it was opened and names an earlier source (`super::entry`).
  let aliases_end = sources.iter().all(|entry| {
    entry.alias_of().is_none_or(|first| {
      entries.get(&first).is_some_and(|target| {
        target.alias_of().is_none()
          && matches!(
            target.outcome(),
            SourceOutcome::Opened | SourceOutcome::Failed | SourceOutcome::Unrecognized(_)
          )
      })
    })
  });
  if once && primary && aliases_end {
    Ok(entries.into_keys().collect())
  } else {
    Err(super::constants::SOURCE_TABLE_MESSAGE)
  }
}

/// Whether the bytes that the entries read add up to at most `total`, the bytes read of the usage beside them.
pub(crate) fn bytes_read_within(sources: &[SourceEntry], total: u64) -> bool {
  sources
    .iter()
    .filter_map(SourceEntry::bytes_read)
    .try_fold(0_u64, u64::checked_add)
    .is_some_and(|read| read <= total)
}
