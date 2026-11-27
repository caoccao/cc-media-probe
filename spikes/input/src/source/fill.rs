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

//! `fill!`, which reads a whole range from the internal source, asking again after each short read. What the source
//! can give at once is read as plain code, and only a read that must wait builds and awaits a future: the spike's
//! measurement showed that an await where nothing waits costs more than the walker's own work (`README.md`
//! section 5).

use std::io;

use cc_media_probe::Limit;

use super::{Delivery, InternalSource};
use crate::budget::Budget;
use crate::utilities::widen;

/// How far a range was filled.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Filled {
  /// Every byte of the range.
  Complete,
  /// The input ended after this many bytes of the range.
  End(usize),
  /// The range, or its rest, can no longer be read (D9).
  Unavailable,
  /// A limit stopped the read after this many bytes of the range.
  Exhausted(usize, Limit),
}

/// A fill whose read failed after `done` bytes of the range had arrived. The walker offers a packet's bytes before
/// the error ends the probe, so the bytes delivered count the same however the pieces are cut (R19).
#[derive(Debug)]
pub(crate) struct Failure {
  /// The bytes of the range that arrived before the error.
  pub(crate) done: usize,
  /// The source's error.
  pub(crate) cause: io::Error,
}

impl From<Failure> for io::Error {
  fn from(failure: Failure) -> io::Error {
    failure.cause
  }
}

/// Where a fill stands after the reads that the source could give at once.
pub(crate) enum Progress {
  /// The fill is over.
  Done(Filled),
  /// This many bytes are filled, and the next read must wait.
  Waiting(usize),
}

/// The reads that the source can give at once.
pub(crate) fn fill_now<Source: InternalSource>(
  source: &mut Source,
  budget: &mut Budget,
  position: u64,
  buffer: &mut [u8],
) -> Progress {
  let mut done = 0_usize;
  while let Some(rest) = buffer.get_mut(done..).filter(|rest| !rest.is_empty()) {
    let at = position.saturating_add(widen(done));
    let Some(delivery) = source.read_now(budget, at, rest) else {
      return Progress::Waiting(done);
    };
    if let Some(filled) = settle(delivery, &mut done) {
      return Progress::Done(filled);
    }
  }
  Progress::Done(Filled::Complete)
}

/// The rest of a fill, from `done` bytes on, through reads that may wait.
pub(crate) async fn fill_rest<Source: InternalSource>(
  source: &mut Source,
  budget: &mut Budget,
  position: u64,
  buffer: &mut [u8],
  mut done: usize,
) -> Result<Filled, Failure> {
  while let Some(rest) = buffer.get_mut(done..).filter(|rest| !rest.is_empty()) {
    let at = position.saturating_add(widen(done));
    let delivery = source
      .read_at(budget, at, rest)
      .await
      .map_err(|cause| Failure { done, cause })?;
    if let Some(filled) = settle(delivery, &mut done) {
      return Ok(filled);
    }
  }
  Ok(Filled::Complete)
}

/// Counts a delivery into `done`, or gives the fill's end.
pub(crate) fn settle(delivery: Delivery, done: &mut usize) -> Option<Filled> {
  match delivery {
    Delivery::Bytes(0) => Some(Filled::End(*done)),
    Delivery::Bytes(count) => {
      *done = done.saturating_add(count);
      None
    }
    Delivery::Unavailable => Some(Filled::Unavailable),
    Delivery::Exhausted(limit) => Some(Filled::Exhausted(*done, limit)),
  }
}

/// Fills `buffer` from `position` on, inside an async function, and gives `Result<Filled, Failure>`: the reads that
/// the source can give at once run as plain code, and only a read that must wait builds and awaits `fill_rest`.
macro_rules! fill {
  ($source:expr, $budget:expr, $position:expr, $buffer:expr) => {
    match $crate::source::fill_now($source, $budget, $position, $buffer) {
      $crate::source::Progress::Done(filled) => Ok(filled),
      $crate::source::Progress::Waiting(done) => {
        $crate::source::fill_rest($source, $budget, $position, $buffer, done).await
      }
    }
  };
}

pub(crate) use fill;
