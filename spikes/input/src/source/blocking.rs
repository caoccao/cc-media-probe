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

//! The blocking raw inputs: a caller's `PositionalSource`, a `Read + Seek` reader, and a forward-only `Read`. Each
//! read is made before its future exists, which is `ready`, so a walker over them never waits (D8).

use std::future::{Future, ready};
use std::io::{self, Read, Seek, SeekFrom};

use cc_media_probe::source::PositionalSource;

use super::{RawPositional, RawSequential};

/// A caller's own blocking positional source (`api.md` section 4.4).
pub(crate) struct BlockingPositional<Source>(pub(crate) Source);

impl<Source: PositionalSource> RawPositional for BlockingPositional<Source> {
  fn read_at(&mut self, position: u64, buffer: &mut [u8]) -> impl Future<Output = io::Result<usize>> {
    ready(self.0.read_at(position, buffer))
  }
}

/// A seekable reader, read as a seek followed by a read (plan section 8.7), from its byte 0 wherever it stands.
pub(crate) struct Seekable<Reader>(pub(crate) Reader);

impl<Reader: Read + Seek> RawPositional for Seekable<Reader> {
  fn read_at(&mut self, position: u64, buffer: &mut [u8]) -> impl Future<Output = io::Result<usize>> {
    ready(self.0.seek(SeekFrom::Start(position)).and_then(|_| self.0.read(buffer)))
  }
}

/// A forward-only reader, which starts where it stands.
pub(crate) struct Forward<Reader>(pub(crate) Reader);

impl<Reader: Read> RawSequential for Forward<Reader> {
  fn read(&mut self, buffer: &mut [u8]) -> impl Future<Output = io::Result<usize>> {
    ready(self.0.read(buffer))
  }
}
