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

//! `Window`, the forward-only path of D9: a bounded sliding window over a sequential input, with the bytes that
//! detection read pinned for the whole probe (plan section 8.6). A read behind the window and outside the prefix
//! gives `Unavailable` for that range only; a read ahead of it reads forward and discards what falls out of the
//! window.

use std::collections::VecDeque;
use std::io;
use std::mem;

use cc_media_probe::Limit;

use super::raw::read_sequential;
use super::{Delivery, Holdings, InternalSource, RawSequential};
use crate::budget::{Budget, Reclaim};
use crate::utilities::{narrow, widen};

/// The sizes of a window: the pinned prefix's capacity, the window, and the chunk of one read.
#[derive(Clone, Copy, Debug)]
pub(crate) struct WindowSizes {
  /// The most bytes that detection may read, all of which stay pinned.
  pub(crate) prefix: usize,
  /// The bytes kept behind the read position.
  pub(crate) window: usize,
  /// The bytes of one read of the input.
  pub(crate) chunk: usize,
}

/// A forward-only input read through a sliding window.
pub(crate) struct Window<Raw> {
  raw: Raw,
  sizes: WindowSizes,
  detecting: bool,
  ended: bool,
  /// The bytes from position 0 that detection read, pinned until the source is released.
  prefix: Option<Vec<u8>>,
  /// The bytes from `ring_start` up to `next`.
  ring: VecDeque<u8>,
  ring_start: u64,
  /// The charge of the ring and the chunk, once reserved.
  ring_charge: Option<u64>,
  next: u64,
  chunk: Vec<u8>,
  delivered: u64,
}

impl<Raw> Window<Raw> {
  /// The window over `raw`, which starts by pinning what detection reads.
  pub(crate) fn new(raw: Raw, sizes: WindowSizes) -> Window<Raw> {
    Window {
      raw,
      sizes,
      detecting: true,
      ended: false,
      prefix: None,
      ring: VecDeque::new(),
      ring_start: 0,
      ring_charge: None,
      next: 0,
      chunk: Vec::new(),
      delivered: 0,
    }
  }

  /// Charges and reserves the ring and the chunk, each charged before it is reserved. The ring is kept only once
  /// both are reserved, so a chunk that does not fit frees the ring with its charge.
  fn allocate(&mut self, budget: &mut Budget) -> Result<(), Limit> {
    let capacity = self.sizes.window.saturating_add(self.sizes.chunk);
    let mut ring = VecDeque::new();
    let charge = budget.require(None, |budget| {
      budget.try_charge_capacity(capacity, || {
        ring.try_reserve_exact(capacity).ok()?;
        Some(ring.capacity())
      })
    })?;
    let chunk = self.sizes.chunk;
    match budget.require(None, |budget| budget.try_allocate(chunk)) {
      Ok(mut chunk) => {
        chunk.resize(self.sizes.chunk, 0);
        self.chunk = chunk;
        self.ring = ring;
        self.ring_charge = Some(charge);
        Ok(())
      }
      Err(limit) => {
        drop(ring);
        budget.release_memory(charge);
        Err(limit)
      }
    }
  }

  /// The length of the pinned prefix, 0 when detection read nothing.
  fn prefix_length(&self) -> u64 {
    widen(self.prefix.as_ref().map_or(0, Vec::len))
  }
}

