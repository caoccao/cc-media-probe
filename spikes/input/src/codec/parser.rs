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

//! `parse`, the toy codec's parser: a pure function of the stream's input, the context of the call, the bytes, the
//! continuation value and whether the packet or stream has ended. The same arguments always give the same result.
//!
//! Each unit is decided as soon as its bytes allow: at a unit boundary in packet mode the parser asks for the kind
//! byte alone, since a payload kind ends the packet's part of the header, and a unit whose kind, length or place
//! contradicts the syntax is malformed once those bytes are there, before its body is needed.

use super::constants::{
  KIND_LENGTH, NEEDS_DECODING_FLAG, ONE_BYTE_LENGTH_SIZE, PACKET_FINISHED, PARAMETER_LENGTH, PARAMETERS_KIND,
  PAYLOAD_KIND, SETUP_KIND, SETUP_LENGTH, TWO_BYTE_LENGTH_SIZE,
};
use super::types::{
  Consumption, Context, Continuation, Framing, Header, Input, Outcome, ParseResult, Representation, Setup,
};

/// Parses the units of `bytes`: a packet's bytes so far in packet mode, or the bytes kept of the stream in stream
/// mode. `ended` says that the packet's last piece, or the stream's last byte, is among them.
#[must_use]
pub fn parse(input: Input, context: Context, bytes: &[u8], continuation: Continuation, ended: bool) -> ParseResult {
  let header_length = KIND_LENGTH.saturating_add(length_size(input.representation));
  let mut scan = Scan {
    framing: input.framing,
    continuation,
    offset: 0,
    length: bytes.len(),
  };
  loop {
    let rest = bytes.get(scan.offset..).unwrap_or_default();
    let Some(&kind) = rest.first() else {
      return scan.incomplete(true, KIND_LENGTH, ended);
    };
    match kind {
      PAYLOAD_KIND if input.framing == Framing::Packet => return scan.payload(),
      PAYLOAD_KIND | SETUP_KIND => {}
      PARAMETERS_KIND if scan.continuation.setup.is_some() => {}
      _ => return scan.malformed(KIND_LENGTH),
    }
    let Some(length) = read_length(input.representation, rest.get(KIND_LENGTH..header_length)) else {
      return scan.incomplete(false, header_length, ended);
    };
    let unit_length = header_length.saturating_add(length);
    let unit_end = scan.offset.saturating_add(unit_length);
    let beyond_packet =
      input.framing == Framing::Packet && context.packet_length.is_some_and(|packet| unit_end > packet);
    if beyond_packet || contradicts(kind, length, scan.continuation) {
      return scan.malformed(header_length);
    }
    let Some(body) = rest.get(header_length..unit_length) else {
      return scan.incomplete(false, unit_length, ended);
    };
    match kind {
      PAYLOAD_KIND => scan.offset = unit_end,
      SETUP_KIND => {
        let Some(setup) = read_setup(body) else {
          return scan.done(Outcome::Malformed, unit_end);
        };
        if setup.parameter_count == 0 {
          let header = Header {
            setup,
            parameter_sum: 0,
          };
          return scan.done(Outcome::Parsed(header), unit_end);
        }
        if setup.flags & NEEDS_DECODING_FLAG != 0 {
          return scan.done(Outcome::PartiallyParsedByDesign(setup), unit_end);
        }
        scan.continuation.setup = Some(setup);
        scan.offset = unit_end;
      }
      PARAMETERS_KIND => {
        let header = scan.continuation.setup.and_then(|setup| read_parameters(setup, body));
        return scan.done(header.map_or(Outcome::Malformed, Outcome::Parsed), unit_end);
      }
      _ => return scan.done(Outcome::Malformed, unit_end),
    }
  }
}

