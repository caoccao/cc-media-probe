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

//! The input encodings of the targets, written once here for the targets and the seed check (R36):
//! - a container target takes the file's bytes as they are;
//! - a codec target takes one byte that names the representation, then a context of the length and layout that the
//!   target defines, then the bytes;
//! - the detection and probe targets take one byte that gives the length of a file-name hint, 0 for none, then the
//!   hint, then the bytes, so that a hint reaches the outcomes that only a hint gives. A seed's hint is its fixture's
//!   file name in UTF-8. On Unix, where the fuzz runs, the hint's bytes are the native name as they stand, so a name
//!   that is not UTF-8 reaches the library as `probe_file` would give it; elsewhere such a hint is rejected, since
//!   no other platform's native name is bytes (`docs/design/test-kit.md` section 6.2).
//!
//! An encoded input is at most the fuzz run's input length.

use std::ffi::OsString;

use super::constants::{MAXIMUM_INPUT_LENGTH, TOY_CODEC_CONTEXT_LENGTH};
use super::targets::Target;
use crate::toy::{Context, Framing, Input, PACKET_MODE, Representation, STREAM_MODE};

/// Why a seed's route cannot be encoded for its target.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EncodingError {
  /// A codec target's route without a representation.
  MissingRepresentation,
  /// A representation that the target does not have.
  UnknownRepresentation(String),
  /// A representation or a context for a target that takes neither.
  UnexpectedRoute,
  /// A context of another length than the target's.
  ContextLength(usize),
  /// A file name longer than its length byte can give.
  FileNameLength(usize),
  /// An encoded input beyond the fuzz run's input length, with its length.
  TooLong(usize),
}

/// Encodes a seed for `target` from its route's representation and context, the input's file name and its bytes.
///
/// # Errors
///
/// Returns why the route does not fit the target, or why the input would be too long for a fuzz run.
pub fn encode(
  target: Target,
  representation: Option<&str>,
  context: &[u8],
  file_name: &str,
  payload: &[u8],
) -> Result<Vec<u8>, EncodingError> {
  let takes_route = target == Target::ToyCodec;
  if !takes_route && (representation.is_some() || !context.is_empty()) {
    return Err(EncodingError::UnexpectedRoute);
  }
  let input = match target {
    Target::Toy => payload.to_vec(),
    Target::ToyCodec => {
      let name = representation.ok_or(EncodingError::MissingRepresentation)?;
      let representation =
        Representation::from_name(name).ok_or_else(|| EncodingError::UnknownRepresentation(name.to_owned()))?;
      if context.len() != TOY_CODEC_CONTEXT_LENGTH {
        return Err(EncodingError::ContextLength(context.len()));
      }
      [&[representation.code()][..], context, payload].concat()
    }
    Target::Detection | Target::Probe => {
      let length = u8::try_from(file_name.len()).map_err(|_| EncodingError::FileNameLength(file_name.len()))?;
      [&[length][..], file_name.as_bytes(), payload].concat()
    }
  };
  if input.len() > MAXIMUM_INPUT_LENGTH {
    return Err(EncodingError::TooLong(input.len()));
  }
  Ok(input)
}

/// The input, context and bytes of a toy codec input, or `None` for one that its encoding rejects: an unknown
/// representation or framing, or a context whose packet length is not the bytes' own in packet mode, where the
/// entry point offers the bytes as one whole packet, or not 0 in stream mode, which knows no packet's length. So a
/// seed cannot claim a packet of another length than it holds.
pub(crate) fn decode_codec(input: &[u8]) -> Option<(Input, Context, &[u8])> {
  let (code, rest) = input.split_first()?;
  let representation = Representation::from_code(*code)?;
  let (context, payload) = rest.split_at_checked(TOY_CODEC_CONTEXT_LENGTH)?;
  let (framing, length) = context.split_first()?;
  let packet_length = usize::try_from(u32::from_be_bytes(length.try_into().ok()?)).ok()?;
  let framing = match *framing {
    PACKET_MODE => Framing::Packet,
    STREAM_MODE => Framing::Stream,
    _ => return None,
  };
  let stated = match framing {
    Framing::Packet => payload.len(),
    Framing::Stream => 0,
  };
  if packet_length != stated {
    return None;
  }
  let context = Context {
    packet_length: (framing == Framing::Packet).then_some(packet_length),
  };
  Some((
    Input {
      framing,
      representation,
    },
    context,
    payload,
  ))
}

/// The file-name hint and the bytes of a detection or probe input, or `None` for one whose hint runs beyond it, or,
/// on a platform whose native names are not bytes, is not UTF-8.
pub(crate) fn decode_named(input: &[u8]) -> Option<(Option<OsString>, &[u8])> {
  let (length, rest) = input.split_first()?;
  let (name, payload) = rest.split_at_checked(usize::from(*length))?;
  if name.is_empty() {
    return Some((None, payload));
  }
  #[cfg(unix)]
  let hint = {
    use std::os::unix::ffi::OsStringExt;
    OsString::from_vec(name.to_vec())
  };
  #[cfg(not(unix))]
  let hint = OsString::from(String::from_utf8(name.to_vec()).ok()?);
  Some((Some(hint), payload))
}

#[cfg(test)]
mod tests {
  use super::decode_named;

  #[test]
  fn a_hint_of_utf_8_is_that_name() {
    let (hint, payload) = decode_named(b"\x09movie.mkvdata").unwrap();
    assert_eq!(hint.as_deref(), Some("movie.mkv".as_ref()));
    assert_eq!(payload, b"data");
    assert_eq!(decode_named(b"\x00data").unwrap().0, None);
    assert_eq!(decode_named(b"\x09movie"), None);
  }

  #[cfg(unix)]
  #[test]
  fn a_hint_that_is_not_utf_8_is_the_native_name_as_it_stands_on_unix() {
    use std::os::unix::ffi::OsStrExt;

    let (hint, _) = decode_named(b"\x05m\xffkv.data").unwrap();
    let hint = hint.unwrap();
    assert_eq!(hint.as_bytes(), b"m\xffkv.");
    assert_eq!(hint.to_str(), None);
  }

  #[cfg(not(unix))]
  #[test]
  fn a_hint_that_is_not_utf_8_is_rejected_where_names_are_not_bytes() {
    assert_eq!(decode_named(b"\x05m\xffkv.data"), None);
  }
}
