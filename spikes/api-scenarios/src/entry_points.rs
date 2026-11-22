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

//! The blocking functions of `api.md` section 4.1: each is the `Prober` method with the default options.

use std::io::{Read, Seek};
use std::path::Path;

use crate::error::Error;
use crate::model::MediaReport;
use crate::prober::Prober;
use crate::source::PositionalSource;

/// Probes the file at `path`.
pub fn probe_file(path: impl AsRef<Path>) -> Result<MediaReport, Error> {
  Prober::new().probe_file(path)
}

/// Probes a seekable reader.
pub fn probe_reader<Reader: Read + Seek>(reader: Reader) -> Result<MediaReport, Error> {
  Prober::new().probe_reader(reader)
}

/// Probes a forward-only reader.
pub fn probe_stream<Reader: Read>(reader: Reader) -> Result<MediaReport, Error> {
  Prober::new().probe_stream(reader)
}

/// Probes bytes in memory.
pub fn probe_bytes(bytes: &[u8]) -> Result<MediaReport, Error> {
  Prober::new().probe_bytes(bytes)
}

/// Probes the caller's own positional source.
pub fn probe_source<Source: PositionalSource>(source: Source) -> Result<MediaReport, Error> {
  Prober::new().probe_source(source)
}
