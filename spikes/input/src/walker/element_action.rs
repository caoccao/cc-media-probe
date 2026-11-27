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

//! What each element of the toy format asks of the walker, what a short body declares (a stream, a title or a
//! companion), and which element a title reference names: only a title element that the walk recorded before it,
//! so bytes shaped like a title inside another element's body are never taken for one. Plain code, shared by the
//! three walkers.

use super::constants::PRIMARY_SOURCE;
use super::cursor::OpenList;
use super::state::StreamState;
use super::{Action, Cursor, Walk};
use crate::assembler::Assembler;
use crate::budget::Reclaim;
use crate::codec::{Framing, Input, Representation};
use crate::companion::is_valid;
use crate::report::{DiagnosticKind, Reason};
use crate::source::Filled;
use crate::toy::Element;
use crate::toy::constants::{
  CODE_LENGTH, COMPANION_CODE, DOUBLE_WORD_LENGTH, ELEMENT_HEADER_LENGTH, LIST_CODE, MAXIMUM_TEXT_LENGTH,
  ONE_BYTE_LENGTHS, PACKET_CODE, PACKET_FRAMING, STREAM_BODY_LENGTH, STREAM_CODE, STREAM_FRAMING, STREAM_HEAD_LENGTH,
  STREAM_NUMBER_LENGTH, TITLE_CODE, TITLE_REFERENCE_CODE, TOY_CODEC, TWO_BYTE_LENGTHS,
};
use crate::utilities::widen;

/// The title that a reference at `element` names at `offset`, whose header was read as `header`: a title element
/// that ends before the reference starts, so its text can never be the reference itself or what follows it. Any
/// other target is `None`, a bad body.
pub(crate) fn title_target(element: &Element, offset: u64, header: [u8; ELEMENT_HEADER_LENGTH]) -> Option<Element> {
  Element::parse(offset, header).filter(|target| target.code == TITLE_CODE && target.body_end <= element.position)
}