impl<Raw: RawSequential> Window<Raw> {
  /// A read while detection runs: it must lie within the prefix's capacity, so every byte that detection read
  /// stays available to the walker. A sample beyond it is `Unavailable`, and nothing is read for it. When the
  /// allowance runs out, the bytes already read at `position` are delivered first, as from any other input.
  async fn read_detecting(&mut self, budget: &mut Budget, position: u64, buffer: &mut [u8]) -> io::Result<Delivery> {
    let end = position.saturating_add(widen(buffer.len()));
    if end > widen(self.sizes.prefix) {
      return Ok(Delivery::Unavailable);
    }
    let prefix_size = self.sizes.prefix;
    let prefix = match self.prefix.take() {
      Some(prefix) => prefix,
      None => match budget.require(None, |budget| budget.try_allocate(prefix_size)) {
        Ok(prefix) => prefix,
        Err(limit) => return Ok(Delivery::Exhausted(limit)),
      },
    };
    let prefix = self.prefix.insert(prefix);
    while widen(prefix.len()) < end && !self.ended {
      let length = budget.clamp_read(narrow(end.saturating_sub(widen(prefix.len()))));
      if length == 0 {
        if widen(prefix.len()) > position {
          break;
        }
        return Ok(Delivery::Exhausted(Limit::BytesRead));
      }
      let old = prefix.len();
      prefix.resize(old.saturating_add(length), 0);
      let Some(target) = prefix.get_mut(old..) else {
        return Ok(Delivery::Bytes(0));
      };
      let result = read_sequential(&mut self.raw, target).await;
      let count = result.as_ref().map_or(0, |count| (*count).min(length));
      prefix.truncate(old.saturating_add(count));
      result?;
      budget.charge_read(count);
      self.delivered = self.delivered.saturating_add(widen(count));
      self.ended = count == 0;
    }
    self.next = widen(prefix.len());
    Ok(Delivery::Bytes(copy_slice(prefix, position, buffer)))
  }

  /// Reads forward until the window holds `position`, or gives what ended the read: the end of the input or the read
  /// allowance. Before a chunk joins the ring, the oldest bytes make room so that the ring keeps the window, or the
  /// whole chunk where a chunk is larger than the window. Every byte dropped lies before the read position, which
  /// is at or before `position`, so the requested byte is never dropped, and the ring never outgrows its reserved
  /// capacity of a window and a chunk.
  async fn advance(&mut self, budget: &mut Budget, position: u64) -> io::Result<Option<Delivery>> {
    while self.next <= position {
      if self.ended {
        return Ok(Some(Delivery::Bytes(0)));
      }
      let length = budget.clamp_read(self.chunk.len());
      if length == 0 {
        return Ok(Some(Delivery::Exhausted(Limit::BytesRead)));
      }
      let Some(target) = self.chunk.get_mut(..length) else {
        return Ok(Some(Delivery::Bytes(0)));
      };
      let count = read_sequential(&mut self.raw, target).await?.min(length);
      budget.charge_read(count);
      self.delivered = self.delivered.saturating_add(widen(count));
      if count == 0 {
        self.ended = true;
        return Ok(Some(Delivery::Bytes(0)));
      }
      let excess = self
        .ring
        .len()
        .saturating_add(count)
        .saturating_sub(self.sizes.window)
        .min(self.ring.len());
      if excess > 0 {
        self.ring.drain(..excess);
        self.ring_start = self.ring_start.saturating_add(widen(excess));
      }
      self.ring.extend(self.chunk.get(..count).unwrap_or_default());
      self.next = self.next.saturating_add(widen(count));
    }
    Ok(None)
  }
}

impl<Raw> Holdings for Window<Raw> {
  /// Every byte that the input gave was delivered, in order from position 0.
  fn delivered_before(&self, position: u64) -> bool {
    position <= self.delivered
  }
}

impl<Raw> Reclaim for Window<Raw> {
  /// The window and the prefix are required, so nothing is optional.
  fn reclaim(&mut self, _budget: &mut Budget) -> bool {
    false
  }
}

impl<Raw: RawSequential> InternalSource for Window<Raw> {
  async fn read_at(&mut self, budget: &mut Budget, position: u64, buffer: &mut [u8]) -> io::Result<Delivery> {
    if buffer.is_empty() {
      return Ok(Delivery::Bytes(0));
    }
    if self.detecting {
      return self.read_detecting(budget, position, buffer).await;
    }
    if let Some(prefix) = self.prefix.as_ref().filter(|prefix| position < widen(prefix.len())) {
      return Ok(Delivery::Bytes(copy_slice(prefix, position, buffer)));
    }
    if position < self.ring_start {
      return Ok(Delivery::Unavailable);
    }
    if self.ring_charge.is_none()
      && let Err(limit) = self.allocate(budget)
    {
      return Ok(Delivery::Exhausted(limit));
    }
    if let Some(delivery) = self.advance(budget, position).await? {
      return Ok(delivery);
    }
    let Some(offset) = position.checked_sub(self.ring_start) else {
      return Ok(Delivery::Unavailable);
    };
    Ok(Delivery::Bytes(copy_ring(&self.ring, narrow(offset), buffer)))
  }

