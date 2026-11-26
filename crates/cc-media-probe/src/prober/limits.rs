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

//! `Limits`, one value for each counter of the shared budget (`api.md` section 4.5; plan section 7.5).

use super::Limit;
use super::constants::{
  DEFAULT_BYTES_READ, DEFAULT_COMPANION_REQUESTS, DEFAULT_DECOMPRESSED_OUTPUT, DEFAULT_DIAGNOSTICS,
  DEFAULT_HEADER_BYTES_PER_STREAM, DEFAULT_NESTING_DEPTH, DEFAULT_RETAINED_DATA, DEFAULT_RETAINED_PAYLOADS,
  DEFAULT_SOURCES_OPENED, DEFAULT_STREAMS, DEFAULT_WORK_UNITS, DEFAULT_WORKING_MEMORY,
};

/// One value for each counter of the shared budget that every source, child probe and companion of a probe
/// shares (plan section 7.5). No counter is ever exceeded, and every limit accepts any value from 0 to `u64::MAX`.
///
/// `Limits::new()` and `Limits::default()` hold the proposed defaults, which gate G3.3 confirms. Once a default
/// has shipped, a later release raises it only in a minor release and lowers it only to fix a security problem.
/// Each counter has a getter, such as `bytes_read()`, and a setter, such as `with_bytes_read(value)`; `get()` and
/// `with()` reach a counter by its [`Limit`].
///
/// ```
/// use cc_media_probe::{Limit, Limits};
///
/// let limits = Limits::new().with_bytes_read(16 * 1024 * 1024).with(&Limit::Diagnostics, 100);
/// assert_eq!(limits.bytes_read(), 16 * 1024 * 1024);
/// assert_eq!(limits.get(&Limit::Diagnostics), Some(100));
/// assert_eq!(limits.working_memory(), Limits::default().working_memory());
/// println!(
///     "{} {} {} {} {} {} {} {} {} {}",
///     limits.header_bytes_per_stream(),
///     limits.retained_data(),
///     limits.retained_payloads(),
///     limits.decompressed_output(),
///     limits.work_units(),
///     limits.nesting_depth(),
///     limits.streams(),
///     limits.diagnostics(),
///     limits.companion_requests(),
///     limits.sources_opened(),
/// );
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct Limits {
  bytes_read: u64,
  working_memory: u64,
  header_bytes_per_stream: u64,
  retained_data: u64,
  retained_payloads: u64,
  decompressed_output: u64,
  work_units: u64,
  nesting_depth: u64,
  streams: u64,
  diagnostics: u64,
  companion_requests: u64,
  sources_opened: u64,
}

impl Limits {
  /// The proposed defaults of plan section 7.5, the same value as `Limits::default()`.
  #[must_use]
  pub fn new() -> Limits {
    Limits {
      bytes_read: DEFAULT_BYTES_READ,
      working_memory: DEFAULT_WORKING_MEMORY,
      header_bytes_per_stream: DEFAULT_HEADER_BYTES_PER_STREAM,
      retained_data: DEFAULT_RETAINED_DATA,
      retained_payloads: DEFAULT_RETAINED_PAYLOADS,
      decompressed_output: DEFAULT_DECOMPRESSED_OUTPUT,
      work_units: DEFAULT_WORK_UNITS,
      nesting_depth: DEFAULT_NESTING_DEPTH,
      streams: DEFAULT_STREAMS,
      diagnostics: DEFAULT_DIAGNOSTICS,
      companion_requests: DEFAULT_COMPANION_REQUESTS,
      sources_opened: DEFAULT_SOURCES_OPENED,
    }
  }

  /// The value of the counter that `limit` names, or `None` for a limit that a newer version wrote.
  #[must_use]
  pub fn get(&self, limit: &Limit) -> Option<u64> {
    let mut copy = self.clone();
    copy.counter(limit).map(|value| *value)
  }

  /// The limits with the counter that `limit` names set to `value`; unchanged for a limit that a newer version
  /// wrote.
  #[must_use]
  pub fn with(mut self, limit: &Limit, value: u64) -> Limits {
    if let Some(counter) = self.counter(limit) {
      *counter = value;
    }
    self
  }

  /// The bytes read from every source together, including bytes read only to skip forward. 64 MiB by default.
  #[must_use]
  pub fn bytes_read(&self) -> u64 {
    self.bytes_read
  }

