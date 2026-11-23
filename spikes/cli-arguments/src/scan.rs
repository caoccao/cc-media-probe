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

//! The plain scans that come before parsing (plan section 8.9): they look for one exact argument before the
//! separator, so their answer does not depend on the rest of the command line being valid.

use std::ffi::OsString;

use crate::constants::SEPARATOR;

/// Whether `argument` appears exactly, before any separator.
pub fn appears_before_separator(arguments: &[OsString], argument: &str) -> bool {
  arguments
    .iter()
    .take_while(|each| each.as_os_str() != SEPARATOR)
    .any(|each| each.as_os_str() == argument)
}
