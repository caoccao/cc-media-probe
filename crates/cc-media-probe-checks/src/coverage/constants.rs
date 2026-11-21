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

//! Crates, areas, thresholds and messages of the coverage check.

/// The published crates, held to the bar as a whole (R28).
pub const PUBLISHED_CRATES: &[&str] = &["cc-media-probe", "cc-media-probe-cli"];
/// The library crate, whose areas are held to the bar too.
pub const LIBRARY_CRATE: &str = "cc-media-probe";
/// The areas of the library held to the bar (plan section 9.3).
pub const LIBRARY_AREAS: &[&str] = &[
  "containers",
  "codecs",
  "tags",
  "text",
  "disc",
  "detect",
  "registry",
  "input",
  "utilities",
];
/// Coverage must be above this many parts …
pub const THRESHOLD_NUMERATOR: u128 = 9;
/// … of this many (90%, where exactly 90.0% fails).
pub const THRESHOLD_DENOMINATOR: u128 = 10;
/// Percentages are shown in hundredths of a percent.
pub const BASIS_POINTS: u128 = 10_000;
/// Hundredths per percent.
pub const HUNDREDTHS: u128 = 100;
/// The record that names a source file.
pub const SOURCE_FILE_PREFIX: &str = "SF:";
/// The record that gives the hits of one line.
pub const LINE_DATA_PREFIX: &str = "DA:";
/// The record that ends a file.
pub const END_OF_RECORD: &str = "end_of_record";
/// Separates the fields of a line record.
pub const FIELD_SEPARATOR: char = ',';
/// Separates Windows path components.
pub const WINDOWS_SEPARATOR: char = '\\';
/// A line record that cannot be read.
pub const MALFORMED_LINE_RECORD: &str = "malformed line record in the coverage report";
/// A line record outside a file record.
pub const LINE_RECORD_OUTSIDE_FILE: &str = "line record outside a file record";
/// A crate or area at or below the bar.
pub const BELOW_THRESHOLD: &str = "line coverage is not above 90% (R28)";
/// A crate or area without executable lines.
pub const NO_EXECUTABLE_LINES: &str = "no executable lines";
/// Prefix of a crate's directory.
pub const CRATE_DIRECTORY_PREFIX: &str = "crates/";
/// The components of `crates/<name>` before a crate's own paths.
pub const CRATE_PATH_COMPONENTS: usize = 2;
/// The source directory inside a crate.
pub const SOURCE_DIRECTORY_SEGMENT: &str = "src";
