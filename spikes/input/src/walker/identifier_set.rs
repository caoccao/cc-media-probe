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

//! `IdentifierSet`, the companion identifiers found so far, so that a repeated identifier is found again in a bounded
//! number of steps however many came before it. An identifier is at most 16 bytes, so each is held as one
//! fixed-size key in an open-addressing table that never fills beyond half. The hash takes random keys for each
//! set, as the standard library's maps do, so identifiers chosen to collide cannot make the lookups quadratic. The
//! table is charged to working memory before it is reserved, as every buffer of the walk is.

use std::hash::{BuildHasher, RandomState};

use cc_media_probe::Limit;

use super::constants::{EMPTY_SLOT, KEY_LENGTH, MINIMUM_SLOTS, SLOT_GROWTH, SLOTS_PER_KEY};
use crate::budget::{Budget, Reclaim};
use crate::utilities::narrow;

/// The identifiers found, as keys in an open-addressing table.
#[derive(Debug, Default)]
pub(crate) struct IdentifierSet {
  slots: Vec<u128>,
  count: usize,
  hasher: RandomState,
}

impl IdentifierSet {
  /// Adds `identifier`, and says whether it is new. Growing the table is a charge that the walk requires.
  pub(crate) fn insert(
    &mut self,
    budget: &mut Budget,
    room: &mut dyn Reclaim,
    identifier: &[u8],
  ) -> Result<bool, Limit> {
    let key = key_of(identifier);
    if self.find(key).is_some_and(|index| self.slots.get(index) == Some(&key)) {
      return Ok(false);
    }
    if self.count.saturating_add(1).saturating_mul(SLOTS_PER_KEY) > self.slots.len() {
      self.grow(budget, room)?;
    }
    if let Some(slot) = self.find(key).and_then(|index| self.slots.get_mut(index)) {
      *slot = key;
      self.count = self.count.saturating_add(1);
    }
    Ok(true)
  }

  /// Releases the table and returns its charge.
  pub(crate) fn release(&mut self, budget: &mut Budget) {
    budget.free(std::mem::take(&mut self.slots));
    self.count = 0;
  }

  /// The slot that holds `key`, or else the empty slot where it belongs: probing goes on from the slot its hash
  /// names, and the table always has an empty slot. `None` for a table not yet reserved.
  fn find(&self, key: u128) -> Option<usize> {
    let mask = self.slots.len().checked_sub(1)?;
    let mut index = narrow(self.hasher.hash_one(key)) & mask;
    loop {
      match self.slots.get(index) {
        Some(&slot) if slot != key && slot != EMPTY_SLOT => index = index.wrapping_add(1) & mask,
        _ => return Some(index),
      }
    }
  }

  /// Doubles the table, charged while the old one is still charged, and moves every key into it.
  fn grow(&mut self, budget: &mut Budget, room: &mut dyn Reclaim) -> Result<(), Limit> {
    let size = self.slots.len().saturating_mul(SLOT_GROWTH).max(MINIMUM_SLOTS);
    let mut table: Vec<u128> = budget.require(Some(room), |budget| budget.try_allocate(size))?;
    table.resize(size, EMPTY_SLOT);
    let old = std::mem::replace(&mut self.slots, table);
    for key in old.iter().copied().filter(|key| *key != EMPTY_SLOT) {
      if let Some(slot) = self.find(key).and_then(|index| self.slots.get_mut(index)) {
        *slot = key;
      }
    }
    budget.free(old);
    Ok(())
  }
}

/// The key of a validated identifier: its bytes, padded with zero bytes, which no identifier holds, so two
/// identifiers have one key only when they are equal, and no key is the empty slot.
fn key_of(identifier: &[u8]) -> u128 {
  let mut bytes = [0_u8; KEY_LENGTH];
  for (target, byte) in bytes.iter_mut().zip(identifier) {
    *target = *byte;
  }
  u128::from_be_bytes(bytes)
}
