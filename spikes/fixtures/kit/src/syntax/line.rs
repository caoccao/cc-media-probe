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

//! The kinds of lines of a fixture's body.

use super::constants::{COMMENT, DIRECTIVE, HEADING, QUOTE};

/// One line of a body.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Line<'text> {
  /// White space only.
  Blank,
  /// A comment, which assembles to nothing.
  Comment,
  /// A heading, with its text.
  Heading(&'text str),
  /// A directive, with its words.
  Directive(&'text str),
  /// Items to assemble, and the comment after them, if any.
  Bytes {
    /// The items before the comment.
    items: &'text str,
    /// The comment after them, without its `#`.
    comment: Option<&'text str>,
  },
}

/// Tells what kind of line `line` is.
pub(crate) fn classify(line: &str) -> Line<'_> {
  let trimmed = line.trim();
  if trimmed.is_empty() {
    return Line::Blank;
  }
  if let Some(text) = trimmed.strip_prefix(HEADING) {
    return Line::Heading(text.trim());
  }
  if trimmed.starts_with(COMMENT) {
    return Line::Comment;
  }
  let (content, comment) = split_comment(trimmed);
  if let Some(words) = content.strip_prefix(DIRECTIVE) {
    return Line::Directive(words.trim());
  }
  Line::Bytes {
    items: content.trim(),
    comment: comment.map(str::trim).filter(|text| !text.is_empty()),
  }
}

/// Splits a line at its first comment character outside quoted text.
fn split_comment(line: &str) -> (&str, Option<&str>) {
  let mut quoted = false;
  for (index, character) in line.char_indices() {
    if character == QUOTE {
      quoted = !quoted;
    } else if character == COMMENT && !quoted {
      let (content, rest) = line.split_at(index);
      return (content, rest.strip_prefix(COMMENT));
    }
  }
  (line, None)
}
