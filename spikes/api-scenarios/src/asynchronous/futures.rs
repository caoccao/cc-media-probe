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

//! The futures-io module, `api.md` section 9.1: seekable and forward-only input.

use std::future::poll_fn;
use std::io::SeekFrom;
use std::pin::Pin;

use futures_io::{AsyncRead, AsyncSeek};

use crate::constants::PREFIX_LENGTH;
use crate::error::Error;
use crate::model::MediaReport;
use crate::prober::Prober;

/// Borrows a prober to probe futures-io readers with its options.
#[derive(Clone, Copy, Debug)]
pub struct FuturesProber<'prober> {
  prober: &'prober Prober,
}

impl Prober {
  /// The handle for futures-io readers.
  pub fn futures(&self) -> FuturesProber<'_> {
    FuturesProber { prober: self }
  }
}

impl FuturesProber<'_> {
  /// Probes a seekable reader.
  pub async fn probe_reader<Reader: AsyncRead + AsyncSeek + Unpin>(
    &self,
    mut reader: Reader,
  ) -> Result<MediaReport, Error> {
    poll_fn(|context| Pin::new(&mut reader).poll_seek(context, SeekFrom::Start(0)))
      .await
      .map_err(Error::from_source_failure)?;
    self.probe_stream(reader).await
  }

  /// Probes a forward-only reader.
  pub async fn probe_stream<Reader: AsyncRead + Unpin>(&self, mut reader: Reader) -> Result<MediaReport, Error> {
    let mut prefix = [0_u8; PREFIX_LENGTH];
    let count = poll_fn(|context| Pin::new(&mut reader).poll_read(context, &mut prefix))
      .await
      .map_err(Error::from_source_failure)?;
    self.prober.probe_bytes(prefix.get(..count).unwrap_or_default())
  }
}

/// Probes a seekable reader with the default options.
pub async fn probe_reader<Reader: AsyncRead + AsyncSeek + Unpin>(reader: Reader) -> Result<MediaReport, Error> {
  Prober::new().futures().probe_reader(reader).await
}

/// Probes a forward-only reader with the default options.
pub async fn probe_stream<Reader: AsyncRead + Unpin>(reader: Reader) -> Result<MediaReport, Error> {
  Prober::new().futures().probe_stream(reader).await
}