impl Walk<'_> {
  /// What `element` asks of the walker. An element that runs beyond its list ends the list; a list deeper than
  /// the nesting limit is skipped. Entering a list grows the stack of open lists, working storage charged before it
  /// grows; a stack that does not fit, even after taking back the optional blocks of `room`, stops the traversal.
  pub(crate) fn action(
    &mut self,
    cursor: &mut Cursor,
    source: u64,
    element: &Element,
    room: &mut dyn Reclaim,
  ) -> Action {
    if self.lists.last().is_some_and(|list| element.body_end > list.end) {
      self.diagnose(source, element.position, DiagnosticKind::ElementOverrunsParent);
      self.leave(cursor);
      return Action::Moved;
    }
    let length = element.body_length();
    match element.code {
      LIST_CODE if widen(self.lists.len()) < self.budget.limits().nesting_depth() => {
        let lists = &mut self.lists;
        if let Err(limit) = self.budget.require(Some(room), |budget| budget.try_reserve_one(lists)) {
          self.stop(Reason::Limit(limit));
          return Action::Stopped;
        }
        lists.push(OpenList {
          position: element.position,
          end: element.body_end,
        });
        cursor.enter(element.body_start);
        Action::Moved
      }
      LIST_CODE => {
        self.diagnose(source, element.position, DiagnosticKind::NestingTooDeep);
        Action::Skip
      }
      STREAM_CODE if length == widen(STREAM_BODY_LENGTH) => Action::Body,
      TITLE_CODE | COMPANION_CODE if length <= widen(MAXIMUM_TEXT_LENGTH) => Action::Body,
      TITLE_REFERENCE_CODE if length == widen(DOUBLE_WORD_LENGTH) => Action::Reference,
      PACKET_CODE if length >= widen(STREAM_NUMBER_LENGTH) => Action::Packet,
      STREAM_CODE | TITLE_CODE | COMPANION_CODE | TITLE_REFERENCE_CODE | PACKET_CODE => {
        self.diagnose(source, element.position, DiagnosticKind::BadBody);
        Action::Skip
      }
      code => {
        self.diagnose(source, element.position, DiagnosticKind::UnknownElement(code));
        Action::Skip
      }
    }
  }

  /// Takes in a short body that was read as `filled`, and says whether the traversal goes on: not where the
  /// working storage that the body needs does not fit, even after taking back the optional blocks of `room`.
  pub(crate) fn take_body(
    &mut self,
    source: u64,
    element: &Element,
    filled: &Filled,
    body: &[u8],
    room: &mut dyn Reclaim,
  ) -> bool {
    if *filled == Filled::Complete {
      return self.on_body(source, element, body, room);
    }
    self.interrupted(source, element.position, filled)
  }

  /// Whether the reference at `element` names, at `offset`, a title element that the walk recorded before it in
  /// the same source. Any other target is a bad body, and nothing is read for it.
  pub(crate) fn names_title(&mut self, source: u64, element: &Element, offset: u64) -> bool {
    let recorded = offset < element.position && self.title_elements.binary_search(&(source, offset)).is_ok();
    if !recorded {
      self.diagnose(source, element.position, DiagnosticKind::BadBody);
    }
    recorded
  }

  /// Takes in the text of a title that `element` references, read as `filled`, and says whether the traversal
  /// goes on.
  pub(crate) fn take_title(&mut self, source: u64, element: &Element, filled: &Filled, text: &[u8]) -> bool {
    if *filled == Filled::Complete {
      self.title(source, text, true);
      return true;
    }
    self.interrupted(source, element.position, filled)
  }

  /// Takes in the whole body of a stream declaration, a title or a companion, and says whether the traversal goes
  /// on.
  fn on_body(&mut self, source: u64, element: &Element, body: &[u8], room: &mut dyn Reclaim) -> bool {
    match element.code {
      STREAM_CODE => self.declare(source, element, body),
      TITLE_CODE => {
        self.title(source, body, false);
        return self.record_title(source, element, room);
      }
      COMPANION_CODE => return self.companion(source, element, body, room),
      _ => {}
    }
    true
  }

  /// Records a title element by its source and position, for the references that follow it. The record is working
  /// storage of 16 bytes for each title, charged before it grows; one that does not fit stops the traversal.
  fn record_title(&mut self, source: u64, element: &Element, room: &mut dyn Reclaim) -> bool {
    let elements = &mut self.title_elements;
    match self
      .budget
      .require(Some(room), |budget| budget.try_reserve_one(elements))
    {
      Ok(()) => {
        elements.push((source, element.position));
        true
      }
      Err(limit) => {
        self.stop(Reason::Limit(limit));
        false
      }
    }
  }

  /// Declares a stream: its number, its framing mode, its representation and its codec. Only the toy codec gets an
  /// assembler.
  fn declare(&mut self, source: u64, element: &Element, body: &[u8]) {
    let Some((&[number, framing, representation], codec)) = body.split_first_chunk::<STREAM_HEAD_LENGTH>() else {
      return self.diagnose(source, element.position, DiagnosticKind::BadBody);
    };
    let framing = match framing {
      PACKET_FRAMING => Framing::Packet,
      STREAM_FRAMING => Framing::Stream,
      _ => return self.diagnose(source, element.position, DiagnosticKind::BadBody),
    };
    let representation = match representation {
      ONE_BYTE_LENGTHS => Representation::OneByteLengths,
      TWO_BYTE_LENGTHS => Representation::TwoByteLengths,
      _ => return self.diagnose(source, element.position, DiagnosticKind::BadBody),
    };
    let Ok(codec) = <[u8; CODE_LENGTH]>::try_from(codec) else {
      return self.diagnose(source, element.position, DiagnosticKind::BadBody);
    };
    if self.waiting(source, number).is_some() {
      return self.diagnose(source, element.position, DiagnosticKind::DuplicateStream(number));
    }
    if widen(self.streams.len()) >= self.budget.limits().streams() {
      return self.diagnose(source, element.position, DiagnosticKind::StreamNotKept);
    }
    self.streams.push(StreamState {
      source,
      number,
      framing,
      representation,
      codec,
      assembler: (codec == TOY_CODEC).then(|| {
        Assembler::new(Input {
          framing,
          representation,
        })
      }),
    });
  }

  /// Lists a companion of the primary input, once per identifier. With lookup on, it waits for its request after
  /// the walk, when its limits are checked (plan section 7.7). Says whether the traversal goes on: not where the
  /// catalog's working storage does not fit.
  fn companion(&mut self, source: u64, element: &Element, body: &[u8], room: &mut dyn Reclaim) -> bool {
    if source != PRIMARY_SOURCE {
      self.diagnose(source, element.position, DiagnosticKind::NestedCompanion);
      return true;
    }
    if !is_valid(body) {
      self.diagnose(source, element.position, DiagnosticKind::InvalidIdentifier);
      return true;
    }
    let queued = self.lookup_enabled();
    if let Err(limit) = self.catalog.add(&mut self.budget, room, body, element.position, queued) {
      self.stop(Reason::Limit(limit));
      return false;
    }
    true
  }
}
