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

//! The blocking methods of `Prober` (`api.md` sections 4.1 and 4.2), with the signatures of the functions at the
//! crate root. Every reader and source is taken by value, and so is a mutable reference to one (C-RW-VALUE).

use std::fs::File;
use std::io::{Read, Seek};
use std::path::Path;

use super::Prober;
use crate::source::PositionalSource;
use crate::{Detection, Error, MediaReport};

impl Prober {
  /// Probes the file at `path`, and opens no other file (R18). Its final component is the file-name hint unless
  /// the builder sets one; detection consults that hint from plan gate G3.5 on, as
  /// [`Prober::with_file_name_hint`] says.
  ///
  /// # Errors
  ///
  /// An ending of the ordered procedure of `api.md` section 5.1 without a report: `SourceFailed` for a file that
  /// cannot be opened or a source that fails, `UnknownFormat`, `FormatNotDetermined`, `UnsupportedFormat`,
  /// `FeatureDisabled` or `NoStreamInventory`. Until the detection inventory of plan Phase 3, the bytes of every
  /// file match nothing, so a file that opens gives `UnknownFormat`.
  ///
  /// ```no_run
  /// # fn main() -> Result<(), cc_media_probe::Error> {
  /// use cc_media_probe::Prober;
  ///
  /// let report = Prober::new().with_file_name_hint("movie.mkv").probe_file("download.part")?;
  /// println!("{} streams", report.streams().len());
  /// # Ok(())
  /// # }
  /// ```
  pub fn probe_file(&self, path: impl AsRef<Path>) -> Result<MediaReport, Error> {
    let file = File::open(path).map_err(Error::not_opened)?;
    self.probe_reader(file)
  }

  /// Probes a seekable reader, the whole input from its byte 0, wherever the reader stands. Pass `&mut reader` to
  /// keep it; its position afterwards is unspecified.
  ///
  /// # Errors
  ///
  /// As [`Prober::probe_file`], with `SourceFailed` for a reader that fails.
  pub fn probe_reader<Reader: Read + Seek>(&self, reader: Reader) -> Result<MediaReport, Error> {
    self.probe_input(reader)
  }

  /// Probes a forward-only reader, such as standard input, from where it stands, through the sliding window of
  /// D9. Pass `&mut reader` to keep it; it is left after exactly the bytes read from it.
  ///
  /// # Errors
  ///
  /// As [`Prober::probe_file`], with `SourceFailed` for a reader that fails.
  pub fn probe_stream<Reader: Read>(&self, reader: Reader) -> Result<MediaReport, Error> {
    self.probe_input(reader)
  }

  /// Probes bytes in memory, from their position 0. The report owns copies of what it keeps.
  ///
  /// # Errors
  ///
  /// As [`Prober::probe_file`], without `SourceFailed`, since bytes in memory never fail.
  pub fn probe_bytes(&self, bytes: &[u8]) -> Result<MediaReport, Error> {
    self.probe_input(bytes)
  }

  /// Probes the caller's own positional source, from its position 0. Pass `&mut source` to keep it.
  ///
  /// # Errors
  ///
  /// As [`Prober::probe_file`], with `SourceFailed` for a source that fails, in a read or in `length()`, with an
  /// error other than `ErrorKind::Interrupted`, which keeps that error as its cause.
  pub fn probe_source<Source: PositionalSource>(&self, source: Source) -> Result<MediaReport, Error> {
    self.probe_input(source)
  }

  /// Detects the format of the file at `path`, whose final component is the file-name hint unless the builder
  /// sets one; detection consults that hint from plan gate G3.5 on, as [`Prober::with_file_name_hint`] says.
  /// Detection never opens a companion.
  ///
  /// # Errors
  ///
  /// `SourceFailed` for a file that cannot be opened or that fails, and nothing else: every outcome of detection,
  /// a detect-only format included, is a `Detection`.
  ///
  /// ```no_run
  /// # fn main() -> Result<(), cc_media_probe::Error> {
  /// let detection = cc_media_probe::Prober::new().detect_file("movie.mkv")?;
  /// println!("{}", detection.outcome().as_str());
  /// # Ok(())
  /// # }
  /// ```
  pub fn detect_file(&self, path: impl AsRef<Path>) -> Result<Detection, Error> {
    let file = File::open(path).map_err(Error::not_opened)?;
    self.detect_reader(file)
  }

  /// Detects the format of a seekable reader, from its byte 0 as [`Prober::probe_reader`] reads it. Pass
  /// `&mut reader` to keep it; its position afterwards is unspecified.
  ///
  /// # Errors
  ///
  /// `SourceFailed` for a reader that fails, and nothing else.
  pub fn detect_reader<Reader: Read + Seek>(&self, reader: Reader) -> Result<Detection, Error> {
    Ok(self.detect_input(reader))
  }