  fn read_now(&mut self, _budget: &mut Budget, position: u64, buffer: &mut [u8]) -> Option<Delivery> {
    if buffer.is_empty() {
      return Some(Delivery::Bytes(0));
    }
    if self.detecting {
      return None;
    }
    if let Some(prefix) = self.prefix.as_ref().filter(|prefix| position < widen(prefix.len())) {
      return Some(Delivery::Bytes(copy_slice(prefix, position, buffer)));
    }
    if position < self.ring_start {
      return Some(Delivery::Unavailable);
    }
    if position < self.next {
      let offset = narrow(position.saturating_sub(self.ring_start));
      return Some(Delivery::Bytes(copy_ring(&self.ring, offset, buffer)));
    }
    None
  }

  fn end_detection(&mut self) {
    self.detecting = false;
    self.next = self.prefix_length();
    self.ring_start = self.next;
  }

  fn delivered(&self) -> u64 {
    self.delivered
  }

  fn release(&mut self, budget: &mut Budget) {
    if let Some(prefix) = self.prefix.take() {
      budget.free(prefix);
    }
    if let Some(charge) = self.ring_charge.take() {
      budget.release_memory(charge);
      budget.free(mem::take(&mut self.chunk));
      self.ring = VecDeque::new();
    }
  }
}

/// Copies what `bytes` hold from `position` on into `buffer`, and returns the count.
fn copy_slice(bytes: &[u8], position: u64, buffer: &mut [u8]) -> usize {
  let held = bytes.get(narrow(position)..).unwrap_or_default();
  let count = held.len().min(buffer.len());
  if let (Some(target), Some(origin)) = (buffer.get_mut(..count), held.get(..count)) {
    target.copy_from_slice(origin);
  }
  count
}

/// Copies what the ring holds from `offset` on into `buffer`, from both of its halves, and returns the count.
fn copy_ring(ring: &VecDeque<u8>, offset: usize, buffer: &mut [u8]) -> usize {
  let (front, back) = ring.as_slices();
  let (first, second) = match front.get(offset..) {
    Some(rest) => (rest, back),
    None => (
      back.get(offset.saturating_sub(front.len())..).unwrap_or_default(),
      &[][..],
    ),
  };
  let from_first = copy_slice(first, 0, buffer);
  let from_second = buffer
    .get_mut(from_first..)
    .map_or(0, |rest| copy_slice(second, 0, rest));
  from_first.saturating_add(from_second)
}

#[cfg(test)]
mod tests {
  use std::io::Cursor;

  use cc_media_probe::{Limit, Limits};

  use super::{Window, WindowSizes};
  use crate::budget::Budget;
  use crate::executor::poll_once;
  use crate::source::{Delivery, Forward, InternalSource};

  #[test]
  fn a_chunk_that_does_not_fit_frees_the_ring_with_its_charge() {
    let sizes = WindowSizes {
      prefix: 16,
      window: 64,
      chunk: 32,
    };
    let mut budget = Budget::new(&Limits::default().with_working_memory(64 + 32 + 31));
    let mut window = Window::new(Forward(Cursor::new(vec![0_u8; 256])), sizes);
    window.end_detection();
    let mut buffer = [0_u8; 8];
    let delivery = poll_once(window.read_at(&mut budget, 0, &mut buffer)).unwrap().unwrap();
    assert_eq!(delivery, Delivery::Exhausted(Limit::WorkingMemory));
    assert_eq!(window.ring.capacity(), 0);
    assert_eq!(window.ring_charge, None);
    assert_eq!(budget.usage().working_memory_at_end, 0);
  }
}
