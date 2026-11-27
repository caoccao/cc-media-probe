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

//! `companion_name`, the toy format's rule for an identifier and the name built from it.

use crate::toy::constants::{COMPANION_EXTENSION, EXTENSION_SEPARATOR, MAXIMUM_IDENTIFIER_LENGTH};

/// The name of the companion that `identifier` names, or `None` when it fails validation: 1 to 16 lowercase ASCII
/// letters and digits. Nothing else from the media enters the name, which takes a fixed extension.
pub(crate) fn companion_name(identifier: &[u8]) -> Option<String> {
  let text = str::from_utf8(identifier).ok().filter(|_| is_valid(identifier))?;
  Some(format!("{text}{EXTENSION_SEPARATOR}{COMPANION_EXTENSION}"))
}

/// Whether `identifier` passes validation, without building its name.
pub(crate) fn is_valid(identifier: &[u8]) -> bool {
  (1..=MAXIMUM_IDENTIFIER_LENGTH).contains(&identifier.len())
    && identifier
      .iter()
      .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit())
}
