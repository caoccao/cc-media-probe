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

//! The blocking sources of the hand-written probe: bytes in place, and a seekable reader through a block cache.

use std::io::{self, Read, Seek, SeekFrom};

use crate::budget::{Budget, Reclaim};
use crate::source::{Blocks, Delivery, Failure, Filled, Holdings, Step, known_before, read_memory, settle};
use crate::utilities::widen;

/// A blocking positional source.
pub(crate) trait BlockingSource: Holdings {
  /// Reads at most `buffer.len()` bytes at `position`, clamped to the read allowance.
  fn read_at(&mut self, budget: &mut Budget, position: u64, buffer: &mut [u8]) -> io::Result<Delivery>;

  /// The bytes that the input delivered.
  fn delivered(&self) -> u64;

  /// Releases the source's buffers.
  fn release(&mut self, budget: &mut Budget);
}

/// Bytes in memory.
pub(crate) struct HandMemory<'bytes> {
  bytes: &'bytes [u8],
  delivered: u64,
}

impl HandMemory<'_> {
  /// The source over `bytes`.
  pub(crate) fn new(bytes: &[u8]) -> HandMemory<'_> {
    HandMemory { bytes, delivered: 0 }
  }
}

impl BlockingSource for HandMemory<'_> {
  fn read_at(&mut self, budget: &mut Budget, position: u64, buffer: &mut [u8]) -> io::Result<Delivery> {
    Ok(read_memory(self.bytes, &mut self.delivered, budget, position, buffer))
  }

  fn delivered(&self) -> u64 {
    self.delivered
  }

  fn release(&mut self, _budget: &mut Budget) {}
}

impl Holdings for HandMemory<'_> {
  fn delivered_before(&self, position: u64) -> bool {
    known_before(self.bytes, position)
  }
}

impl Reclaim for HandMemory<'_> {
  fn reclaim(&mut self, _budget: &mut Budget) -> bool {
    false
  }
}

/// A seekable reader through a block cache.
pub(crate) struct HandCache<Reader> {
  reader: Reader,
  blocks: Blocks,
}

impl<Reader> HandCache<Reader> {
  /// The cache over `reader`.
  pub(crate) fn new(reader: Reader, block_size: usize, capacity: usize) -> HandCache<Reader> {
    HandCache {
      reader,
      blocks: Blocks::new(block_size, capacity),
    }
  }
}

impl<Reader> Holdings for HandCache<Reader> {
  fn delivered_before(&self, position: u64) -> bool {
    self.blocks.delivered_before(position)
  }
}

impl<Reader> Reclaim for HandCache<Reader> {
  fn reclaim(&mut self, budget: &mut Budget) -> bool {
    self.blocks.reclaim(budget)
  }
}

impl<Reader: Read + Seek> BlockingSource for HandCache<Reader> {
  fn read_at(&mut self, budget: &mut Budget, position: u64, buffer: &mut [u8]) -> io::Result<Delivery> {
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
          let result = read_retrying(&mut self.reader, at, target);
          match result {
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

  fn delivered(&self) -> u64 {
    self.blocks.delivered()
  }

  fn release(&mut self, budget: &mut Budget) {
    self.blocks.release(budget);
  }
}

/// A seek and a read, with `ErrorKind::Interrupted` retried.
fn read_retrying<Reader: Read + Seek>(reader: &mut Reader, position: u64, buffer: &mut [u8]) -> io::Result<usize> {
  loop {
    let result = reader.seek(SeekFrom::Start(position)).and_then(|_| reader.read(buffer));
    match result {
      Err(error) if error.kind() == io::ErrorKind::Interrupted => {}
      result => return result,
    }
  }
}

/// Fills `buffer` from `position` on, asking again after each short read. A read that fails gives the bytes that
/// arrived before it with the error.
pub(super) fn fill<Source: BlockingSource>(
  source: &mut Source,
  budget: &mut Budget,
  position: u64,
  buffer: &mut [u8],
) -> Result<Filled, Failure> {
  let mut done = 0_usize;
  while let Some(rest) = buffer.get_mut(done..).filter(|rest| !rest.is_empty()) {
    let at = position.saturating_add(widen(done));
    let delivery = source
      .read_at(budget, at, rest)
      .map_err(|cause| Failure { done, cause })?;
    if let Some(filled) = settle(delivery, &mut done) {
      return Ok(filled);
    }
  }
  Ok(Filled::Complete)
}
