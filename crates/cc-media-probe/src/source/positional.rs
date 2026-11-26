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

//! `PositionalSource`, the trait of a caller's own blocking source (`api.md` section 4.4).

use std::io;

/// A source that is neither a file nor a reader, such as ranges of a remote object. Callers implement it, so it
/// is kept minimal.
///
/// - **Short reads.** A read may return fewer bytes than asked for. The library asks again for the rest.
/// - **Interruptions.** `ErrorKind::Interrupted` is retried, from a read as the standard library's `read_exact`
///   retries it, and from `length()` alike.
/// - **Failures.** Any other error ends the probe with the error of kind `SourceFailed`, which keeps it as its
///   cause (R19).
/// - **An empty buffer.** A read into an empty buffer returns 0 at any position and says nothing about the end, as
///   `Read::read` does. The library never makes such a read: a read that the remaining allowance would clamp to
///   zero bytes is not issued, so every 0 that the library receives answers a read of at least one byte.
/// - **The end.** A source of unknown length has ended only when a read of at least one byte returned no bytes.
///   Such a read at position `P` says only that the source ends at or before `P`. The length is `P` exactly when
///   the source also delivered the byte at `P - 1`, or when `P` is 0, so a read that returns 0 beyond a gap
///   establishes no length. A length that `length()` states is exact without such a read.
/// - **The budget.** Every read is charged to the shared budget and clamped to the remaining read allowance before
///   it is issued (plan section 7.5).
///
/// The library implements the trait for `&mut Source` and `Box<Source>`, so a caller can keep a source or pass a
/// `Box<dyn PositionalSource + Send>`.
///
/// ```
/// # fn main() -> Result<(), cc_media_probe::Error> {
/// use std::io;
///
/// use cc_media_probe::source::PositionalSource;
///
/// /// Ranges of an object in memory, as a remote object would serve them.
/// struct Object {
///     data: Vec<u8>,
/// }
///
/// impl PositionalSource for Object {
///     fn read_at(&mut self, position: u64, buffer: &mut [u8]) -> io::Result<usize> {
///         let start = usize::try_from(position).map_or(self.data.len(), |start| start.min(self.data.len()));
///         let mut available = self.data.get(start..).unwrap_or_default();
///         io::Read::read(&mut available, buffer)
///     }
///
///     fn length(&mut self) -> io::Result<Option<u64>> {
///         Ok(u64::try_from(self.data.len()).ok())
///     }
/// }
///
/// let mut object = Object { data: b"RIFF".to_vec() };
/// let detection = cc_media_probe::detect_source(&mut object)?;
/// println!("{}", detection.outcome().as_str());
/// let boxed: Box<dyn PositionalSource + Send> = Box::new(object);
/// let detection = cc_media_probe::detect_source(boxed)?;
/// println!("{}", detection.outcome().as_str());
/// # Ok(())
/// # }
/// ```
pub trait PositionalSource {
  /// Reads at most `buffer.len()` bytes starting at `position`. Returns 0 for an empty buffer, and for a nonempty
  /// one only at or past the end. The library never asks for zero bytes.
  ///
  /// # Errors
  ///
  /// The error with which the source failed. `ErrorKind::Interrupted` is retried; any other error ends the probe.
  fn read_at(&mut self, position: u64, buffer: &mut [u8]) -> io::Result<usize>;

  /// The total length, or `None` when the source does not know it.
  ///
  /// # Errors
  ///
  /// The error with which the source failed. `ErrorKind::Interrupted` is retried, as for a read; any other error
  /// ends the probe.
  fn length(&mut self) -> io::Result<Option<u64>>;
}

/// A borrowed source, which a caller keeps.
impl<Source: PositionalSource + ?Sized> PositionalSource for &mut Source {
  fn read_at(&mut self, position: u64, buffer: &mut [u8]) -> io::Result<usize> {
    (**self).read_at(position, buffer)
  }

  fn length(&mut self) -> io::Result<Option<u64>> {
    (**self).length()
  }
}

/// A boxed source, such as a `Box<dyn PositionalSource + Send>`.
impl<Source: PositionalSource + ?Sized> PositionalSource for Box<Source> {
  fn read_at(&mut self, position: u64, buffer: &mut [u8]) -> io::Result<usize> {
    (**self).read_at(position, buffer)
  }

  fn length(&mut self) -> io::Result<Option<u64>> {
    (**self).length()
  }
}

#[cfg(test)]
pub(crate) mod tests {
  use std::io::{self, Read};

  use super::PositionalSource;

  /// Ranges of bytes in memory, served from position 0, with a known length.
  pub(crate) struct Ranges {
    data: Vec<u8>,
  }

  impl Ranges {
    /// The ranges of `data`.
    pub(crate) fn new(data: &[u8]) -> Ranges {
      Ranges { data: data.to_vec() }
    }
  }

  impl PositionalSource for Ranges {
    fn read_at(&mut self, position: u64, buffer: &mut [u8]) -> io::Result<usize> {
      let start = usize::try_from(position).map_or(self.data.len(), |start| start.min(self.data.len()));
      let mut available = self.data.get(start..).unwrap_or_default();
      available.read(buffer)
    }

    fn length(&mut self) -> io::Result<Option<u64>> {
      Ok(u64::try_from(self.data.len()).ok())
    }
  }

  /// Reads three bytes at `position` and the length through the trait, as the library would.
  fn read_through<Source: PositionalSource>(mut source: Source, position: u64) -> io::Result<(usize, Option<u64>)> {
    let mut buffer = [0_u8; 3];
    Ok((source.read_at(position, &mut buffer)?, source.length()?))
  }

  #[test]
  fn reads_through_a_borrow_and_a_box() -> io::Result<()> {
    let mut ranges = Ranges::new(b"abcd");
    assert_eq!(read_through(&mut ranges, 2)?, (2, Some(4)));
    let boxed: Box<dyn PositionalSource + Send> = Box::new(Ranges::new(b"abcd"));
    assert_eq!(read_through(boxed, 0)?, (3, Some(4)));
    assert_eq!(read_through(Box::new(&mut ranges), 9)?, (0, Some(4)));
    let mut empty: [u8; 0] = [];
    assert_eq!(ranges.read_at(0, &mut empty)?, 0);
    Ok(())
  }
}
