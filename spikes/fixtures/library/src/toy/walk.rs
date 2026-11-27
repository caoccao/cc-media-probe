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

//! The toy container's walk (`spikes/input/README.md` section 2), reduced from the I/O spike's walker to what a
//! container target needs: detection, then each element in order, lists entered within the library's default
//! nesting depth, each stream declared and each packet handed to its stream's assembler. Titles, references and
//! companions are skipped, since no target's outcome depends on them. As the accepted walker does, an element that
//! the input's end cuts short keeps what arrived of it: a list is entered and its children before the end are
//! walked, and a packet's bytes before the end are offered to its stream, before the walk ends truncated.

use cc_media_probe::Limits;

use super::constants::{
  CODE_LENGTH, ELEMENT_HEADER_LENGTH, INSUFFICIENT_DATA, LIST_CODE, PACKET_CODE, REPORT, STREAM_CODE, TRUNCATED,
  UNKNOWN_FORMAT, WORD_LENGTH,
};
use super::detect::{Detected, detect};
use super::streams::Streams;

/// What a walk found: its outcome, and each declared stream's header state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Walked {
  /// `report`, `truncated`, or detection's `unknown_format` or `insufficient_data`.
  pub(crate) outcome: &'static str,
  /// Each declared stream's header state, in the order of the declarations.
  pub(crate) streams: Vec<&'static str>,
}

/// An element's header at a position: its code, the range of its body, and the length that its header states.
struct Element {
  /// The code.
  code: [u8; CODE_LENGTH],
  /// The offset of the body.
  body_start: usize,
  /// The offset after the body.
  body_end: usize,
  /// The body's length, as its header states it.
  length: usize,
}

/// Walks the toy file `bytes`.
pub(crate) fn walk(bytes: &[u8]) -> Walked {
  let mut position = match detect(bytes) {
    Detected::Toy { first_element } => first_element,
    Detected::Unknown => return walked(UNKNOWN_FORMAT, Streams::default()),
    Detected::InsufficientData => return walked(INSUFFICIENT_DATA, Streams::default()),
  };
  let nesting = usize::try_from(Limits::new().nesting_depth()).unwrap_or(usize::MAX);
  let mut lists: Vec<usize> = Vec::new();
  let mut streams = Streams::default();
  let outcome = loop {
    if let Some(&end) = lists.last() {
      if end.saturating_sub(position) < ELEMENT_HEADER_LENGTH {
        lists.pop();
        position = end;
        continue;
      }
    } else if position == bytes.len() {
      break REPORT;
    }
    let Some(element) = element(bytes, position) else {
      break TRUNCATED;
    };
    if let Some(&end) = lists.last().filter(|end| element.body_end > **end) {
      lists.pop();
      position = end;
      continue;
    }
    let arrived = bytes.get(element.body_start..).unwrap_or_default();
    let body = arrived.get(..element.length).unwrap_or(arrived);
    let whole = body.len() == element.length;
    match element.code {
      LIST_CODE if lists.len() < nesting => {
        lists.push(element.body_end);
        position = element.body_start;
        continue;
      }
      STREAM_CODE if whole => streams.declare(body),
      PACKET_CODE => streams.offer(body, element.length),
      _ => {}
    }
    if !whole {
      break TRUNCATED;
    }
    position = element.body_end;
  };
  walked(outcome, streams)
}

/// The walk's result.
fn walked(outcome: &'static str, streams: Streams) -> Walked {
  Walked {
    outcome,
    streams: streams.finish(),
  }
}

/// The element header at `position`, or `None` where the input ends inside it or its body would end beyond
/// `usize::MAX`.
fn element(bytes: &[u8], position: usize) -> Option<Element> {
  let header = bytes.get(position..)?.first_chunk::<ELEMENT_HEADER_LENGTH>()?;
  let (code, length) = header.split_first_chunk::<CODE_LENGTH>()?;
  let length = usize::try_from(u32::from_be_bytes(*length.first_chunk::<WORD_LENGTH>()?)).ok()?;
  let body_start = position.checked_add(ELEMENT_HEADER_LENGTH)?;
  Some(Element {
    code: *code,
    body_start,
    body_end: body_start.checked_add(length)?,
    length,
  })
}
