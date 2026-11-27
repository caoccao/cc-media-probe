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

//! `Catalog`, the companions that the primary input names (plan section 7.7). Each identifier is listed once, in
//! the order found, which the report keeps as its expected companions, and found again through the identifier set.
//! With lookup on, the queue of requests holds only the offset of each identifier's first element beside it, so it
//! duplicates no name; the queue and the set are working storage, charged before they grow.

use std::mem;

use cc_media_probe::Limit;

use super::identifier_set::IdentifierSet;
use crate::budget::{Budget, Reclaim};

/// The companions found so far.
#[derive(Debug, Default)]
pub(crate) struct Catalog {
  /// The validated identifiers in the order found.
  pub(crate) identifiers: Vec<String>,
  /// With lookup on, the offset of the element that first named each identifier, in the same order.
  offsets: Vec<u64>,
  set: IdentifierSet,
}

impl Catalog {
  /// Lists `identifier`, named by the element at `offset`, unless it is listed already, and with `queued` makes it
  /// wait for its request. Says whether it is new; a charge that working memory cannot hold names the limit.
  pub(crate) fn add(
    &mut self,
    budget: &mut Budget,
    room: &mut dyn Reclaim,
    identifier: &[u8],
    offset: u64,
    queued: bool,
  ) -> Result<bool, Limit> {
    if !self.set.insert(budget, &mut *room, identifier)? {
      return Ok(false);
    }
    self.identifiers.push(String::from_utf8_lossy(identifier).into_owned());
    if queued {
      let offsets = &mut self.offsets;
      budget.require(Some(room), |budget| budget.try_reserve_one(offsets))?;
      offsets.push(offset);
    }
    Ok(true)
  }

  /// The count of companions that wait for their request.
  pub(crate) fn queued(&self) -> usize {
    self.offsets.len()
  }

  /// The request at `index` of the queue, in the order found: the offset of its element and its identifier.
  pub(crate) fn request(&self, index: usize) -> Option<(u64, &str)> {
    Some((*self.offsets.get(index)?, self.identifiers.get(index)?.as_str()))
  }

  /// Releases the queue and the set and returns their charges; the identifiers stay for the report.
  pub(crate) fn release(&mut self, budget: &mut Budget) {
    budget.free(mem::take(&mut self.offsets));
    self.set.release(budget);
  }
}
