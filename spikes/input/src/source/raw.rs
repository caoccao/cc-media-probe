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

//! The raw inputs under the internal source: positional ones, which the block cache reads, and sequential ones,
//! which the sliding window reads. `ErrorKind::Interrupted` is retried here, as `api.md` section 4.4 says.

use std::io;

/// A raw input that reads at a position: a caller's positional source, a seekable reader, or a file.
pub(crate) trait RawPositional {
  /// Reads at most `buffer.len()` bytes at `position`. A short read is allowed.
  async fn read_at(&mut self, position: u64, buffer: &mut [u8]) -> io::Result<usize>;
}

/// A raw input that reads forward only: a reader that cannot seek.
pub(crate) trait RawSequential {
  /// Reads at most `buffer.len()` bytes where the input stands.
  async fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize>;
}

/// Reads at a position, with `ErrorKind::Interrupted` retried as `read_exact` retries it.
pub(crate) async fn read_positional<Raw: RawPositional>(
  raw: &mut Raw,
  position: u64,
  buffer: &mut [u8],
) -> io::Result<usize> {
  loop {
    match raw.read_at(position, buffer).await {
      Err(error) if error.kind() == io::ErrorKind::Interrupted => {}
      result => return result,
    }
  }
}

/// Reads forward, with `ErrorKind::Interrupted` retried.
pub(crate) async fn read_sequential<Raw: RawSequential>(raw: &mut Raw, buffer: &mut [u8]) -> io::Result<usize> {
  loop {
    match raw.read(buffer).await {
      Err(error) if error.kind() == io::ErrorKind::Interrupted => {}
      result => return result,
    }
  }
}
