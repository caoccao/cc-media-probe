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

//! The body of a fixture: headings, lines of bytes with their comments, and the regions that `@seed` and `@end`
//! mark.

use super::constants::{ASSIGNMENT, END, SEED};
use super::error::{SyntaxError, SyntaxErrorKind};
use super::item::assemble_items;
use super::line::{Line, classify};
use super::route::{Route, RouteMembers};

/// The bytes of a body and its regions.
///
/// ```
/// # fn main() -> Result<(), cc_media_probe_testkit::syntax::SyntaxError> {
/// use cc_media_probe_testkit::syntax::parse;
///
/// let text = "description: two fields\nsource: a section\ntarget: toy\noutcome: report\n---\n## A section\n\
///             00 01 # a field of 16 bits\n\"ab\" * 2 # a text, twice\n";
/// let body = parse(text)?.body;
/// assert_eq!(body.bytes, b"\x00\x01abab");
/// assert!(body.regions.is_empty());
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Body {
  /// The assembled bytes.
  pub bytes: Vec<u8>,
  /// The regions, in the order of the body.
  pub regions: Vec<Region>,
}

/// A range of the body's bytes that is a seed of its own, such as a codec header that a container embeds.
///
/// ```
/// # fn main() -> Result<(), cc_media_probe_testkit::syntax::SyntaxError> {
/// use cc_media_probe_testkit::syntax::parse;
///
/// let text = "description: a container\nsource: a section\ntarget: toy\noutcome: report\n---\n## A section\n\
///             00 # a field\n\
///             @seed target=toy_codec representation=one_byte_lengths outcome=parsed\n\
///             01 02 # the codec header\n@end\n";
/// let body = parse(text)?.body;
/// let region = &body.regions[0];
/// assert_eq!((region.start, region.end), (1, 3));
/// assert_eq!(body.bytes.get(region.start..region.end), Some(&[1, 2][..]));
/// assert_eq!(region.route.target, "toy_codec");
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Region {
  /// Where the region's bytes go.
  pub route: Route,
  /// The offset of its first byte.
  pub start: usize,
  /// The offset after its last byte.
  pub end: usize,
}

/// A region that `@seed` opened and no `@end` closed yet.
struct Open {
  /// Where its bytes go.
  route: Route,
  /// The offset of its first byte.
  start: usize,
  /// The line of its `@seed`.
  line: usize,
}

/// Assembles a body from its lines, each with its number.
pub(crate) fn assemble_body<'text>(lines: impl IntoIterator<Item = (usize, &'text str)>) -> Result<Body, SyntaxError> {
  let mut bytes = Vec::new();
  let mut regions = Vec::new();
  let mut open: Option<Open> = None;
  let mut heading = false;
  for (number, text) in lines {
    match classify(text) {
      Line::Blank | Line::Comment => {}
      Line::Heading(text) => {
        if text.is_empty() {
          return Err(SyntaxErrorKind::EmptyHeading.at(number));
        }
        heading = true;
      }
      Line::Directive(words) => {
        let position = (bytes.len(), number);
        open = directive(words, open, position, &mut regions).map_err(|kind| kind.at(number))?;
      }
      Line::Bytes { items, comment } => {
        if comment.is_none() {
          return Err(SyntaxErrorKind::BytesWithoutComment.at(number));
        }
        if !heading {
          return Err(SyntaxErrorKind::BytesBeforeHeading.at(number));
        }
        assemble_items(items, &mut bytes).map_err(|kind| kind.at(number))?;
      }
    }
  }
  if let Some(region) = open {
    return Err(SyntaxErrorKind::UnclosedRegion.at(region.line));
  }
  if bytes.is_empty() {
    return Err(SyntaxErrorKind::NoBytes.at(0));
  }
  Ok(Body { bytes, regions })
}

/// Applies one directive at `(position, line)`: `@seed` opens a region there with the route its `name=value` words
/// give, and a bare `@end` closes the open one. Returns the region left open. The name starts right after the `@`,
/// so `@ seed` names no directive: it opened a region before the second review of plan Phase 3, task 4, found it.
fn directive(
  text: &str,
  open: Option<Open>,
  (position, line): (usize, usize),
  regions: &mut Vec<Region>,
) -> Result<Option<Open>, SyntaxErrorKind> {
  let (name, rest) = text.split_at(text.find(char::is_whitespace).unwrap_or(text.len()));
  let mut words = rest.split_whitespace();
  match name {
    SEED => {
      if open.is_some() {
        return Err(SyntaxErrorKind::NestedRegion);
      }
      let mut route = RouteMembers::default();
      for word in words {
        let (name, value) = word
          .split_once(ASSIGNMENT)
          .ok_or_else(|| SyntaxErrorKind::UnknownMember(word.to_owned()))?;
        if !route.take(name, value)? {
          return Err(SyntaxErrorKind::UnknownMember(name.to_owned()));
        }
      }
      Ok(Some(Open {
        route: route.finish_region()?,
        start: position,
        line,
      }))
    }
    END => {
      if let Some(word) = words.next() {
        return Err(SyntaxErrorKind::EndWithWords(word.to_owned()));
      }
      let region = open.ok_or(SyntaxErrorKind::NoOpenRegion)?;
      if region.start == position {
        return Err(SyntaxErrorKind::EmptyRegion);
      }
      regions.push(Region {
        route: region.route,
        start: region.start,
        end: position,
      });
      Ok(None)
    }
    other => Err(SyntaxErrorKind::UnknownDirective(other.to_owned())),
  }
}
