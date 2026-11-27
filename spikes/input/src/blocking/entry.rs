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

//! The five blocking entry points.

use std::fs::File;
use std::io::{Read, Seek};
use std::path::Path;
use std::sync::Arc;

use cc_media_probe::source::PositionalSource;

use crate::companion::{BlockingLookup, BuiltInResolver, CompanionLookup, CompanionResolver, file_numbers};
use crate::executor::poll_once;
use crate::options::Options;
use crate::report::{ProbeError, SourceOutcome, ToyReport};
use crate::source::{BlockCache, BlockingPositional, Forward, InternalSource, Memory, Seekable, Window};
use crate::walker::{PRIMARY, Primary, Walk, probe};

/// Probes bytes in memory, read in place.
///
/// # Errors
///
/// `SourceFailed` when a companion fails.
pub fn probe_bytes(bytes: &[u8], options: &Options) -> Result<ToyReport, ProbeError> {
  run(Memory::new(bytes), options, None, Primary::default())
}

/// Probes a seekable reader from its byte 0, through the block cache. Pass `&mut reader` to keep it.
///
/// # Errors
///
/// `SourceFailed` when the reader or a companion fails.
pub fn probe_reader<Reader: Read + Seek>(reader: Reader, options: &Options) -> Result<ToyReport, ProbeError> {
  let source = BlockCache::new(Seekable(reader), options.block_size.get(), options.cached_blocks.get());
  run(source, options, None, Primary::default())
}

/// Probes a forward-only reader from where it stands, through the sliding window. Pass `&mut reader` to keep it.
///
/// # Errors
///
/// `SourceFailed` when the reader or a companion fails.
pub fn probe_stream<Reader: Read>(reader: Reader, options: &Options) -> Result<ToyReport, ProbeError> {
  run(
    Window::new(Forward(reader), options.window_sizes()),
    options,
    None,
    Primary::default(),
  )
}

/// Probes the caller's own positional source from its position 0. Pass `&mut source` to keep it.
///
/// # Errors
///
/// `SourceFailed` when the source or a companion fails.
pub fn probe_source<Source: PositionalSource>(source: Source, options: &Options) -> Result<ToyReport, ProbeError> {
  let source = BlockCache::new(
    BlockingPositional(source),
    options.block_size.get(),
    options.cached_blocks.get(),
  );
  run(source, options, None, Primary::default())
}

/// Probes the file at `path`, which the built-in resolver takes its trusted root from, and with it the primary
/// file's identity: a custom resolver's tokens are never compared with it (`api.md` section 13).
///
/// # Errors
///
/// `SourceFailed` when the file cannot be opened or read, or a companion fails.
pub fn probe_file(path: impl AsRef<Path>, options: &Options) -> Result<ToyReport, ProbeError> {
  let path = path.as_ref();
  let file = File::open(path).map_err(|cause| ProbeError::SourceFailed {
    source: PRIMARY,
    cause,
    partial: Box::new(Walk::new(options, false).into_report(0, SourceOutcome::Failed)),
  })?;
  let built_in = matches!(options.companions, CompanionLookup::BuiltIn);
  let primary = Primary {
    numbers: built_in.then(|| file_numbers(&file)).flatten(),
    built_in,
    path: Some(path.to_path_buf()),
  };
  let source = BlockCache::new(Seekable(file), options.block_size.get(), options.cached_blocks.get());
  run(source, options, Some(path), primary)
}

/// Runs the probe with the blocking lookup of `options` and polls it once.
fn run<Source: InternalSource>(
  source: Source,
  options: &Options,
  path: Option<&Path>,
  primary: Primary,
) -> Result<ToyReport, ProbeError> {
  let resolver: Option<Arc<dyn CompanionResolver>> = match &options.companions {
    CompanionLookup::Off => None,
    CompanionLookup::BuiltIn => Some(Arc::new(BuiltInResolver::new(path))),
    CompanionLookup::Custom(resolver) => Some(Arc::clone(resolver)),
  };
  poll_once(probe(source, BlockingLookup(resolver), options, primary)).unwrap_or(Err(ProbeError::Stalled))
}
