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

//! Constants modules (R33): recognized by name (`constants.rs`, or a file
//! under `constants/`) or by content (a file whose items are all constants),
//! and allowed to hold nothing but constants, imports and documentation.

use crate::constants::paths::{CONSTANTS_DIRECTORY, CONSTANTS_FILE, PATH_SEPARATOR};
use crate::findings::{Finding, Rule};
use crate::literals::constants::NOT_A_CONSTANT;
use crate::syntax::{Item, ItemKind, ParsedSource};

/// Whether `relative_path` names a constants module: a `constants.rs`, or a
/// file inside a `constants` directory.
#[must_use]
pub fn is_named_constants_module(relative_path: &str) -> bool {
  let mut components: Vec<&str> = relative_path.split(PATH_SEPARATOR).collect();
  let file_name = components.pop().unwrap_or_default();
  file_name == CONSTANTS_FILE || components.contains(&CONSTANTS_DIRECTORY)
}

/// Whether every item of the file outside its test block is a constant, a
/// static, an import or an inner attribute, and at least one is a constant
/// or a static. Such a file is a data table.
#[must_use]
pub fn holds_only_constants(parsed: &ParsedSource) -> bool {
  let items: Vec<&Item> = items_outside_tests(parsed).collect();
  items.iter().all(|item| is_allowed(item.kind))
    && items
      .iter()
      .any(|item| matches!(item.kind, ItemKind::Constant | ItemKind::Static))
}

/// The findings for everything in a constants module that is not allowed.
#[must_use]
pub fn check_constants_module_content(path: &str, parsed: &ParsedSource) -> Vec<Finding> {
  items_outside_tests(parsed)
    .filter(|item| !is_allowed(item.kind))
    .map(|item| {
      let message = match item.name.as_deref() {
        Some(name) => format!("{NOT_A_CONSTANT} (`{name}`)"),
        None => NOT_A_CONSTANT.to_owned(),
      };
      Finding::new(path, item.line, Rule::Constants, message)
    })
    .collect()
}

fn items_outside_tests(parsed: &ParsedSource) -> impl Iterator<Item = &Item> {
  let test_start = parsed.test_block.map(|block| block.first_token);
  parsed
    .items
    .iter()
    .filter(move |item| test_start != Some(item.first_token))
}

fn is_allowed(kind: ItemKind) -> bool {
  matches!(
    kind,
    ItemKind::Constant | ItemKind::Static | ItemKind::Use | ItemKind::InnerAttribute
  )
}

#[cfg(test)]
mod tests {
  use super::{check_constants_module_content, holds_only_constants, is_named_constants_module};
  use crate::syntax::ParsedSource;

  #[test]
  fn recognizes_constants_modules_by_name() {
    assert!(is_named_constants_module("crates/library/src/containers/constants.rs"));
    assert!(is_named_constants_module("src/constants/limits.rs"));
    assert!(!is_named_constants_module("src/limits.rs"));
    assert!(!is_named_constants_module("src/constants_extra.rs"));
  }

  #[test]
  fn recognizes_data_tables_by_content() {
    let table = ParsedSource::parse("//! Table.\nuse crate::Signature;\n/// Entries.\npub static ENTRIES: &[Signature] = &[];\n#[cfg(test)]\nmod tests {\n    fn helper() {}\n}\n").unwrap();
    assert!(holds_only_constants(&table));
    let code = ParsedSource::parse("pub const LIMIT: u8 = 2;\npub fn limit() -> u8 { LIMIT }\n").unwrap();
    assert!(!holds_only_constants(&code));
    let imports = ParsedSource::parse("pub use crate::Signature;\n").unwrap();
    assert!(!holds_only_constants(&imports));
  }

  #[test]
  fn rejects_code_in_a_constants_module() {
    let parsed = ParsedSource::parse(
      "pub const LIMIT: u8 = 2;\npub fn limit() -> u8 { LIMIT }\nimpl Unit {}\n#[cfg(test)]\nmod tests {}\n",
    )
    .unwrap();
    let findings = check_constants_module_content("src/constants.rs", &parsed);
    assert_eq!(findings.len(), 2);
    assert_eq!(findings[0].line, 2);
    assert!(findings[0].message.ends_with("(`limit`)"));
    assert!(!findings[1].message.contains('`'));
  }
}
