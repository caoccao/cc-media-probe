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

//! `AsynchronousProber`, the handle of a prober for async positional sources (`api.md` section 9.1), which needs
//! no feature.

use std::future::ready;

use super::AsyncPositionalSource;
use crate::{Detection, Error, MediaReport, Prober};

/// Borrows a prober to probe and detect async positional sources with its options. [`Prober::asynchronous`] gives
/// it, and each future is `Send` exactly when the source is.
///
/// ```
/// # fn main() -> Result<(), cc_media_probe::Error> {
/// use cc_media_probe::model::DetectionOutcome;
/// use cc_media_probe::{Format, Prober};
///
/// # struct Object;
/// # impl cc_media_probe::asynchronous::AsyncPositionalSource for Object {
/// #     fn poll_read_at(
/// #         self: std::pin::Pin<&mut Self>,
/// #         _context: &mut std::task::Context<'_>,
/// #         _position: u64,
/// #         _buffer: &mut [u8],
/// #     ) -> std::task::Poll<std::io::Result<usize>> {
/// #         std::task::Poll::Ready(Ok(0))
/// #     }
/// #     fn poll_length(
/// #         self: std::pin::Pin<&mut Self>,
/// #         _context: &mut std::task::Context<'_>,
/// #     ) -> std::task::Poll<std::io::Result<Option<u64>>> {
/// #         std::task::Poll::Ready(Ok(Some(0)))
/// #     }
/// # }
/// let prober = Prober::new().with_format_hint(Format::Matroska);
/// let detection = futures_lite::future::block_on(prober.asynchronous().detect_source(Object))?;
/// assert!(matches!(detection.outcome(), DetectionOutcome::HintOnly));
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Debug)]
pub struct AsynchronousProber<'prober> {
  prober: &'prober Prober,
}

impl Prober {
  /// The handle that probes and detects async positional sources with these options.
  #[must_use]
  pub fn asynchronous(&self) -> AsynchronousProber<'_> {
    AsynchronousProber { prober: self }
  }
}

impl AsynchronousProber<'_> {
  /// Probes the caller's own async positional source, from its position 0, as [`Prober::probe_source`] probes a
  /// blocking one. Pass `&mut source` to keep it, or `Box::pin(source)` for a source that is not `Unpin`. Dropping
  /// the future stops the probe, as [the rule of cancellation](crate::asynchronous#cancellation) says.
  ///
  /// # Errors
  ///
  /// As [`Prober::probe_source`].
  pub async fn probe_source<Source: AsyncPositionalSource + Unpin>(
    &self,
    source: Source,
  ) -> Result<MediaReport, Error> {
    // This build reads nothing of the source, so the outcome is ready at once.
    ready(self.prober.probe_input(source)).await
  }

  /// Detects the format of the caller's own async positional source. Pass `&mut source` to keep it, or
  /// `Box::pin(source)` for a source that is not `Unpin`. Dropping the future stops the detection, as
  /// [the rule of cancellation](crate::asynchronous#cancellation) says.
  ///
  /// # Errors
  ///
  /// `SourceFailed` for a source that fails, and nothing else.
  pub async fn detect_source<Source: AsyncPositionalSource + Unpin>(&self, source: Source) -> Result<Detection, Error> {
    // This build reads nothing of the source, so the outcome is ready at once.
    ready(Ok(self.prober.detect_input(source))).await
  }
}

#[cfg(test)]
mod tests {
  use std::pin::pin;

  use futures_lite::future::block_on;

  use crate::asynchronous::ReadyRanges;
  use crate::model::{DetectionOutcome, Hint};
  use crate::{Detection, Error, ErrorKind, Format, Prober};

  /// Whether the future is `Send`, which the call compiles only for.
  fn is_send<Value: Send>(_: &Value) -> bool {
    true
  }

  #[test]
  fn probes_and_detects_a_source_with_the_options() {
    let prober = Prober::new().with_format_hint(Format::WebM);
    let handle = prober.asynchronous();
    let mut boxed = Box::pin(ReadyRanges::new(b"OggS"));
    let ended = block_on(handle.probe_source(boxed.as_mut())).err();
    assert_eq!(ended.as_ref().map(Error::kind), Some(&ErrorKind::UnknownFormat));
    let pinned = pin!(ReadyRanges::new(b"OggS"));
    let detection = block_on(handle.clone().detect_source(pinned)).ok();
    assert_eq!(
      detection.as_ref().map(Detection::outcome),
      Some(&DetectionOutcome::HintOnly)
    );
    let hinted = detection
      .as_ref()
      .and_then(|detection| detection.hint())
      .map(Hint::format);
    assert_eq!(hinted, Some(&Format::WebM));
    assert!(format!("{handle:?}").starts_with("AsynchronousProber"));
  }

  #[test]
  fn gives_a_send_future_for_a_send_source() {
    let prober = Prober::new();
    let handle = prober.asynchronous();
    let future = handle.probe_source(Box::pin(ReadyRanges::new(b"")));
    assert!(is_send(&future));
    assert!(block_on(future).is_err());
  }
}
