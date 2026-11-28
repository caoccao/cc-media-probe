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

//! The paths of the generated test files below `target/test-files/` at the repository's root, and of the recipes
//! that describe them in the kit's `test-files/` (plan section 9.1). A name follows the generator's own rules, so
//! that a missing file is one that a recipe could describe. A test whose file is missing fails with the message that
//! names the generator, and is never skipped (`docs/guide/test-files.md` section 6). Only an absent file is missing:
//! a path whose metadata cannot be read keeps its error, and one that holds no file says so, since rerunning the
//! generator fixes neither, as the second review of plan Phase 3, task 4, found.

use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use super::constants::{
  COMPONENTS, EXTENSION_SEPARATOR, HYPHEN, NAME_SEPARATOR, PARENT, RECIPE_EXTENSION, RECIPES, TARGET, TEST_FILES,
  TOPICS,
};
use super::error::GeneratedError;

/// The generated test file `name`, `<topic>/<format>/<file>` with `/` on every platform, such as
/// `containers/matroska/h264-opus.mkv`, below `target/test-files/`.
///
/// ```
/// use cc_media_probe_testkit::generated::{GeneratedError, folder, path};
///
/// // A name that the generator refuses is refused before any file is looked for.
/// assert!(matches!(path("video/H264/a.h264"), Err(GeneratedError::MalformedName(_))));
/// // A name that it allows gives a file below `folder()`, or says why there is none there.
/// match path("video/h264/baseline-320x240.h264") {
///     Ok(file) => assert!(file.starts_with(folder())),
///     Err(error) => println!("{error}"),
/// }
/// ```
///
/// # Errors
///
/// The error of a name that is not `<topic>/<format>/<file>` by the generator's rules; of a file that is missing,
/// whose message names `node scripts/generate-test-files.ts`; of a path whose metadata cannot be read; and of a
/// path that holds something other than a file.
pub fn path(name: &str) -> Result<PathBuf, GeneratedError> {
  path_in(&folder(), name)
}

/// The generated test file `name` below `base`, as `path` finds it below `target/test-files/`.
///
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use std::fs;
///
/// use cc_media_probe_testkit::generated::{GeneratedError, path_in};
///
/// // A folder of the example's own, which nothing else writes, so that what is generated decides nothing.
/// let base = std::env::temp_dir().join(format!("cc-media-probe-testkit-path-in-{}", std::process::id()));
/// let _ = fs::remove_dir_all(&base);
/// let error = path_in(&base, "audio/opus/stereo-48k.opus").unwrap_err();
/// assert!(matches!(&error, GeneratedError::Missing(file) if file.starts_with(&base)));
/// assert!(error.to_string().contains("run `node scripts/generate-test-files.ts`"));
///
/// let format = base.join("audio").join("opus");
/// fs::create_dir_all(&format)?;
/// fs::write(format.join("stereo-48k.opus"), [])?;
/// assert_eq!(path_in(&base, "audio/opus/stereo-48k.opus")?, format.join("stereo-48k.opus"));
/// fs::remove_dir_all(&base)?;
/// # Ok(())
/// # }
/// ```
///
/// # Errors
///
/// The errors of `path`.
pub fn path_in(base: &Path, name: &str) -> Result<PathBuf, GeneratedError> {
  let components = components(name)?;
  let mut path = base.to_path_buf();
  path.extend(components);
  match fs::metadata(&path) {
    Ok(metadata) if metadata.is_file() => Ok(path),
    Ok(_) => Err(GeneratedError::NotAFile(path)),
    Err(error) if error.kind() == ErrorKind::NotFound => Err(GeneratedError::Missing(path)),
    Err(error) => Err(GeneratedError::Unreadable(path, error)),
  }
}

/// The recipe file that describes the generated test file `name`: `test-files/<topic>/<format>.json` in the kit,
/// such as `test-files/containers/matroska.json` for `containers/matroska/h264-opus.mkv`. Its entry of that file's
/// name holds the values that the file's tests expect, in a member `expected` (`docs/design/test-kit.md` section
/// 4.4).
///
/// ```
/// # fn main() -> Result<(), cc_media_probe_testkit::generated::GeneratedError> {
/// use std::path::Path;
///
/// use cc_media_probe_testkit::generated::recipe_path;
///
/// let recipe = recipe_path("containers/matroska/h264-opus.mkv")?;
/// assert!(recipe.ends_with(Path::new("test-files").join("containers").join("matroska.json")));
/// # Ok(())
/// # }
/// ```
///
/// # Errors
///
/// The error of a name that is not `<topic>/<format>/<file>` by the generator's rules.
pub fn recipe_path(name: &str) -> Result<PathBuf, GeneratedError> {
  let [topic, format, _] = components(name)?;
  Ok(
    Path::new(env!("CARGO_MANIFEST_DIR"))
      .join(RECIPES)
      .join(topic)
      .join(format!("{format}.{RECIPE_EXTENSION}")),
  )
}

/// The folder of the generated test files, `target/test-files/` at the repository's root.
///
/// ```
/// use std::path::Path;
///
/// use cc_media_probe_testkit::generated::folder;
///
/// assert!(folder().ends_with(Path::new("target").join("test-files")));
/// ```
#[must_use]
pub fn folder() -> PathBuf {
  Path::new(env!("CARGO_MANIFEST_DIR"))
    .join(PARENT)
    .join(PARENT)
    .join(TARGET)
    .join(TEST_FILES)
}

/// The topic, the format and the file of `name`, each as the generator allows it: one of its topics, a format that
/// names a recipe file, and a file that a recipe may name (`docs/guide/test-files.md` sections 1 and 3). Lookups
/// took any plain component before the review of plan Phase 3, task 4, found `video/H264/no-extension` accepted.
fn components(name: &str) -> Result<[&str; COMPONENTS], GeneratedError> {
  let malformed = || GeneratedError::MalformedName(name.to_owned());
  let parts: Vec<&str> = name.split(NAME_SEPARATOR).collect();
  let components: [&str; COMPONENTS] = parts.try_into().map_err(|_| malformed())?;
  let [topic, format, file] = components;
  if TOPICS.contains(&topic) && is_format_name(format) && is_file_name(file) {
    Ok(components)
  } else {
    Err(malformed())
  }
}

/// Whether `format` names a recipe file as the generator requires: lowercase letters and digits, its words joined by
/// `-`.
fn is_format_name(format: &str) -> bool {
  format
    .split(HYPHEN)
    .all(|word| !word.is_empty() && word.chars().all(is_lowercase_alphanumeric))
}

/// Whether `file` is a generated file's name as the generator requires: a lowercase letter or digit, then lowercase
/// letters, digits and `-`, then one or more extensions of lowercase letters and digits, each after a `.`.
fn is_file_name(file: &str) -> bool {
  let mut parts = file.split(EXTENSION_SEPARATOR);
  let stem = parts.next().unwrap_or_default();
  let mut extensions = parts.peekable();
  stem.chars().next().is_some_and(is_lowercase_alphanumeric)
    && stem
      .chars()
      .all(|character| is_lowercase_alphanumeric(character) || character == HYPHEN)
    && extensions.peek().is_some()
    && extensions.all(|extension| !extension.is_empty() && extension.chars().all(is_lowercase_alphanumeric))
}

/// Whether `character` is a lowercase ASCII letter or a digit.
fn is_lowercase_alphanumeric(character: char) -> bool {
  character.is_ascii_lowercase() || character.is_ascii_digit()
}
