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

//! The hand-written probe's entry points.

use std::io::{Read, Seek};

use super::source::{BlockingSource, HandCache, HandMemory};
use super::walk::run;
use crate::options::Options;
use crate::report::{ProbeError, SourceOutcome, ToyReport};
use crate::walker::{PRIMARY, Walk};

/// Probes bytes in memory, as `blocking::probe_bytes` does without companions.
///
/// # Errors
///
/// None in practice: bytes in memory do not fail.
pub fn probe_bytes(bytes: &[u8], options: &Options) -> Result<ToyReport, ProbeError> {
  probe(HandMemory::new(bytes), options)
}

/// Probes a seekable reader, as `blocking::probe_reader` does without companions.
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

/// Runs the probe over `source`.
fn probe<Source: BlockingSource>(mut source: Source, options: &Options) -> Result<ToyReport, ProbeError> {
  let mut walk = Walk::new(options, false);
  let result = run(&mut source, &mut walk);
  let delivered = source.delivered();
  source.release(&mut walk.budget);
  match result {
    Ok(()) => Ok(walk.into_report(delivered, SourceOutcome::Opened)),
    Err(cause) => Err(ProbeError::SourceFailed {
      source: PRIMARY,
      cause,
      partial: Box::new(walk.into_report(delivered, SourceOutcome::Failed)),
    }),
  }
}
