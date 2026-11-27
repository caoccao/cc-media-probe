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

//! The approved copyright header, limits and messages of the structure rules.

/// The first line of the copyright header.
pub const HEADER_OPENING: &str = "/*";
/// The copyright line before its year or year range.
pub const COPYRIGHT_PREFIX: &str = "* Copyright (c) ";
/// The copyright line after its year or year range.
pub const COPYRIGHT_SUFFIX: &str = ". caoccao.com Sam Cao";
/// Separates the years of a year range.
pub const YEAR_SEPARATOR: char = '-';
/// The number of digits of a year.
pub const YEAR_DIGITS: usize = 4;
/// The lines of the copyright header after the copyright line.
pub const HEADER_BODY: &[&str] = &[
  "* All rights reserved.",
  "",
  "* Licensed under the Apache License, Version 2.0 (the \"License\");",
  "* you may not use this file except in compliance with the License.",
  "* You may obtain a copy of the License at",
  "",
  "* http://www.apache.org/licenses/LICENSE-2.0",
  "",
  "* Unless required by applicable law or agreed to in writing, software",
  "* distributed under the License is distributed on an \"AS IS\" BASIS,",
  "* WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.",
  "* See the License for the specific language governing permissions and",
  "* limitations under the License.",
  "*/",
];
/// The line of the copyright line, counted from one.
pub const COPYRIGHT_LINE_NUMBER: usize = 2;
/// The line of the first body line, counted from one.
pub const HEADER_BODY_FIRST_LINE_NUMBER: usize = 3;
/// A file without the copyright header.
pub const MISSING_HEADER: &str = "the file does not start with the copyright header";
/// A copyright line with a malformed year or year range.
pub const MALFORMED_COPYRIGHT: &str = "the copyright line needs a year or a year range, such as 2026 or 2026-2027";
/// A header line that differs from the approved text.
pub const DIFFERENT_HEADER_LINE: &str = "the copyright header differs from the approved text";

/// The most lines a file may have before its embedded tests (R31).
pub const MAXIMUM_COUNTED_LINES: usize = 500;
/// A file over the limit of R31.
pub const LINE_LIMIT_EXCEEDED: &str = "more lines before the embedded tests than R31 allows";

/// A definition in a module root.
pub const DEFINITION_IN_MODULE_ROOT: &str = "a module root may only declare modules, import and re-export";
/// An inline module body in a module root.
pub const INLINE_MODULE_IN_MODULE_ROOT: &str = "inline module body in a module root; give the module a file of its own";
/// A macro call or definition in a module root.
pub const MACRO_IN_MODULE_ROOT: &str = "macro call or definition in a module root";
/// A `main` function that does more than hand over.
pub const MAIN_MUST_HAND_OVER: &str = "`main` may only hand over to the crate's modules";
/// The most statements `main` may hold.
pub const MAXIMUM_MAIN_STATEMENTS: usize = 1;

/// A `#[path]` attribute.
pub const PATH_ATTRIBUTE_FORBIDDEN: &str = "`#[path]` attributes are not used (R31)";
/// An `include!` call.
pub const INCLUDE_FORBIDDEN: &str = "`include!` of Rust source is not used (R31)";

/// The library's source files, relative to the repository root, with `/`: the files whose fields stay private.
pub const LIBRARY_SOURCE_PREFIX: &str = "crates/cc-media-probe/src/";
/// A field of the library declared `pub` without a restriction (plan section 8.1).
pub const PUBLIC_FIELD: &str =
  "a public field; keep it private, or within the crate, and give it an accessor (plan section 8.1)";

/// A file named like a directory module beside that directory.
pub const MODULE_FILE_BESIDE_DIRECTORY: &str = "module file beside a directory of the same name; use name/mod.rs (R30)";
