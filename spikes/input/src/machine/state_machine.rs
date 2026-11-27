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

//! `Machine`, the async walker's traversal of the primary input written as an explicit state machine: detection,
//! element headers in file order, short bodies, packets offered piece by piece, and title references followed
//! backward. Each phase holds what the walker would keep across a read; the decisions between reads are the same
//! plain code of `Walk`, `Packet` and the detection, so only the form of the walker differs. The driver lends the
//! machine its source as `Holdings` at each delivery, for the end of the input and the optional cache blocks.

use std::mem;

use super::phase::{Fill, Next, Phase};
use crate::detection::{Detected, judge_first_element, judge_header};
use crate::report::{DetectionOutcome, DiagnosticKind, Reason};
use crate::source::{Delivery, Filled, Holdings};
use crate::toy::Element;
use crate::toy::constants::{DOUBLE_WORD_LENGTH, ELEMENT_HEADER_LENGTH, FILE_HEADER_LENGTH, STREAM_NUMBER_LENGTH};
use crate::utilities::{narrow, widen};
use crate::walker::{Action, Cursor, Header, PRIMARY, Packet, Walk, title_target};

/// What the machine needs next.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Step {
  /// A read into `Machine::target`.
  Read,
  /// Nothing: the walk is over.
  Done,
}

/// The state machine of one probe's primary input, with the buffers that its reads fill.
pub(super) struct Machine {
  phase: Phase,
  fill: Fill,
  cursor: Cursor,
  file_header: [u8; FILE_HEADER_LENGTH],
  header: [u8; ELEMENT_HEADER_LENGTH],
  word: [u8; DOUBLE_WORD_LENGTH],
  number: [u8; STREAM_NUMBER_LENGTH],
  last_byte: [u8; 1],
  piece: Vec<u8>,
  body: Vec<u8>,
}

impl Machine {
  /// A machine whose first read is the file header.
  pub(super) fn new() -> Machine {
    Machine {
      phase: Phase::FileHeader,
      fill: Fill {
        position: 0,
        length: FILE_HEADER_LENGTH,
        done: 0,
      },
      cursor: Cursor::new(0),
      file_header: [0; FILE_HEADER_LENGTH],
      header: [0; ELEMENT_HEADER_LENGTH],
      word: [0; DOUBLE_WORD_LENGTH],
      number: [0; STREAM_NUMBER_LENGTH],
      last_byte: [0],
      piece: Vec::new(),
      body: Vec::new(),
    }
  }

  /// Where the pending read goes: its position, and the part of the range that has not arrived yet.
  pub(super) fn target(&mut self) -> (u64, &mut [u8]) {
    let Fill { position, length, done } = self.fill;
    let buffer: &mut [u8] = match self.phase {
      Phase::FileHeader => &mut self.file_header,
      Phase::FirstElement(_) | Phase::ElementHeader(_) | Phase::ReferenceHeader(..) => &mut self.header,
      Phase::Body(..) | Phase::ReferenceText(..) => &mut self.body,
      Phase::Number(_) => &mut self.number,
      Phase::LastByte => &mut self.last_byte,
      Phase::Piece(..) => &mut self.piece,
      Phase::ReferenceOffset(_) => &mut self.word,
      Phase::Finished => &mut [],
    };
    let rest = buffer
      .get_mut(..length)
      .and_then(|range| range.get_mut(done..))
      .unwrap_or_default();
    (position.saturating_add(widen(done)), rest)
  }

  /// Takes in what the pending read delivered. Once the range is complete, or the read ended it, the walker's plain
  /// code runs until the next range is needed or the walk is over.
  pub(super) fn deliver(&mut self, walk: &mut Walk<'_>, delivery: Delivery, holdings: &mut dyn Holdings) -> Step {
    let filled = match delivery {
      Delivery::Bytes(0) => Filled::End(self.fill.done),
      Delivery::Bytes(count) => {
        self.fill.done = self.fill.done.saturating_add(count);
        if self.fill.done < self.fill.length {
          return Step::Read;
        }
        Filled::Complete
      }
      Delivery::Unavailable => Filled::Unavailable,
      Delivery::Exhausted(limit) => Filled::Exhausted(self.fill.done, limit),
    };
    self.advance(walk, filled, holdings)
  }

