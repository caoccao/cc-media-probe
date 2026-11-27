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

//! `Phase`, where the state machine stands, and `Fill`, the range that its pending read fills.

use crate::toy::Element;
use crate::walker::Packet;

/// What the pending fill is for.
#[derive(Clone, Copy, Debug)]
pub(super) enum Phase {
  /// The file header, at position 0.
  FileHeader,
  /// The header of the first element, at the offset that the file header gives.
  FirstElement(u64),
  /// The header of the element at this position.
  ElementHeader(u64),
  /// The short body of the element, this many bytes of it.
  Body(Element, usize),
  /// The stream number at the start of the element's packet.
  Number(Element),
  /// A piece of the element's packet, this many bytes long.
  Piece(Element, Packet, usize),
  /// The offset that the title reference holds.
  ReferenceOffset(Element),
  /// The header of the element that the reference names at this offset.
  ReferenceHeader(Element, u64),
  /// The text of the title that the reference names, this many bytes long.
  ReferenceText(Element, usize),
  /// The byte just before the position where the input gave nothing, which tells a clean end from a jump past it.
  LastByte,
  /// The walk is over.
  Finished,
}

/// The range that the pending fill reads, and how much of it has arrived.
#[derive(Clone, Copy, Debug)]
pub(super) struct Fill {
  /// Where the range starts.
  pub(super) position: u64,
  /// The length of the range.
  pub(super) length: usize,
  /// The bytes of it that have arrived.
  pub(super) done: usize,
}

/// What the machine does next: the phase that it enters and the range that the phase fills.
#[derive(Clone, Copy, Debug)]
pub(super) struct Next {
  /// The phase.
  pub(super) phase: Phase,
  /// Where its range starts.
  pub(super) position: u64,
  /// The length of its range.
  pub(super) length: usize,
}
