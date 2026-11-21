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

//! Module roots only declare (R30): `lib.rs`, `main.rs` and every `mod.rs`
//! hold module declarations, imports, re-exports, attributes and
//! documentation; `main.rs` may also hold a `main` that hands over.

use crate::constants::keywords::{LET_KEYWORD, MAIN_FUNCTION_NAME};
use crate::constants::punctuation::SEMICOLON;
use crate::findings::{Finding, Rule};
use crate::rules::constants::{
  DEFINITION_IN_MODULE_ROOT, INLINE_MODULE_IN_MODULE_ROOT, MACRO_IN_MODULE_ROOT, MAIN_MUST_HAND_OVER,
  MAXIMUM_MAIN_STATEMENTS,
};
use crate::syntax::{Item, ItemKind, ParsedSource};

/// Checks a module root; `is_binary_root` allows the `main` function.
#[must_use]
pub fn check_module_root(path: &str, parsed: &ParsedSource, is_binary_root: bool) -> Vec<Finding> {
  let mut findings = Vec::new();
  for item in &parsed.items {
    let name = item.name.as_deref().unwrap_or_default();
    let message = match item.kind {
      ItemKind::InnerAttribute | ItemKind::ModuleDeclaration | ItemKind::Use | ItemKind::ExternCrate => continue,
      ItemKind::Function if is_binary_root && item.is_named(MAIN_FUNCTION_NAME) => {
        if only_hands_over(parsed, item) {
          continue;
        }
        MAIN_MUST_HAND_OVER.to_owned()
      }
      ItemKind::InlineModule => format!("{INLINE_MODULE_IN_MODULE_ROOT} (`{name}`)"),
      ItemKind::MacroCall | ItemKind::MacroRules => {
        format!("{MACRO_IN_MODULE_ROOT} (`{name}!`)")
      }
      _ if name.is_empty() => DEFINITION_IN_MODULE_ROOT.to_owned(),
      _ => format!("{DEFINITION_IN_MODULE_ROOT} (`{name}`)"),
    };
    findings.push(Finding::new(path, item.line, Rule::ModuleRoot, message));
  }
  findings
}

/// Whether the body of `main` holds no `let` and at most one statement.
fn only_hands_over(parsed: &ParsedSource, item: &Item) -> bool {
  let Some((opening, closing)) = item.body else {
    return false;
  };
  let mut statements: usize = 0;
  let mut index = opening.saturating_add(1);
  while index < closing {
    let Some(token) = parsed.token(index) else {
      break;
    };
    if token.is_word(LET_KEYWORD) {
      return false;
    }
    if token.is_punctuation(SEMICOLON) {
      statements = statements.saturating_add(1);
    }
    index = parsed.delimiters.skip_group(index);
  }
  statements <= MAXIMUM_MAIN_STATEMENTS
}

#[cfg(test)]
mod tests {
  use super::check_module_root;
  use crate::syntax::ParsedSource;

  fn messages(source: &str, is_binary_root: bool) -> Vec<String> {
    let parsed = ParsedSource::parse(source).unwrap();
    check_module_root("src/mod.rs", &parsed, is_binary_root)
      .into_iter()
      .map(|finding| finding.message)
      .collect()
  }

  #[test]
  fn accepts_declarations_imports_and_attributes() {
    let source = "//! Docs.\n#![forbid(unsafe_code)]\npub mod segment;\n#[cfg(test)]\nmod tests;\npub use segment::Segment;\nextern crate alloc;\n";
    assert_eq!(messages(source, false), Vec::<String>::new());
  }

  #[test]
  fn rejects_definitions() {
    let found = messages("pub fn helper() {}\nstruct Unit;\nimpl Unit {}\n", false);
    assert_eq!(found.len(), 3);
    assert!(found[0].contains("`helper`"));
    assert!(!found[2].contains('`'));
  }

  #[test]
  fn rejects_inline_modules_and_macros() {
    let found = messages(
      "mod inline {}\n#[cfg(test)]\nmod tests {}\nmacro_rules! repeat { () => {} }\nitems!();\n",
      false,
    );
    assert_eq!(found.len(), 4);
    assert!(found[0].contains("`inline`"));
    assert!(found[1].contains("`tests`"));
    assert!(found[2].contains("`repeat!`"));
    assert!(found[3].contains("`items!`"));
  }

  #[test]
  fn allows_a_main_that_hands_over() {
    let source =
      "mod command_line;\nfn main() -> std::process::ExitCode {\n    command_line::run(std::env::args_os())\n}\n";
    assert_eq!(messages(source, true), Vec::<String>::new());
    let statement = "fn main() {\n    command_line::run();\n}\n";
    assert_eq!(messages(statement, true), Vec::<String>::new());
    assert_eq!(messages(source, false).len(), 1);
  }

  #[test]
  fn rejects_a_main_that_does_more() {
    assert_eq!(
      messages("fn main() {\n    let value = 2;\n    run(value)\n}\n", true).len(),
      1
    );
    assert_eq!(messages("fn main() {\n    first();\n    second();\n}\n", true).len(), 1);
    assert_eq!(messages("fn main();\n", true).len(), 1);
    assert_eq!(messages("fn other() {}\n", true).len(), 1);
  }
}
