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

//! Messages of the structural analysis.

/// The offset of the token two places after the current one.
pub const TWO_TOKENS_AHEAD: usize = 2;
/// The offset of the token three places after the current one.
pub const THREE_TOKENS_AHEAD: usize = 3;
/// The number of tokens in `::`, `=>` and `->`.
pub const PAIR_LENGTH: usize = 2;
/// A closing delimiter without a matching opening one.
pub const UNEXPECTED_CLOSING_DELIMITER: &str = "closing delimiter without an opening one";
/// An opening delimiter closed by a different kind of delimiter.
pub const MISMATCHED_DELIMITER: &str = "mismatched delimiter";
/// An opening delimiter that is never closed.
pub const UNCLOSED_DELIMITER: &str = "unclosed delimiter";
