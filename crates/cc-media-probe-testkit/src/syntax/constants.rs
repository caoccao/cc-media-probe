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

//! The characters, words, bounds and messages of the fixture text format (`docs/design/test-kit.md` section 2).

/// The line that ends the header and starts the body.
pub const HEADER_END: &str = "---";
/// Separates a header member's name from its value.
pub const MEMBER_SEPARATOR: char = ':';
/// Starts a comment, which runs to the end of its line, outside quoted text.
pub const COMMENT: char = '#';
/// Starts a heading, which names the specification and section of the lines of bytes below it.
pub const HEADING: &str = "##";
/// Starts a directive.
pub const DIRECTIVE: char = '@';
/// Opens and closes quoted text.
pub const QUOTE: char = '"';
/// Repeats the item before it as many times as the count after it says.
pub const REPEAT: &str = "*";
/// Separates a route member's name from its value in a directive.
pub const ASSIGNMENT: char = '=';
/// Separates the words of a target, a representation or an outcome.
pub const NAME_SEPARATOR: char = '_';
/// Separates the words of a fixture's name, of its format's folder and of a case.
pub const HYPHEN: char = '-';
/// A character that a file name may hold, which separates its extension.
pub const PERIOD: char = '.';
/// A character that a file name may hold.
pub const UNDERSCORE: char = '_';

/// The directive that opens a region whose bytes are a seed of their own.
pub const SEED: &str = "seed";
/// The directive that closes the open region.
pub const END: &str = "end";

/// The header member that says in one line what the fixture holds.
pub const DESCRIPTION: &str = "description";
/// The header member that names the specification and section that the fixture comes from as a whole.
pub const SOURCE: &str = "source";
/// The header member, which may repeat, that names an erratum that the fixture depends on.
pub const ERRATUM: &str = "erratum";
/// The route member that names the fuzz target.
pub const TARGET: &str = "target";
/// The target of a fixture whose whole bytes are no seed, such as one beyond the fuzz run's input length, which
/// gives bounded seeds through its regions.
pub const NO_TARGET: &str = "none";
/// The route member that names the representation of a codec target's bytes.
pub const REPRESENTATION: &str = "representation";
/// The route member that holds a codec target's context in hex.
pub const CONTEXT: &str = "context";
/// The route member that names the outcome of the target's entry point.
pub const OUTCOME: &str = "outcome";
/// The header member that states how many bytes the body assembles to.
pub const LENGTH: &str = "length";
/// The header member that names the input as a file, which every adapter of a parity run is given.
pub const FILE_NAME: &str = "file_name";
/// The header member, which may repeat, that gives a case: its name and its conditions.
pub const CASE: &str = "case";
/// The header member that gives the default case's conditions, without a name.
pub const DEFAULT: &str = "default";
/// The most bytes of a file name, as most file systems allow.
pub const MAXIMUM_FILE_NAME_LENGTH: usize = 255;
/// The device names that Windows reserves in every folder, with any extension, compared without regard to ASCII
/// case (Microsoft, "Naming Files, Paths, and Namespaces"). Its `COM¹` to `LPT³` are not ASCII, so no name of the
/// format can be one.
pub const RESERVED_DEVICE_NAMES: [&str; 22] = [
  "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8", "COM9", "LPT1", "LPT2",
  "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
];

/// The radix of hex digits.
pub const HEXADECIMAL_RADIX: u32 = 16;
/// The hex digits of one byte.
pub const DIGITS_PER_BYTE: usize = 2;
/// The radix of a repeat count and of a length.
pub const DECIMAL_RADIX: u32 = 10;
/// The most bytes that a fixture assembles to, 16 MiB, so that a repeat cannot ask for more memory than a test
/// should hold.
pub const MAXIMUM_LENGTH: usize = 16 * 1024 * 1024;
/// The first character that quoted text may hold.
pub const FIRST_PRINTABLE: char = ' ';
/// The last character that quoted text may hold.
pub const LAST_PRINTABLE: char = '~';

