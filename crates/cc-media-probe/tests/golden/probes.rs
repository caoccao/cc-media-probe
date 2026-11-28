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

//! A fixture probed in one case through each adapter under equal conditions (`docs/design/test-kit.md` section
//! 4.1): the blocking adapters, a file, a seekable reader, bytes, a custom positional source and a forward-only
//! reader, and the async ones, a file, a seekable and a forward-only reader on Tokio, the same two readers on
//! futures-io, and an async custom source. Every adapter's prober starts from `fuzzing_entry_points::prober`, the
//! one that the `probe` target runs, with lookup off as the API's is, and the case's limits. Every adapter is given
//! the fixture's file name: the file adapters as their file's name alone, without a hint, so that a probe of a file
//! that stopped taking its name from its path fails the parity, as the review of plan Phase 3, task 4, found; and
//! the others as the hint. A hint-only case gives those others its hint, and a nameless case none, without the file
//! adapters. The file adapters' retained-data limit is raised by their path's charge, so that the rest of their probe
//! has the others' budget exactly. The readers and the custom sources deliver in the case's pieces and count their
//! reads, and the async ones are pending once before each read. Every reader and source borrows the fixture's
//! bytes, which none copies, as the third review of plan Phase 3, task 4, asked. Each result's text reads back
//! through the library's strict reader before it is kept (`common::round_trip`).

use std::fs;
use std::io::Cursor;
use std::path::Path;

use cc_media_probe::fuzzing_entry_points::prober;
use cc_media_probe::{Error, Limits, MediaReport, Prober};
use cc_media_probe_testkit::fixture::Fixture;
use cc_media_probe_testkit::readers::{CountingReader, Counts, FaultingReader};
use futures_lite::future::block_on;
use serde_json::Value;

use crate::common::conditions::{Conditions, Naming};
use crate::common::memory::{AsyncMemory, Memory};
use crate::common::pending::Pending;
use crate::common::round_trip::read_back;

/// A probe of a fixture through one adapter.
pub struct Probed {
  /// The adapter's name, its member in the files keyed by adapter.
  pub adapter: &'static str,
  /// Whether the adapter is forward only.
  pub forward_only: bool,
  /// The report or error record in the JSON form.
  pub record: Value,
  /// The reads that the adapter's counter saw, where the test counts them. The file adapters' are counted by the
  /// repository-only counter of `docs/design/test-kit.md` section 7, which the I/O layer of plan Phase 3, task 6,
  /// brings.
  pub counts: Option<Counts>,
  /// The limits that the adapter was given: the case's, the file adapters' raised by their file's path.
  pub limits: Limits,
  /// Whether the adapter was given the probed file's path, which its primary input's entry then holds; the others
  /// are given none.
  pub given_path: bool,
}

/// A reader of the fixture's bytes, which it borrows, that delivers in the case's pieces and counts its reads.
type Reader<'bytes> = CountingReader<FaultingReader<Cursor<&'bytes [u8]>>>;

/// The record of a probe's result of fixture `name` in the JSON form, once its text reads back through the
/// library's strict reader as the record that it is, or a failure that names the fixture.
fn record(name: &str, result: Result<MediaReport, Error>) -> Value {
  let read = match result {
    Ok(report) => read_back(&report),
    Err(error) => read_back(&error.to_record()),
  };
  read.unwrap_or_else(|message| panic!("{name}: {message}"))
}

/// A reader of `bytes` that delivers in the case's pieces and counts its reads.
fn reader<'bytes>(bytes: &'bytes [u8], conditions: &Conditions) -> Reader<'bytes> {
  CountingReader::new(FaultingReader::new(Cursor::new(bytes)).with_pieces(&conditions.pieces))
}

/// The limits of the file adapters: the case's, with the retained-data limit raised by the charge of the probed
/// file's path, which the library charges before anything else that the probe retains. A companion that built-in
/// lookup opens is charged within the case's limit, as any companion is.
pub fn raised(limits: &Limits, charge: u64) -> Limits {
  limits
    .clone()
    .with_retained_data(limits.retained_data().saturating_add(charge))
}

