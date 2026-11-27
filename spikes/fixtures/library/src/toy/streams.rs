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

//! The streams of a toy walk (`spikes/input/README.md` section 2): each declaration's framing, representation and
//! codec, and an assembler for the toy codec alone, which offers each packet of its stream to the parser with the
//! continuation of the last call, until the parser reaches a final outcome. In packet mode each packet is one call
//! with its known length, and a packet that the input's end cut short is a call with the bytes that arrived, which
//! has not ended; in stream mode the bytes that the parser did not finish are kept and offered again with the next
//! packet's, and the end of the walk is the end of the stream.

use super::codec::{Context, Continuation, Framing, Input, Outcome, parse};
use super::constants::{
  CODE_LENGTH, CUT_SHORT, NOT_FOUND, PACKET_MODE, STREAM_BODY_LENGTH, STREAM_HEAD_LENGTH, STREAM_MODE, TOY_CODEC,
  UNSUPPORTED,
};
use super::representation::Representation;

/// Where a stream's header stands.
#[derive(Debug)]
enum State {
  /// A stream of another codec, which gets no assembler.
  Unsupported,
  /// The header is still to come: what the parser established, and in stream mode the bytes it has not finished.
  Waiting {
    /// What the last call established.
    continuation: Continuation,
    /// The bytes kept for the next call, in stream mode.
    kept: Vec<u8>,
  },
  /// The parser reached a final outcome.
  Final(Outcome),
}

/// A declared stream.
#[derive(Debug)]
struct Declared {
  /// The stream number.
  number: u8,
  /// The framing and the representation.
  input: Input,
  /// Where its header stands.
  state: State,
}

/// The streams that the walk has declared, in order.
#[derive(Debug, Default)]
pub(crate) struct Streams {
  /// The declarations.
  declared: Vec<Declared>,
}

impl Streams {
  /// Declares a stream from a declaration's body. A body of another length, an unknown framing or representation,
  /// or a number already declared is a bad body, and declares nothing.
  pub(crate) fn declare(&mut self, body: &[u8]) {
    let Some((&[number, framing, representation], codec)) = body.split_first_chunk::<STREAM_HEAD_LENGTH>() else {
      return;
    };
    let framing = match framing {
      PACKET_MODE => Framing::Packet,
      STREAM_MODE => Framing::Stream,
      _ => return,
    };
    let (Some(representation), Ok(codec)) = (
      Representation::from_code(representation),
      <[u8; CODE_LENGTH]>::try_from(codec),
    ) else {
      return;
    };
    if body.len() != STREAM_BODY_LENGTH || self.declared.iter().any(|stream| stream.number == number) {
      return;
    }
    let state = if codec == TOY_CODEC {
      State::Waiting {
        continuation: Continuation::default(),
        kept: Vec::new(),
      }
    } else {
      State::Unsupported
    };
    self.declared.push(Declared {
      number,
      input: Input {
        framing,
        representation,
      },
      state,
    });
  }

  /// Offers what arrived of a packet's body, its stream number and its payload, to the stream that it names, where
  /// `length` is the body's length as its header states it: a packet that the input's end cut short offers the bytes
  /// before the end, and has not ended. A packet of a stream that no declaration names is skipped.
  pub(crate) fn offer(&mut self, body: &[u8], length: usize) {
    let Some((number, payload)) = body.split_first() else {
      return;
    };
    let packet_length = length.saturating_sub(1);
    let ended = payload.len() == packet_length;
    if let Some(stream) = self.declared.iter_mut().find(|stream| stream.number == *number) {
      stream.offer(payload, packet_length, ended);
    }
  }

  /// Each stream's header state at the end of the walk, in the order of the declarations.
  pub(crate) fn finish(self) -> Vec<&'static str> {
    self.declared.into_iter().map(Declared::finish).collect()
  }
}

impl Declared {
  /// Offers what arrived of one packet's payload, of the length that the container knows, and whether it ended.
  fn offer(&mut self, payload: &[u8], packet_length: usize, ended: bool) {
    let State::Waiting { continuation, kept } = &mut self.state else {
      return;
    };
    let result = match self.input.framing {
      Framing::Packet => {
        let context = Context {
          packet_length: Some(packet_length),
        };
        parse(self.input, context, payload, *continuation, ended)
      }
      Framing::Stream => {
        kept.extend_from_slice(payload);
        let result = parse(self.input, Context::default(), kept, *continuation, false);
        kept.drain(..result.finished_bytes.min(kept.len()));
        result
      }
    };
    *continuation = result.continuation;
    if result.outcome.is_final() {
      self.state = State::Final(result.outcome);
    }
  }

  /// The header state at the end of the walk: in stream mode, the last call is told that the stream ended; a header
  /// still to come is cut short where the setup was read, and not found otherwise.
  fn finish(self) -> &'static str {
    match self.state {
      State::Unsupported => UNSUPPORTED,
      State::Final(outcome) => outcome.name(),
      State::Waiting { continuation, kept } => {
        let last = (self.input.framing == Framing::Stream)
          .then(|| parse(self.input, Context::default(), &kept, continuation, true))
          .filter(|result| result.outcome.is_final());
        match last {
          Some(result) => result.outcome.name(),
          None if continuation.setup.is_some() => CUT_SHORT,
          None => NOT_FOUND,
        }
      }
    }
  }
}