  /// The limits with `bytes_read()` set to `value`.
  #[must_use]
  pub fn with_bytes_read(self, value: u64) -> Limits {
    self.with(&Limit::BytesRead, value)
  }

  /// The working memory that the probe's buffers hold at any moment, in bytes. 32 MiB by default.
  #[must_use]
  pub fn working_memory(&self) -> u64 {
    self.working_memory
  }

  /// The limits with `working_memory()` set to `value`.
  #[must_use]
  pub fn with_working_memory(self, value: u64) -> Limits {
    self.with(&Limit::WorkingMemory, value)
  }

  /// The bytes held for one header: a stream's codec header, or the image header of one attachment. 1 MiB by
  /// default.
  #[must_use]
  pub fn header_bytes_per_stream(&self) -> u64 {
    self.header_bytes_per_stream
  }

  /// The limits with `header_bytes_per_stream()` set to `value`.
  #[must_use]
  pub fn with_header_bytes_per_stream(self, value: u64) -> Limits {
    self.with(&Limit::HeaderBytesPerStream, value)
  }

  /// The data that the report keeps, other than attachment payloads, in bytes. 16 MiB by default.
  #[must_use]
  pub fn retained_data(&self) -> u64 {
    self.retained_data
  }

  /// The limits with `retained_data()` set to `value`.
  #[must_use]
  pub fn with_retained_data(self, value: u64) -> Limits {
    self.with(&Limit::RetainedData, value)
  }

  /// The attachment payloads that the report keeps, in bytes. 64 MiB by default.
  #[must_use]
  pub fn retained_payloads(&self) -> u64 {
    self.retained_payloads
  }

  /// The limits with `retained_payloads()` set to `value`.
  #[must_use]
  pub fn with_retained_payloads(self, value: u64) -> Limits {
    self.with(&Limit::RetainedPayloads, value)
  }

  /// The output of every inflate operation together, in bytes. 16 MiB by default.
  #[must_use]
  pub fn decompressed_output(&self) -> u64 {
    self.decompressed_output
  }

  /// The limits with `decompressed_output()` set to `value`.
  #[must_use]
  pub fn with_decompressed_output(self, value: u64) -> Limits {
    self.with(&Limit::DecompressedOutput, value)
  }

  /// The units of work, a bound on the loops over the file's structure. 2,000,000 by default.
  #[must_use]
  pub fn work_units(&self) -> u64 {
    self.work_units
  }

  /// The limits with `work_units()` set to `value`.
  #[must_use]
  pub fn with_work_units(self, value: u64) -> Limits {
    self.with(&Limit::WorkUnits, value)
  }

  /// The depth of nested structures, in levels. 64 by default.
  #[must_use]
  pub fn nesting_depth(&self) -> u64 {
    self.nesting_depth
  }

  /// The limits with `nesting_depth()` set to `value`.
  #[must_use]
  pub fn with_nesting_depth(self, value: u64) -> Limits {
    self.with(&Limit::NestingDepth, value)
  }

  /// The stream declarations that one report keeps. 4,096 by default.
  #[must_use]
  pub fn streams(&self) -> u64 {
    self.streams
  }

  /// The limits with `streams()` set to `value`.
  #[must_use]
  pub fn with_streams(self, value: u64) -> Limits {
    self.with(&Limit::Streams, value)
  }

  /// The diagnostics that the report keeps; beyond them, diagnostics are counted, not stored. 1,000 by default.
  #[must_use]
  pub fn diagnostics(&self) -> u64 {
    self.diagnostics
  }

  /// The limits with `diagnostics()` set to `value`.
  #[must_use]
  pub fn with_diagnostics(self, value: u64) -> Limits {
    self.with(&Limit::Diagnostics, value)
  }

  /// The distinct requests that one probe puts to a companion resolver, whatever their result. 256 by default.
  #[must_use]
  pub fn companion_requests(&self) -> u64 {
    self.companion_requests
  }

  /// The limits with `companion_requests()` set to `value`.
  #[must_use]
  pub fn with_companion_requests(self, value: u64) -> Limits {
    self.with(&Limit::CompanionRequests, value)
  }

  /// The companion sources that one probe opens. 64 by default.
  #[must_use]
  pub fn sources_opened(&self) -> u64 {
    self.sources_opened
  }

  /// The limits with `sources_opened()` set to `value`.
  #[must_use]
  pub fn with_sources_opened(self, value: u64) -> Limits {
    self.with(&Limit::SourcesOpened, value)
  }

