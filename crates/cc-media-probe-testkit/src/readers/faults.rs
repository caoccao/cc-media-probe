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

//! The faults of a test reader at fixed positions of its input (`docs/design/test-kit.md` section 5, plan section
//! 9.2, "Fault injection"): the pieces that cut its reads, an error at a chosen offset, either of a chosen kind or
//! wrapping any Rust error, and an interruption once. `FaultingReader` applies them to a standard-library reader at
//! the position that it reached, and a custom source of the library's tests applies them at the positions of its
//! own reads.
//!
//! A read is cut short before each fault's offset, so that the read that reaches an offset starts there. A read
//! that starts at the error's offset fails every time, so that a seek back to it fails again. A read that starts at
//! the interruption's offset fails once with `ErrorKind::Interrupted`, which every reader of the standard library's
//! contract retries, and the read after it is served. A read into an empty buffer meets no fault and consumes none,
//! since it asks for no byte: such a read returns 0 at any position, as the library's `PositionalSource` requires,
//! and the fault waits for the first read of a byte. One consumed a pending interruption before the second review
//! of plan Phase 3, task 4, found it.

use std::error::Error;
use std::fmt;
use std::io::{self, ErrorKind};

use super::constants::{FAILURE, FAULTS, INTERRUPTION, PIECES};
use super::pieces::{Pieces, shortened};

/// Makes the error of a failing read, so that each read at the offset fails with an error of its own.
type MakeError = Box<dyn Fn() -> io::Error + Send + Sync>;

/// What a reader does at fixed positions of its input.
///
/// ```
/// # fn main() -> std::io::Result<()> {
/// use std::io::ErrorKind;
///
/// use cc_media_probe_testkit::readers::Faults;
///
/// // A custom source applies the faults at the positions of its own reads.
/// let mut faults = Faults::new().with_pieces(&[40, 8]).with_interruption_at(44);
/// let mut buffer = [0; 64];
/// assert_eq!(faults.apply(0, &mut buffer)?.len(), 40);
/// assert_eq!(faults.apply(40, &mut buffer)?.len(), 4);
/// assert_eq!(faults.apply(44, &mut []).map(|piece| piece.len())?, 0);
/// assert_eq!(faults.apply(44, &mut buffer).unwrap_err().kind(), ErrorKind::Interrupted);
/// assert_eq!(faults.apply(44, &mut buffer)?.len(), 4);
/// # Ok(())
/// # }
/// ```
#[derive(Default)]
pub struct Faults {
  /// The pieces of its reads.
  pieces: Pieces,
  /// The offset at which reads fail, with the error that each failing read gives.
  failure: Option<(u64, MakeError)>,
  /// The offset at which a read is interrupted once, until it has been.
  interruption: Option<u64>,
}

impl Faults {
  /// No fault: reads are not cut, and none fails.
  #[must_use]
  pub fn new() -> Faults {
    Faults::default()
  }

  /// Cuts the input into pieces of `sizes`, in order from byte 0, the last repeating.
  #[must_use]
  pub fn with_pieces(mut self, sizes: &[usize]) -> Faults {
    self.pieces = Pieces::new(sizes);
    self
  }

  /// Fails each read that starts at `offset` with an error of `kind`, in place of any failure set before.
  #[must_use]
  pub fn with_error_at(mut self, offset: u64, kind: ErrorKind) -> Faults {
    self.failure = Some((offset, Box::new(move || io::Error::from(kind))));
    self
  }

  /// Fails each read that starts at `offset` with an error of `kind` that wraps the error that `cause` makes, which
  /// the error's `get_ref` gives back, in place of any failure set before.
  #[must_use]
  pub fn with_wrapped_error_at<Cause, MakeCause>(mut self, offset: u64, kind: ErrorKind, cause: MakeCause) -> Faults
  where
    Cause: Error + Send + Sync + 'static,
    MakeCause: Fn() -> Cause + Send + Sync + 'static,
  {
    self.failure = Some((offset, Box::new(move || io::Error::new(kind, cause()))));
    self
  }

  /// Interrupts the first read that starts at `offset`, once, in place of any interruption set before.
  #[must_use]
  pub fn with_interruption_at(mut self, offset: u64) -> Faults {
    self.interruption = Some(offset);
    self
  }

  /// The pieces of the reads.
  #[must_use]
  pub fn pieces(&self) -> &Pieces {
    &self.pieces
  }

  /// The part of `buffer` that a read at `position` may fill: cut at the end of its piece and before each fault's
  /// offset beyond `position`, and an empty buffer as it is.
  ///
  /// # Errors
  ///
  /// The error of a read of at least one byte that starts at the failure's offset, and the interruption of the first
  /// such read that starts at the interruption's offset.
  pub fn apply<'buffer>(&mut self, position: u64, buffer: &'buffer mut [u8]) -> io::Result<&'buffer mut [u8]> {
    if buffer.is_empty() {
      return Ok(buffer);
    }
    if self.interruption == Some(position) {
      self.interruption = None;
      return Err(io::Error::from(ErrorKind::Interrupted));
    }
    if let Some((offset, make)) = &self.failure
      && *offset == position
    {
      return Err(make());
    }
    let mut piece = self.pieces.cut(position, buffer);
    let offsets = [self.failure.as_ref().map(|(offset, _)| *offset), self.interruption];
    for offset in offsets.into_iter().flatten() {
      if let Some(room) = offset.checked_sub(position).filter(|room| *room > 0) {
        piece = shortened(piece, room);
      }
    }
    Ok(piece)
  }
}

impl fmt::Debug for Faults {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter
      .debug_struct(FAULTS)
      .field(PIECES, &self.pieces)
      .field(FAILURE, &self.failure.as_ref().map(|(offset, _)| offset))
      .field(INTERRUPTION, &self.interruption)
      .finish()
  }
}
