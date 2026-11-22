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

//! Why a document that was read breaks the rules of `api.md` section 12.

use std::fmt;

/// Why a document that was read breaks the rules of `api.md` section 12.
#[derive(Debug)]
pub struct InvalidDocument(&'static str);

impl InvalidDocument {
  /// The rule that the document breaks.
  pub(crate) fn new(message: &'static str) -> InvalidDocument {
    InvalidDocument(message)
  }
}

impl fmt::Display for InvalidDocument {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(self.0)
  }
}