  /// Takes in a read that failed: a piece of a packet offers the bytes that arrived before the error.
  pub(super) fn fail(&mut self, walk: &mut Walk<'_>, holdings: &mut dyn Holdings) {
    if let Phase::Piece(_, mut packet, _) = self.phase {
      packet.offer_delivered(walk, PRIMARY, &self.piece, self.fill.done, holdings);
    }
  }

  /// Gives the scratch buffers back to `walk`, which returns their charge with the report.
  pub(super) fn finish(&mut self, walk: &mut Walk<'_>) {
    if self.piece.capacity() > 0 {
      walk.piece = mem::take(&mut self.piece);
    }
    if self.body.capacity() > 0 {
      walk.body = mem::take(&mut self.body);
    }
  }

  /// Runs the phases from what a fill gave, until one needs bytes or the walk is over. A range of no bytes is
  /// complete at once, as a fill of an empty buffer is.
  fn advance(&mut self, walk: &mut Walk<'_>, mut filled: Filled, holdings: &mut dyn Holdings) -> Step {
    loop {
      let Some(next) = self.decide(walk, &filled, holdings) else {
        self.phase = Phase::Finished;
        return Step::Done;
      };
      self.phase = next.phase;
      self.fill = Fill {
        position: next.position,
        length: next.length,
        done: 0,
      };
      if next.length > 0 {
        return Step::Read;
      }
      filled = Filled::Complete;
    }
  }

  /// What the current phase makes of `filled`: the next phase and its range, or `None` where the walk ends.
  fn decide(&mut self, walk: &mut Walk<'_>, filled: &Filled, holdings: &mut dyn Holdings) -> Option<Next> {
    match self.phase {
      Phase::FileHeader => match judge_header(filled, &self.file_header) {
        Ok(first_element) => Some(Next {
          phase: Phase::FirstElement(first_element),
          position: first_element,
          length: ELEMENT_HEADER_LENGTH,
        }),
        Err(outcome) => {
          walk.detection = outcome;
          None
        }
      },
      Phase::FirstElement(first_element) => {
        self.identified(walk, judge_first_element(filled, self.header, first_element), holdings)
      }
      Phase::ElementHeader(position) => match walk.element(PRIMARY, position, filled, self.header, holdings) {
        Header::Element(element) => self.act(walk, element, holdings),
        Header::Ended => None,
        Header::Confirm(last) => Some(Next {
          phase: Phase::LastByte,
          position: last,
          length: self.last_byte.len(),
        }),
      },
      Phase::LastByte => {
        walk.confirm_end(&self.cursor, PRIMARY, filled);
        None
      }
      Phase::Body(element, length) => {
        let body = self.body.get(..length).unwrap_or_default();
        let goes_on = walk.take_body(PRIMARY, &element, filled, body, holdings);
        self.after(walk, element, goes_on)
      }
      Phase::Number(element) => match Packet::start(walk, PRIMARY, &element, filled, self.number) {
        Err(goes_on) => self.after(walk, element, goes_on),
        Ok(packet) => Some(self.next_piece(element, packet)),
      },
      Phase::Piece(element, mut packet, length) => {
        match packet.offer(walk, PRIMARY, &self.piece, length, filled, holdings) {
          Some(goes_on) => self.after(walk, element, goes_on),
          None => Some(self.next_piece(element, packet)),
        }
      }
      Phase::ReferenceOffset(element) => self.reference_offset(walk, element, filled),
      Phase::ReferenceHeader(element, offset) => self.reference_header(walk, element, offset, filled),
      Phase::ReferenceText(element, length) => {
        let goes_on = walk.take_title(PRIMARY, &element, filled, self.body.get(..length).unwrap_or_default());
        self.after(walk, element, goes_on)
      }
      Phase::Finished => None,
    }
  }