/// The message of a header without its end.
pub const NO_HEADER_END: &str = "the header has no line `---` that ends it";
/// The message of a header line that is not a member.
pub const MALFORMED_MEMBER: &str = "a header line is `name: value`";
/// The message of an unknown member.
pub const UNKNOWN_MEMBER: &str = "the format defines no member";
/// The message of a member given twice.
pub const REPEATED_MEMBER: &str = "a member is given once only";
/// The message of a required member that is missing.
pub const MISSING_MEMBER: &str = "a required member is missing";
/// The message of a context without a representation.
pub const CONTEXT_WITHOUT_REPRESENTATION: &str = "a context belongs to a codec target, which names its representation";
/// The message of a context member without hex bytes.
pub const EMPTY_CONTEXT: &str = "a context holds hex bytes; a target without a context leaves the member out";
/// The message of a name that is not lowercase words.
pub const MALFORMED_NAME: &str = "a name is lowercase letters and digits, its words joined by `_`";
/// The message of a length that is not a decimal number.
pub const MALFORMED_LENGTH: &str = "the length is a decimal number";
/// The message of a body whose length differs from the header's.
pub const LENGTH_MISMATCH: &str = "the body assembles to another length than the header states";
/// The message of a line of bytes without a comment.
pub const BYTES_WITHOUT_COMMENT: &str = "a line of bytes ends with a comment that names its field";
/// The message of a line of bytes before the first heading.
pub const BYTES_BEFORE_HEADING: &str =
  "a line of bytes stands below a heading that names its specification and section";
/// The message of a heading without text.
pub const EMPTY_HEADING: &str = "a heading names a specification and section";
/// The message of a word that is no item.
pub const NOT_HEXADECIMAL: &str = "an item is hex bytes, quoted text or a repeat";
/// The message of hex digits of an odd count.
pub const ODD_DIGITS: &str = "hex bytes are pairs of digits";
/// The message of quoted text without its end.
pub const UNTERMINATED_TEXT: &str = "quoted text ends with a quote on its line";
/// The message of quoted text beyond printable ASCII.
pub const TEXT_BEYOND_ASCII: &str = "quoted text holds printable ASCII only; write other bytes in hex";
/// The message of empty quoted text.
pub const EMPTY_TEXT: &str = "quoted text holds at least one character";
/// The message of a repeat without its item or count.
pub const MALFORMED_REPEAT: &str = "a repeat stands between an item and its count";
/// The message of a repeat count that is not a positive decimal number.
pub const MALFORMED_COUNT: &str = "a repeat count is a decimal number of at least 1";
/// The message of a body that is too long.
pub const TOO_LONG: &str = "the body assembles to more than 16 MiB";
/// The message of an unknown directive.
pub const UNKNOWN_DIRECTIVE: &str = "the format defines the directives `@seed` and `@end`, each name right after its \
                                     `@`";
/// The message of a region inside another.
pub const NESTED_REGION: &str = "a region opens only after the one before it ends";
/// The message of an end without a region.
pub const NO_OPEN_REGION: &str = "`@end` closes the region that `@seed` opened";
/// The message of a region that the body leaves open.
pub const UNCLOSED_REGION: &str = "the region that `@seed` opened needs its `@end`";
/// The message of a region without bytes.
pub const EMPTY_REGION: &str = "a region holds at least one byte";
/// The message of a body without bytes.
pub const NO_BYTES: &str = "the body holds at least one byte";
/// The message of an `@end` with words after it.
pub const END_WITH_WORDS: &str = "`@end` stands alone on its line";
/// The message of a malformed file name.
pub const MALFORMED_FILE_NAME: &str = "a file name is ASCII letters, digits, `.`, `-` and `_`, at most 255 bytes, \
  that every platform can write: it ends with no `.`, and its part before the first `.` is no device name that \
  Windows reserves";
/// The message of a malformed case.
pub const MALFORMED_CASE: &str = "a case is a name of lowercase words joined by `-`, then `name=value` conditions";
/// The message of a condition that is not `name=value`.
pub const MALFORMED_CONDITION: &str = "a condition is `name=value`";
/// The message of a case given twice.
pub const REPEATED_CASE: &str = "a case is given once only";
/// The message of a route beside the target `none`.
pub const ROUTE_OF_NO_TARGET: &str = "the target `none` gives no seed, so it has no representation, context or outcome";
/// The message of a region routed to the target `none`.
pub const REGION_WITHOUT_TARGET: &str = "a region is a seed, so its target is not `none`";
/// The message of a fixture of the target `none` without a region.
pub const NO_TARGET_WITHOUT_REGION: &str =
  "the target `none` serves a fixture beyond the fuzz input bound, whose bounded regions are its seeds";
