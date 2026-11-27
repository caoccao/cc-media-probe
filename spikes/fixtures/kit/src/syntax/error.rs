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

//! What is wrong with a fixture's text, and on which line.

use std::fmt;

use super::constants::{
  BYTES_BEFORE_HEADING, BYTES_WITHOUT_COMMENT, CONTEXT_WITHOUT_REPRESENTATION, EMPTY_HEADING, EMPTY_REGION, EMPTY_TEXT,
  END_WITH_WORDS, LENGTH_MISMATCH, MALFORMED_CASE, MALFORMED_CONDITION, MALFORMED_COUNT, MALFORMED_FILE_NAME,
  MALFORMED_LENGTH, MALFORMED_MEMBER, MALFORMED_NAME, MALFORMED_REPEAT, MISSING_MEMBER, NESTED_REGION, NO_BYTES,
  NO_HEADER_END, NO_OPEN_REGION, NO_TARGET_WITHOUT_REGION, NOT_HEXADECIMAL, ODD_DIGITS, REGION_WITHOUT_TARGET,
  REPEATED_CASE, REPEATED_MEMBER, ROUTE_OF_NO_TARGET, TEXT_BEYOND_ASCII, TOO_LONG, UNCLOSED_REGION, UNKNOWN_DIRECTIVE,
  UNKNOWN_MEMBER, UNTERMINATED_TEXT,
};

/// What is wrong with a fixture's text, with the line where it was found.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyntaxError {
  /// The line, counted from 1, or 0 for the fixture as a whole.
  pub line: usize,
  /// What is wrong.
  pub kind: SyntaxErrorKind,
}

/// What is wrong with a fixture's text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyntaxErrorKind {
  /// The header has no line `---` that ends it.
  NoHeaderEnd,
  /// A header line is not `name: value`.
  MalformedMember,
  /// A member that the format does not define.
  UnknownMember(String),
  /// A member given twice.
  RepeatedMember(String),
  /// A required member that is missing.
  MissingMember(&'static str),
  /// A context without a representation.
  ContextWithoutRepresentation,
  /// A target, representation or outcome that is not lowercase words.
  MalformedName(String),
  /// A length that is not a decimal number.
  MalformedLength,
  /// The body assembles to another length than the header states.
  LengthMismatch {
    /// The length that the header states.
    declared: usize,
    /// The length that the body assembled to.
    assembled: usize,
  },
  /// A line of bytes without a comment that names its field.
  BytesWithoutComment,
  /// A line of bytes before the first heading.
  BytesBeforeHeading,
  /// A heading without text.
  EmptyHeading,
  /// A word that is neither hex bytes nor a repeat.
  NotHexadecimal(String),
  /// Hex digits of an odd count.
  OddDigits(String),
  /// Quoted text without its closing quote.
  UnterminatedText,
  /// Quoted text that holds a character beyond printable ASCII.
  TextBeyondAscii,
  /// Quoted text without a character.
  EmptyText,
  /// A repeat without an item before it or a count after it.
  MalformedRepeat,
  /// A repeat count that is not a decimal number of at least 1.
  MalformedCount(String),
  /// A body of more than 16 MiB.
  TooLong,
  /// A directive that the format does not define.
  UnknownDirective(String),
  /// A region opened inside another.
  NestedRegion,
  /// An end without an open region.
  NoOpenRegion,
  /// A region still open at the end of the body.
  UnclosedRegion,
  /// A region without bytes.
  EmptyRegion,
  /// A body without bytes.
  NoBytes,
  /// An `@end` with a word after it.
  EndWithWords(String),
  /// A file name that the format does not allow.
  MalformedFileName(String),
  /// A case without its name or its conditions.
  MalformedCase(String),
  /// A condition that is not `name=value`.
  MalformedCondition(String),
  /// A case given twice.
  RepeatedCase(String),
  /// A representation, a context or an outcome beside the target `none`.
  RouteOfNoTarget,
  /// A region routed to the target `none`.
  RegionWithoutTarget,
  /// A fixture of the target `none` that marks no region, so that it gives no seed at all.
  NoTargetWithoutRegion,
}

impl SyntaxErrorKind {
  /// The error of this kind on that line.
  pub(crate) fn at(self, line: usize) -> SyntaxError {
    SyntaxError { line, kind: self }
  }

