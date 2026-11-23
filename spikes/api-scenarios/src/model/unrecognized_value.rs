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

//! The catch-all of an enum that is a string, `api.md` section 1, rule 5: a value that a newer version wrote. Its
//! owning enum is its type parameter, as the maintainer chose at the ninth review, so only a reader of that enum
//! makes one and its text never names a value of another enum.

use std::fmt;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

/// The text that a newer version wrote for a value of the enum `Owner` that this version does not know. It has no
/// serialized form of its own (`api.md` section 3.3): the enum writes the text as its own value.
///
/// A payload goes back into the enum that read it:
///
/// ```
/// use cc_media_probe::StreamKind;
/// use cc_media_probe::model::UnrecognizedValue;
///
/// fn again(value: UnrecognizedValue<StreamKind>) -> StreamKind {
///     StreamKind::Unrecognized(value)
/// }
/// ```
///
/// and never into another, where its text could name a value that the other enum knows:
///
/// ```compile_fail
/// use cc_media_probe::{Limit, StreamKind};
/// use cc_media_probe::model::UnrecognizedValue;
///
/// fn moved(value: UnrecognizedValue<StreamKind>) -> Limit {
///     Limit::Unrecognized(value)
/// }
/// ```
#[cfg_attr(
  feature = "serde",
  doc = "",
  doc = "No caller reads one on its own either, so the text `error` reads as an outcome only into `EntryOutcome::Error`:",
  doc = "",
  doc = "```",
  doc = "use cc_media_probe::document::EntryOutcome;",
  doc = "",
  doc = "let read: EntryOutcome = serde_json::from_str(\"\\\"error\\\"\").unwrap();",
  doc = "assert_eq!(read, EntryOutcome::Error);",
  doc = "```",
  doc = "",
  doc = "```compile_fail",
  doc = "use cc_media_probe::document::EntryOutcome;",
  doc = "use cc_media_probe::model::UnrecognizedValue;",
  doc = "",
  doc = "let forged: UnrecognizedValue<EntryOutcome> = serde_json::from_str(\"\\\"error\\\"\").unwrap();",
  doc = "```"
)]
pub struct UnrecognizedValue<Owner> {
  text: String,
  owner: PhantomData<fn() -> Owner>,
}

impl<Owner> UnrecognizedValue<Owner> {
  /// The text as written.
  pub fn as_str(&self) -> &str {
    &self.text
  }

  /// The text of a value that the reader of `Owner` does not know, as it keeps it.
  #[cfg(feature = "serde")]
  pub(crate) fn new(text: String) -> UnrecognizedValue<Owner> {
    UnrecognizedValue {
      text,
      owner: PhantomData,
    }
  }
}

impl<Owner> Clone for UnrecognizedValue<Owner> {
  fn clone(&self) -> UnrecognizedValue<Owner> {
    UnrecognizedValue {
      text: self.text.clone(),
      owner: PhantomData,
    }
  }
}

impl<Owner> PartialEq for UnrecognizedValue<Owner> {
  fn eq(&self, other: &UnrecognizedValue<Owner>) -> bool {
    self.text == other.text
  }
}

impl<Owner> Eq for UnrecognizedValue<Owner> {}

impl<Owner> Hash for UnrecognizedValue<Owner> {
  fn hash<State: Hasher>(&self, state: &mut State) {
    self.text.hash(state);
  }
}

impl<Owner> fmt::Debug for UnrecognizedValue<Owner> {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt::Debug::fmt(&self.text, formatter)
  }
}