/// The fixture probed through each adapter under `conditions`: a file written into `folder` under the fixture's
/// file name, unless the case gives another name or none, whose adapters' retained-data limit is raised by
/// `charge`, its path's; and every other adapter, blocking and async.
pub fn probes(fixture: &Fixture, conditions: &Conditions, folder: &Path, charge: u64) -> Vec<Probed> {
  let (name, bytes) = (fixture.name.as_str(), &fixture.body.bytes);
  let hint = match &conditions.naming {
    Naming::FileName => Some(fixture.file_name()),
    Naming::Hint(hint) => Some(hint.as_str()),
    Naming::Absent => None,
  };
  let mut results = Vec::new();
  if conditions.has_file() {
    let path = folder.join(fixture.file_name());
    fs::write(&path, bytes).unwrap();
    let limits = raised(&conditions.limits, charge);
    let filed = prober(None).with_limits(limits.clone());
    let file = |adapter, record| Probed {
      given_path: true,
      ..probed(adapter, false, record, None, &limits)
    };
    results.push(file("file", record(name, filed.probe_file(&path))));
    results.push(file(
      "tokio_file",
      record(name, runtime().block_on(filed.tokio().probe_file(&path))),
    ));
  }
  let prober = prober(hint.map(Into::into)).with_limits(conditions.limits.clone());
  results.extend(blocking(&prober, name, bytes, conditions));
  results.extend(asynchronous(&prober, name, bytes, conditions));
  results
}

/// The probe through `adapter` that gave `record`, under `limits`, with the reads that its counter saw, and no path.
fn probed(adapter: &'static str, forward_only: bool, record: Value, counts: Option<Counts>, limits: &Limits) -> Probed {
  Probed {
    adapter,
    forward_only,
    record,
    counts,
    limits: limits.clone(),
    given_path: false,
  }
}

/// The probes of fixture `name`, whose bytes are `bytes`, through each blocking adapter but the file.
fn blocking(prober: &Prober, name: &str, bytes: &[u8], conditions: &Conditions) -> [Probed; 4] {
  let limits = &conditions.limits;
  let mut seekable = reader(bytes, conditions);
  let read = record(name, prober.probe_reader(&mut seekable));
  let mut forward = reader(bytes, conditions);
  let streamed = record(name, prober.probe_stream(&mut forward));
  let mut memory = Memory::new(bytes, &conditions.pieces);
  let sourced = record(name, prober.probe_source(&mut memory));
  [
    probed("bytes", false, record(name, prober.probe_bytes(bytes)), None, limits),
    probed("reader", false, read, Some(seekable.counts()), limits),
    probed("source", false, sourced, Some(memory.counts()), limits),
    probed("stream", true, streamed, Some(forward.counts()), limits),
  ]
}

/// The probes of fixture `name`, whose bytes are `bytes`, through each async adapter but the file.
fn asynchronous(prober: &Prober, name: &str, bytes: &[u8], conditions: &Conditions) -> [Probed; 5] {
  let limits = &conditions.limits;
  let counted = |reader: &Pending<Reader<'_>>| Some(reader.get_ref().counts());
  let mut tokio_reader = Pending::new(reader(bytes, conditions));
  let mut tokio_stream = Pending::new(reader(bytes, conditions));
  let runtime = runtime();
  let tokio_read = record(name, runtime.block_on(prober.tokio().probe_reader(&mut tokio_reader)));
  let tokio_streamed = record(name, runtime.block_on(prober.tokio().probe_stream(&mut tokio_stream)));
  let mut futures_reader = Pending::new(reader(bytes, conditions));
  let futures_read = record(name, block_on(prober.futures().probe_reader(&mut futures_reader)));
  let mut futures_stream = Pending::new(reader(bytes, conditions));
  let futures_streamed = record(name, block_on(prober.futures().probe_stream(&mut futures_stream)));
  let mut source = AsyncMemory::new(bytes, &conditions.pieces);
  let sourced = record(name, block_on(prober.asynchronous().probe_source(&mut source)));
  [
    probed("tokio_reader", false, tokio_read, counted(&tokio_reader), limits),
    probed("tokio_stream", true, tokio_streamed, counted(&tokio_stream), limits),
    probed("futures_reader", false, futures_read, counted(&futures_reader), limits),
    probed(
      "futures_stream",
      true,
      futures_streamed,
      counted(&futures_stream),
      limits,
    ),
    probed("async_source", false, sourced, Some(source.counts()), limits),
  ]
}

/// A runtime of Tokio on the current thread, which runs the Tokio adapters' probes.
fn runtime() -> tokio::runtime::Runtime {
  tokio::runtime::Builder::new_current_thread().build().unwrap()
}
