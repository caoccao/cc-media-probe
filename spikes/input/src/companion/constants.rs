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

//! The reasons with which the built-in resolver declines a companion.

/// The input has no path, so the resolver has no root.
pub const NO_FOLDER: &str = "the input has no folder to look in";
/// The resolved location lies outside the resolved root.
pub const OUTSIDE_ROOT: &str = "the companion resolves outside the folder of the probed file";
/// The path resolved differently after the file was opened.
pub const CHANGED: &str = "the companion resolved differently after it was opened";
/// The opened object is not a regular file.
pub const NOT_A_FILE: &str = "the companion is not a regular file";
/// The bytes of a file's device and file numbers, two numbers of 64 bits.
pub const FILE_NUMBERS_LENGTH: usize = 2 * size_of::<u64>();
/// The folder of a path that names a file in the current folder.
pub const CURRENT_FOLDER: &str = ".";
