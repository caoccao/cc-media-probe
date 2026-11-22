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

//! The classes of characters in Rust source that the tokenizer tells apart.

use crate::constants::punctuation::{APOSTROPHE, UNDERSCORE};

/// Whether `character` can start an identifier.
pub fn is_identifier_start(character: char) -> bool {
  character == UNDERSCORE || character.is_alphabetic()
}

/// Whether `character` can continue an identifier.
pub fn is_identifier_continue(character: char) -> bool {
  character == UNDERSCORE || character.is_alphanumeric()
}

/// Whether `character` closes a character literal.
pub fn is_apostrophe(character: char) -> bool {
  character == APOSTROPHE
}

#[cfg(test)]
mod tests {
  use super::{is_apostrophe, is_identifier_continue, is_identifier_start};

  #[test]
  fn classifies_identifier_characters() {
    assert!(is_identifier_start('_'));
    assert!(is_identifier_start('é'));
    assert!(!is_identifier_start('7'));
    assert!(is_identifier_continue('7'));
    assert!(!is_identifier_continue('-'));
    assert!(is_apostrophe('\''));
  }
}
