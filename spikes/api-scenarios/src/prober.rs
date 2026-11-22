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

//! The `Prober` builder, `api.md` sections 4.2 and 4.3, with its blocking methods.

use std::ffi::{OsStr, OsString};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

use crate::constants::PREFIX_LENGTH;
use crate::error::{Error, ErrorKind};
use crate::limits::Limits;
use crate::model::{AttachmentHandling, Format, MediaReport};
use crate::source::PositionalSource;

/// Carries the options; built once and used for any number of probes, from any number of threads.
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct Prober {
  format_hint: Option<Format>,
  file_name_hint: Option<OsString>,
  limits: Limits,
  attachments: AttachmentHandling,
}

impl Prober {
  /// The default options.
  pub fn new() -> Prober {
    Prober::default()
  }

  /// Orders the candidates that the bytes allow.
  pub fn with_format_hint(mut self, format: Format) -> Prober {
    self.format_hint = Some(format);
    self
  }

  /// Orders the candidates by a file name, which is never opened.
  pub fn with_file_name_hint(mut self, name: impl Into<OsString>) -> Prober {
    self.file_name_hint = Some(name.into());
    self
  }

  /// The limits of the shared budget.
  pub fn with_limits(mut self, limits: Limits) -> Prober {
    self.limits = limits;
    self
  }

  /// Whether attachments are read, and with their data.
  pub fn with_attachments(mut self, handling: AttachmentHandling) -> Prober {
    self.attachments = handling;
    self
  }

  /// The format hint.
  pub fn format_hint(&self) -> Option<&Format> {
    self.format_hint.as_ref()
  }

  /// The file-name hint.
  pub fn file_name_hint(&self) -> Option<&OsStr> {
    self.file_name_hint.as_deref()
  }

  /// The limits.
  pub fn limits(&self) -> &Limits {
    &self.limits
  }

  /// The attachment handling.
  pub fn attachments(&self) -> &AttachmentHandling {
    &self.attachments
  }

  /// Probes the file at `path`, and opens no other file.
  pub fn probe_file(&self, path: impl AsRef<Path>) -> Result<MediaReport, Error> {
    let file = File::open(path).map_err(Error::from_source_failure)?;
    self.probe_reader(file)
  }

  /// Probes a seekable reader, taken by value.
  pub fn probe_reader<Reader: Read + Seek>(&self, mut reader: Reader) -> Result<MediaReport, Error> {
    reader.seek(SeekFrom::Start(0)).map_err(Error::from_source_failure)?;
    self.probe_stream(reader)
  }

  /// Probes a forward-only reader, such as standard input.
  pub fn probe_stream<Reader: Read>(&self, mut reader: Reader) -> Result<MediaReport, Error> {
    let mut prefix = [0_u8; PREFIX_LENGTH];
    let count = reader.read(&mut prefix).map_err(Error::from_source_failure)?;
    self.probe_bytes(prefix.get(..count).unwrap_or_default())
  }

  /// Probes bytes in memory; the report copies what it keeps.
  pub fn probe_bytes(&self, bytes: &[u8]) -> Result<MediaReport, Error> {
    Err(Error::new(if bytes.is_empty() {
      ErrorKind::FormatNotDetermined
    } else {
      ErrorKind::UnsupportedFormat
    }))
  }

  /// Probes the caller's own positional source.
  pub fn probe_source<Source: PositionalSource>(&self, mut source: Source) -> Result<MediaReport, Error> {
    source.length().map_err(Error::from_source_failure)?;
    let mut prefix = [0_u8; PREFIX_LENGTH];
    let count = source.read_at(0, &mut prefix).map_err(Error::from_source_failure)?;
    self.probe_bytes(prefix.get(..count).unwrap_or_default())
  }
}
