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

//! `HeaderStatus`, where a stream's header ended (plan section 7.4), and `Reason`, why a part fell short.

use cc_media_probe::Limit;

use crate::codec::{Header, Setup};

/// Where a stream's header ended.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HeaderStatus {
  /// Every field was read.
  Parsed(Header),
  /// The setup was read, and the rest needs decoding.
  PartiallyParsedByDesign(Setup),
  /// The setup was read, and the rest lay outside the bytes available, for this reason.
  PartiallyParsedCutShort(Setup, Reason),
  /// No field was read, for this reason.
  NotFound(Reason),
  /// No byte of the header was offered before this reason ended the probe: the source failed before the stream's
  /// first packet (plan section 7.4, "End of input, limits").
  NotAttempted(Reason),
  /// The bytes contradict the syntax.
  Malformed,
  /// The stream's codec is not the toy codec, so no parser runs.
  Unsupported,
}

/// Why a part fell short.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Reason {
  /// The input ended.
  TruncatedInput,
  /// A limit ran out.
  Limit(Limit),
  /// The bytes lay behind the forward-only window (D9).
  RangeUnavailable,
  /// The source failed while the header was read, and the probe ended with the error (R19).
  SourceFailed,
}
