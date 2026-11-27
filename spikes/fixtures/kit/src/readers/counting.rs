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

//! A reader that counts what it is asked for (plan section 9.4): read calls, bytes delivered and seeks, which the
//! adapter part of an expectation holds.

use std::io::{self, Read, Seek, SeekFrom};

/// What a `CountingReader` was asked for.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Counts {
  /// The calls of `read`.
  pub read_calls: u64,
  /// The bytes that `read` delivered.
  pub delivered: u64,
  /// The calls of `seek`.
  pub seeks: u64,
}

/// A reader that counts its calls and the bytes it delivers, around any standard-library reader.
#[derive(Debug)]
pub struct CountingReader<Inner> {
  /// The reader counted.
  inner: Inner,
  /// What it was asked for so far.
  counts: Counts,
}

impl Counts {
  /// Counts one read call that delivered `delivered` bytes.
  pub fn record(&mut self, delivered: usize) {
    self.read_calls = self.read_calls.saturating_add(1);
    let delivered_count = u64::try_from(delivered).unwrap_or(u64::MAX);
    self.delivered = self.delivered.saturating_add(delivered_count);
  }
}

impl<Inner> CountingReader<Inner> {
  /// Counts the calls to `inner`.
  pub fn new(inner: Inner) -> CountingReader<Inner> {
    CountingReader {
      inner,
      counts: Counts::default(),
    }
  }

  /// What the reader was asked for so far.
  pub fn counts(&self) -> Counts {
    self.counts
  }
}

impl<Inner: Read> Read for CountingReader<Inner> {
  fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
    let delivered = self.inner.read(buffer);
    self.counts.record(*delivered.as_ref().unwrap_or(&0));
    delivered
  }
}

impl<Inner: Seek> Seek for CountingReader<Inner> {
  fn seek(&mut self, position: SeekFrom) -> io::Result<u64> {
    self.counts.seeks = self.counts.seeks.saturating_add(1);
    self.inner.seek(position)
  }
}