  /// The end of detection: an identified file starts its walk with the scratch buffers reserved.
  fn identified(&mut self, walk: &mut Walk<'_>, detected: Detected, holdings: &mut dyn Holdings) -> Option<Next> {
    let first_element = match detected {
      Detected::Identified { first_element } => first_element,
      Detected::Concluded(outcome) => {
        walk.detection = outcome;
        return None;
      }
    };
    walk.detection = DetectionOutcome::Identified;
    if let Err(limit) = walk.prepare(holdings) {
      walk.stop(Reason::Limit(limit));
      return None;
    }
    self.piece = mem::take(&mut walk.piece);
    self.body = mem::take(&mut walk.body);
    self.cursor = walk.begin(first_element);
    Some(self.element_header(walk))
  }

  /// What `element` asks of the walker, as the range that it needs.
  fn act(&mut self, walk: &mut Walk<'_>, element: Element, holdings: &mut dyn Holdings) -> Option<Next> {
    match walk.action(&mut self.cursor, PRIMARY, &element, holdings) {
      Action::Moved => Some(self.element_header(walk)),
      Action::Stopped => None,
      Action::Skip => self.after(walk, element, true),
      Action::Body => {
        let length = narrow(element.body_length()).min(self.body.len());
        Some(Next {
          phase: Phase::Body(element, length),
          position: element.body_start,
          length,
        })
      }
      Action::Packet => Some(Next {
        phase: Phase::Number(element),
        position: element.body_start,
        length: STREAM_NUMBER_LENGTH,
      }),
      Action::Reference => Some(Next {
        phase: Phase::ReferenceOffset(element),
        position: element.body_start,
        length: DOUBLE_WORD_LENGTH,
      }),
    }
  }

  /// The offset of a title reference: a title element recorded before it is read, anything else is a bad body.
  fn reference_offset(&mut self, walk: &mut Walk<'_>, element: Element, filled: &Filled) -> Option<Next> {
    if *filled != Filled::Complete {
      let goes_on = walk.interrupted(PRIMARY, element.position, filled);
      return self.after(walk, element, goes_on);
    }
    let offset = u64::from_be_bytes(self.word);
    if !walk.names_title(PRIMARY, &element, offset) {
      return self.after(walk, element, true);
    }
    Some(Next {
      phase: Phase::ReferenceHeader(element, offset),
      position: offset,
      length: ELEMENT_HEADER_LENGTH,
    })
  }

  /// The header that a title reference names: a title that ends before the reference, and fits the scratch buffer,
  /// has its text read; anything else is a bad body.
  fn reference_header(&mut self, walk: &mut Walk<'_>, element: Element, offset: u64, filled: &Filled) -> Option<Next> {
    if *filled != Filled::Complete {
      let goes_on = walk.interrupted(PRIMARY, element.position, filled);
      return self.after(walk, element, goes_on);
    }
    let capacity = self.body.len();
    let Some(target) =
      title_target(&element, offset, self.header).filter(|target| narrow(target.body_length()) <= capacity)
    else {
      walk.diagnose(PRIMARY, element.position, DiagnosticKind::BadBody);
      return self.after(walk, element, true);
    };
    let length = narrow(target.body_length());
    Some(Next {
      phase: Phase::ReferenceText(element, length),
      position: target.body_start,
      length,
    })
  }

  /// The element is done: the walk goes on with the next one, or ends.
  fn after(&mut self, walk: &mut Walk<'_>, element: Element, goes_on: bool) -> Option<Next> {
    if !goes_on {
      return None;
    }
    self.cursor.advance(&element);
    Some(self.element_header(walk))
  }

  /// The header of the next element, which reaches no further than its list.
  fn element_header(&mut self, walk: &mut Walk<'_>) -> Next {
    let position = walk.next_header(&mut self.cursor, PRIMARY);
    Next {
      phase: Phase::ElementHeader(position),
      position,
      length: ELEMENT_HEADER_LENGTH,
    }
  }

  /// The next piece of `packet`.
  fn next_piece(&self, element: Element, packet: Packet) -> Next {
    let (position, length) = packet.next_piece(self.piece.len());
    Next {
      phase: Phase::Piece(element, packet, length),
      position,
      length,
    }
  }
}