  /// Detects the format of a forward-only reader, from where it stands, reading only the pinned prefix. Pass
  /// `&mut reader` to keep it; it is left after exactly the bytes read from it.
  ///
  /// # Errors
  ///
  /// `SourceFailed` for a reader that fails, and nothing else.
  pub fn detect_stream<Reader: Read>(&self, reader: Reader) -> Result<Detection, Error> {
    Ok(self.detect_input(reader))
  }

  /// Detects the format of bytes in memory.
  ///
  /// # Errors
  ///
  /// None in practice: bytes in memory never fail, and every outcome of detection is a `Detection`. The `Result`
  /// keeps the signature of the other entry points.
  ///
  /// ```
  /// # fn main() -> Result<(), cc_media_probe::Error> {
  /// use cc_media_probe::model::DetectionOutcome;
  ///
  /// let detection = cc_media_probe::Prober::new().detect_bytes(b"not a media file")?;
  /// assert!(!matches!(detection.outcome(), DetectionOutcome::Identified));
  /// # Ok(())
  /// # }
  /// ```
  pub fn detect_bytes(&self, bytes: &[u8]) -> Result<Detection, Error> {
    Ok(self.detect_input(bytes))
  }

  /// Detects the format of the caller's own positional source, from its position 0. Pass `&mut source` to keep it.
  ///
  /// # Errors
  ///
  /// `SourceFailed` for a source that fails, and nothing else.
  pub fn detect_source<Source: PositionalSource>(&self, source: Source) -> Result<Detection, Error> {
    Ok(self.detect_input(source))
  }
}

#[cfg(test)]
mod tests {
  use std::io::{self, Cursor};

  use crate::model::DetectionOutcome;
  use crate::source::{Ranges, SourceIdentifier};
  use crate::tests::TemporaryDirectory;
  use crate::{Detection, Error, ErrorKind, Format, Prober};

  #[test]
  fn ends_every_blocking_probe_with_an_unknown_format() {
    let prober = Prober::new();
    let mut cursor = Cursor::new(vec![0_u8; 16]);
    let outcomes = [
      prober.probe_reader(&mut cursor).err(),
      prober.probe_stream(io::empty()).err(),
      prober.probe_bytes(b"\x1a\x45\xdf\xa3").err(),
      prober.probe_source(Ranges::new(b"data")).err(),
    ];
    for error in outcomes {
      assert_eq!(error.as_ref().map(Error::kind), Some(&ErrorKind::UnknownFormat));
    }
    assert_eq!(cursor.position(), 0);
  }

  #[test]
  fn detects_nothing_in_every_blocking_input() {
    let prober = Prober::new().with_format_hint(Format::Matroska);
    let outcomes = [
      prober.detect_reader(Cursor::new([0_u8; 4])).ok(),
      prober.detect_stream(io::empty()).ok(),
      prober.detect_bytes(&[]).ok(),
      prober.detect_source(Ranges::new(b"")).ok(),
    ];
    for detection in outcomes {
      assert_eq!(
        detection.as_ref().map(Detection::outcome),
        Some(&DetectionOutcome::HintOnly)
      );
    }
  }

  #[test]
  fn opens_the_file_before_it_detects() -> io::Result<()> {
    let directory = TemporaryDirectory::new()?;
    let path = directory.join("movie.bin");
    std::fs::write(&path, b"RIFF")?;
    let probed = Prober::new().probe_file(&path).err();
    let detected = Prober::new().detect_file(&path).ok();
    assert_eq!(probed.as_ref().map(Error::kind), Some(&ErrorKind::UnknownFormat));
    assert_eq!(
      detected.as_ref().map(Detection::outcome),
      Some(&DetectionOutcome::Unknown)
    );
    Ok(())
  }

  #[test]
  fn fails_for_a_file_that_cannot_be_opened() -> io::Result<()> {
    let directory = TemporaryDirectory::new()?;
    let missing = directory.join("movie.mkv");
    let errors: Vec<Error> = [
      Prober::new().probe_file(&missing).err(),
      Prober::new().detect_file(&missing).err(),
    ]
    .into_iter()
    .flatten()
    .collect();
    assert_eq!(errors.len(), 2);
    for error in errors {
      assert_eq!(error.kind(), &ErrorKind::SourceFailed);
      assert_eq!(error.to_string(), "the source could not be opened");
      assert_eq!(error.source_identifier(), Some(SourceIdentifier::primary()));
      assert_eq!(error.offset(), None);
      assert_eq!(
        error.source_failure().map(io::Error::kind),
        Some(io::ErrorKind::NotFound)
      );
      assert_eq!(error.to_record().source_failure_kind(), Some("not_found"));
    }
    Ok(())
  }
}
