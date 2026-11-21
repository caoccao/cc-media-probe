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

//! Which crate and which library area a source file belongs to.

use crate::constants::paths::PATH_SEPARATOR;
use crate::coverage::constants::{
  CRATE_DIRECTORY_PREFIX, CRATE_PATH_COMPONENTS, LIBRARY_AREAS, LIBRARY_CRATE, SOURCE_DIRECTORY_SEGMENT,
};

/// The crate of a repository-relative path such as
/// `crates/cc-media-probe/src/lib.rs`.
#[must_use]
pub fn crate_of(relative_path: &str) -> Option<&str> {
  relative_path
    .strip_prefix(CRATE_DIRECTORY_PREFIX)?
    .split(PATH_SEPARATOR)
    .next()
}

/// The library area of a repository-relative path such as
/// `crates/cc-media-probe/src/containers/matroska/mod.rs`.
#[must_use]
pub fn area_of(relative_path: &str) -> Option<&'static str> {
  if crate_of(relative_path) != Some(LIBRARY_CRATE) {
    return None;
  }
  let mut components = relative_path.split(PATH_SEPARATOR).skip(CRATE_PATH_COMPONENTS);
  if components.next() != Some(SOURCE_DIRECTORY_SEGMENT) {
    return None;
  }
  let area = components.next()?;
  components.next()?;
  LIBRARY_AREAS.iter().copied().find(|candidate| *candidate == area)
}

#[cfg(test)]
mod tests {
  use super::{area_of, crate_of};

  #[test]
  fn finds_crates() {
    assert_eq!(
      crate_of("crates/cc-media-probe-cli/src/main.rs"),
      Some("cc-media-probe-cli")
    );
    assert_eq!(crate_of("scripts/check.sh"), None);
  }

  #[test]
  fn finds_areas_of_the_library_only() {
    assert_eq!(
      area_of("crates/cc-media-probe/src/containers/matroska/mod.rs"),
      Some("containers")
    );
    assert_eq!(
      area_of("crates/cc-media-probe/src/utilities/bits.rs"),
      Some("utilities")
    );
    assert_eq!(area_of("crates/cc-media-probe/src/lib.rs"), None);
    assert_eq!(area_of("crates/cc-media-probe/src/model/stream.rs"), None);
    assert_eq!(area_of("crates/cc-media-probe/src/containers.rs"), None);
    assert_eq!(area_of("crates/cc-media-probe/tests/containers/golden.rs"), None);
    assert_eq!(area_of("crates/cc-media-probe-cli/src/containers/view.rs"), None);
  }
}
