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

//! The text format of a hand-annotated hex fixture (`docs/design/test-kit.md` section 2): a header of
//! `name: value` members up to the line `---`, among them the cases of its tests, then a body of headings, lines of
//! bytes that each end with a comment naming their field, and the directives `@seed` and `@end` around a region
//! that is a seed of its own.

mod body;
mod case;
mod constants;
mod error;
mod header;
mod item;
mod line;
mod names;
mod parse;
mod route;

pub use body::{Body, Region};
pub use case::{Case, Condition};
pub use error::{SyntaxError, SyntaxErrorKind};
pub use header::Header;
pub use names::{is_hyphenated_name, is_portable_file_name};
pub use parse::{Parsed, parse};
pub use route::Route;
