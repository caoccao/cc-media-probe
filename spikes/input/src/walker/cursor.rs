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

//! `Cursor`, the walker's position and the element or list that it last moved past; `OpenList`, one list that the
//! walk is inside, which the walk's explicit stack holds (plan section 7.6: no input-driven recursion), charged to
//! working memory as it grows; `Header`, what the read of an element's header gives; and `Action`, what an element
//! asks of the walker.

use crate::toy::Element;

/// What the read of an element's header gives.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Header {
  /// The element.
  Element(Element),
  /// The traversal of the source is over.
  Ended,
  /// The input gave nothing at the header's position: read the byte at this position, just before, to tell a
  /// clean end from a jump past the end.
  Confirm(u64),
}

/// What an element asks of the walker.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Action {
  /// Nothing: go to the next element.
  Skip,
  /// The cursor has moved, into a list or out of one: read the element where it now stands.
  Moved,
  /// Read the element's whole body, which is short.
  Body,
  /// Offer the packet to its stream.
  Packet,
  /// Follow the title reference.
  Reference,
  /// A limit stopped the traversal.
  Stopped,
}

/// One open list: where its element starts and where its body ends.
#[derive(Clone, Copy, Debug)]
pub(super) struct OpenList {
  /// The position of the list's element.
  pub(super) position: u64,
  /// The end of its body.
  pub(super) end: u64,
}

/// The position of the next element, and the element or list that the cursor last moved past, which a jump past the
/// end of the input truncates. The lists that hold the position are the walk's stack.
#[derive(Debug)]
pub(crate) struct Cursor {
  position: u64,
  last_element: u64,
}

impl Cursor {
  /// A cursor at `start`. A walk starts one with `Walk::begin`, which empties the stack of open lists.
  pub(crate) fn new(start: u64) -> Cursor {
    Cursor {
      position: start,
      last_element: start,
    }
  }

  /// The position of the next element.
  pub(super) fn position(&self) -> u64 {
    self.position
  }

  /// The position of the element or list that the cursor last moved past.
  pub(crate) fn last_element(&self) -> u64 {
    self.last_element
  }

  /// Moves into a list whose body starts at `body_start`.
  pub(super) fn enter(&mut self, body_start: u64) {
    self.position = body_start;
  }

  /// Moves past `list`, whose rest is skipped.
  pub(super) fn leave(&mut self, list: OpenList) {
    self.last_element = list.position;
    self.position = list.end;
  }

  /// Moves past `element`, the element just handled, to its end.
  pub(crate) fn advance(&mut self, element: &Element) {
    self.last_element = element.position;
    self.position = element.body_end;
  }
}
