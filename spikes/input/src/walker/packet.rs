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

//! `Packet`, the plain code of offering one packet to its stream piece by piece: whether it is offered at all,
//! where the next piece lies, and what the read of a piece gives. The async walker and the hand-written one make
//! only the reads, so the rest of a packet is never read on a seekable input once its stream wants no more.

use super::Walk;
use crate::assembler::Piece;
use crate::budget::Reclaim;
use crate::report::DiagnosticKind;
use crate::source::Filled;
use crate::toy::Element;
use crate::toy::constants::STREAM_NUMBER_LENGTH;
use crate::utilities::{narrow, widen};

/// A packet being offered to its stream.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Packet {
  number: u8,
  payload_start: u64,
  payload_length: u64,
  offset: u64,
}

impl Packet {
  /// The packet of `element`, whose stream number was read into `number` as `filled`; or, where nothing is to be
  /// offered, whether the traversal goes on: a stream that no declaration names gives a diagnostic, and a stream
  /// that waits for nothing more is passed over.
  pub(crate) fn start(
    walk: &mut Walk<'_>,
    identifier: u64,
    element: &Element,
    filled: &Filled,
    number: [u8; STREAM_NUMBER_LENGTH],
  ) -> Result<Packet, bool> {
    if *filled != Filled::Complete {
      return Err(walk.interrupted(identifier, element.position, filled));
    }
    let [number] = number;
    match walk.waiting(identifier, number) {
      None => {
        walk.diagnose(identifier, element.position, DiagnosticKind::UndeclaredStream(number));
        Err(true)
      }
      Some(false) => Err(true),
      Some(true) => {
        let payload_start = element.body_start.saturating_add(widen(STREAM_NUMBER_LENGTH));
        Ok(Packet {
          number,
          payload_start,
          payload_length: element.body_end.saturating_sub(payload_start),
          offset: 0,
        })
      }
    }
  }

  /// The position and the length of the next piece, within a buffer of `capacity` bytes.
  pub(crate) fn next_piece(&self, capacity: usize) -> (u64, usize) {
    let length = narrow(self.payload_length.saturating_sub(self.offset)).min(capacity);
    (self.payload_start.saturating_add(self.offset), length)
  }

  /// Offers what the read of the next piece, `length` bytes long, gave, and once the packet is done, says whether
  /// the traversal goes on: not where the input ended inside the packet or a limit stopped the read.
  pub(crate) fn offer(
    &mut self,
    walk: &mut Walk<'_>,
    identifier: u64,
    piece: &[u8],
    length: usize,
    filled: &Filled,
    room: &mut dyn Reclaim,
  ) -> Option<bool> {
    let count = match filled {
      Filled::Complete => length,
      Filled::End(count) | Filled::Exhausted(count, _) => *count,
      Filled::Unavailable => 0,
    };
    let ends_packet = self.offset.saturating_add(widen(count)) == self.payload_length;
    let wants = self.deliver(
      walk,
      identifier,
      piece,
      count,
      ends_packet && *filled == Filled::Complete,
      room,
    );
    if *filled != Filled::Complete {
      return Some(walk.interrupted(identifier, self.payload_start, filled));
    }
    (!wants || ends_packet).then_some(true)
  }

  /// Offers the `count` bytes that a read of the next piece delivered before it failed, as a piece that ends
  /// nothing, so the stream keeps what they establish before the error ends the probe (R19).
  pub(crate) fn offer_delivered(
    &mut self,
    walk: &mut Walk<'_>,
    identifier: u64,
    piece: &[u8],
    count: usize,
    room: &mut dyn Reclaim,
  ) {
    if count > 0 {
      self.deliver(walk, identifier, piece, count, false, room);
    }
  }

  /// Offers the first `count` bytes of `piece` as the next part of the packet, and says whether the stream wants
  /// more of it.
  fn deliver(
    &mut self,
    walk: &mut Walk<'_>,
    identifier: u64,
    piece: &[u8],
    count: usize,
    ends_packet: bool,
    room: &mut dyn Reclaim,
  ) -> bool {
    let offered = Piece {
      bytes: piece.get(..count).unwrap_or_default(),
      starts_packet: self.offset == 0,
      ends_packet,
      packet_length: Some(self.payload_length),
    };
    let wants = walk.offer(identifier, self.number, &offered, room);
    self.offset = self.offset.saturating_add(widen(count));
    wants
  }
}