  /// The counter that `limit` names, or `None` for a limit that a newer version wrote.
  fn counter(&mut self, limit: &Limit) -> Option<&mut u64> {
    match limit {
      Limit::BytesRead => Some(&mut self.bytes_read),
      Limit::WorkingMemory => Some(&mut self.working_memory),
      Limit::HeaderBytesPerStream => Some(&mut self.header_bytes_per_stream),
      Limit::RetainedData => Some(&mut self.retained_data),
      Limit::RetainedPayloads => Some(&mut self.retained_payloads),
      Limit::DecompressedOutput => Some(&mut self.decompressed_output),
      Limit::WorkUnits => Some(&mut self.work_units),
      Limit::NestingDepth => Some(&mut self.nesting_depth),
      Limit::Streams => Some(&mut self.streams),
      Limit::Diagnostics => Some(&mut self.diagnostics),
      Limit::CompanionRequests => Some(&mut self.companion_requests),
      Limit::SourcesOpened => Some(&mut self.sources_opened),
      Limit::Unrecognized(_) => None,
    }
  }
}

impl Default for Limits {
  /// The proposed defaults, the same value as `Limits::new()`.
  fn default() -> Limits {
    Limits::new()
  }
}

#[cfg(test)]
mod tests {
  use super::Limits;
  use crate::Limit;
  use crate::model::UnrecognizedValue;

  const EVERY_LIMIT: [Limit; 12] = [
    Limit::BytesRead,
    Limit::WorkingMemory,
    Limit::HeaderBytesPerStream,
    Limit::RetainedData,
    Limit::RetainedPayloads,
    Limit::DecompressedOutput,
    Limit::WorkUnits,
    Limit::NestingDepth,
    Limit::Streams,
    Limit::Diagnostics,
    Limit::CompanionRequests,
    Limit::SourcesOpened,
  ];

  #[test]
  fn holds_the_proposed_defaults() {
    let limits = Limits::new();
    assert_eq!(limits, Limits::default());
    let defaults: Vec<Option<u64>> = EVERY_LIMIT.iter().map(|limit| limits.get(limit)).collect();
    assert_eq!(
      defaults,
      [
        Some(64 * 1024 * 1024),
        Some(32 * 1024 * 1024),
        Some(1024 * 1024),
        Some(16 * 1024 * 1024),
        Some(64 * 1024 * 1024),
        Some(16 * 1024 * 1024),
        Some(2_000_000),
        Some(64),
        Some(4_096),
        Some(1_000),
        Some(256),
        Some(64),
      ]
    );
  }

  #[test]
  fn sets_each_counter_through_its_own_setter() {
    let limits = Limits::new()
      .with_bytes_read(1)
      .with_working_memory(2)
      .with_header_bytes_per_stream(3)
      .with_retained_data(4)
      .with_retained_payloads(5)
      .with_decompressed_output(6)
      .with_work_units(7)
      .with_nesting_depth(8)
      .with_streams(9)
      .with_diagnostics(10)
      .with_companion_requests(11)
      .with_sources_opened(12);
    let getters = [
      limits.bytes_read(),
      limits.working_memory(),
      limits.header_bytes_per_stream(),
      limits.retained_data(),
      limits.retained_payloads(),
      limits.decompressed_output(),
      limits.work_units(),
      limits.nesting_depth(),
      limits.streams(),
      limits.diagnostics(),
      limits.companion_requests(),
      limits.sources_opened(),
    ];
    assert_eq!(getters, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
    for (limit, value) in EVERY_LIMIT.iter().zip(1..) {
      assert_eq!(limits.get(limit), Some(value));
    }
  }

  #[test]
  fn reaches_each_counter_by_name() {
    for limit in &EVERY_LIMIT {
      let zero = Limits::new().with(limit, 0);
      let maximum = Limits::new().with(limit, u64::MAX);
      assert_eq!(zero.get(limit), Some(0));
      assert_eq!(maximum.get(limit), Some(u64::MAX));
      for other in EVERY_LIMIT.iter().filter(|other| *other != limit) {
        assert_eq!(zero.get(other), Limits::new().get(other));
      }
    }
  }

  #[test]
  fn leaves_the_limits_unchanged_for_a_newer_limit() {
    let newer = Limit::Unrecognized(UnrecognizedValue::sample("seconds"));
    assert_eq!(Limits::new().get(&newer), None);
    assert_eq!(Limits::new().with(&newer, 0), Limits::new());
  }
}
