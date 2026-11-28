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

//! The names of the fixtures' folder and files, and the messages of a layout that the test kit refuses.

/// The folder of the fixtures, inside the kit's manifest folder.
pub const FOLDER: &str = "fixtures";
/// Separates a fixture's format from its name, such as `flac/decoding-example-1`, on every platform.
pub const NAME_SEPARATOR: char = '/';
/// The extension of a fixture.
pub const FIXTURE_EXTENSION: &str = "hex";
/// The extension of an expectation.
pub const EXPECTATION_EXTENSION: &str = "json";
/// Separates a fixture's name, a case, a part and an extension in a file name.
pub const PART_SEPARATOR: char = '.';
/// The parts of an expectation: the semantic part of the seekable class, the forward-only class's where it
/// differs, each adapter's semantic part in a case whose adapters may stop differently at a limit, and each
/// adapter's part.
///
/// ```
/// use cc_media_probe_testkit::fixture::{PARTS, expectation_path};
///
/// for part in PARTS {
///     let path = expectation_path("toy/setup-and-parameters", None, part);
///     assert!(path.ends_with(format!("setup-and-parameters.{part}.json")));
/// }
/// ```
pub const PARTS: [&str; 4] = ["expected", "forward", "outcomes", "adapters"];

/// The message of a fixture that cannot be read, whose cause the error gives as its source.
pub const UNREADABLE: &str = "the fixture cannot be read";
/// The cause of a fixture that no entry of its folder names exactly, such as `toy/Sample.hex` for `toy/sample` on a
/// file system that ignores case.
pub const ABSENT: &str = "no entry of its folder has this name, spelled exactly so";
/// The message of a text that is not a fixture, whose syntax error the error gives as its source.
pub const NOT_A_FIXTURE: &str = "the text is not a fixture";
/// The message of a folder that cannot be listed, whose cause the error gives as its source.
pub const UNLISTED: &str = "the folder cannot be listed";
/// The message of a name that is not `<format>/<name>` of lowercase words joined by `-`.
pub const MALFORMED_NAME: &str = "a fixture's name is `<format>/<name>`, each lowercase words joined by `-`";
/// The message of a file or folder that the layout does not allow.
pub const MISPLACED: &str = "the fixtures' folder holds format folders of `<name>.hex` fixtures and their \
                             `<name>[.<case>].<part>.json` expectations only";
/// The message of an expectation without its fixture.
pub const ORPHAN: &str = "an expectation file belongs to a fixture beside it";
/// The message of a symbolic link in the fixtures' folder, or of the folder as one.
pub const LINKED: &str = "the fixtures' folder holds no symbolic link, and is none, since writing expectations \
                          would follow it out of the folder";
/// The message of a case whose expectation files would have a name longer than a file system allows.
pub const LONG_EXPECTATION_NAME: &str = "an expectation's file name, `<name>[.<case>].<part>.json`, is at most 255 \
                                         bytes, as most file systems allow, so the case or the fixture needs a \
                                         shorter name";
/// Names the default case, which has no name of its own, in a message.
pub const DEFAULT_CASE: &str = "the default case";
