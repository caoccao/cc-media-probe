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

//! An error in a source file that stops it from being analyzed.

use std::fmt;

/// A source file that cannot be tokenized or whose delimiters do not match.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceError {
  /// The line on which the problem was found, counted from one.
  pub line: usize,
  /// What went wrong.
  pub message: &'static str,
}

impl fmt::Display for SourceError {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(self.message)
  }
}

impl std::error::Error for SourceError {}

#[cfg(test)]
mod tests {
  use super::SourceError;

  #[test]
  fn displays_its_message() {
    let error = SourceError {
      line: 3,
      message: "unterminated comment",
    };
    assert_eq!(error.to_string(), "unterminated comment");
  }
}
