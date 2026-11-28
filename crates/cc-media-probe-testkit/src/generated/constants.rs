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

//! The places and messages of the generated test files (plan section 9.1, `docs/guide/test-files.md` section 1).

/// The parent folder, from the kit's manifest folder up to the repository's root in two steps.
pub const PARENT: &str = "..";
/// The build folder at the repository's root, which the generator writes into whatever Cargo's target folder is.
pub const TARGET: &str = "target";
/// The folder of the generated files below it.
pub const TEST_FILES: &str = "test-files";
/// The folder of the recipes, inside the kit's manifest folder.
pub const RECIPES: &str = "test-files";
/// The extension of a recipe file.
pub const RECIPE_EXTENSION: &str = "json";
/// Separates the components of a generated file's name on every platform: `containers/matroska/h264-opus.mkv`.
pub const NAME_SEPARATOR: char = '/';
/// The topics of the recipes, the topics of `docs/specs/files/` (`docs/guide/test-files.md` section 1).
pub const TOPICS: [&str; 5] = ["containers", "video", "audio", "subtitles", "tags-and-registries"];
/// The components of a generated file's name: its topic, its format and its file.
pub const COMPONENTS: usize = 3;
/// Joins the words of a format and may stand in a file's name (`docs/guide/test-files.md` sections 1 and 3).
pub const HYPHEN: char = '-';
/// Starts each extension of a generated file's name (`docs/guide/test-files.md` section 3).
pub const EXTENSION_SEPARATOR: char = '.';

/// The message of a name that is not `<topic>/<format>/<file>` by the generator's rules.
pub const MALFORMED_NAME: &str = "a generated test file's name is `<topic>/<format>/<file>`, such as \
  `containers/matroska/h264-opus.mkv`, as the generator names it: one of the topics of docs/guide/test-files.md \
  section 1, a format of lowercase words and digits joined by `-`, and a file of lowercase letters, digits and `-` \
  with one or more extensions";
/// The message of a path whose metadata cannot be read, which rerunning the generator does not change.
pub const UNREADABLE: &str = "the generated test file's metadata cannot be read";
/// The message of a path that holds something other than a file.
pub const NOT_A_FILE: &str = "something other than a file stands where the generator writes this test file; \
  remove it, and run `node scripts/generate-test-files.ts`";
/// The message of a generated file that is missing.
pub const MISSING: &str = "the generated test file is missing; run `node scripts/generate-test-files.ts`, which \
  makes the files that its recipes describe (docs/guide/test-files.md section 2)";
