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

//! The Tokio module (feature `tokio`; `api.md` sections 9.1 and 9.2): seekable, forward-only and path input,
//! through Tokio's `AsyncRead` and `AsyncSeek`. The feature adds those traits and Tokio's file type, and no runtime.

use std::path::Path;

use ::tokio::fs::File;
use ::tokio::io::{AsyncRead, AsyncSeek};

use super::reader_methods::reader_methods;
use crate::{Detection, Error, MediaReport, Prober};

/// Borrows a prober to probe and detect Tokio readers with its options. [`Prober::tokio`] gives it, and each future
/// is `Send` exactly when the reader is.
///
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use cc_media_probe::Prober;
///
/// let runtime = tokio::runtime::Builder::new_current_thread().build()?;
/// let reader = std::io::Cursor::new(b"not a media file".to_vec());
/// let detection = runtime.block_on(Prober::new().tokio().detect_reader(reader))?;
/// println!("{}", detection.outcome().as_str());
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Debug)]
pub struct TokioProber<'prober> {
  prober: &'prober Prober,
}

impl Prober {
  /// The handle that probes and detects Tokio readers with these options (feature `tokio`).
  #[must_use]
  pub fn tokio(&self) -> TokioProber<'_> {
    TokioProber { prober: self }
  }
}

reader_methods!(TokioProber);

impl TokioProber<'_> {
  /// Opens the file at `path` with Tokio's `File` and probes it as a seekable reader. Its final component is the
  /// file-name hint unless the builder sets one; detection consults that hint from plan gate G3.5 on, as
  /// [`Prober::with_file_name_hint`] says. Dropping the future stops the probe, but an open or a read already
  /// handed to Tokio's blocking pool runs to its end there, as
  /// [the rule of cancellation](crate::asynchronous#cancellation) says.
  ///
  /// # Errors
  ///
  /// As [`Prober::probe_file`].
  ///
  /// # Panics
  ///
  /// When the future is polled outside a Tokio runtime: Tokio runs file operations on the blocking pool of the
  /// runtime, and panics without one.
  pub async fn probe_file(&self, path: impl AsRef<Path>) -> Result<MediaReport, Error> {
    let file = File::open(path).await.map_err(Error::not_opened)?;
    self.probe_reader(file).await
  }

  /// Opens the file at `path` with Tokio's `File` and detects its format as a seekable reader's. Its final
  /// component is the file-name hint unless the builder sets one; detection consults that hint from plan gate G3.5
  /// on, as [`Prober::with_file_name_hint`] says. It never fails for a recognized detect-only format. Dropping the
  /// future stops the detection, with what [`TokioProber::probe_file`] says of an operation on the blocking pool.
  ///
  /// # Errors
  ///
  /// `SourceFailed` for a file that cannot be opened or that fails, and nothing else.
  ///
  /// # Panics
  ///
  /// When the future is polled outside a Tokio runtime, as [`TokioProber::probe_file`] says.
  pub async fn detect_file(&self, path: impl AsRef<Path>) -> Result<Detection, Error> {
    let file = File::open(path).await.map_err(Error::not_opened)?;
    self.detect_reader(file).await
  }
}

/// Probes a seekable Tokio reader with the default options, the whole input from its byte 0, as
/// [`TokioProber::probe_reader`] says, `&mut reader`, `Box::pin(reader)` and cancellation included.
///
/// # Errors
///
/// As [`crate::probe_reader`].
pub async fn probe_reader<Reader: AsyncRead + AsyncSeek + Unpin>(reader: Reader) -> Result<MediaReport, Error> {
  Prober::new().tokio().probe_reader(reader).await
}

/// Probes a forward-only Tokio reader with the default options, from where it stands, as
/// [`TokioProber::probe_stream`] says, `&mut reader`, `Box::pin(reader)` and cancellation included.
///
/// # Errors
///
/// As [`crate::probe_stream`].
pub async fn probe_stream<Reader: AsyncRead + Unpin>(reader: Reader) -> Result<MediaReport, Error> {
  Prober::new().tokio().probe_stream(reader).await
}

/// Opens the file at `path` with Tokio's `File` and probes it with the default options, as
/// [`TokioProber::probe_file`] says, the file-name hint and cancellation included.
///
/// # Errors
///
/// As [`crate::probe_file`].
///
/// # Panics
///
/// When the future is polled outside a Tokio runtime, as [`TokioProber::probe_file`] says.
///
/// ```no_run
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let runtime = tokio::runtime::Builder::new_current_thread().build()?;
/// let report = runtime.block_on(cc_media_probe::asynchronous::tokio::probe_file("movie.mkv"))?;
/// println!("{} streams", report.streams().len());
/// # Ok(())
/// # }
/// ```
pub async fn probe_file(path: impl AsRef<Path>) -> Result<MediaReport, Error> {
  Prober::new().tokio().probe_file(path).await
}

/// Detects the format of a seekable Tokio reader with the default options, as [`TokioProber::detect_reader`]
/// says, `&mut reader`, `Box::pin(reader)` and cancellation included.
///
/// # Errors
///
/// `SourceFailed` for a reader that fails, and nothing else.
pub async fn detect_reader<Reader: AsyncRead + AsyncSeek + Unpin>(reader: Reader) -> Result<Detection, Error> {
  Prober::new().tokio().detect_reader(reader).await
}

