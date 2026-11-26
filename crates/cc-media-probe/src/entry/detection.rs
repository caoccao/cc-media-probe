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

//! The five `detect_*` functions at the crate root (`api.md` sections 4.1 and 8), each with the default options.
//! Their only error is a source failure.

use std::io::{Read, Seek};
use std::path::Path;

use crate::source::PositionalSource;
use crate::{Detection, Error, Prober};

/// Detects the format of the file at `path` with the default options. Its final component is the file-name hint,
/// which detection consults from plan gate G3.5 on, as [`Prober::with_file_name_hint`] says.
///
/// # Errors
///
/// `SourceFailed` for a file that cannot be opened or that fails, and nothing else.
///
/// ```no_run
/// # fn main() -> Result<(), cc_media_probe::Error> {
/// let detection = cc_media_probe::detect_file("movie.mkv")?;
/// println!("{:?}", detection.format().map(|format| format.name()));
/// # Ok(())
/// # }
/// ```
pub fn detect_file(path: impl AsRef<Path>) -> Result<Detection, Error> {
  Prober::new().detect_file(path)
}

/// Detects the format of a seekable reader with the default options, from its byte 0, wherever the reader stands.
/// Pass `&mut reader` to keep it; its position afterwards is unspecified.
///
/// # Errors
///
/// `SourceFailed` for a reader that fails, and nothing else.
///
/// ```
/// # fn main() -> Result<(), cc_media_probe::Error> {
/// let detection = cc_media_probe::detect_reader(std::io::Cursor::new(b"not a media file"))?;
/// println!("{}", detection.outcome().as_str());
/// # Ok(())
/// # }
/// ```
pub fn detect_reader<Reader: Read + Seek>(reader: Reader) -> Result<Detection, Error> {
  Prober::new().detect_reader(reader)
}

/// Detects the format of a forward-only reader with the default options, from where it stands, reading only the
/// pinned prefix. Pass `&mut reader` to keep it; it is left after exactly the bytes read from it.
///
/// # Errors
///
/// `SourceFailed` for a reader that fails, and nothing else.
///
/// ```no_run
/// # fn main() -> Result<(), cc_media_probe::Error> {
/// let detection = cc_media_probe::detect_stream(std::io::stdin().lock())?;
/// println!("{}", detection.outcome().as_str());
/// # Ok(())
/// # }
/// ```
pub fn detect_stream<Reader: Read>(reader: Reader) -> Result<Detection, Error> {
  Prober::new().detect_stream(reader)
}

/// Detects the format of bytes in memory with the default options.
///
/// # Errors
///
/// None in practice, since bytes in memory never fail; the `Result` keeps the signature of the other entry points.
///
/// ```
/// # fn main() -> Result<(), cc_media_probe::Error> {
/// let detection = cc_media_probe::detect_bytes(b"not a media file")?;
/// println!("{} bytes read", detection.usage().bytes_read());
/// # Ok(())
/// # }
/// ```
pub fn detect_bytes(bytes: &[u8]) -> Result<Detection, Error> {
  Prober::new().detect_bytes(bytes)
}

/// Detects the format of the caller's own positional source with the default options, from its position 0. Pass
/// `&mut source` to keep it. [`PositionalSource`] shows a source.
///
/// # Errors
///
/// `SourceFailed` for a source that fails, and nothing else.
pub fn detect_source<Source: PositionalSource>(source: Source) -> Result<Detection, Error> {
  Prober::new().detect_source(source)
}

#[cfg(test)]
mod tests {
  use std::io::{self, Cursor};

  use super::{detect_bytes, detect_file, detect_reader, detect_source, detect_stream};
  use crate::Detection;
  use crate::model::DetectionOutcome;
  use crate::source::Ranges;
  use crate::tests::TemporaryDirectory;

  #[test]
  fn detects_with_the_default_options() -> io::Result<()> {
    let directory = TemporaryDirectory::new()?;
    let missing = detect_file(directory.join("movie.mkv"));
    assert!(missing.is_err());
    let outcomes = [
      detect_reader(Cursor::new(b"RIFF")).ok(),
      detect_stream(io::empty()).ok(),
      detect_bytes(b"").ok(),
      detect_source(Ranges::new(b"OggS")).ok(),
    ];
    for detection in outcomes {
      assert_eq!(
        detection.as_ref().map(Detection::outcome),
        Some(&DetectionOutcome::Unknown)
      );
    }
    Ok(())
  }
}
