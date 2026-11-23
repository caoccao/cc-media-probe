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

//! The names and messages of the command line (`docs/design/cli.md`, sections 2 and 4).

/// The program's name, a constant rather than the first argument, as the GNU coding standards ask of `--version`.
pub const PROGRAM_NAME: &str = "cc-media-probe";

/// The argument that ends the options, and the start of every long option.
pub const SEPARATOR: &str = "--";
/// The sign between a long option and a value given to it, as in `--json=yes`.
pub const VALUE_SIGN: char = '=';
/// The input that names standard input.
pub const STANDARD_INPUT: &str = "-";
/// The exact argument that turns JSON mode on.
pub const JSON_ARGUMENT: &str = "--json";
/// The exact argument that asks for help.
pub const HELP_ARGUMENT: &str = "--help";

/// The long name, and the identifier, of each option.
pub const JSON_NAME: &str = "json";
/// The long name, and the identifier, of each option.
pub const DETECT_NAME: &str = "detect";
/// The long name, and the identifier, of each option.
pub const STRICT_NAME: &str = "strict";
/// The long name, and the identifier, of each option.
pub const HELP_NAME: &str = "help";
/// The long name, and the identifier, of each option.
pub const VERSION_NAME: &str = "version";
/// The identifier of the inputs.
pub const INPUT_NAME: &str = "input";

/// The message of an unknown option, followed by the argument.
pub const UNKNOWN_OPTION: &str = "unknown option";
/// The message of a value given to an option that takes none, after the option.
pub const TAKES_NO_VALUE: &str = "takes no value";
/// The message of a command line without inputs.
pub const NO_INPUT: &str = "no input was named";
/// The message of standard input named twice, as plan section 8.9 writes it.
pub const STANDARD_INPUT_TWICE: &str = "standard input (-) was named twice";
/// The message of any other error that the parser reports.
pub const UNREADABLE: &str = "the command line could not be read";

/// The first word of each line that the spike's program prints.
pub const HELP_LINE: &str = "help";
/// The first word of each line that the spike's program prints.
pub const VERSION_LINE: &str = "version";
/// The first word of each line that the spike's program prints.
pub const RUN_LINE: &str = "run";
/// The first word of each line that the spike's program prints.
pub const INPUT_LINE: &str = "input";
/// The first word of each line that the spike's program prints.
pub const USAGE_ERROR_LINE: &str = "usage error";
/// The line that stands for the usage-error document on standard output in JSON mode.
pub const USAGE_DOCUMENT_LINE: &str = "usage error document";
/// The line on standard error after standard output failed (`cli.md` section 9).
pub const OUTPUT_FAILED_LINE: &str = "standard output failed";

/// The exit status of a usage error (R21).
pub const USAGE_ERROR_STATUS: u8 = 2;

/// The characters of the Unicode property `Bidi_Control`, which standard error writes escaped (`docs/design/cli.md`
/// section 8.1): U+061C, U+200E, U+200F, U+202A to U+202E and U+2066 to U+2069.
pub const BIDIRECTIONAL_CONTROLS: [char; 12] = [
  '\u{061c}', '\u{200e}', '\u{200f}', '\u{202a}', '\u{202b}', '\u{202c}', '\u{202d}', '\u{202e}', '\u{2066}',
  '\u{2067}', '\u{2068}', '\u{2069}',
];
