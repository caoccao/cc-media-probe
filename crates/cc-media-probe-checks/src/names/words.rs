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

//! Splitting a name into its words.

use crate::constants::punctuation::{APOSTROPHE, UNDERSCORE};

/// The words of `name`: split at underscores, at a change from a lower-case
/// letter or digit to an upper-case letter, and before the last capital of
/// an acronym that a lower-case letter follows. Letter-digit changes do not
/// split, so `Id3v2` and `H264` stay one word. A leading apostrophe is
/// dropped.
#[must_use]
pub fn split_words(name: &str) -> Vec<String> {
  let mut words = Vec::new();
  for part in name
    .trim_start_matches(APOSTROPHE)
    .split(UNDERSCORE)
    .filter(|part| !part.is_empty())
  {
    split_camel_case(part, &mut words);
  }
  words
}

fn split_camel_case(part: &str, words: &mut Vec<String>) {
  let characters: Vec<char> = part.chars().collect();
  let mut current = String::new();
  for (index, character) in characters.iter().enumerate() {
    let previous = index.checked_sub(1).and_then(|before| characters.get(before));
    let next = index.checked_add(1).and_then(|after| characters.get(after));
    let ends_lower_case_run = previous.is_some_and(|before| before.is_lowercase() || before.is_ascii_digit());
    let ends_acronym =
      previous.is_some_and(|before| before.is_uppercase()) && next.is_some_and(|after| after.is_lowercase());
    if character.is_uppercase() && !current.is_empty() && (ends_lower_case_run || ends_acronym) {
      words.push(std::mem::take(&mut current));
    }
    current.push(*character);
  }
  if !current.is_empty() {
    words.push(current);
  }
}

#[cfg(test)]
mod tests {
  use super::split_words;

  fn words(name: &str) -> Vec<&'static str> {
    split_words(name)
      .into_iter()
      .map(|word| &*Box::leak(word.into_boxed_str()))
      .collect()
  }

  #[test]
  fn splits_snake_and_camel_case() {
    assert_eq!(words("header_buf"), vec!["header", "buf"]);
    assert_eq!(words("HeaderBuf"), vec!["Header", "Buf"]);
    assert_eq!(words("MAXIMUM_LENGTH"), vec!["MAXIMUM", "LENGTH"]);
    assert_eq!(words("HTTPServer"), vec!["HTTP", "Server"]);
    assert_eq!(words("_private__name_"), vec!["private", "name"]);
  }

  #[test]
  fn keeps_format_names_whole() {
    assert_eq!(words("Id3v2"), vec!["Id3v2"]);
    assert_eq!(words("id3v2_frame"), vec!["id3v2", "frame"]);
    assert_eq!(words("H264Header"), vec!["H264", "Header"]);
    assert_eq!(words("Mp4Box"), vec!["Mp4", "Box"]);
  }

  #[test]
  fn drops_the_apostrophe_of_lifetimes() {
    assert_eq!(words("'input_buf"), vec!["input", "buf"]);
    assert_eq!(words("_"), Vec::<&str>::new());
  }
}
