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
//! adapter part of an expectation holds. A custom source of the library's tests counts its own reads with `Counts`.

use std::io::{self, Read, Seek, SeekFrom};

/// What a `CountingReader` was asked for.
///
/// ```
/// use cc_media_probe_testkit::readers::Counts;
///
/// // A custom source counts its own reads, a failed one as a call that delivered nothing.
/// let mut counts = Counts::default();
/// counts.record(4);
/// counts.record(0);
/// counts.record_seek();
/// assert_eq!(counts, Counts { read_calls: 2, delivered: 4, seeks: 1 });
/// ```
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
///
/// ```
/// # fn main() -> std::io::Result<()> {
/// use std::io::{Cursor, Read, Seek, SeekFrom};
///
/// use cc_media_probe_testkit::readers::{CountingReader, Counts};
///
/// let mut reader = CountingReader::new(Cursor::new(vec![1, 2, 3]));
/// let mut buffer = [0; 2];
/// assert_eq!(reader.read(&mut buffer)?, 2);
/// reader.seek(SeekFrom::Start(0))?;
/// assert_eq!(reader.counts(), Counts { read_calls: 1, delivered: 2, seeks: 1 });
/// assert_eq!(reader.into_inner().position(), 0);
/// # Ok(())
/// # }
/// ```
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

  /// Counts one seek.
  pub fn record_seek(&mut self) {
    self.seeks = self.seeks.saturating_add(1);
  }
}

impl<Inner> CountingReader<Inner> {
  /// Counts the calls to `inner`.
  pub fn new(inner: Inner) -> CountingReader<Inner> {
    let counts = Counts::default();
    CountingReader { inner, counts }
  }

  /// What the reader was asked for so far.
  #[must_use]
  pub fn counts(&self) -> Counts {
    self.counts
  }

  /// The reader counted.
  pub fn into_inner(self) -> Inner {
    self.inner
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
    self.counts.record_seek();
    self.inner.seek(position)
  }
}
