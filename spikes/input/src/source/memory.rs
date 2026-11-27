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

//! `Memory`, the bytes of `probe_bytes`, read in place without a cache.

use std::future::{Future, ready};
use std::io;

use cc_media_probe::Limit;

use super::{Delivery, Holdings, InternalSource};
use crate::budget::{Budget, Reclaim};
use crate::utilities::{narrow, widen};

/// Bytes in memory, read in place. Their length is known, so the end needs no read.
pub(crate) struct Memory<'bytes> {
  bytes: &'bytes [u8],
  delivered: u64,
}

impl Memory<'_> {
  /// The source over `bytes`.
  pub(crate) fn new(bytes: &[u8]) -> Memory<'_> {
    Memory { bytes, delivered: 0 }
  }
}

impl InternalSource for Memory<'_> {
  fn read_at(
    &mut self,
    budget: &mut Budget,
    position: u64,
    buffer: &mut [u8],
  ) -> impl Future<Output = io::Result<Delivery>> {
    ready(Ok(read_memory(
      self.bytes,
      &mut self.delivered,
      budget,
      position,
      buffer,
    )))
  }

  fn read_now(&mut self, budget: &mut Budget, position: u64, buffer: &mut [u8]) -> Option<Delivery> {
    Some(read_memory(self.bytes, &mut self.delivered, budget, position, buffer))
  }

  fn end_detection(&mut self) {}

  fn delivered(&self) -> u64 {
    self.delivered
  }

  fn release(&mut self, _budget: &mut Budget) {}
}

impl Holdings for Memory<'_> {
  fn delivered_before(&self, position: u64) -> bool {
    known_before(self.bytes, position)
  }
}

impl Reclaim for Memory<'_> {
  fn reclaim(&mut self, _budget: &mut Budget) -> bool {
    false
  }
}

/// Whether `bytes` hold the byte just before `position`: their length is known, so the end needs no read.
pub(crate) fn known_before(bytes: &[u8], position: u64) -> bool {
  position <= widen(bytes.len())
}

/// Copies what the allowance allows of the bytes at `position`, which the hand-written version shares.
pub(crate) fn read_memory(
  bytes: &[u8],
  delivered: &mut u64,
  budget: &mut Budget,
  position: u64,
  buffer: &mut [u8],
) -> Delivery {
  let start = narrow(position).min(bytes.len());
  let available = bytes.get(start..).unwrap_or_default();
  let wanted = buffer.len().min(available.len());
  if wanted == 0 {
    return Delivery::Bytes(0);
  }
  let count = budget.clamp_read(wanted);
  let (Some(target), Some(origin)) = (buffer.get_mut(..count), available.get(..count)) else {
    return Delivery::Exhausted(Limit::BytesRead);
  };
  if count == 0 {
    return Delivery::Exhausted(Limit::BytesRead);
  }
  target.copy_from_slice(origin);
  budget.charge_read(count);
  *delivered = delivered.saturating_add(widen(count));
  Delivery::Bytes(count)
}
