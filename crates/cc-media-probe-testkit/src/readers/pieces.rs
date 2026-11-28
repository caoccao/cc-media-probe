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
//! (`docs/design/test-kit.md` section 2.3).

/// The sizes of the pieces, in order from byte 0, the last repeating; without sizes, a read is not cut.
///
/// ```
/// use cc_media_probe_testkit::readers::Pieces;
///
/// let pieces = Pieces::new(&[40, 8]);
/// assert_eq!(pieces.room(0), Some(40));
/// assert_eq!(pieces.room(44), Some(4));
/// assert_eq!(pieces.room(50), Some(6));
/// let mut buffer = [0; 64];
/// assert_eq!(pieces.cut(44, &mut buffer).len(), 4);
/// assert_eq!(Pieces::default().room(50), None);
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Pieces {
  /// The sizes, each at least 1.
  sizes: Vec<u64>,
}

impl Pieces {
  /// The pieces of `sizes`, each taken as at least 1.
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
    shortened(buffer, room)
  }
}

/// The first `room` bytes of `buffer`, or all of it when it is no longer.
pub(crate) fn shortened(buffer: &mut [u8], room: u64) -> &mut [u8] {
  let length = usize::try_from(room).map_or(buffer.len(), |room| buffer.len().min(room));
  buffer.get_mut(..length).unwrap_or_default()
}
