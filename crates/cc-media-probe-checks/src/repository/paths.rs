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

//! Repository paths. A repository-relative name has `/` between its components on every platform, as Git writes
//! it; it becomes a file-system path only by joining its components, so no separator is assumed.

use std::path::{Path, PathBuf};

use crate::constants::paths::PATH_SEPARATOR;

/// The file-system path of the components below `root`.
#[must_use]
pub fn repository_path<'component>(root: &Path, components: impl IntoIterator<Item = &'component str>) -> PathBuf {
  components
    .into_iter()
    .fold(root.to_path_buf(), |path, component| path.join(component))
}

/// The file-system path of a repository-relative name such as `src/lib.rs`.
#[must_use]
pub fn repository_file(root: &Path, relative: &str) -> PathBuf {
  repository_path(root, relative.split(PATH_SEPARATOR))
}

/// The repository-relative name of the components, with `/`, as findings show it.
#[must_use]
pub fn repository_name(components: &[&str]) -> String {
  components.join(&PATH_SEPARATOR.to_string())
}

#[cfg(test)]
mod tests {
  use super::{repository_file, repository_name, repository_path};
  use std::path::Path;

  #[test]
  fn joins_components_with_the_platform_separator() {
    let root = Path::new("root");
    let expected = root.join("crates").join("library").join("lib.rs");
    assert_eq!(repository_path(root, ["crates", "library", "lib.rs"]), expected);
    assert_eq!(repository_file(root, "crates/library/lib.rs"), expected);
    assert_eq!(
      repository_name(&["crates", "library", "lib.rs"]),
      "crates/library/lib.rs"
    );
  }
}
