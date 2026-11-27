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

//! What the two samples of detection say. Plain code, shared by the async detection and the hand-written one.

use crate::report::{DetectionOutcome, Reason};
use crate::source::Filled;
use crate::toy::constants::{CODE_LENGTH, ELEMENT_HEADER_LENGTH, FILE_HEADER_LENGTH, KNOWN_CODES};
use crate::toy::{FileHeader, is_header_prefix};

/// What detection concluded, with where the walk starts.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Detected {
  /// A toy file whose first element starts here.
  Identified {
    /// The offset of the first element.
    first_element: u64,
  },
  /// Anything else, which ends the probe with this outcome.
  Concluded(DetectionOutcome),
}

/// What the read of the file header says: the offset of the first element, or a conclusion. A short read is
/// insufficient data only while the bytes read can still begin a toy file.
pub(crate) fn judge_header(filled: &Filled, header: &[u8; FILE_HEADER_LENGTH]) -> Result<u64, DetectionOutcome> {
  let insufficient = |count: usize, reason: Reason| {
    if is_header_prefix(header.get(..count).unwrap_or_default()) {
      DetectionOutcome::InsufficientData(reason)
    } else {
      DetectionOutcome::Unknown
    }
  };
  match filled {
    Filled::Complete => match FileHeader::parse(header) {
      FileHeader::Toy { first_element } => Ok(first_element),
      FileHeader::Other => Err(DetectionOutcome::Unknown),
    },
    Filled::End(count) => Err(insufficient(*count, Reason::TruncatedInput)),
    Filled::Exhausted(count, limit) => Err(insufficient(*count, Reason::Limit(limit.clone()))),
    Filled::Unavailable => Err(DetectionOutcome::InsufficientData(Reason::RangeUnavailable)),
  }
}

/// What the read of the first element's header says. A short read whose bytes no known code begins with already
/// rules the format out, whether the code is whole or not.
pub(crate) fn judge_first_element(
  filled: &Filled,
  element: [u8; ELEMENT_HEADER_LENGTH],
  first_element: u64,
) -> Detected {
  let known = element
    .first_chunk::<CODE_LENGTH>()
    .is_some_and(|code| KNOWN_CODES.contains(code));
  let insufficient = |count: usize, reason: Reason| {
    let read = element.get(..count.min(CODE_LENGTH)).unwrap_or_default();
    if KNOWN_CODES.iter().any(|code| code.starts_with(read)) {
      DetectionOutcome::InsufficientData(reason)
    } else {
      DetectionOutcome::Unknown
    }
  };
  let outcome = match filled {
    Filled::Complete if known => return Detected::Identified { first_element },
    Filled::Complete => DetectionOutcome::Unknown,
    Filled::End(count) => insufficient(*count, Reason::TruncatedInput),
    Filled::Exhausted(count, limit) => insufficient(*count, Reason::Limit(limit.clone())),
    Filled::Unavailable => DetectionOutcome::InsufficientData(Reason::RangeUnavailable),
  };
  Detected::Concluded(outcome)
}
