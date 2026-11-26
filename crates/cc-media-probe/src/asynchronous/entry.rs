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

//! The two functions of the `asynchronous` module (`api.md` section 9.1), each with the default options: an async
//! positional source needs no feature.

use super::AsyncPositionalSource;
use crate::{Detection, Error, MediaReport, Prober};

/// Probes the caller's own async positional source with the default options, from its position 0. Pass
/// `&mut source` to keep it, or `Box::pin(source)` for a source that is not `Unpin`. Dropping the future stops the
/// probe, as [the rule of cancellation](crate::asynchronous#cancellation) says. [`AsyncPositionalSource`] shows a
/// source.
///
/// # Errors
///
/// As [`crate::probe_source`].
pub async fn probe_source<Source: AsyncPositionalSource + Unpin>(source: Source) -> Result<MediaReport, Error> {
  Prober::new().asynchronous().probe_source(source).await
}

/// Detects the format of the caller's own async positional source with the default options. Pass `&mut source`
/// to keep it, or `Box::pin(source)` for a source that is not `Unpin`. Dropping the future stops the detection, as
/// [the rule of cancellation](crate::asynchronous#cancellation) says.
///
/// # Errors
///
/// `SourceFailed` for a source that fails, and nothing else.
pub async fn detect_source<Source: AsyncPositionalSource + Unpin>(source: Source) -> Result<Detection, Error> {
  Prober::new().asynchronous().detect_source(source).await
}

#[cfg(test)]
mod tests {
  use futures_lite::future::block_on;

  use super::{detect_source, probe_source};
  use crate::asynchronous::ReadyRanges;
  use crate::model::DetectionOutcome;
  use crate::{Detection, Error, ErrorKind};

  #[test]
  fn probes_and_detects_with_the_default_options() {
    let probed = block_on(probe_source(Box::pin(ReadyRanges::new(b"fLaC")))).err();
    assert_eq!(probed.as_ref().map(Error::kind), Some(&ErrorKind::UnknownFormat));
    let detected = block_on(detect_source(Box::pin(ReadyRanges::new(b"fLaC")))).ok();
    assert_eq!(
      detected.as_ref().map(Detection::outcome),
      Some(&DetectionOutcome::Unknown)
    );
  }
}
