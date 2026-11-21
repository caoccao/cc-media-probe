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

//! Commands, options, exit statuses and messages of the command line.

/// Checks the code rules and the copyright header.
pub const STRUCTURE_COMMAND: &str = "structure";
/// Checks line coverage from an LCOV report.
pub const COVERAGE_COMMAND: &str = "coverage";
/// Checks target ownership and the excluded workspaces.
pub const TARGETS_COMMAND: &str = "targets";
/// Lists the published crates' dependency graph.
pub const INVENTORY_COMMAND: &str = "inventory";
/// Inspects the outputs of build scripts.
pub const BUILD_OUTPUT_COMMAND: &str = "build-output";
/// Compares a binary's dynamic libraries with the allow-list.
pub const DYNAMIC_LIBRARIES_COMMAND: &str = "dynamic-libraries";
/// Names the repository root; the default is the current directory.
pub const ROOT_OPTION: &str = "--root";
/// Prints the usage.
pub const HELP_OPTION: &str = "--help";
/// Prints the usage.
pub const SHORT_HELP_OPTION: &str = "-h";
/// The default repository root.
pub const CURRENT_DIRECTORY: &str = ".";
/// The exit status when everything passes.
pub const SUCCESS_STATUS: u8 = 0;
/// The exit status when a check finds problems or cannot run.
pub const FAILURE_STATUS: u8 = 1;
/// The exit status of a usage error.
pub const USAGE_ERROR_STATUS: u8 = 2;
/// The usage text.
pub const USAGE: &str = "usage: cc-media-probe-checks <command> [--root <repository>]

commands:
  structure                          the code rules R30 to R33 and the copyright header
  coverage <lcov-report>             line coverage per published crate and library area (R28)
  targets                            target ownership and the excluded workspaces
  inventory                          the dependency graph of the published crates (D4, layer 1)
  build-output <target-directory>    link instructions and native objects of build scripts (D4, layer 3)
  dynamic-libraries <platform> <library>...
                                     a binary's dynamic libraries against the allow-list (D4, layer 4)

exit status: 0 when the check passes, 1 when it finds problems, 2 on a usage error";
/// An unknown command or wrong arguments.
pub const UNKNOWN_COMMAND: &str = "unknown command or wrong arguments";
/// `--root` without a value.
pub const MISSING_ROOT: &str = "--root needs a path";
/// The word after the count in the closing line.
pub const FINDINGS_WORD: &str = "findings";
/// The word after the count of inspected build-script runs.
pub const BUILD_SCRIPT_RUNS_WORD: &str = "build-script runs inspected";
