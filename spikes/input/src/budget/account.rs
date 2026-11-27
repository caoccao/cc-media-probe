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

//! `Budget`, the one account that every source, assembler and companion of a probe charges.

use std::mem::{self, size_of};

use cc_media_probe::{Limit, Limits};

use super::constants::{
  BYTES_READ_FLAG, HEADER_BYTES_FLAG, LIST_GROWTH_FACTOR, MINIMUM_LIST_CAPACITY, WORK_UNITS_FLAG, WORKING_MEMORY_FLAG,
};
use super::{Reclaim, Usage};
use crate::utilities::{narrow, widen};

/// The budget of one probe (D16). Every limit is exact: a read is clamped to the remaining allowance before it is
/// issued, and a buffer is charged before it is reserved, with its real capacity read back afterwards.
///
/// The charges of working memory are attempts that mark nothing: a cache block beyond the first is optional, and
/// one that does not fit stops nothing. A buffer that the probe requires is charged through `require`, which marks
/// the working memory exhausted only when the buffer still does not fit after the optional blocks were given back.
#[derive(Debug)]
pub(crate) struct Budget {
  limits: Limits,
  bytes_read: u64,
  working_memory: u64,
  peak_working_memory: u64,
  work_units: u64,
  /// Which counters ran out, one flag each: a fixed-size record, as plan section 7.5 asks.
  exhausted: u8,
}

impl Budget {
  /// A budget with nothing charged.
  pub(crate) fn new(limits: &Limits) -> Budget {
    Budget {
      limits: limits.clone(),
      bytes_read: 0,
      working_memory: 0,
      peak_working_memory: 0,
      work_units: 0,
      exhausted: 0,
    }
  }

  /// The limits of the probe.
  pub(crate) fn limits(&self) -> &Limits {
    &self.limits
  }

  /// Clamps a read of `wanted` bytes to the remaining allowance. A result of 0 for a nonempty request marks the
  /// allowance exhausted; the caller then issues no read.
  pub(crate) fn clamp_read(&mut self, wanted: usize) -> usize {
    let remaining = self.limits.bytes_read().saturating_sub(self.bytes_read);
    let clamped = narrow(remaining).min(wanted);
    if clamped == 0 && wanted > 0 {
      self.exhausted |= BYTES_READ_FLAG;
    }
    clamped
  }

  /// Charges the bytes that a read delivered, which its clamp allowed.
  pub(crate) fn charge_read(&mut self, delivered: usize) {
    self.bytes_read = self.bytes_read.saturating_add(widen(delivered));
  }

  /// Charges working memory, or names the limit when it would not fit, without marking it.
  pub(crate) fn try_charge_memory(&mut self, bytes: u64) -> Result<(), Limit> {
    let limit = self.limits.working_memory();
    let total = self
      .working_memory
      .checked_add(bytes)
      .filter(|total| *total <= limit)
      .ok_or(Limit::WorkingMemory)?;
    self.working_memory = total;
    self.peak_working_memory = self.peak_working_memory.max(total);
    Ok(())
  }

  /// Runs `charge` for a buffer that the probe requires. When working memory refuses it and `room` gives back
  /// optional memory, the charge runs once more; a charge that still fails, or that the allocator refused, marks
  /// the working memory exhausted (plan section 7.5).
  pub(crate) fn require<Value>(
    &mut self,
    room: Option<&mut dyn Reclaim>,
    mut charge: impl FnMut(&mut Budget) -> Result<Value, Limit>,
  ) -> Result<Value, Limit> {
    let mut result = charge(self);
    if matches!(result, Err(Limit::WorkingMemory))
      && let Some(room) = room
      && room.reclaim(self)
    {
      result = charge(self);
    }
    if matches!(result, Err(Limit::WorkingMemory)) {
      self.exhausted |= WORKING_MEMORY_FLAG;
    }
    result
  }

  /// Returns a charge of working memory.
  pub(crate) fn release_memory(&mut self, bytes: u64) {
    self.working_memory = self.working_memory.saturating_sub(bytes);
  }

  /// Charges `wanted` bytes, lets `reserve` reserve them and give back the real capacity in bytes, and corrects the
  /// charge to it. A reservation that fails, or a real capacity that the limit cannot hold, returns the whole charge.
  pub(crate) fn try_charge_capacity(
    &mut self,
    wanted: usize,
    reserve: impl FnOnce() -> Option<usize>,
  ) -> Result<u64, Limit> {
    let charged = widen(wanted);
    self.try_charge_memory(charged)?;
    let Some(capacity) = reserve() else {
      self.release_memory(charged);
      return Err(Limit::WorkingMemory);
    };
    let real = widen(capacity);
    let extra = real.saturating_sub(charged);
    if extra > 0 && self.try_charge_memory(extra).is_err() {
      self.release_memory(charged);
      return Err(Limit::WorkingMemory);
    }
    Ok(real.max(charged))
  }

