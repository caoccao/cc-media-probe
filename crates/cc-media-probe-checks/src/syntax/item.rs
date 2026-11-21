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

//! A top-level item of a source file.

use std::ops::Range;

/// What a top-level item is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ItemKind {
  /// An inner attribute such as `#![forbid(unsafe_code)]`.
  InnerAttribute,
  /// A module declaration without a body: `mod name;`.
  ModuleDeclaration,
  /// A module with an inline body: `mod name { … }`.
  InlineModule,
  /// A `use` declaration.
  Use,
  /// An `extern crate` declaration.
  ExternCrate,
  /// A function.
  Function,
  /// A `const` item.
  Constant,
  /// A `static` item.
  Static,
  /// A `struct`.
  Structure,
  /// An `enum`.
  Enumeration,
  /// A `union`.
  Union,
  /// A `trait`.
  Trait,
  /// A `type` alias.
  TypeAlias,
  /// An `impl` block.
  Implementation,
  /// A `macro_rules!` definition.
  MacroRules,
  /// A macro call in item position.
  MacroCall,
  /// Anything else, such as an `extern` block.
  Other,
}

/// One top-level item with its token range.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Item {
  /// What the item is.
  pub kind: ItemKind,
  /// The name the item defines, if it has one.
  pub name: Option<String>,
  /// The index of its first token, which may be the `#` of an attribute.
  pub first_token: usize,
  /// The index of its last token.
  pub last_token: usize,
  /// The line of its first token.
  pub line: usize,
  /// The token ranges of its outer attributes.
  pub attributes: Vec<Range<usize>>,
  /// The indices of the braces around its body, if it has one.
  pub body: Option<(usize, usize)>,
}

impl Item {
  /// Whether the item has the name `name`.
  #[must_use]
  pub fn is_named(&self, name: &str) -> bool {
    self.name.as_deref() == Some(name)
  }
}

#[cfg(test)]
mod tests {
  use super::{Item, ItemKind};

  #[test]
  fn compares_names() {
    let item = Item {
      kind: ItemKind::Function,
      name: Some("main".to_owned()),
      first_token: 0,
      last_token: 4,
      line: 1,
      attributes: Vec::new(),
      body: Some((3, 4)),
    };
    assert!(item.is_named("main"));
    assert!(!item.is_named("other"));
  }
}
