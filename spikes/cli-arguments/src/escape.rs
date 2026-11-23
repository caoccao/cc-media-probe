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

//! The escaping of `docs/design/cli.md` section 8.1, applied only where text reaches a terminal: a message is built
//! from the decoded argument, and standard error writes it escaped, while a JSON document holds it as it is (ninth
//! review).

use std::fmt::Write;

use crate::constants::BIDIRECTIONAL_CONTROLS;

/// The text with every control character and every bidirectional control written as `\u{…}`, the form of
/// `char::escape_unicode`. Every other character, the backslash included, is written as it is.
pub fn for_terminal(text: &str) -> String {
  let mut escaped = String::with_capacity(text.len());
  for character in text.chars() {
    if character.is_control() || BIDIRECTIONAL_CONTROLS.contains(&character) {
      let _ = write!(escaped, "{}", character.escape_unicode());
    } else {
      escaped.push(character);
    }
  }
  escaped
}
