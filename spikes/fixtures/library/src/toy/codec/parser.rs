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

//! `parse`, the toy codec's parser (`spikes/input/README.md` section 3), ported from the I/O spike's
//! (`spikes/input/src/codec/parser.rs`), which G3.1 accepted: a pure function of the stream's input, the context of
//! the call, the bytes, the continuation value and whether the packet or stream has ended.
//!
//! Each unit is decided as soon as its bytes allow. In packet mode, a payload unit ends the packet's part of the
//! header, so a packet that starts with one has no header, and a packet that ends at a unit's boundary is finished
//! while the header continues in a later packet. In stream mode, payload units are skipped, and running out of
//! bytes is never malformed. A setup with a count of 0 completes the header, and one whose flag bit 0 is set ends
//! partially parsed by design.

use super::super::constants::{
  KIND_LENGTH, NEEDS_DECODING_FLAG, PARAMETER_LENGTH, PARAMETERS_KIND, PAYLOAD_KIND, SETUP_COUNT_OFFSET,
  SETUP_FLAGS_OFFSET, SETUP_KIND, SETUP_LENGTH,
};
use super::types::{Context, Continuation, Framing, Input, Outcome, ParseResult, Setup};

/// Parses the units of `bytes`: a packet's bytes in packet mode, or the bytes kept of the stream in stream mode.
/// `ended` says that the packet's last byte, or the stream's, is among them.
pub(crate) fn parse(
  input: Input,
  context: Context,
  bytes: &[u8],
  continuation: Continuation,
  ended: bool,
) -> ParseResult {
  let header_length = KIND_LENGTH.saturating_add(input.representation.length_size());
  let mut scan = Scan {
    framing: input.framing,
    continuation,
    offset: 0,
  };
  loop {
    let rest = bytes.get(scan.offset..).unwrap_or_default();
    let Some(&kind) = rest.first() else {
      return scan.incomplete(true, ended);
    };
    match kind {
      PAYLOAD_KIND if input.framing == Framing::Packet => return scan.payload(),
      PAYLOAD_KIND | SETUP_KIND => {}
      PARAMETERS_KIND if scan.continuation.setup.is_some() => {}
      _ => return scan.malformed(),
    }
    let Some(length) = read_length(rest.get(KIND_LENGTH..header_length)) else {
      return scan.incomplete(false, ended);
    };
    let unit_length = header_length.saturating_add(length);
    let unit_end = scan.offset.saturating_add(unit_length);
    let beyond_packet =
      input.framing == Framing::Packet && context.packet_length.is_some_and(|packet| unit_end > packet);
    if beyond_packet || contradicts(kind, length, scan.continuation) {
      return scan.malformed();
    }
    let Some(body) = rest.get(header_length..unit_length) else {
      return scan.incomplete(false, ended);
    };
    match kind {
      SETUP_KIND => {
        let setup = Setup {
          parameter_count: body.get(SETUP_COUNT_OFFSET).copied().unwrap_or_default(),
          flags: body.get(SETUP_FLAGS_OFFSET).copied().unwrap_or_default(),
        };
        if setup.parameter_count == 0 {
          return scan.result(Outcome::Parsed, unit_end);
        }
        if setup.flags & NEEDS_DECODING_FLAG != 0 {
          return scan.result(Outcome::PartiallyParsedByDesign, unit_end);
        }
        scan.continuation.setup = Some(setup);
      }
      PARAMETERS_KIND => return scan.result(Outcome::Parsed, unit_end),
      _ => {}
    }
    scan.offset = unit_end;
  }
}

/// Whether a unit's stated length contradicts its kind: a setup holds exactly its fields, and a parameters unit
/// exactly the setup's count of parameters.
fn contradicts(kind: u8, length: usize, continuation: Continuation) -> bool {
  match kind {
    SETUP_KIND => length != SETUP_LENGTH,
    PARAMETERS_KIND => continuation
      .setup
      .is_none_or(|setup| length != usize::from(setup.parameter_count).saturating_mul(PARAMETER_LENGTH)),
    _ => false,
  }
}

/// The body length that a unit's length field of one or two bytes states, once its bytes are all there.
fn read_length(field: Option<&[u8]>) -> Option<usize> {
  match *field? {
    [length] => Some(usize::from(length)),
    [high, low] => Some(usize::from(u16::from_be_bytes([high, low]))),
    _ => None,
  }
}

/// The state of one call: how far it has scanned, and what it has established.
struct Scan {
  /// The stream's framing.
  framing: Framing,
  /// What the call has established.
  continuation: Continuation,
  /// The offset of the next unit.
  offset: usize,
}

impl Scan {
  /// The bytes end before a whole unit. In packet mode a packet that has ended at a unit's boundary is finished and
  /// the header continues in a later packet, while a unit cut by its packet's end is malformed. In stream mode the
  /// bytes before the offset are finished, and the end of the stream changes nothing.
  fn incomplete(self, at_boundary: bool, ended: bool) -> ParseResult {
    let outcome = match self.framing {
      Framing::Packet if ended && !at_boundary => Outcome::Malformed,
      Framing::Packet | Framing::Stream => Outcome::NeedsMoreBytes,
    };
    let finished_bytes = self.offset;
    self.result(outcome, finished_bytes)
  }

  /// A payload unit in packet mode: the packet carries nothing more of the header, and one that starts with it,
  /// before any setup, has no header at all.
  fn payload(self) -> ParseResult {
    let outcome = if self.continuation.setup.is_none() && self.offset == 0 {
      Outcome::NoHeaderPresent
    } else {
      Outcome::NeedsMoreBytes
    };
    let finished_bytes = self.offset;
    self.result(outcome, finished_bytes)
  }

  /// A unit that its kind or length already contradicts: an unknown kind, parameters before any setup, a length
  /// that its kind or the setup rules out, or an end beyond the packet's known length.
  fn malformed(self) -> ParseResult {
    let finished_bytes = self.offset;
    self.result(Outcome::Malformed, finished_bytes)
  }

  /// The result of the call, with the leading bytes that are finished.
  fn result(self, outcome: Outcome, finished_bytes: usize) -> ParseResult {
    ParseResult {
      outcome,
      finished_bytes,
      continuation: self.continuation,
    }
  }
}
