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

//! Detection of a toy file in memory (`spikes/input/README.md` section 2), as the I/O spike decides it
//! (`spikes/input/src/detection/decide.rs`): the file header, then the first element's code, which must be one of
//! the six. Bytes too few to tell give insufficient data while they can still begin a toy file, and unknown once
//! they rule it out, so a file of its header alone, or whose first element lies beyond its end, is never a toy file
//! that the walk certifies.

use super::constants::{
  CODE_LENGTH, ELEMENT_HEADER_LENGTH, FIRST_ELEMENT_FIELD, HEADER_LENGTH, KNOWN_CODES, SIGNATURE, SIGNATURE_LENGTH,
  VERSION, WORD_LENGTH,
};

/// What detection concluded.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Detected {
  /// A toy file whose first element starts here.
  Toy {
    /// The offset of the first element.
    first_element: usize,
  },
  /// Bytes that are not a toy file.
  Unknown,
  /// Bytes too few to tell, which can still begin a toy file.
  InsufficientData,
}

/// Detects a toy file at the start of `bytes`.
pub(crate) fn detect(bytes: &[u8]) -> Detected {
  let Some(header) = bytes.first_chunk::<HEADER_LENGTH>() else {
    return insufficient_or_unknown(is_header_prefix(bytes));
  };
  let Some(first_element) = first_element(header) else {
    return Detected::Unknown;
  };
  let element = bytes.get(first_element..).unwrap_or_default();
  let code = element.get(..element.len().min(CODE_LENGTH)).unwrap_or_default();
  if element.len() >= ELEMENT_HEADER_LENGTH {
    return if KNOWN_CODES.iter().any(|known| known == code) {
      Detected::Toy { first_element }
    } else {
      Detected::Unknown
    };
  }
  insufficient_or_unknown(KNOWN_CODES.iter().any(|known| known.starts_with(code)))
}

/// Insufficient data where the bytes can still begin a toy file, and unknown otherwise.
fn insufficient_or_unknown(possible: bool) -> Detected {
  if possible {
    Detected::InsufficientData
  } else {
    Detected::Unknown
  }
}

/// The first element's offset that a whole file header gives, or `None` for another signature or version, or an
/// offset inside the header.
fn first_element(header: &[u8; HEADER_LENGTH]) -> Option<usize> {
  let (signature, _) = header.split_first_chunk::<SIGNATURE_LENGTH>()?;
  let version = header.get(SIGNATURE_LENGTH)?;
  let field = header.get(FIRST_ELEMENT_FIELD..)?.first_chunk::<WORD_LENGTH>()?;
  let offset = usize::try_from(u32::from_be_bytes(*field)).ok()?;
  (*signature == SIGNATURE && *version == VERSION && offset >= HEADER_LENGTH).then_some(offset)
}

/// Whether `bytes`, a file header that ended early, can still begin a toy file: a start of the signature and, once
/// the version byte is there, the one version.
fn is_header_prefix(bytes: &[u8]) -> bool {
  let signature = SIGNATURE.starts_with(bytes.get(..bytes.len().min(SIGNATURE_LENGTH)).unwrap_or_default());
  signature && bytes.get(SIGNATURE_LENGTH).is_none_or(|version| *version == VERSION)
}
