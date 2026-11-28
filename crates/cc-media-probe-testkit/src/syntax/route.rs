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

//! Where a seed goes (plan section 9.5): its fuzz target; for a codec target, the representation of its bytes and
//! the context that the container knows; and the outcome that the target's entry point gives it. The header states
//! the route of the whole fixture, or the target `none` for a fixture whose whole bytes are no seed, and each
//! `@seed` directive the route of its region.

use super::constants::{CONTEXT, NO_TARGET, OUTCOME, REPRESENTATION, TARGET};
use super::error::SyntaxErrorKind;
use super::item::hexadecimal_bytes;
use super::names::underscore_name;

/// Where a seed goes, by name: the test kit knows no target, so the library's entry points read the names.
///
/// ```
/// # fn main() -> Result<(), cc_media_probe_testkit::syntax::SyntaxError> {
/// use cc_media_probe_testkit::syntax::{Route, parse};
///
/// let text = "description: a codec header\nsource: a section\ntarget: toy_codec\n\
///             representation: one_byte_lengths\ncontext: 00 0c\noutcome: parsed\n---\n## A section\n\
///             01 02 # a codec header\n";
/// let route = Route {
///     target: "toy_codec".to_owned(),
///     representation: Some("one_byte_lengths".to_owned()),
///     context: vec![0, 0x0c],
///     outcome: "parsed".to_owned(),
/// };
/// assert_eq!(parse(text)?.header.route, Some(route));
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Route {
  /// The fuzz target, such as `toy` or `toy_codec`.
  pub target: String,
  /// For a codec target, the representation of its bytes, such as `one_byte_lengths`.
  pub representation: Option<String>,
  /// For a codec target, the context that the container knows, in the layout that the target defines.
  pub context: Vec<u8>,
  /// The outcome that the target's entry point gives the seed, such as `parsed`.
  pub outcome: String,
}

/// The members of a route while they are read.
#[derive(Debug, Default)]
pub(crate) struct RouteMembers {
  /// The target, once read.
  target: Option<String>,
  /// The representation, once read.
  representation: Option<String>,
  /// The context, once read.
  context: Option<Vec<u8>>,
  /// The outcome, once read.
  outcome: Option<String>,
}

impl RouteMembers {
  /// Takes the member `name` with `value` when it is a route member, and says whether it was. A context is hex words
  /// separated by white space, at least one, so that a member left without its value is refused rather than read as
  /// no context, which only the member's absence gives.
  pub(crate) fn take(&mut self, name: &str, value: &str) -> Result<bool, SyntaxErrorKind> {
    let slot = match name {
      TARGET => &mut self.target,
      REPRESENTATION => &mut self.representation,
      OUTCOME => &mut self.outcome,
      CONTEXT => {
        if value.split_whitespace().next().is_none() {
          return Err(SyntaxErrorKind::EmptyContext);
        }
        let bytes = value
          .split_whitespace()
          .map(hexadecimal_bytes)
          .collect::<Result<Vec<Vec<u8>>, SyntaxErrorKind>>()?;
        return once(&mut self.context, name, bytes.concat()).map(|()| true);
      }
      _ => return Ok(false),
    };
    once(slot, name, underscore_name(value)?).map(|()| true)
  }

  /// The route of a whole fixture: as a region's, or none for the target `none`, which states nothing else.
  pub(crate) fn finish_header(self) -> Result<Option<Route>, SyntaxErrorKind> {
    if self.target.as_deref() != Some(NO_TARGET) {
      return self.finish_region().map(Some);
    }
    if self.representation.is_some() || self.context.is_some() || self.outcome.is_some() {
      return Err(SyntaxErrorKind::RouteOfNoTarget);
    }
    Ok(None)
  }

  /// The route of a region: a target other than `none` and an outcome, and a context only beside a representation.
  pub(crate) fn finish_region(self) -> Result<Route, SyntaxErrorKind> {
    if self.context.is_some() && self.representation.is_none() {
      return Err(SyntaxErrorKind::ContextWithoutRepresentation);
    }
    let target = self.target.ok_or(SyntaxErrorKind::MissingMember(TARGET))?;
    if target == NO_TARGET {
      return Err(SyntaxErrorKind::RegionWithoutTarget);
    }
    Ok(Route {
      target,
      representation: self.representation,
      context: self.context.unwrap_or_default(),
      outcome: self.outcome.ok_or(SyntaxErrorKind::MissingMember(OUTCOME))?,
    })
  }
}

/// Fills `slot` with `value`, which a member may give once.
pub(crate) fn once<Value>(slot: &mut Option<Value>, name: &str, value: Value) -> Result<(), SyntaxErrorKind> {
  if slot.is_some() {
    return Err(SyntaxErrorKind::RepeatedMember(name.to_owned()));
  }
  *slot = Some(value);
  Ok(())
}
