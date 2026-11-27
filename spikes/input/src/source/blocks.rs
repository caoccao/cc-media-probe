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

//! `Blocks`, the bookkeeping of the block cache: which aligned blocks it holds, what a read must fill, and the end
//! that a read observed. It issues no read itself, so the async cache and the hand-written one share it.

use cc_media_probe::Limit;

use crate::budget::Budget;
use crate::utilities::{narrow, widen};

/// The blocks of a seekable input that the cache holds, each aligned to the block size and charged once.
pub(crate) struct Blocks {
  block_size: usize,
  capacity: usize,
  held: Vec<Block>,
  clock: u64,
  end: Option<u64>,
  delivered: u64,
}

/// One block: its start, the bytes read of it so far, and when it was last used.
struct Block {
  start: u64,
  data: Vec<u8>,
  last_use: u64,
}

/// What a request needs.
pub(crate) enum Step {
  /// This many bytes were copied from a block.
  Served(usize),
  /// The input has ended at or before the position.
  End,
  /// A limit keeps the block from being read or held.
  Exhausted(Limit),
  /// Read `length` bytes at `position` into the spare part of block `index`, then ask again.
  Read { index: usize, position: u64, length: usize },
}

impl Blocks {
  /// A cache of `capacity` blocks of `block_size` bytes, none of them held yet.
  pub(crate) fn new(block_size: usize, capacity: usize) -> Blocks {
    Blocks {
      block_size,
      capacity,
      held: Vec::new(),
      clock: 0,
      end: None,
      delivered: 0,
    }
  }

  /// Serves what a block holds at `position`, or says which read fills it. The read is clamped to the allowance.
  pub(crate) fn step(&mut self, budget: &mut Budget, position: u64, buffer: &mut [u8]) -> Step {
    if buffer.is_empty() || self.end.is_some_and(|end| position >= end) {
      return Step::End;
    }
    self.clock = self.clock.wrapping_add(1);
    let size = widen(self.block_size);
    let start = position.saturating_sub(position.checked_rem(size).unwrap_or(0));
    let index = match self.held.iter().position(|block| block.start == start) {
      Some(index) => index,
      None => match self.slot(budget, start) {
        Ok(index) => index,
        Err(limit) => return Step::Exhausted(limit),
      },
    };
    let Some(block) = self.held.get_mut(index) else {
      return Step::End;
    };
    block.last_use = self.clock;
    let filled_end = block.start.saturating_add(widen(block.data.len()));
    if position < filled_end {
      let held = block
        .data
        .get(narrow(position.saturating_sub(block.start))..)
        .unwrap_or_default();
      let count = held.len().min(buffer.len());
      if let (Some(target), Some(origin)) = (buffer.get_mut(..count), held.get(..count)) {
        target.copy_from_slice(origin);
      }
      return Step::Served(count);
    }
    if self.end.is_some_and(|end| filled_end >= end) {
      return Step::End;
    }
    let length = budget.clamp_read(self.block_size.saturating_sub(block.data.len()));
    if length == 0 {
      return Step::Exhausted(Limit::BytesRead);
    }
    Step::Read {
      index,
      position: filled_end,
      length,
    }
  }

  /// The spare part of block `index` that a read of `length` bytes fills, within its reserved capacity.
  pub(crate) fn target(&mut self, index: usize, length: usize) -> Option<&mut [u8]> {
    let block = self.held.get_mut(index)?;
    let old = block.data.len();
    block.data.resize(old.saturating_add(length), 0);
    block.data.get_mut(old..)
  }

  /// Keeps the `count` bytes that a read of `length` bytes at `position` delivered. A read that delivered none
  /// observed the end at or before `position` (plan section 7.5, "The end of the input is observed").
  pub(crate) fn complete(&mut self, budget: &mut Budget, index: usize, position: u64, length: usize, count: usize) {
    let count = count.min(length);
    self.abandon(index, length.saturating_sub(count));
    budget.charge_read(count);
    self.delivered = self.delivered.saturating_add(widen(count));
    if count == 0 {
      self.end = Some(position);
    }
  }

  /// Drops the last `length` bytes of block `index`, which a read did not fill.
  pub(crate) fn abandon(&mut self, index: usize, length: usize) {
    if let Some(block) = self.held.get_mut(index) {
      let kept = block.data.len().saturating_sub(length);
      block.data.truncate(kept);
    }
  }

  /// The bytes that the input delivered.
  pub(crate) fn delivered(&self) -> u64 {
    self.delivered
  }

  /// Whether a held block has the byte just before `position`, so a read at `position` that gave nothing found
  /// the end exactly there (plan section 7.5, "The end of the input is observed").
  pub(crate) fn delivered_before(&self, position: u64) -> bool {
    let Some(last) = position.checked_sub(1) else {
      return true;
    };
    self
      .held
      .iter()
      .any(|block| block.start <= last && last < block.start.saturating_add(widen(block.data.len())))
  }

  /// Releases every block and returns its charge.
  pub(crate) fn release(&mut self, budget: &mut Budget) {
    for block in self.held.drain(..) {
      budget.free(block.data);
    }
  }

  /// Gives back every block but the one used last, which is all a cache of one block would hold, and says whether
  /// any was given back.
  pub(crate) fn reclaim(&mut self, budget: &mut Budget) -> bool {
    let newest = self
      .held
      .iter()
      .enumerate()
      .max_by_key(|(_, block)| block.last_use)
      .map(|(index, _)| index);
    let Some(newest) = newest.filter(|_| self.held.len() > 1) else {
      return false;
    };
    let kept = self.held.swap_remove(newest);
    self.release(budget);
    self.held.push(kept);
    true
  }

  /// A block for `start`: a new one while the cache has room and the working memory holds it, or else the one used
  /// least recently, so a larger cache never reads less than a smaller one under the same limit. The first block is
  /// required, so one that does not fit marks the working memory exhausted; a further block is optional, and one that
  /// does not fit marks nothing.
  fn slot(&mut self, budget: &mut Budget, start: u64) -> Result<usize, Limit> {
    if self.held.len() < self.capacity {
      let size = self.block_size;
      let allocated = if self.held.is_empty() {
        budget.require(None, |budget| budget.try_allocate(size))
      } else {
        budget.try_allocate(size)
      };
      match allocated {
        Ok(data) => {
          self.held.push(Block {
            start,
            data,
            last_use: self.clock,
          });
          return Ok(self.held.len().saturating_sub(1));
        }
        Err(limit) if self.held.is_empty() => return Err(limit),
        Err(_) => {}
      }
    }
    let index = self
      .held
      .iter()
      .enumerate()
      .min_by_key(|(_, block)| block.last_use)
      .map(|(index, _)| index)
      .ok_or(Limit::WorkingMemory)?;
    if let Some(block) = self.held.get_mut(index) {
      block.start = start;
      block.data.clear();
    }
    Ok(index)
  }
}
