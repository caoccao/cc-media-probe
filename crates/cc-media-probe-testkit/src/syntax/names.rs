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

//! The names of the format: a target, a representation or an outcome is lowercase words joined by `_`; a fixture,
//! its format's folder and a case are lowercase words joined by `-`; and a file name is the name of the input as a
//! file, which every adapter of a parity run is given. Each name that becomes a file or a folder is one that every
//! platform can write (`docs/design/test-kit.md` section 2.1).

use super::constants::{HYPHEN, MAXIMUM_FILE_NAME_LENGTH, NAME_SEPARATOR, PERIOD, RESERVED_DEVICE_NAMES, UNDERSCORE};
use super::error::SyntaxErrorKind;

/// A name of words joined by `_`, such as `one_byte_lengths`.
pub(crate) fn underscore_name(value: &str) -> Result<String, SyntaxErrorKind> {
  if words(value, NAME_SEPARATOR) {
    Ok(value.to_owned())
  } else {
    Err(SyntaxErrorKind::MalformedName(value.to_owned()))
  }
}

/// Whether `text` is lowercase words and digits joined by `-`, as the name of a fixture, of its format's folder and
/// of a case is: `setup-and-parameters`. A device name that Windows reserves, such as `nul`, is none, since
/// `nul.hex` names the device there.
///
/// ```
/// use cc_media_probe_testkit::syntax::is_hyphenated_name;
///
/// assert!(is_hyphenated_name("setup-and-parameters"));
/// assert!(is_hyphenated_name("h264"));
/// assert!(!is_hyphenated_name("Setup"));
/// assert!(!is_hyphenated_name("setup--parameters"));
/// assert!(!is_hyphenated_name("nul"));
/// ```
#[must_use]
pub fn is_hyphenated_name(text: &str) -> bool {
  words(text, HYPHEN) && !is_reserved_device_name(text)
}

/// Whether `text` is a file name that every platform can write: ASCII letters, digits, `.`, `-` and `_`, at most
/// 255 bytes, ending with no `.`, so neither `.` nor `..`, and whose part before the first `.` is no device name
/// that Windows reserves, compared without regard to ASCII case, so neither `NUL.idx` nor `aux`. Its case is kept,
/// since a file-name hint may differ in case.
///
/// ```
/// use cc_media_probe_testkit::syntax::is_portable_file_name;
///
/// assert!(is_portable_file_name("Movie.IDX"));
/// assert!(is_portable_file_name("setup-and-parameters"));
/// assert!(!is_portable_file_name("NUL.idx"));
/// assert!(!is_portable_file_name("a/b.idx"));
/// assert!(!is_portable_file_name("movie."));
/// ```
#[must_use]
pub fn is_portable_file_name(text: &str) -> bool {
  let allowed = |character: char| {
    character.is_ascii_alphanumeric() || character == PERIOD || character == HYPHEN || character == UNDERSCORE
  };
  !text.is_empty()
    && text.len() <= MAXIMUM_FILE_NAME_LENGTH
    && text.chars().all(allowed)
    && !text.ends_with(PERIOD)
    && !is_reserved_device_name(text)
}

/// A file name that every platform can write, such as `movie.idx`.
pub(crate) fn file_name(value: &str) -> Result<String, SyntaxErrorKind> {
  if is_portable_file_name(value) {
    Ok(value.to_owned())
  } else {
    Err(SyntaxErrorKind::MalformedFileName(value.to_owned()))
  }
}

/// Whether the part of `text` before its first `.` is a device name that Windows reserves with any extension.
fn is_reserved_device_name(text: &str) -> bool {
  let stem = text.split_once(PERIOD).map_or(text, |(stem, _)| stem);
  RESERVED_DEVICE_NAMES
    .iter()
    .any(|reserved| reserved.eq_ignore_ascii_case(stem))
}

/// Whether `text` is lowercase words and digits joined by `separator`, none of them empty.
fn words(text: &str, separator: char) -> bool {
  text.split(separator).all(|word| {
    !word.is_empty()
      && word
        .chars()
        .all(|character| character.is_ascii_lowercase() || character.is_ascii_digit())
  })
}
