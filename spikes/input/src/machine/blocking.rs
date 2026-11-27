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

//! The blocking driver of the state machine and its entry points. It reads through the hand-written probe's own
//! blocking sources, bytes in place and a seekable reader through the block bookkeeping, so the overhead
//! measurement compares only the walker's form.

use std::io::{self, Read, Seek};

use super::state_machine::{Machine, Step};
use crate::handwritten::{BlockingSource, HandCache, HandMemory};
use crate::options::Options;
use crate::report::{ProbeError, SourceOutcome, ToyReport};
use crate::walker::{PRIMARY, Walk};

/// Probes bytes in memory with the state machine, as `blocking::probe_bytes` does without companions.
///
/// # Errors
///
/// None in practice: bytes in memory do not fail.
pub fn probe_bytes(bytes: &[u8], options: &Options) -> Result<ToyReport, ProbeError> {
  probe(HandMemory::new(bytes), options)
}

/// Probes a seekable reader with the state machine, as `blocking::probe_reader` does without companions.
///
/// # Errors
///
/// `SourceFailed` when the reader fails.
pub fn probe_reader<Reader: Read + Seek>(reader: Reader, options: &Options) -> Result<ToyReport, ProbeError> {
  probe(
    HandCache::new(reader, options.block_size.get(), options.cached_blocks.get()),
    options,
  )
}

/// Runs the machine over `source`.
fn probe<Source: BlockingSource>(mut source: Source, options: &Options) -> Result<ToyReport, ProbeError> {
  let mut walk = Walk::new(options, false);
  let result = drive(&mut source, &mut walk);
  let delivered = source.delivered();
  source.release(&mut walk.budget);
  conclude(walk, delivered, result)
}

/// Feeds the machine with blocking reads until the walk is over or a read fails, and ends the streams.
fn drive<Source: BlockingSource>(source: &mut Source, walk: &mut Walk<'_>) -> io::Result<()> {
  let mut machine = Machine::new();
  let mut step = Step::Read;
  let mut result = Ok(());
  while step == Step::Read {
    let (position, buffer) = machine.target();
    match source.read_at(&mut walk.budget, position, buffer) {
      Ok(delivery) => step = machine.deliver(walk, delivery, &mut *source),
      Err(error) => {
        machine.fail(walk, &mut *source);
        result = Err(error);
        break;
      }
    }
  }
  machine.finish(walk);
  walk.end_streams(PRIMARY, &result);
  result
}

/// The report, or the error that carries it, once the source is released.
pub(super) fn conclude(walk: Walk<'_>, delivered: u64, result: io::Result<()>) -> Result<ToyReport, ProbeError> {
  match result {
    Ok(()) => Ok(walk.into_report(delivered, SourceOutcome::Opened)),
    Err(cause) => Err(ProbeError::SourceFailed {
      source: PRIMARY,
      cause,
      partial: Box::new(walk.into_report(delivered, SourceOutcome::Failed)),
    }),
  }
}
