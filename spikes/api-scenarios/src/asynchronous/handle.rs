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

//! The prober's handle for async positional sources, `api.md` section 9.1.

use std::future::poll_fn;
use std::pin::Pin;

use super::source::AsyncPositionalSource;
use crate::constants::PREFIX_LENGTH;
use crate::error::Error;
use crate::model::MediaReport;
use crate::prober::Prober;

/// Borrows a prober to probe async positional sources with its options.
#[derive(Clone, Copy, Debug)]
pub struct AsynchronousProber<'prober> {
  prober: &'prober Prober,
}

impl Prober {
  /// The handle for async positional sources.
  pub fn asynchronous(&self) -> AsynchronousProber<'_> {
    AsynchronousProber { prober: self }
  }
}

impl AsynchronousProber<'_> {
  /// Probes the caller's own async positional source.
  pub async fn probe_source<Source: AsyncPositionalSource + Unpin>(
    &self,
    mut source: Source,
  ) -> Result<MediaReport, Error> {
    poll_fn(|context| Pin::new(&mut source).poll_length(context))
      .await
      .map_err(Error::from_source_failure)?;
    let mut prefix = [0_u8; PREFIX_LENGTH];
    let count = poll_fn(|context| Pin::new(&mut source).poll_read_at(context, 0, &mut prefix))
      .await
      .map_err(Error::from_source_failure)?;
    self.prober.probe_bytes(prefix.get(..count).unwrap_or_default())
  }
}