  /// A buffer of `capacity` items, charged by their bytes before it is reserved (plan section 7.5, "Charge before
  /// reserving").
  pub(crate) fn try_allocate<Item>(&mut self, capacity: usize) -> Result<Vec<Item>, Limit> {
    let mut items = Vec::new();
    self.try_charge_capacity(capacity.saturating_mul(size_of::<Item>()), || {
      items.try_reserve_exact(capacity).ok()?;
      Some(items.capacity().saturating_mul(size_of::<Item>()))
    })?;
    Ok(items)
  }

  /// Releases a buffer and returns its charge, the capacity of its items.
  pub(crate) fn free<Item>(&mut self, items: Vec<Item>) {
    self.release_memory(capacity_bytes(&items));
    drop(items);
  }

  /// Grows a contiguous buffer to at least `capacity` bytes: the new allocation is charged while the old one is
  /// still charged, and the old charge is returned after the move, so the peak of the two is seen (plan section
  /// 7.5, "Growth is bounded by design").
  pub(crate) fn try_grow(&mut self, buffer: &mut Vec<u8>, capacity: usize) -> Result<(), Limit> {
    if buffer.capacity() >= capacity {
      return Ok(());
    }
    let mut grown = self.try_allocate(capacity)?;
    grown.extend_from_slice(buffer);
    let old = mem::replace(buffer, grown);
    self.free(old);
    Ok(())
  }

  /// Makes room for one more item in `items`: a full list doubles its capacity, charged while the old capacity is
  /// still charged, as `try_grow` charges, so a list that grows with the input stays under working memory.
  pub(crate) fn try_reserve_one<Item>(&mut self, items: &mut Vec<Item>) -> Result<(), Limit> {
    if items.len() < items.capacity() {
      return Ok(());
    }
    let old = capacity_bytes(items);
    let wanted = items
      .capacity()
      .saturating_mul(LIST_GROWTH_FACTOR)
      .max(MINIMUM_LIST_CAPACITY);
    let additional = wanted.saturating_sub(items.len());
    self.try_charge_capacity(wanted.saturating_mul(size_of::<Item>()), || {
      items.try_reserve_exact(additional).ok()?;
      Some(items.capacity().saturating_mul(size_of::<Item>()))
    })?;
    self.release_memory(old);
    Ok(())
  }

  /// Charges work units, or names the limit when they would not fit.
  pub(crate) fn charge_work(&mut self, units: u64) -> Result<(), Limit> {
    if let Some(total) = self
      .work_units
      .checked_add(units)
      .filter(|total| *total <= self.limits.work_units())
    {
      self.work_units = total;
      Ok(())
    } else {
      self.exhausted |= WORK_UNITS_FLAG;
      Err(Limit::WorkUnits)
    }
  }

  /// The work units left.
  pub(crate) fn remaining_work(&self) -> u64 {
    self.limits.work_units().saturating_sub(self.work_units)
  }

  /// Records that the work units ran out: a parser needed to examine more bytes than were left.
  pub(crate) fn exhaust_work(&mut self) {
    self.exhausted |= WORK_UNITS_FLAG;
  }

  /// Records that a stream's header needed more than the per-stream header limit.
  pub(crate) fn exhaust_header_bytes(&mut self) {
    self.exhausted |= HEADER_BYTES_FLAG;
  }

  /// The usage so far.
  pub(crate) fn usage(&self) -> Usage {
    Usage {
      bytes_read: self.bytes_read,
      peak_working_memory: self.peak_working_memory,
      working_memory_at_end: self.working_memory,
      work_units: self.work_units,
    }
  }

  /// The counters that ran out, in the order of the record.
  pub(crate) fn exhausted(&self) -> Vec<Limit> {
    [
      (BYTES_READ_FLAG, Limit::BytesRead),
      (WORKING_MEMORY_FLAG, Limit::WorkingMemory),
      (HEADER_BYTES_FLAG, Limit::HeaderBytesPerStream),
      (WORK_UNITS_FLAG, Limit::WorkUnits),
    ]
    .into_iter()
    .filter_map(|(flag, limit)| (self.exhausted & flag != 0).then_some(limit))
    .collect()
  }
}

/// The bytes that the capacity of `items` holds, which their charge counts.
fn capacity_bytes<Item>(items: &Vec<Item>) -> u64 {
  widen(items.capacity().saturating_mul(size_of::<Item>()))
}
