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

//! Short reads on purpose: the input cut into pieces at fixed bytes, the sizes of a case's `pieces=40,8` from byte
//! 0, the last size repeating, so that no read delivers bytes of two pieces. `pieces=40,8` ends reads at byte 40,
//! 48, 56 and so on, whatever sizes the reads ask for, since a cut that counted read calls would move with the
//! library's requests. The library may join short reads before its parsers see them, so pieces test the joining,
//! and a header split across deliveries is a matter of the fixture's own bytes, such as a stream's packets
//! (`docs/design/test-kit.md` section 2.3). `FaultingReader` applies them to any standard-library reader; the
//! permanent kit adds its other faults (section 5), and a custom source in the library's tests applies `Pieces` to
//! its own reads at their positions.

use std::io::{self, Read, Seek, SeekFrom};

/// The sizes of the pieces, in order from byte 0, the last repeating; without sizes, a read is not cut.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Pieces {
  /// The sizes, each at least 1.
  sizes: Vec<u64>,
}

impl Pieces {
  /// The pieces of `sizes`, each at least 1.
  #[must_use]
  pub fn new(sizes: &[usize]) -> Pieces {
    Pieces {
      sizes: sizes
        .iter()
        .map(|size| u64::try_from(*size).unwrap_or(u64::MAX).max(1))
        .collect(),
    }
  }

  /// The bytes from `position` to the end of its piece, or `None` where reads are not cut.
  #[must_use]
  pub fn room(&self, position: u64) -> Option<u64> {
    let mut end = 0_u64;
    for size in &self.sizes {
      end = end.saturating_add(*size);
      if let Some(room) = end.checked_sub(position).filter(|room| *room > 0) {
        return Some(room);
      }
    }
    let last = *self.sizes.last()?;
    let into = position.saturating_sub(end).checked_rem(last)?;
    last.checked_sub(into)
  }

  /// The part of `buffer` that a read at `position` may fill.
  pub fn cut<'buffer>(&self, position: u64, buffer: &'buffer mut [u8]) -> &'buffer mut [u8] {
    let Some(room) = self.room(position) else {
      return buffer;
    };
    let length = usize::try_from(room).map_or(buffer.len(), |room| buffer.len().min(room));
    buffer.get_mut(..length).unwrap_or_default()
  }
}

/// A reader that delivers in pieces, around any standard-library reader.
#[derive(Debug)]
pub struct FaultingReader<Inner> {
  /// The reader cut.
  inner: Inner,
  /// The pieces of its reads.
  pieces: Pieces,
  /// The position of the next read: the bytes delivered so far, or where the last seek went.
  position: u64,
}

impl<Inner> FaultingReader<Inner> {
  /// A reader of `inner`, from its byte 0, that does not cut its reads yet.
  pub fn new(inner: Inner) -> FaultingReader<Inner> {
    FaultingReader {
      inner,
      pieces: Pieces::default(),
      position: 0,
    }
  }

  /// Cuts the input into pieces of `sizes`, in order from byte 0, the last repeating.
  #[must_use]
  pub fn with_pieces(mut self, sizes: &[usize]) -> FaultingReader<Inner> {
    self.pieces = Pieces::new(sizes);
    self
  }
}

impl<Inner: Read> Read for FaultingReader<Inner> {
  fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
    let piece = self.pieces.cut(self.position, buffer);
    let delivered = self.inner.read(piece)?;
    self.position = self
      .position
      .saturating_add(u64::try_from(delivered).unwrap_or(u64::MAX));
    Ok(delivered)
  }
}

impl<Inner: Seek> Seek for FaultingReader<Inner> {
  fn seek(&mut self, position: SeekFrom) -> io::Result<u64> {
    self.position = self.inner.seek(position)?;
    Ok(self.position)
  }
}
