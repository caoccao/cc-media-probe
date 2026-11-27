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

//! The async driver of the state machine and its entry point for a caller's async positional source, through the
//! block cache. A read that the source can give at once is made as plain code, and only one that must wait is
//! awaited, as the async walker's `fill!` does.

use std::io;

use cc_media_probe::asynchronous::AsyncPositionalSource;

use super::blocking::conclude;
use super::state_machine::{Machine, Step};
use crate::options::Options;
use crate::report::{ProbeError, ToyReport};
use crate::source::{AsyncCustom, BlockCache, InternalSource};
use crate::walker::{PRIMARY, Walk};

/// Probes the caller's own async positional source with the state machine, as `asynchronous::probe_source` does
/// without companions.
///
/// # Errors
///
/// `SourceFailed` when the source fails.
pub async fn probe_source<Source: AsyncPositionalSource + Unpin>(
  source: Source,
  options: &Options,
) -> Result<ToyReport, ProbeError> {
  let mut source = BlockCache::new(
    AsyncCustom(source),
    options.block_size.get(),
    options.cached_blocks.get(),
  );
  let mut walk = Walk::new(options, false);
  let result = drive(&mut source, &mut walk).await;
  let delivered = source.delivered();
  source.release(&mut walk.budget);
  conclude(walk, delivered, result)
}

/// Feeds the machine with reads, awaiting only those that must wait, until the walk is over or a read fails, and
/// ends the streams.
async fn drive<Source: InternalSource>(source: &mut Source, walk: &mut Walk<'_>) -> io::Result<()> {
  let mut machine = Machine::new();
  let mut step = Step::Read;
  let mut result = Ok(());
  while step == Step::Read {
    let (position, buffer) = machine.target();
    let delivery = match source.read_now(&mut walk.budget, position, buffer) {
      Some(delivery) => Ok(delivery),
      None => source.read_at(&mut walk.budget, position, buffer).await,
    };
    match delivery {
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
