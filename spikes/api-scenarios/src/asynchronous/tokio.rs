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

//! The Tokio module, `api.md` sections 9.1 and 9.2: seekable, forward-only and path input.

use std::future::poll_fn;
use std::io::SeekFrom;
use std::path::Path;
use std::pin::Pin;

use ::tokio::io::{AsyncRead, AsyncSeek, ReadBuf};

use crate::constants::PREFIX_LENGTH;
use crate::error::Error;
use crate::model::MediaReport;
use crate::prober::Prober;

/// Borrows a prober to probe Tokio readers with its options.
#[derive(Clone, Copy, Debug)]
pub struct TokioProber<'prober> {
  prober: &'prober Prober,
}

impl Prober {
  /// The handle for Tokio readers.
  pub fn tokio(&self) -> TokioProber<'_> {
    TokioProber { prober: self }
  }
}

impl TokioProber<'_> {
  /// Probes a seekable reader.
  pub async fn probe_reader<Reader: AsyncRead + AsyncSeek + Unpin>(
    &self,
    mut reader: Reader,
  ) -> Result<MediaReport, Error> {
    Pin::new(&mut reader)
      .start_seek(SeekFrom::Start(0))
      .map_err(Error::from_source_failure)?;
    poll_fn(|context| Pin::new(&mut reader).poll_complete(context))
      .await
      .map_err(Error::from_source_failure)?;
    self.probe_stream(reader).await
  }

  /// Probes a forward-only reader.
  pub async fn probe_stream<Reader: AsyncRead + Unpin>(&self, mut reader: Reader) -> Result<MediaReport, Error> {
    let mut prefix = [0_u8; PREFIX_LENGTH];
    let mut buffer = ReadBuf::new(&mut prefix);
    poll_fn(|context| Pin::new(&mut reader).poll_read(context, &mut buffer))
      .await
      .map_err(Error::from_source_failure)?;
    self.prober.probe_bytes(buffer.filled())
  }

  /// Opens the file at `path` with Tokio's file type and probes it.
  ///
  /// # Panics
  ///
  /// When it is polled outside a Tokio runtime, as every operation of `tokio::fs` does.
  pub async fn probe_file(&self, path: impl AsRef<Path>) -> Result<MediaReport, Error> {
    let file = ::tokio::fs::File::open(path)
      .await
      .map_err(Error::from_source_failure)?;
    self.probe_reader(file).await
  }
}

/// Probes a seekable reader with the default options.
pub async fn probe_reader<Reader: AsyncRead + AsyncSeek + Unpin>(reader: Reader) -> Result<MediaReport, Error> {
  Prober::new().tokio().probe_reader(reader).await
}

/// Probes a forward-only reader with the default options.
pub async fn probe_stream<Reader: AsyncRead + Unpin>(reader: Reader) -> Result<MediaReport, Error> {
  Prober::new().tokio().probe_stream(reader).await
}

/// Opens the file at `path` and probes it with the default options; it must run inside a Tokio runtime.
pub async fn probe_file(path: impl AsRef<Path>) -> Result<MediaReport, Error> {
  Prober::new().tokio().probe_file(path).await
}
