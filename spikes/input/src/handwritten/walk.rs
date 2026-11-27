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

//! The hand-written blocking traversal: the async walker's code with every read made directly. Detection, element
//! headers, short bodies, packets offered piece by piece, and title references share the walker's plain code.

use std::io;
use std::mem;

use super::source::{BlockingSource, fill};
use crate::budget::Budget;
use crate::detection::{Detected, judge_first_element, judge_header};
use crate::report::{DetectionOutcome, DiagnosticKind, Reason};
use crate::source::Filled;
use crate::toy::Element;
use crate::toy::constants::{DOUBLE_WORD_LENGTH, ELEMENT_HEADER_LENGTH, FILE_HEADER_LENGTH, STREAM_NUMBER_LENGTH};
use crate::utilities::narrow;
use crate::walker::{Action, Header, PRIMARY, Packet, Walk, title_target};

/// Detection, then the walk of the primary input.
pub(super) fn run<Source: BlockingSource>(source: &mut Source, walk: &mut Walk<'_>) -> io::Result<()> {
  let first_element = match detect(source, &mut walk.budget)? {
    Detected::Identified { first_element } => first_element,
    Detected::Concluded(outcome) => {
      walk.detection = outcome;
      return Ok(());
    }
  };
  walk.detection = DetectionOutcome::Identified;
  let walked = match walk.prepare(source) {
    Ok(()) => walk_source(source, walk, PRIMARY, first_element),
    Err(limit) => {
      walk.stop(Reason::Limit(limit));
      Ok(())
    }
  };
  if walked.is_err() {
    walk.fail_streams(PRIMARY);
    return walked;
  }
  walk.finish_streams(PRIMARY);
  Ok(())
}

/// Reads the file header and the first element's header.
fn detect<Source: BlockingSource>(source: &mut Source, budget: &mut Budget) -> io::Result<Detected> {
  let mut header = [0_u8; FILE_HEADER_LENGTH];
  let filled = fill(source, budget, 0, &mut header)?;
  Ok(match judge_header(&filled, &header) {
    Ok(first_element) => {
      let mut element = [0_u8; ELEMENT_HEADER_LENGTH];
      let filled = fill(source, budget, first_element, &mut element)?;
      judge_first_element(&filled, element, first_element)
    }
    Err(outcome) => Detected::Concluded(outcome),
  })
}

/// Walks the elements of the source `identifier` from `start`.
fn walk_source<Source: BlockingSource>(
  source: &mut Source,
  walk: &mut Walk<'_>,
  identifier: u64,
  start: u64,
) -> io::Result<()> {
  let mut cursor = walk.begin(start);
  let mut header = [0_u8; ELEMENT_HEADER_LENGTH];
  loop {
    let position = walk.next_header(&mut cursor, identifier);
    let filled = fill(source, &mut walk.budget, position, &mut header)?;
    let element = match walk.element(identifier, position, &filled, header, &*source) {
      Header::Element(element) => element,
      Header::Ended => break,
      Header::Confirm(last) => {
        let mut byte = [0_u8; 1];
        let filled = fill(source, &mut walk.budget, last, &mut byte)?;
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
        let filled = fill(source, &mut walk.budget, element.body_start, target);
        let read = body.get(..length).unwrap_or_default();
        let goes_on = filled.map(|filled| walk.take_body(identifier, &element, &filled, read, &mut *source));
        walk.body = body;
        goes_on?
      }
      Action::Packet => {
        let mut number = [0_u8; STREAM_NUMBER_LENGTH];
        let filled = fill(source, &mut walk.budget, element.body_start, &mut number)?;
        match Packet::start(walk, identifier, &element, &filled, number) {
          Err(goes_on) => goes_on,
          Ok(mut packet) => {
            let mut piece = mem::take(&mut walk.piece);
            let verdict = loop {
              let (at, length) = packet.next_piece(piece.len());
              let target = piece.get_mut(..length).unwrap_or_default();
              let filled = match fill(source, &mut walk.budget, at, target) {
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
      Action::Reference => follow(source, walk, identifier, &element)?,
    };
    if !goes_on {
      break;
    }
    cursor.advance(&element);
  }
  Ok(())
}

/// Follows a title reference to an earlier title element.
fn follow<Source: BlockingSource>(
  source: &mut Source,
  walk: &mut Walk<'_>,
  identifier: u64,
  element: &Element,
) -> io::Result<bool> {
  let mut target = [0_u8; DOUBLE_WORD_LENGTH];
  let filled = fill(source, &mut walk.budget, element.body_start, &mut target)?;
  if filled != Filled::Complete {
    return Ok(walk.interrupted(identifier, element.position, &filled));
  }
  let offset = u64::from_be_bytes(target);
  if !walk.names_title(identifier, element, offset) {
    return Ok(true);
  }
  let mut header = [0_u8; ELEMENT_HEADER_LENGTH];
  let filled = fill(source, &mut walk.budget, offset, &mut header)?;
  if filled != Filled::Complete {
    return Ok(walk.interrupted(identifier, element.position, &filled));
  }
  let referenced = title_target(element, offset, header);
  let mut body = mem::take(&mut walk.body);
  let length = referenced.map_or(0, |referenced| narrow(referenced.body_length()));
  let goes_on = if let (Some(referenced), Some(target)) = (referenced, body.get_mut(..length)) {
    let filled = fill(source, &mut walk.budget, referenced.body_start, target);
    let read = body.get(..length).unwrap_or_default();
    filled.map(|filled| walk.take_title(identifier, element, &filled, read))
  } else {
    walk.diagnose(identifier, element.position, DiagnosticKind::BadBody);
    Ok(true)
  };
  walk.body = body;
  Ok(goes_on?)
}