  /// The message for people, and the value that it is about, if any.
  fn message(&self) -> (&'static str, Option<String>) {
    match self {
      SyntaxErrorKind::NoHeaderEnd => (NO_HEADER_END, None),
      SyntaxErrorKind::MalformedMember => (MALFORMED_MEMBER, None),
      SyntaxErrorKind::UnknownMember(name) => (UNKNOWN_MEMBER, Some(name.clone())),
      SyntaxErrorKind::RepeatedMember(name) => (REPEATED_MEMBER, Some(name.clone())),
      SyntaxErrorKind::MissingMember(name) => (MISSING_MEMBER, Some((*name).to_owned())),
      SyntaxErrorKind::ContextWithoutRepresentation => (CONTEXT_WITHOUT_REPRESENTATION, None),
      SyntaxErrorKind::MalformedName(name) => (MALFORMED_NAME, Some(name.clone())),
      SyntaxErrorKind::MalformedLength => (MALFORMED_LENGTH, None),
      SyntaxErrorKind::LengthMismatch { declared, assembled } => {
        (LENGTH_MISMATCH, Some(format!("{declared} != {assembled}")))
      }
      SyntaxErrorKind::BytesWithoutComment => (BYTES_WITHOUT_COMMENT, None),
      SyntaxErrorKind::BytesBeforeHeading => (BYTES_BEFORE_HEADING, None),
      SyntaxErrorKind::EmptyHeading => (EMPTY_HEADING, None),
      SyntaxErrorKind::NotHexadecimal(word) => (NOT_HEXADECIMAL, Some(word.clone())),
      SyntaxErrorKind::OddDigits(word) => (ODD_DIGITS, Some(word.clone())),
      SyntaxErrorKind::UnterminatedText => (UNTERMINATED_TEXT, None),
      SyntaxErrorKind::TextBeyondAscii => (TEXT_BEYOND_ASCII, None),
      SyntaxErrorKind::EmptyText => (EMPTY_TEXT, None),
      SyntaxErrorKind::MalformedRepeat => (MALFORMED_REPEAT, None),
      SyntaxErrorKind::MalformedCount(word) => (MALFORMED_COUNT, Some(word.clone())),
      SyntaxErrorKind::TooLong => (TOO_LONG, None),
      SyntaxErrorKind::UnknownDirective(word) => (UNKNOWN_DIRECTIVE, Some(word.clone())),
      SyntaxErrorKind::NestedRegion => (NESTED_REGION, None),
      SyntaxErrorKind::NoOpenRegion => (NO_OPEN_REGION, None),
      SyntaxErrorKind::UnclosedRegion => (UNCLOSED_REGION, None),
      SyntaxErrorKind::EmptyRegion => (EMPTY_REGION, None),
      SyntaxErrorKind::NoBytes => (NO_BYTES, None),
      SyntaxErrorKind::EndWithWords(word) => (END_WITH_WORDS, Some(word.clone())),
      SyntaxErrorKind::MalformedFileName(name) => (MALFORMED_FILE_NAME, Some(name.clone())),
      SyntaxErrorKind::MalformedCase(value) => (MALFORMED_CASE, Some(value.clone())),
      SyntaxErrorKind::MalformedCondition(word) => (MALFORMED_CONDITION, Some(word.clone())),
      SyntaxErrorKind::RepeatedCase(name) => (REPEATED_CASE, Some(name.clone())),
      SyntaxErrorKind::RouteOfNoTarget => (ROUTE_OF_NO_TARGET, None),
      SyntaxErrorKind::RegionWithoutTarget => (REGION_WITHOUT_TARGET, None),
      SyntaxErrorKind::NoTargetWithoutRegion => (NO_TARGET_WITHOUT_REGION, None),
    }
  }
}

impl fmt::Display for SyntaxError {
  /// Writes the line, the message and the value that it is about: `12: hex bytes are pairs of digits: 0a0`.
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    let line = self.line;
    match self.kind.message() {
      (message, Some(value)) => write!(formatter, "{line}: {message}: {value}"),
      (message, None) => write!(formatter, "{line}: {message}"),
    }
  }
}

impl std::error::Error for SyntaxError {}
