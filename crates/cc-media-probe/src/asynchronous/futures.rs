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

//! The futures-io module (feature `futures-io`; `api.md` section 9.1): seekable and forward-only input, through the
//! `AsyncRead` and `AsyncSeek` of futures-io. futures-io defines no file type, so the module has no path input.

use futures_io::{AsyncRead, AsyncSeek};

use super::reader_methods::reader_methods;
use crate::{Detection, Error, MediaReport, Prober};

/// Borrows a prober to probe and detect futures-io readers with its options. [`Prober::futures`] gives it, and each
/// future is `Send` exactly when the reader is.
///
/// ```
/// # fn main() -> Result<(), cc_media_probe::Error> {
/// use cc_media_probe::Prober;
///
/// let reader = futures_lite::io::Cursor::new(b"not a media file".to_vec());
/// let detection = futures_lite::future::block_on(Prober::new().futures().detect_stream(reader))?;
/// println!("{}", detection.outcome().as_str());
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Debug)]
pub struct FuturesProber<'prober> {
  prober: &'prober Prober,
}

impl Prober {
  /// The handle that probes and detects futures-io readers with these options (feature `futures-io`).
  #[must_use]
  pub fn futures(&self) -> FuturesProber<'_> {
    FuturesProber { prober: self }
  }
}

reader_methods!(FuturesProber);

/// Probes a seekable futures-io reader with the default options, the whole input from its byte 0, as
/// [`FuturesProber::probe_reader`] says, `&mut reader`, `Box::pin(reader)` and cancellation included.
///
/// # Errors
///
/// As [`crate::probe_reader`].
pub async fn probe_reader<Reader: AsyncRead + AsyncSeek + Unpin>(reader: Reader) -> Result<MediaReport, Error> {
  Prober::new().futures().probe_reader(reader).await
}

/// Probes a forward-only futures-io reader with the default options, from where it stands, as
/// [`FuturesProber::probe_stream`] says, `&mut reader`, `Box::pin(reader)` and cancellation included.
///
/// # Errors
///
/// As [`crate::probe_stream`].
pub async fn probe_stream<Reader: AsyncRead + Unpin>(reader: Reader) -> Result<MediaReport, Error> {
  Prober::new().futures().probe_stream(reader).await
}

/// Detects the format of a seekable futures-io reader with the default options, as
/// [`FuturesProber::detect_reader`] says, `&mut reader`, `Box::pin(reader)` and cancellation included.
///
/// # Errors
///
/// `SourceFailed` for a reader that fails, and nothing else.
pub async fn detect_reader<Reader: AsyncRead + AsyncSeek + Unpin>(reader: Reader) -> Result<Detection, Error> {
  Prober::new().futures().detect_reader(reader).await
}

/// Detects the format of a forward-only futures-io reader with the default options, as
/// [`FuturesProber::detect_stream`] says, `&mut reader`, `Box::pin(reader)` and cancellation included.
///
/// # Errors
///
/// `SourceFailed` for a reader that fails, and nothing else.
pub async fn detect_stream<Reader: AsyncRead + Unpin>(reader: Reader) -> Result<Detection, Error> {
  Prober::new().futures().detect_stream(reader).await
}

#[cfg(test)]
mod tests {
  use futures_lite::future::block_on;
  use futures_lite::io::{Cursor, empty};

  use super::{detect_reader, detect_stream, probe_reader, probe_stream};
  use crate::model::DetectionOutcome;
  use crate::{Detection, Error, ErrorKind, Format, Prober};

  /// Whether the future is `Send`, which the call compiles only for.
  fn is_send<Value: Send>(_: &Value) -> bool {
    true
  }

  #[test]
  fn probes_every_futures_input_with_the_default_options() {
    let probed = [
      block_on(probe_reader(Cursor::new(b"RIFF"))).err(),
      block_on(probe_stream(empty())).err(),
    ];
    for error in probed {
      assert_eq!(error.as_ref().map(Error::kind), Some(&ErrorKind::UnknownFormat));
    }
    let detected = [
      block_on(detect_reader(Cursor::new(b"RIFF"))).ok(),
      block_on(detect_stream(empty())).ok(),
    ];
    for detection in detected {
      assert_eq!(
        detection.as_ref().map(Detection::outcome),
        Some(&DetectionOutcome::Unknown)
      );
    }
  }

  #[test]
  fn keeps_the_options_of_the_prober() {
    let prober = Prober::new().with_format_hint(Format::Avi);
    let handle = prober.futures();
    let mut cursor = Cursor::new(vec![0_u8; 8]);
    let detection = block_on(handle.detect_reader(&mut cursor)).ok();
    assert_eq!(
      detection.as_ref().map(Detection::outcome),
      Some(&DetectionOutcome::HintOnly)
    );
    assert_eq!(cursor.position(), 0);
    let stream = block_on(handle.clone().detect_stream(Box::pin(empty()))).ok();
    assert!(stream.is_some_and(|detection| detection.hint().is_some()));
    let ended = block_on(handle.probe_stream(&b"RIFF"[..])).err();
    assert!(ended.is_some_and(|error| error.hint().is_some()));
    let future = handle.probe_reader(Cursor::new(Vec::new()));
    assert!(is_send(&future));
    assert!(block_on(future).is_err());
    assert!(format!("{handle:?}").starts_with("FuturesProber"));
  }
}
