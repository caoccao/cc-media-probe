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

//! `ToyReport` and its records.

use cc_media_probe::Limit;

use super::{HeaderStatus, Reason};
use crate::budget::Usage;
use crate::codec::{Framing, Representation};
use crate::toy::constants::CODE_LENGTH;

/// What a probe of a toy file found.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ToyReport {
  /// What detection concluded about the primary input.
  pub detection: DetectionOutcome,
  /// The streams of every source, in the order of their declarations.
  pub streams: Vec<StreamReport>,
  /// The titles, read in place or through a reference.
  pub titles: Vec<Title>,
  /// The diagnostics, in the order in which they were raised.
  pub diagnostics: Vec<Diagnostic>,
  /// The identifiers of the companions that the primary input expects, whether or not lookup is on.
  pub expected_companions: Vec<String>,
  /// The source table: the primary input, then every companion requested.
  pub sources: Vec<SourceEntry>,
  /// The counters that ran out.
  pub exhausted: Vec<Limit>,
  /// What the probe charged.
  pub usage: Usage,
}

/// What detection concluded.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DetectionOutcome {
  /// A toy file.
  Identified,
  /// Not a toy file.
  Unknown,
  /// The bytes that would tell were not available, for this reason.
  InsufficientData(Reason),
}

/// One stream and its header.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StreamReport {
  /// The source that declared it.
  pub source: u64,
  /// Its number within that source.
  pub number: u8,
  /// How its bytes are delivered.
  pub framing: Framing,
  /// The form of its bytes.
  pub representation: Representation,
  /// Its codec.
  pub codec: [u8; CODE_LENGTH],
  /// Where its header ended.
  pub header: HeaderStatus,
  /// The packets offered to its assembler.
  pub packets_offered: u64,
}

/// A title.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Title {
  /// The source that holds it.
  pub source: u64,
  /// The text, with any invalid UTF-8 replaced.
  pub text: String,
  /// Whether it was reached through a reference.
  pub referenced: bool,
}

/// A diagnostic, at an offset of its source.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Diagnostic {
  /// The source of the element.
  pub source: u64,
  /// The offset of the element within its source.
  pub offset: u64,
  /// What was found.
  pub kind: DiagnosticKind,
}

/// What a diagnostic says.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DiagnosticKind {
  /// The element, or the element it references, lies behind the forward-only window, so it was not read.
  RangeUnavailable,
  /// An element of an unknown code, skipped.
  UnknownElement([u8; CODE_LENGTH]),
  /// An element whose body runs beyond its list or beyond `u64::MAX`; the rest of the list is skipped.
  ElementOverrunsParent,
  /// A list deeper than the nesting limit, skipped.
  NestingTooDeep,
  /// A body that the walker does not read whole: too short, or too long.
  BadBody,
  /// A packet of a stream that no declaration names.
  UndeclaredStream(u8),
  /// A second declaration of the same stream number, ignored.
  DuplicateStream(u8),
  /// A stream declaration beyond the stream limit, not kept.
  StreamNotKept,
  /// A companion's identifier that fails validation, so no request is made.
  InvalidIdentifier,
  /// A companion that this limit refused before any request was made: no source identifier, no entry.
  CompanionLimited(Limit),
  /// A companion named inside a companion, which the spike does not follow.
  NestedCompanion,
  /// The input ended inside an element.
  TruncatedElement,
}

/// One entry of the source table.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceEntry {
  /// The source identifier: 0 for the primary input, then the requests in order.
  pub identifier: u64,
  /// The name that the resolver was asked for, absent for the primary input.
  pub name: Option<String>,
  /// The outcome.
  pub outcome: SourceOutcome,
  /// The bytes that the source delivered.
  pub bytes_read: u64,
  /// The earlier source whose identity this one has, which is read instead.
  pub alias_of: Option<u64>,
}

/// The outcome of a source.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SourceOutcome {
  /// Opened and read.
  Opened,
  /// Nothing there.
  NotFound,
  /// Refused, with the resolver's reason.
  Declined(String),
  /// Not allowed by this limit, by the resolver's own account.
  LimitReached(Limit),
  /// Failed while it was opened or read; the probe ended with the error.
  Failed,
}
