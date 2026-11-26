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

//! `OwnedText`, the text of a catch-all bound to the enum that read it, which every payload of a catch-all holds.

use std::fmt;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

/// A text that a newer version wrote for a value of the enum `Owner`. The traits compare, hash and show the text
/// alone, and need nothing of `Owner`.
pub(crate) struct OwnedText<Owner> {
  text: String,
  owner: PhantomData<fn() -> Owner>,
}

impl<Owner> OwnedText<Owner> {
  /// The text that a reader of `Owner` kept.
  #[cfg(feature = "serde")]
  pub(crate) fn new(text: String) -> OwnedText<Owner> {
    OwnedText {
      text,
      owner: PhantomData,
    }
  }

  /// The text as written.
  pub(crate) fn as_str(&self) -> &str {
    &self.text
  }
}

impl<Owner> Clone for OwnedText<Owner> {
  fn clone(&self) -> OwnedText<Owner> {
    OwnedText {
      text: self.text.clone(),
      owner: PhantomData,
    }
  }
}

impl<Owner> PartialEq for OwnedText<Owner> {
  fn eq(&self, other: &OwnedText<Owner>) -> bool {
    self.text == other.text
  }
}

impl<Owner> Eq for OwnedText<Owner> {}

impl<Owner> Hash for OwnedText<Owner> {
  fn hash<State: Hasher>(&self, state: &mut State) {
    self.text.hash(state);
  }
}

impl<Owner> fmt::Debug for OwnedText<Owner> {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt::Debug::fmt(&self.text, formatter)
  }
}

#[cfg(test)]
pub(crate) mod tests {
  use std::collections::HashSet;
  use std::marker::PhantomData;

  use super::OwnedText;

  impl<Owner> OwnedText<Owner> {
    /// The text of a value that the reader of `Owner` does not know.
    pub(crate) fn sample(text: &str) -> OwnedText<Owner> {
      OwnedText {
        text: text.to_owned(),
        owner: PhantomData,
      }
    }
  }

  /// An enum that owns no text of its own: the traits need nothing of `Owner`.
  struct Owner;

  #[test]
  fn compares_hashes_and_shows_its_text() {
    let text = OwnedText::<Owner>::sample("hologram");
    assert_eq!(text.as_str(), "hologram");
    assert_eq!(text.clone(), text);
    assert_ne!(text, OwnedText::sample("other"));
    assert_eq!(format!("{text:?}"), "\"hologram\"");
    let set: HashSet<_> = [text.clone(), text].into_iter().collect();
    assert_eq!(set.len(), 1);
  }
}
