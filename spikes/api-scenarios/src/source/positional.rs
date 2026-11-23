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

//! The blocking positional source, `api.md` section 4.4.

use std::io;

/// A source that is neither a file nor a reader, such as ranges of a remote object.
pub trait PositionalSource {
  /// Reads at most `buffer.len()` bytes starting at `position`. Returns 0 for an empty buffer, and for a nonempty one
  /// only at or past the end. The library never asks for zero bytes.
  fn read_at(&mut self, position: u64, buffer: &mut [u8]) -> io::Result<usize>;

  /// The total length, or `None` when the source does not know it.
  fn length(&mut self) -> io::Result<Option<u64>>;
}

impl<Source: PositionalSource + ?Sized> PositionalSource for &mut Source {
  fn read_at(&mut self, position: u64, buffer: &mut [u8]) -> io::Result<usize> {
    (**self).read_at(position, buffer)
  }

  fn length(&mut self) -> io::Result<Option<u64>> {
    (**self).length()
  }
}

impl<Source: PositionalSource + ?Sized> PositionalSource for Box<Source> {
  fn read_at(&mut self, position: u64, buffer: &mut [u8]) -> io::Result<usize> {
    (**self).read_at(position, buffer)
  }

  fn length(&mut self) -> io::Result<Option<u64>> {
    (**self).length()
  }
}
