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

//! The types of the toy codec's parsing contract (plan section 7.4), as the I/O spike's parser has them
//! (`spikes/input/src/codec/types.rs`), reduced to what the outcomes need: the stream's input, the context of one
//! call, the continuation value, the outcome and the consumption.

use super::super::constants::{MALFORMED, NEEDS_MORE_BYTES, NO_HEADER_PRESENT, PARSED, PARTIALLY_PARSED_BY_DESIGN};
use super::super::representation::Representation;

/// How the container delivers a stream's bytes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Framing {
  /// Packets, never concatenated.
  Packet,
  /// A byte stream cut at arbitrary points, concatenated in order.
  Stream,
}

/// What the container states about a stream for all of its calls.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Input {
  /// How the container delivers the stream's bytes.
  pub(crate) framing: Framing,
  /// The form of those bytes.
  pub(crate) representation: Representation,
}

/// What the container knows of the bytes of one call: in packet mode, the packet's length.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct Context {
  /// The packet's total length, where the container knows it.
  pub(crate) packet_length: Option<usize>,
}

/// The fields of the setup that a later unit needs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Setup {
  /// The number of parameters that the parameters unit holds.
  pub(crate) parameter_count: u8,
  /// The flags.
  pub(crate) flags: u8,
}

/// What the parser established so far, which the next call is given.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct Continuation {
  /// The setup, once read.
  pub(crate) setup: Option<Setup>,
}

/// The outcome of one call.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Outcome {
  /// Every field was read.
  Parsed,
  /// The setup was read, and the parameters cannot be reached without decoding: a final state.
  PartiallyParsedByDesign,
  /// More of the same packet or stream is needed; in packet mode with the packet finished, a later packet.
  NeedsMoreBytes,
  /// Well-formed data without a header; a later packet may carry it.
  NoHeaderPresent,
  /// The bytes contradict the syntax. Running out of bytes never gives this.
  Malformed,
}

impl Outcome {
  /// The outcome's name, as fixtures record it.
  pub(crate) fn name(self) -> &'static str {
    match self {
      Outcome::Parsed => PARSED,
      Outcome::PartiallyParsedByDesign => PARTIALLY_PARSED_BY_DESIGN,
      Outcome::NeedsMoreBytes => NEEDS_MORE_BYTES,
      Outcome::NoHeaderPresent => NO_HEADER_PRESENT,
      Outcome::Malformed => MALFORMED,
    }
  }

  /// Whether no later call changes it.
  pub(crate) fn is_final(self) -> bool {
    matches!(
      self,
      Outcome::Parsed | Outcome::PartiallyParsedByDesign | Outcome::Malformed
    )
  }
}

/// What one call gives.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ParseResult {
  /// The outcome.
  pub(crate) outcome: Outcome,
  /// In stream mode, the leading bytes that never need to be offered again.
  pub(crate) finished_bytes: usize,
  /// What was established, which the next call is given.
  pub(crate) continuation: Continuation,
}
