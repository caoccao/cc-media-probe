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

//! A whole fixture: its header, the line `---`, and its body.

use super::body::{Body, assemble_body};
use super::constants::HEADER_END;
use super::error::{SyntaxError, SyntaxErrorKind};
use super::header::{Header, parse_header};

/// A fixture's header and its assembled body.
///
/// ```
/// # fn main() -> Result<(), cc_media_probe_testkit::syntax::SyntaxError> {
/// use cc_media_probe_testkit::syntax::parse;
///
/// let text = "description: a field\nsource: a section\ntarget: toy\noutcome: report\n---\n## A section\n\
///             2a # a field\n";
/// let parsed = parse(text)?;
/// assert_eq!((parsed.header.description.as_str(), parsed.body.bytes.as_slice()), ("a field", &[0x2a][..]));
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parsed {
  /// What the header states.
  pub header: Header,
  /// The bytes and regions of the body.
  pub body: Body,
}

/// Reads a fixture's text. Lines end with a line feed, with or without a carriage return before it.
///
/// ```
/// # fn main() -> Result<(), cc_media_probe_testkit::syntax::SyntaxError> {
/// use cc_media_probe_testkit::syntax::parse;
///
/// let text = "description: two fields\r\nsource: a section\r\ntarget: toy\r\noutcome: report\r\n\
///             length: 3\r\n---\r\n## A section\r\n00 01 # a field of 16 bits\r\n\"a\" # a letter\r\n";
/// assert_eq!(parse(text)?.body.bytes, [0, 1, b'a']);
/// assert!(parse(&text.replace("length: 3", "length: 4")).is_err());
/// # Ok(())
/// # }
/// ```
///
/// # Errors
///
/// Returns the first thing wrong with the text, with its line.
pub fn parse(text: &str) -> Result<Parsed, SyntaxError> {
  let lines: Vec<(usize, &str)> = text.lines().zip(1..).map(|(line, number)| (number, line)).collect();
  let end = lines
    .iter()
    .position(|(_, line)| line.trim_end() == HEADER_END)
    .ok_or_else(|| SyntaxErrorKind::NoHeaderEnd.at(0))?;
  let (header_lines, rest) = lines.split_at(end);
  let (end_line, body_lines) = rest
    .split_first()
    .map(|((number, _), body)| (*number, body))
    .ok_or_else(|| SyntaxErrorKind::NoHeaderEnd.at(0))?;
  let header = parse_header(header_lines.iter().copied(), end_line)?;
  let body = assemble_body(body_lines.iter().copied())?;
  if let Some(declared) = header.length.filter(|declared| *declared != body.bytes.len()) {
    let assembled = body.bytes.len();
    return Err(SyntaxErrorKind::LengthMismatch { declared, assembled }.at(end_line));
  }
  if header.route.is_none() && body.regions.is_empty() {
    return Err(SyntaxErrorKind::NoTargetWithoutRegion.at(0));
  }
  Ok(Parsed { header, body })
}
