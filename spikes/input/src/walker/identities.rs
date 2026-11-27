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

//! `Identities`, the identities of the sources that one probe has open, which find a second request that leads to
//! a source already open. Each is held for the rest of the probe in a buffer of its own, charged to working memory
//! before it is reserved, as the list that holds them is before it grows (plan section 7.7).

use cc_media_probe::Limit;

use crate::budget::Budget;
use crate::utilities::widen;

/// The identities held, with the source identifier of each, and their charge.
#[derive(Debug, Default)]
pub(super) struct Identities {
  held: Vec<(u64, Vec<u8>)>,
  charged: u64,
}

impl Identities {
  /// The earlier source whose identity is `identity`.
  pub(super) fn alias(&self, identity: &[u8]) -> Option<u64> {
    self
      .held
      .iter()
      .find(|(_, known)| known.as_slice() == identity)
      .map(|(source, _)| *source)
  }

  /// Holds a copy of the identity of `source`, charged before it is reserved, or names the limit when working
  /// memory cannot hold it.
  pub(super) fn keep(&mut self, budget: &mut Budget, source: u64, identity: &[u8]) -> Result<(), Limit> {
    let held = &mut self.held;
    budget.require(None, |budget| budget.try_reserve_one(held))?;
    let mut copy: Vec<u8> = budget.require(None, |budget| budget.try_allocate(identity.len()))?;
    copy.extend_from_slice(identity);
    self.charged = self.charged.saturating_add(widen(copy.capacity()));
    held.push((source, copy));
    Ok(())
  }

  /// Releases every identity and the list, and returns their charges.
  pub(super) fn release(self, budget: &mut Budget) {
    budget.release_memory(self.charged);
    budget.free(self.held);
  }
}
