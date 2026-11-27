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

//! The async traversal of one source: element headers in file order, short bodies, packets offered piece by piece
//! to their streams, and title references followed backward. Every read goes through the internal source, so the
//! same code serves every input (D8).
//!
//! The per-element path is one `async fn` whose only awaits are those of `fill!`, which awaits only a read that
//! must wait; the decisions between the reads are plain code. A read that fails inside a packet offers the bytes
//! that arrived before the error, and the source is passed to the walk's plain code as the owner of the optional
//! cache blocks that a required buffer may take back. A nested `async fn` per element, or an await where
//! nothing waits, cost more than the walker's own work in the spike's measurement (`README.md` section 5), so only
//! the rare title reference has a future of its own.

use std::io;
use std::mem;

use super::{Action, Header, Packet, Walk, title_target};
use crate::report::DiagnosticKind;
use crate::source::{Filled, InternalSource, fill};
use crate::toy::Element;
use crate::toy::constants::{DOUBLE_WORD_LENGTH, ELEMENT_HEADER_LENGTH, STREAM_NUMBER_LENGTH};
use crate::utilities::narrow;

/// Walks the elements of the source `identifier` from `start`.
pub(crate) async fn walk_source<Source: InternalSource>(
  source: &mut Source,
  walk: &mut Walk<'_>,
  identifier: u64,
  start: u64,
) -> io::Result<()> {
  let mut cursor = walk.begin(start);
  let mut header = [0_u8; ELEMENT_HEADER_LENGTH];
  loop {
    let position = walk.next_header(&mut cursor, identifier);
    let filled = fill!(source, &mut walk.budget, position, &mut header)?;
    let element = match walk.element(identifier, position, &filled, header, &*source) {
      Header::Element(element) => element,
      Header::Ended => break,
      Header::Confirm(last) => {
        let mut byte = [0_u8; 1];
        let filled = fill!(source, &mut walk.budget, last, &mut byte)?;
        walk.confirm_end(&cursor, identifier, &filled);
        break;
      }
    };
    let goes_on = match walk.action(&mut cursor, identifier, &element, &mut *source) {
      Action::Moved => continue,
      Action::Stopped => false,
      Action::Skip => true,
      Action::Body => {
        let mut body = mem::take(&mut walk.body);
        let length = narrow(element.body_length()).min(body.len());
        let target = body.get_mut(..length).unwrap_or_default();
        let filled = fill!(source, &mut walk.budget, element.body_start, target);
        let read = body.get(..length).unwrap_or_default();
        let goes_on = filled.map(|filled| walk.take_body(identifier, &element, &filled, read, &mut *source));
        walk.body = body;
        goes_on?
      }
      Action::Packet => {
        let mut number = [0_u8; STREAM_NUMBER_LENGTH];
        let filled = fill!(source, &mut walk.budget, element.body_start, &mut number)?;
        match Packet::start(walk, identifier, &element, &filled, number) {
          Err(goes_on) => goes_on,
          Ok(mut packet) => {
            let mut piece = mem::take(&mut walk.piece);
            let verdict = loop {
              let (at, length) = packet.next_piece(piece.len());
              let target = piece.get_mut(..length).unwrap_or_default();
              let filled = match fill!(source, &mut walk.budget, at, target) {
                Ok(filled) => filled,
                Err(failure) => {
                  packet.offer_delivered(walk, identifier, &piece, failure.done, &mut *source);
                  break Err(failure.cause);
                }
              };
              if let Some(goes_on) = packet.offer(walk, identifier, &piece, length, &filled, &mut *source) {
                break Ok(goes_on);
              }
            };
            walk.piece = piece;
            verdict?
          }
        }
      }
      Action::Reference => follow(source, walk, identifier, &element).await?,
    };
    if !goes_on {
      break;
    }
    cursor.advance(&element);
  }
  Ok(())
}

/// Follows a title reference to an earlier title element. A target behind the forward-only window gives a
/// diagnostic, and the walk goes on with the next sibling (D9); a reference to anything but a title element that
/// the walk recorded before it is a bad body.
async fn follow<Source: InternalSource>(
  source: &mut Source,
  walk: &mut Walk<'_>,
  identifier: u64,
  element: &Element,
) -> io::Result<bool> {
  let mut target = [0_u8; DOUBLE_WORD_LENGTH];
  let filled = fill!(source, &mut walk.budget, element.body_start, &mut target)?;
  if filled != Filled::Complete {
    return Ok(walk.interrupted(identifier, element.position, &filled));
  }
  let offset = u64::from_be_bytes(target);
  if !walk.names_title(identifier, element, offset) {
    return Ok(true);
  }
  let mut header = [0_u8; ELEMENT_HEADER_LENGTH];
  let filled = fill!(source, &mut walk.budget, offset, &mut header)?;
  if filled != Filled::Complete {
    return Ok(walk.interrupted(identifier, element.position, &filled));
  }
  let referenced = title_target(element, offset, header);
  let mut body = mem::take(&mut walk.body);
  let length = referenced.map_or(0, |referenced| narrow(referenced.body_length()));
  let goes_on = if let (Some(referenced), Some(target)) = (referenced, body.get_mut(..length)) {
    let filled = fill!(source, &mut walk.budget, referenced.body_start, target);
    let read = body.get(..length).unwrap_or_default();
    filled.map(|filled| walk.take_title(identifier, element, &filled, read))
  } else {
    walk.diagnose(identifier, element.position, DiagnosticKind::BadBody);
    Ok(true)
  };
  walk.body = body;
  Ok(goes_on?)
}
