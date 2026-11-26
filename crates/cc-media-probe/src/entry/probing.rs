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

//! The five `probe_*` functions at the crate root (`api.md` section 4.1), each with the default options. No
//! function panics on any input, and a report never borrows its input.

use std::io::{Read, Seek};
use std::path::Path;

use crate::source::PositionalSource;
use crate::{Error, MediaReport, Prober};

/// Probes the file at `path` with the default options, and opens no other file (R18). Its final component is the
/// file-name hint, which detection consults from plan gate G3.5 on, as [`Prober::with_file_name_hint`] says.
/// [`Prober::probe_file`] takes the options of a builder.
///
/// # Errors
///
/// An ending of the ordered procedure without a report, as [`Prober::probe_file`] lists them: `SourceFailed` for
/// a file that cannot be opened, the error that `Error::to_string()` writes as "the source could not be opened".
///
/// ```no_run
/// # fn main() -> Result<(), cc_media_probe::Error> {
/// let report = cc_media_probe::probe_file("movie.mkv")?;
/// for stream in report.streams() {
///     println!("{} {}", stream.index(), stream.kind().as_str());
/// }
/// # Ok(())
/// # }
/// ```
pub fn probe_file(path: impl AsRef<Path>) -> Result<MediaReport, Error> {
  Prober::new().probe_file(path)
}

/// Probes a seekable reader with the default options: the whole input from its byte 0, wherever the reader
/// stands. Pass `&mut reader` to keep it; its position afterwards is unspecified.
///
/// # Errors
///
/// As [`probe_file`], with `SourceFailed` for a reader that fails.
///
/// ```no_run
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use std::fs::File;
///
/// let report = cc_media_probe::probe_reader(File::open("movie.mkv")?)?;
/// println!("{}", report.format());
/// # Ok(())
/// # }
/// ```
pub fn probe_reader<Reader: Read + Seek>(reader: Reader) -> Result<MediaReport, Error> {
  Prober::new().probe_reader(reader)
}

/// Probes a forward-only reader, such as standard input, with the default options, from where it stands. Pass
/// `&mut reader` to keep it; it is left after exactly the bytes read from it.
///
/// # Errors
///
/// As [`probe_file`], with `SourceFailed` for a reader that fails.
///
/// ```no_run
/// # fn main() -> Result<(), cc_media_probe::Error> {
/// let report = cc_media_probe::probe_stream(std::io::stdin().lock())?;
/// println!("{} bytes read", report.usage().bytes_read());
/// # Ok(())
/// # }
/// ```
pub fn probe_stream<Reader: Read>(reader: Reader) -> Result<MediaReport, Error> {
  Prober::new().probe_stream(reader)
}

/// Probes bytes in memory with the default options. The report owns copies of what it keeps; it does not borrow
/// `bytes`.
///
/// # Errors
///
/// As [`probe_file`], without `SourceFailed`, since bytes in memory never fail.
///
/// ```no_run
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let bytes = std::fs::read("movie.mkv")?;
/// let report = cc_media_probe::probe_bytes(&bytes)?;
/// println!("{}: {} streams", report.format(), report.streams().len());
/// # Ok(())
/// # }
/// ```
pub fn probe_bytes(bytes: &[u8]) -> Result<MediaReport, Error> {
  Prober::new().probe_bytes(bytes)
}

/// Probes the caller's own positional source with the default options, from its position 0. Pass `&mut source`
/// to keep it. [`PositionalSource`] shows a source.
///
/// # Errors
///
/// As [`probe_file`], with `SourceFailed` for a source that fails, which keeps the source's error as its cause.
pub fn probe_source<Source: PositionalSource>(source: Source) -> Result<MediaReport, Error> {
  Prober::new().probe_source(source)
}

#[cfg(test)]
mod tests {
  use std::io::{self, Cursor};

  use super::{probe_bytes, probe_file, probe_reader, probe_source, probe_stream};
  use crate::source::Ranges;
  use crate::tests::TemporaryDirectory;
  use crate::{Error, ErrorKind};

  #[test]
  fn probes_with_the_default_options() -> io::Result<()> {
    let directory = TemporaryDirectory::new()?;
    let outcomes = [
      probe_file(directory.join("movie.mkv")).err(),
      probe_reader(Cursor::new(b"RIFF")).err(),
      probe_stream(io::empty()).err(),
      probe_bytes(b"").err(),
      probe_source(Ranges::new(b"OggS")).err(),
    ];
    let kinds: Vec<Option<&ErrorKind>> = outcomes.iter().map(|error| error.as_ref().map(Error::kind)).collect();
    assert_eq!(
      kinds,
      [
        Some(&ErrorKind::SourceFailed),
        Some(&ErrorKind::UnknownFormat),
        Some(&ErrorKind::UnknownFormat),
        Some(&ErrorKind::UnknownFormat),
        Some(&ErrorKind::UnknownFormat),
      ]
    );
    Ok(())
  }
}
