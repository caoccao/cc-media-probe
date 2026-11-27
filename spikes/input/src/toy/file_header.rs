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

//! The file header of the toy format: the signature, the version, the flags and the offset of the first element.

use super::constants::{
  FILE_HEADER_LENGTH, SIGNATURE, SIGNATURE_LENGTH, VERSION, VERSION_AND_FLAGS_LENGTH, WORD_LENGTH,
};
use crate::utilities::widen;

/// What the file header says.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum FileHeader {
  /// A toy file whose first element starts at this offset.
  Toy {
    /// The offset of the first element, at least the header's length.
    first_element: u64,
  },
  /// Not a toy file of this version.
  Other,
}

impl FileHeader {
  /// Reads the file header.
  pub(crate) fn parse(header: &[u8; FILE_HEADER_LENGTH]) -> FileHeader {
    let Some((signature, rest)) = header.split_first_chunk::<SIGNATURE_LENGTH>() else {
      return FileHeader::Other;
    };
    let Some((&[version, _flags], offset)) = rest.split_first_chunk::<VERSION_AND_FLAGS_LENGTH>() else {
      return FileHeader::Other;
    };
    let Some(offset) = offset.first_chunk::<WORD_LENGTH>() else {
      return FileHeader::Other;
    };
    let first_element = u64::from(u32::from_be_bytes(*offset));
    if *signature != SIGNATURE || version != VERSION || first_element < widen(FILE_HEADER_LENGTH) {
      return FileHeader::Other;
    }
    FileHeader::Toy { first_element }
  }
}

/// Whether `bytes`, the start of a file header that ended early, can still begin a toy file: a start of the
/// signature and, once the version byte is there, the one version. Then detection lacked data, rather than finding
/// another format.
pub(crate) fn is_header_prefix(bytes: &[u8]) -> bool {
  let signature = SIGNATURE.starts_with(bytes.get(..bytes.len().min(SIGNATURE_LENGTH)).unwrap_or_default());
  signature && bytes.get(SIGNATURE_LENGTH).is_none_or(|version| *version == VERSION)
}
