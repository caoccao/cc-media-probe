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

//! `Assembler`, which holds at most the per-stream header limit of one stream's bytes, its continuation value
//! included, calls the parser in the three situations of plan section 7.4, keeps the continuation value, and drops
//! what the parser finished. No call examines more bytes than the work units and the stream's cap on bytes examined
//! leave, so every field that it establishes was paid for, and a parser that still waits once the cap is reached
//! ends with it named. Its buffer and continuation are required, so each charge may take back the source's optional
//! cache blocks before it names working memory.

use std::mem;

use cc_media_probe::Limit;

use super::Piece;
use super::constants::{EXAMINED_MULTIPLE, GROWTH_FACTOR};
use crate::budget::{Budget, Reclaim};
use crate::codec::{
  CONTINUATION_SIZE, Consumption, Context, Continuation, Framing, Input, Outcome, ParseResult, parse,
};
use crate::report::{HeaderStatus, Reason};
use crate::utilities::{narrow, widen};

/// The header assembler of one stream.
#[derive(Debug)]
pub(crate) struct Assembler {
  input: Input,
  buffer: Vec<u8>,
  continuation: Continuation,
  /// Whether the continuation value's fixed size is charged: from the first offer until the header is final.
  holding: bool,
  stated_total: Option<usize>,
  new_bytes: bool,
  status: Option<HeaderStatus>,
  examined: u64,
  packet_length: Option<usize>,
  packet_finished: bool,
  packets_offered: u64,
}

impl Assembler {
  /// An assembler that holds nothing yet.
  pub(crate) fn new(input: Input) -> Assembler {
    Assembler {
      input,
      buffer: Vec::new(),
      continuation: Continuation::default(),
      holding: false,
      stated_total: None,
      new_bytes: false,
      status: None,
      examined: 0,
      packet_length: None,
      packet_finished: false,
      packets_offered: 0,
    }
  }

  /// Whether the stream still waits for its header.
  pub(crate) fn is_waiting(&self) -> bool {
    self.status.is_none()
  }

  /// The packets, or in stream mode the deliveries, offered so far.
  pub(crate) fn packets_offered(&self) -> u64 {
    self.packets_offered
  }

  /// Where the header ended. A stream that still waits has not been finished.
  pub(crate) fn status(&self) -> HeaderStatus {
    self
      .status
      .clone()
      .unwrap_or(HeaderStatus::NotFound(Reason::TruncatedInput))
  }

  /// Offers one piece, and says whether more of the same packet is wanted. A piece that does not fit under the
  /// header limit is taken as far as it fits; only a parser that needs more than the limit ends with it named.
  pub(crate) fn offer(&mut self, budget: &mut Budget, room: &mut dyn Reclaim, piece: &Piece<'_>) -> bool {
    if self.status.is_some() || (!self.holding && !self.start(budget, room)) {
      return false;
    }
    if piece.starts_packet {
      self.start_packet(piece);
    }
    let limit = narrow(budget.limits().header_bytes_per_stream()).saturating_sub(CONTINUATION_SIZE);
    let mut rest = piece.bytes;
    while !self.packet_finished && self.status.is_none() {
      let taken = rest.len().min(limit.saturating_sub(self.buffer.len()));
      let (head, tail) = rest.split_at_checked(taken).unwrap_or((rest, &[]));
      if !head.is_empty() && !self.hold(budget, room, head, limit) {
        return false;
      }
      rest = tail;
      let limited = !rest.is_empty();
      let ended = self.input.framing == Framing::Packet && piece.ends_packet && !limited;
      let due = ended
        || self
          .stated_total
          .map_or(self.new_bytes, |total| self.buffer.len() >= total);
      if !due {
        if limited {
          self.exceed(budget);
        }
        return !limited;
      }
      if !self.call(budget, ended) {
        break;
      }
      if self.stated_total.is_some_and(|total| total > limit) || (limited && self.buffer.len() >= limit) {
        self.exceed(budget);
        return false;
      }
      if !limited {
        return true;
      }
    }
    false
  }

  /// Ends a stream that still waits: at the end of the input, or when a limit stopped the traversal. The header is
  /// cut short when the setup was read, and not found otherwise. In stream mode the end of the input is the end of
  /// the stream, which the parser is told once before.
  pub(crate) fn finish(&mut self, budget: &mut Budget, reason: Reason) {
    if self.status.is_some() {
      return;
    }
    if self.holding && self.input.framing == Framing::Stream && reason == Reason::TruncatedInput {
      self.call(budget, true);
    }
    if self.status.is_none() {
      self.end(budget, reason);
    }
  }

  /// Ends a stream that still waits because its source failed: not attempted when nothing of it was offered, and
  /// otherwise cut short or not found, by whether the setup was read (plan section 7.4, "End of input, limits").
  pub(crate) fn fail(&mut self, budget: &mut Budget) {
    if self.status.is_some() {
      return;
    }
    if self.packets_offered == 0 {
      self.settle(budget, HeaderStatus::NotAttempted(Reason::SourceFailed));
    } else {
      self.end(budget, Reason::SourceFailed);
    }
  }

  /// Starts holding: the continuation value's fixed size is charged to working memory before any byte is held, and
  /// a header limit that cannot hold it holds nothing.
  fn start(&mut self, budget: &mut Budget, room: &mut dyn Reclaim) -> bool {
    if budget.limits().header_bytes_per_stream() < widen(CONTINUATION_SIZE) {
      self.exceed(budget);
      return false;
    }
    let charged = budget.require(Some(room), |budget| budget.try_charge_memory(widen(CONTINUATION_SIZE)));
    if let Err(limit) = charged {
      self.end(budget, Reason::Limit(limit));
      return false;
    }
    self.holding = true;
    true
  }