/// The length of a unit's length field in `representation`.
#[must_use]
pub fn length_size(representation: Representation) -> usize {
  match representation {
    Representation::OneByteLengths => ONE_BYTE_LENGTH_SIZE,
    Representation::TwoByteLengths => TWO_BYTE_LENGTH_SIZE,
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

/// The body length that a unit's length field states, from the field's bytes once they are all there.
fn read_length(representation: Representation, field: Option<&[u8]>) -> Option<usize> {
  match (representation, field?) {
    (Representation::OneByteLengths, &[length]) => Some(usize::from(length)),
    (Representation::TwoByteLengths, &[high, low]) => Some(usize::from(u16::from_be_bytes([high, low]))),
    _ => None,
  }
}

/// The state of one call: how far it has scanned, and what it has established.
struct Scan {
  framing: Framing,
  continuation: Continuation,
  offset: usize,
  length: usize,
}

impl Scan {
  /// The bytes end before a whole unit, `needed` bytes long from the scan's offset. In packet mode a packet that
  /// has ended at a unit's boundary is finished and the header continues in a later packet, while a unit cut by
  /// its packet's end is malformed. In stream mode the bytes before the offset are finished, and the end of the
  /// stream changes nothing: running out of bytes is never malformed.
  fn incomplete(self, at_boundary: bool, needed: usize, ended: bool) -> ParseResult {
    let (outcome, consumption) = match self.framing {
      Framing::Packet if ended && at_boundary => (Outcome::NeedsMoreBytes { total: None }, PACKET_FINISHED),
      Framing::Packet if ended => (Outcome::Malformed, PACKET_FINISHED),
      Framing::Packet => {
        let total = Some(self.offset.saturating_add(needed));
        (
          Outcome::NeedsMoreBytes { total },
          Consumption::Packet { finished: false },
        )
      }
      Framing::Stream => {
        let total = (!at_boundary).then_some(needed);
        let finished_bytes = self.offset;
        (
          Outcome::NeedsMoreBytes { total },
          Consumption::Stream { finished_bytes },
        )
      }
    };
    ParseResult {
      outcome,
      consumption,
      continuation: self.continuation,
      examined: self.length,
    }
  }

  /// A payload unit in packet mode, known by its kind byte alone: the packet carries nothing more of the header.
  /// A packet that starts with one, before any setup, has no header at all.
  fn payload(self) -> ParseResult {
    let outcome = if self.continuation.setup.is_none() && self.offset == 0 {
      Outcome::NoHeaderPresent
    } else {
      Outcome::NeedsMoreBytes { total: None }
    };
    ParseResult {
      outcome,
      consumption: PACKET_FINISHED,
      continuation: self.continuation,
      examined: self.offset.saturating_add(KIND_LENGTH),
    }
  }

  /// A unit that its first `examined` bytes already contradict: an unknown kind, parameters before any setup, a
  /// length that its kind or the setup rules out, or an end beyond the packet's known length. The rest of the unit
  /// is never needed.
  fn malformed(self, examined: usize) -> ParseResult {
    let end = self.offset.saturating_add(examined);
    self.done(Outcome::Malformed, end)
  }

  /// A final outcome after the unit that ends at `unit_end`.
  fn done(self, outcome: Outcome, unit_end: usize) -> ParseResult {
    let consumption = match self.framing {
      Framing::Packet => PACKET_FINISHED,
      Framing::Stream => Consumption::Stream {
        finished_bytes: unit_end,
      },
    };
    ParseResult {
      outcome,
      consumption,
      continuation: self.continuation,
      examined: unit_end,
    }
  }
}

/// The setup unit's fields, from a body of exactly its length.
fn read_setup(body: &[u8]) -> Option<Setup> {
  let body: &[u8; SETUP_LENGTH] = body.try_into().ok()?;
  let (width, rest) = body.split_first_chunk::<PARAMETER_LENGTH>()?;
  let (height, rest) = rest.split_first_chunk::<PARAMETER_LENGTH>()?;
  let &[parameter_count, flags] = rest else {
    return None;
  };
  Some(Setup {
    width: u16::from_be_bytes(*width),
    height: u16::from_be_bytes(*height),
    parameter_count,
    flags,
  })
}

/// The header, from the parameters unit, which holds exactly the setup's count of parameters.
fn read_parameters(setup: Setup, body: &[u8]) -> Option<Header> {
  if body.len() != usize::from(setup.parameter_count).saturating_mul(PARAMETER_LENGTH) {
    return None;
  }
  let parameter_sum = body
    .as_chunks::<PARAMETER_LENGTH>()
    .0
    .iter()
    .map(|parameter| u32::from(u16::from_be_bytes(*parameter)))
    .fold(0_u32, u32::saturating_add);
  Some(Header { setup, parameter_sum })
}
