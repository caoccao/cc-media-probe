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

//! A standard-library reader with faults on purpose (`docs/design/test-kit.md` section 5): its reads cut at the
//! fixed bytes of a case's pieces, an error at a chosen offset, and an interruption once, applied at the position
//! that the reader reached: the bytes delivered so far, or where the last seek went.

use std::error::Error;
use std::io::{self, ErrorKind, Read, Seek, SeekFrom};

use super::faults::Faults;

/// A reader with faults, around any standard-library reader.
///
/// ```
/// # fn main() -> std::io::Result<()> {
/// use std::io::{Cursor, ErrorKind, Read};
///
/// use cc_media_probe_testkit::readers::FaultingReader;
///
/// let bytes: Vec<u8> = (0..10).collect();
/// let mut reader = FaultingReader::new(Cursor::new(bytes))
///     .with_pieces(&[4])
///     .with_error_at(8, ErrorKind::TimedOut);
/// let mut buffer = [0; 16];
/// assert_eq!(reader.read(&mut buffer)?, 4);
/// assert_eq!(reader.read(&mut buffer)?, 4);
/// assert_eq!(reader.read(&mut buffer).unwrap_err().kind(), ErrorKind::TimedOut);
/// assert_eq!(reader.position(), 8);
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct FaultingReader<Inner> {
  /// The reader that delivers the bytes.
  inner: Inner,
  /// What happens at fixed positions.
  faults: Faults,
  /// The offset of the next read: the bytes delivered so far, or where the last seek went.
  offset: u64,
}

impl<Inner> FaultingReader<Inner> {
  /// A reader of `inner`, which stands at its byte 0, without a fault yet; `with_position` gives another start.
  pub fn new(inner: Inner) -> FaultingReader<Inner> {
    FaultingReader::with_faults(inner, Faults::new())
  }

  /// A reader of `inner`, which stands at its byte 0, with `faults`; `with_position` gives another start.
  pub fn with_faults(inner: Inner, faults: Faults) -> FaultingReader<Inner> {
    let offset = 0;
    FaultingReader { inner, faults, offset }
  }

  /// The reader whose inner reader stands at `position` when it is wrapped, such as a cursor already moved, so
  /// that the faults apply at the positions of the inner reader's bytes and `position()` gives where it stands.
  /// Without it such a reader applied them as if it stood at byte 0 until its first seek, as the review of plan
  /// Phase 3, task 4, found.
  #[must_use]
  pub fn with_position(mut self, position: u64) -> FaultingReader<Inner> {
    self.offset = position;
    self
  }

  /// Cuts the input into pieces of `sizes`, in order from byte 0, the last repeating.
  #[must_use]
  pub fn with_pieces(mut self, sizes: &[usize]) -> FaultingReader<Inner> {
    self.faults = self.faults.with_pieces(sizes);
    self
  }

  /// Fails each read that starts at `offset` with an error of `kind`.
  #[must_use]
  pub fn with_error_at(mut self, offset: u64, kind: ErrorKind) -> FaultingReader<Inner> {
    self.faults = self.faults.with_error_at(offset, kind);
    self
  }

  /// Fails each read that starts at `offset` with an error of `kind` that wraps the error that `cause` makes.
  #[must_use]
  pub fn with_wrapped_error_at<Cause, MakeCause>(
    mut self,
    offset: u64,
    kind: ErrorKind,
    cause: MakeCause,
  ) -> FaultingReader<Inner>
  where
    Cause: Error + Send + Sync + 'static,
    MakeCause: Fn() -> Cause + Send + Sync + 'static,
  {
    self.faults = self.faults.with_wrapped_error_at(offset, kind, cause);
    self
  }

  /// Interrupts the first read that starts at `offset`, once.
  #[must_use]
  pub fn with_interruption_at(mut self, offset: u64) -> FaultingReader<Inner> {
    self.faults = self.faults.with_interruption_at(offset);
    self
  }

  /// The offset of the next read.
  #[must_use]
  pub fn position(&self) -> u64 {
    self.offset
  }

  /// The reader that delivers the bytes.
  pub fn into_inner(self) -> Inner {
    self.inner
  }
}

impl<Inner: Read> Read for FaultingReader<Inner> {
  fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
    let piece = self.faults.apply(self.offset, buffer)?;
    let delivered = self.inner.read(piece)?;
    self.offset = self.offset.saturating_add(u64::try_from(delivered).unwrap_or(u64::MAX));
    Ok(delivered)
  }
}

impl<Inner: Seek> Seek for FaultingReader<Inner> {
  fn seek(&mut self, position: SeekFrom) -> io::Result<u64> {
    self.offset = self.inner.seek(position)?;
    Ok(self.offset)
  }
}
