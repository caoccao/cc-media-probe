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

/// The folder of the fixtures, beside the kit's manifest folder; in the permanent test kit, `fixtures/` inside it.
pub const FOLDER: &str = "fixtures";
/// The parent folder, from the kit's manifest folder.
pub const PARENT: &str = "..";
/// Separates a fixture's format from its name, such as `flac/decoding-example-1`, on every platform.
pub const NAME_SEPARATOR: char = '/';
/// The extension of a fixture.
pub const FIXTURE_EXTENSION: &str = "hex";
/// The extension of an expectation.
pub const EXPECTATION_EXTENSION: &str = "json";
/// Separates a fixture's name, a case, a part and an extension in a file name.
pub const PART_SEPARATOR: char = '.';
/// Separates a fixture's name from a region's number in the name of a seed: `toy/setup-and-parameters#1`. A name
/// never holds it, so no seed's name is another's.
pub const REGION_SEPARATOR: char = '#';
/// The parts of an expectation: the semantic part of the seekable class, the forward-only class's where it
/// differs, each adapter's semantic part in a case whose adapters may stop differently at a limit, and each
/// adapter's part.
pub const PARTS: [&str; 4] = ["expected", "forward", "outcomes", "adapters"];

/// The message of a fixture that cannot be read.
pub const UNREADABLE: &str = "the fixture cannot be read";
/// The message of a name that is not `<format>/<name>` of lowercase words joined by `-`.
pub const MALFORMED_NAME: &str = "a fixture's name is `<format>/<name>`, each lowercase words joined by `-`";
/// The message of a file or folder that the layout does not allow.
pub const MISPLACED: &str = "the fixtures' folder holds format folders of `<name>.hex` fixtures and their \
                             `<name>[.<case>].<part>.json` expectations only";
/// The message of an expectation without its fixture.
pub const ORPHAN: &str = "an expectation file belongs to a fixture beside it";