  /// A new packet: units are never concatenated across packets, so its bytes start afresh, while the continuation
  /// value stays. In stream mode a new delivery changes nothing but the count.
  fn start_packet(&mut self, piece: &Piece<'_>) {
    self.packets_offered = self.packets_offered.saturating_add(1);
    if self.input.framing == Framing::Packet {
      self.buffer.clear();
      self.stated_total = None;
      self.packet_finished = false;
      self.new_bytes = false;
      self.packet_length = piece.packet_length.map(narrow);
    }
  }

  /// Holds `bytes`, growing the buffer within the limit, charged before it is reserved.
  fn hold(&mut self, budget: &mut Budget, room: &mut dyn Reclaim, bytes: &[u8], limit: usize) -> bool {
    let wanted = self.buffer.len().saturating_add(bytes.len());
    if wanted > self.buffer.capacity() {
      let capacity = self
        .buffer
        .capacity()
        .saturating_mul(GROWTH_FACTOR)
        .min(limit)
        .max(wanted);
      let buffer = &mut self.buffer;
      if let Err(limit) = budget.require(Some(room), |budget| budget.try_grow(buffer, capacity)) {
        self.end(budget, Reason::Limit(limit));
        return false;
      }
    }
    self.buffer.extend_from_slice(bytes);
    self.new_bytes = true;
    true
  }

  /// Calls the parser on no more bytes than the work units and the cap on bytes examined leave, and says whether
  /// the stream still waits for more of this packet or stream. A parser that needs the bytes beyond that allowance
  /// ends with the limit that set it, and so does one that still waits once the cap is reached, since no later call
  /// could examine a byte more; at the stream's end in stream mode the end of the input stopped it instead.
  fn call(&mut self, budget: &mut Budget, ended: bool) -> bool {
    let cap = budget
      .limits()
      .header_bytes_per_stream()
      .saturating_mul(EXAMINED_MULTIPLE);
    let examinable = cap.saturating_sub(self.examined);
    let work = budget.remaining_work();
    let allowance = narrow(examinable.min(work));
    let cut = self.buffer.len() > allowance;
    let context = Context {
      packet_length: self.packet_length,
    };
    let bytes = self.buffer.get(..allowance).unwrap_or(&self.buffer);
    let result = parse(self.input, context, bytes, self.continuation, ended && !cut);
    self.new_bytes = false;
    self.continuation = result.continuation;
    let examined = widen(result.examined);
    self.examined = self.examined.saturating_add(examined);
    if let Err(limit) = budget.charge_work(examined) {
      self.end(budget, Reason::Limit(limit));
      return false;
    }
    if cut && wants_more(&result) {
      if work <= examinable {
        budget.exhaust_work();
        self.end(budget, Reason::Limit(Limit::WorkUnits));
      } else {
        self.exceed(budget);
      }
      return false;
    }
    match result.outcome {
      Outcome::Parsed(header) => self.settle(budget, HeaderStatus::Parsed(header)),
      Outcome::PartiallyParsedByDesign(setup) => self.settle(budget, HeaderStatus::PartiallyParsedByDesign(setup)),
      Outcome::Malformed => self.settle(budget, HeaderStatus::Malformed),
      Outcome::NoHeaderPresent | Outcome::NeedsMoreBytes { .. } => {
        let stream_end = self.input.framing == Framing::Stream && ended;
        if self.examined >= cap && !stream_end {
          self.capped(budget);
          return false;
        }
        self.stated_total = match result.outcome {
          Outcome::NeedsMoreBytes { total } => total,
          _ => None,
        };
        match result.consumption {
          Consumption::Packet { finished } => {
            self.packet_finished = finished;
            !finished
          }
          Consumption::Stream { finished_bytes } => {
            self.buffer.drain(..finished_bytes.min(self.buffer.len()));
            true
          }
        }
      }
    }
  }

  /// A final outcome: the buffer and the continuation value's charge are returned.
  fn settle(&mut self, budget: &mut Budget, status: HeaderStatus) -> bool {
    self.status = Some(status);
    budget.free(mem::take(&mut self.buffer));
    if mem::take(&mut self.holding) {
      budget.release_memory(widen(CONTINUATION_SIZE));
    }
    false
  }

  /// The parser still waits and the bytes examined reached their cap: the work units are named when they ran out at
  /// the same time, as on a tie of the two allowances, and the header limit otherwise.
  fn capped(&mut self, budget: &mut Budget) {
    if budget.remaining_work() == 0 {
      budget.exhaust_work();
      self.end(budget, Reason::Limit(Limit::WorkUnits));
    } else {
      self.exceed(budget);
    }
  }

  /// The parser needs more than the per-stream header limit, or more bytes examined than its cap.
  fn exceed(&mut self, budget: &mut Budget) {
    budget.exhaust_header_bytes();
    self.end(budget, Reason::Limit(Limit::HeaderBytesPerStream));
  }

  /// Ends the stream for `reason`, by whether the setup was read.
  fn end(&mut self, budget: &mut Budget, reason: Reason) {
    let status = match self.continuation.setup {
      Some(setup) => HeaderStatus::PartiallyParsedCutShort(setup, reason),
      None => HeaderStatus::NotFound(reason),
    };
    self.settle(budget, status);
  }
}

/// Whether a call that was not final still needs bytes of the same packet or stream.
fn wants_more(result: &ParseResult) -> bool {
  matches!(result.outcome, Outcome::NeedsMoreBytes { .. })
    && matches!(
      result.consumption,
      Consumption::Stream { .. } | Consumption::Packet { finished: false }
    )
}
