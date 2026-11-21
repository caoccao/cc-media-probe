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

//! The fourth layer of D4: the dynamic libraries of a linked binary, as the
//! platform's own tool lists them, against the allow-list.

use crate::findings::{Finding, Rule};
use crate::inventory::constants::{LIBRARY_NOT_ALLOWED, PREFIX_WILDCARD};

/// One finding for every library of `libraries` that no entry of `allowed`
/// for `platform` matches. Names compare without regard to ASCII case, and
/// an entry that ends with `*` matches by prefix.
#[must_use]
pub fn check_dynamic_libraries(platform: &str, libraries: &[String], allowed: &[(String, String)]) -> Vec<Finding> {
  let entries: Vec<&str> = allowed
    .iter()
    .filter(|(entry_platform, _)| entry_platform == platform)
    .map(|(_, library)| library.as_str())
    .collect();
  libraries
    .iter()
    .filter(|library| !entries.iter().any(|entry| matches(entry, library)))
    .map(|library| Finding::new(library, 0, Rule::BuildOutput, LIBRARY_NOT_ALLOWED))
    .collect()
}

fn matches(entry: &str, library: &str) -> bool {
  match entry.strip_suffix(PREFIX_WILDCARD) {
    Some(prefix) => library
      .get(..prefix.len())
      .is_some_and(|start| start.eq_ignore_ascii_case(prefix)),
    None => entry.eq_ignore_ascii_case(library),
  }
}

#[cfg(test)]
mod tests {
  use super::check_dynamic_libraries;

  fn allowed() -> Vec<(String, String)> {
    vec![
      ("macos".to_owned(), "/usr/lib/libSystem.B.dylib".to_owned()),
      ("windows".to_owned(), "api-ms-win-crt-*".to_owned()),
      ("windows".to_owned(), "KERNEL32.dll".to_owned()),
    ]
  }

  #[test]
  fn accepts_listed_libraries_by_name_or_prefix() {
    let libraries = vec![
      "kernel32.DLL".to_owned(),
      "api-ms-win-crt-runtime-l1-1-0.dll".to_owned(),
    ];
    assert_eq!(check_dynamic_libraries("windows", &libraries, &allowed()), Vec::new());
  }

  #[test]
  fn rejects_unlisted_libraries_and_other_platforms() {
    let libraries = vec![
      "/usr/lib/libSystem.B.dylib".to_owned(),
      "/opt/lib/libnative.dylib".to_owned(),
    ];
    let findings = check_dynamic_libraries("macos", &libraries, &allowed());
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].path, "/opt/lib/libnative.dylib");
    assert_eq!(check_dynamic_libraries("linux", &libraries, &allowed()).len(), 2);
  }
}
