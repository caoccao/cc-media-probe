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

//! `BlockCache`, the seekable path: positional reads of a raw input through a cache of aligned blocks (plan section
//! 8.7). Each raw read is clamped to the allowance, and a pending one is repeated with the same range, since the
//! future holds it until it is ready (`api.md` section 9.3, "Repeated requests").

use std::io;

use super::blocks::{Blocks, Step};
use super::raw::read_positional;
use super::{Delivery, Holdings, InternalSource, RawPositional};
use crate::budget::{Budget, Reclaim};

/// A seekable input read through a block cache.
pub(crate) struct BlockCache<Raw> {
  raw: Raw,
  blocks: Blocks,
}

impl<Raw> BlockCache<Raw> {
  /// The cache over `raw`, with `capacity` blocks of `block_size` bytes.
  pub(crate) fn new(raw: Raw, block_size: usize, capacity: usize) -> BlockCache<Raw> {
    BlockCache {
      raw,
      blocks: Blocks::new(block_size, capacity),
    }
  }
}

impl<Raw> Holdings for BlockCache<Raw> {
  fn delivered_before(&self, position: u64) -> bool {
    self.blocks.delivered_before(position)
  }
}

impl<Raw> Reclaim for BlockCache<Raw> {
  fn reclaim(&mut self, budget: &mut Budget) -> bool {
    self.blocks.reclaim(budget)
  }
}

impl<Raw: RawPositional> InternalSource for BlockCache<Raw> {
  async fn read_at(&mut self, budget: &mut Budget, position: u64, buffer: &mut [u8]) -> io::Result<Delivery> {
    loop {
      match self.blocks.step(budget, position, buffer) {
        Step::Served(count) => return Ok(Delivery::Bytes(count)),
        Step::End => return Ok(Delivery::Bytes(0)),
        Step::Exhausted(limit) => return Ok(Delivery::Exhausted(limit)),
        Step::Read {
          index,
          position: at,
          length,
        } => {
          let Some(target) = self.blocks.target(index, length) else {
            return Ok(Delivery::Bytes(0));
          };
          match read_positional(&mut self.raw, at, target).await {
            Ok(count) => self.blocks.complete(budget, index, at, length, count),
            Err(error) => {
              self.blocks.abandon(index, length);
              return Err(error);
            }
          }
        }
      }
    }
  }

  fn read_now(&mut self, budget: &mut Budget, position: u64, buffer: &mut [u8]) -> Option<Delivery> {
    match self.blocks.step(budget, position, buffer) {
      Step::Served(count) => Some(Delivery::Bytes(count)),
      Step::End => Some(Delivery::Bytes(0)),
      Step::Exhausted(limit) => Some(Delivery::Exhausted(limit)),
      Step::Read { .. } => None,
    }
  }

  fn end_detection(&mut self) {}

  fn delivered(&self) -> u64 {
    self.blocks.delivered()
  }

  fn release(&mut self, budget: &mut Budget) {
    self.blocks.release(budget);
  }
}