/// Detects the format of a forward-only Tokio reader with the default options, as [`TokioProber::detect_stream`]
/// says, `&mut reader`, `Box::pin(reader)` and cancellation included.
///
/// # Errors
///
/// `SourceFailed` for a reader that fails, and nothing else.
pub async fn detect_stream<Reader: AsyncRead + Unpin>(reader: Reader) -> Result<Detection, Error> {
  Prober::new().tokio().detect_stream(reader).await
}

/// Opens the file at `path` with Tokio's `File` and detects its format with the default options, as
/// [`TokioProber::detect_file`] says, the file-name hint and cancellation included.
///
/// # Errors
///
/// `SourceFailed` for a file that cannot be opened or that fails, and nothing else.
///
/// # Panics
///
/// When the future is polled outside a Tokio runtime, as [`TokioProber::probe_file`] says.
pub async fn detect_file(path: impl AsRef<Path>) -> Result<Detection, Error> {
  Prober::new().tokio().detect_file(path).await
}

#[cfg(test)]
mod tests {
  use std::future::Future;
  use std::io::{self, Cursor};
  use std::pin::pin;
  use std::task::{Context, Waker};

  use super::{detect_file, detect_reader, detect_stream, probe_file, probe_reader, probe_stream};
  use crate::model::DetectionOutcome;
  use crate::source::SourceIdentifier;
  use crate::tests::TemporaryDirectory;
  use crate::{Detection, Error, ErrorKind, Format, Prober};

  /// Whether the future is `Send`, which the call compiles only for.
  fn is_send<Value: Send>(_: &Value) -> bool {
    true
  }

  #[tokio::test(flavor = "current_thread")]
  async fn probes_every_tokio_input_with_the_default_options() -> io::Result<()> {
    let directory = TemporaryDirectory::new()?;
    let path = directory.join("movie.bin");
    std::fs::write(&path, b"ftyp")?;
    let probed = [
      probe_reader(Cursor::new(b"ftyp")).await.err(),
      probe_stream(tokio::io::empty()).await.err(),
      probe_file(&path).await.err(),
    ];
    let detected = [
      detect_reader(Cursor::new(b"ftyp")).await.ok(),
      detect_stream(tokio::io::empty()).await.ok(),
      detect_file(&path).await.ok(),
    ];
    for error in probed {
      assert_eq!(error.as_ref().map(Error::kind), Some(&ErrorKind::UnknownFormat));
    }
    for detection in detected {
      assert_eq!(
        detection.as_ref().map(Detection::outcome),
        Some(&DetectionOutcome::Unknown)
      );
    }
    Ok(())
  }

  #[tokio::test(flavor = "current_thread")]
  async fn keeps_the_options_of_the_prober() {
    let prober = Prober::new().with_format_hint(Format::Mp4);
    let handle = prober.tokio();
    let mut cursor = Cursor::new(vec![0_u8; 8]);
    let detection = handle.detect_reader(&mut cursor).await.ok();
    assert_eq!(
      detection.as_ref().map(Detection::outcome),
      Some(&DetectionOutcome::HintOnly)
    );
    assert_eq!(cursor.position(), 0);
    let stream = handle.clone().detect_stream(Box::pin(tokio::io::empty())).await.ok();
    assert!(stream.is_some_and(|detection| detection.hint().is_some()));
    let ended = handle.probe_stream(&b"ftyp"[..]).await.err();
    assert!(ended.is_some_and(|error| error.hint().is_some()));
    let future = handle.probe_reader(Cursor::new(Vec::new()));
    assert!(is_send(&future));
    assert!(future.await.is_err());
    assert!(format!("{handle:?}").starts_with("TokioProber"));
  }

  #[tokio::test(flavor = "current_thread")]
  async fn fails_for_a_file_that_cannot_be_opened() -> io::Result<()> {
    let directory = TemporaryDirectory::new()?;
    let missing = directory.join("movie.mp4");
    let errors: Vec<Error> = [probe_file(&missing).await.err(), detect_file(&missing).await.err()]
      .into_iter()
      .flatten()
      .collect();
    assert_eq!(errors.len(), 2);
    for error in errors {
      assert_eq!(error.kind(), &ErrorKind::SourceFailed);
      assert_eq!(error.to_string(), "the source could not be opened");
      assert_eq!(error.source_identifier(), Some(SourceIdentifier::primary()));
    }
    Ok(())
  }

  /// Tokio 1.54.1 panics on the first poll of a file operation outside a runtime, which the "Panics" section of
  /// the path functions states; a version that returned an error instead would fail here.
  #[test]
  fn panics_outside_a_runtime() -> io::Result<()> {
    let directory = TemporaryDirectory::new()?;
    let missing = directory.join("movie.mp4");
    let outcome = std::panic::catch_unwind(|| {
      let mut future = pin!(probe_file(&missing));
      let _ = future.as_mut().poll(&mut Context::from_waker(Waker::noop()));
    });
    assert!(outcome.is_err());
    Ok(())
  }
}
