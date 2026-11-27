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

//! The types of the parsing contract (plan section 7.4): the parser's input of framing mode and representation, the
//! context of one call, the fields, the continuation value, the outcome and the consumption.

/// How the container delivers a stream's bytes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Framing {
  /// The container delivers packets, never concatenated, each in bounded pieces.
  Packet,
  /// The container delivers a byte stream cut at arbitrary points, concatenated in order.
  Stream,
}

/// The representation of the toy codec's bytes, which the stream declaration states: how long a unit's length field
/// is (plan section 7.4, "Input": length-prefixed units with a stated length size).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Representation {
  /// A length field of one byte, so a body of at most 255 bytes.
  OneByteLengths,
  /// A length field of two bytes, big-endian, which holds a parameters unit of more than 127 parameters.
  TwoByteLengths,
}

/// What the container states about a stream for all of its calls (plan section 7.4, "Input" and "Framing mode").
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Input {
  /// How the container delivers the stream's bytes.
  pub framing: Framing,
  /// The form of those bytes.
  pub representation: Representation,
}

/// What the container knows of the bytes of one call (plan section 7.4, "Input": the context the container already
/// knows).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Context {
  /// In packet mode, the total length of the packet whose bytes are parsed, where the container knows it. A parser
  /// can then call a unit that runs past the packet malformed on its first piece.
  pub packet_length: Option<usize>,
}

/// The fields of the setup unit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Setup {
  /// The width.
  pub width: u16,
  /// The height.
  pub height: u16,
  /// The number of parameters that the parameters unit holds.
  pub parameter_count: u8,
  /// The flags.
  pub flags: u8,
}

/// A whole header: the setup and the sum of its parameters.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Header {
  /// The setup unit's fields.
  pub setup: Setup,
  /// The sum of the parameters.
  pub parameter_sum: u32,
}

/// What the parser established so far, in a fixed-size form: the assembler keeps it between calls, after it has
/// dropped the bytes that gave it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Continuation {
  /// The setup, once read.
  pub setup: Option<Setup>,
}

/// The outcome of one call.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
  /// Every field was read.
  Parsed(Header),
  /// The setup was read, and the parameters cannot be reached without decoding: a final state.
  PartiallyParsedByDesign(Setup),
  /// More of the same packet or stream is needed, at least `total` bytes where the parser knows it. In packet mode
  /// with the packet finished, a later packet is needed.
  NeedsMoreBytes {
    /// The smallest total that the parser can make progress with, of the packet or of the bytes kept.
    total: Option<usize>,
  },
  /// Well-formed data without a header; a later packet may carry it.
  NoHeaderPresent,
  /// The bytes contradict the syntax. Running out of bytes never gives this.
  Malformed,
}

/// How much of its input the parser finished.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Consumption {
  /// In packet mode: whether the packet is finished, so the container skips its rest.
  Packet {
    /// The parser needs nothing more of this packet.
    finished: bool,
  },
  /// In stream mode: the leading bytes that never need to be offered again.
  Stream {
    /// The count of those bytes.
    finished_bytes: usize,
  },
}

/// What one call gives.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ParseResult {
  /// The outcome.
  pub outcome: Outcome,
  /// How much of the input is finished.
  pub consumption: Consumption,
  /// What was established, which the next call is given.
  pub continuation: Continuation,
  /// The bytes that the call examined, which the work counter is charged with.
  pub examined: usize,
}
