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

//! Collects every definition site of a file.

use crate::names::definitions::closures::closure_parameter_names;
use crate::names::definitions::fields::field_and_variant_names;
use crate::names::definitions::generics::generic_parameter_names;
use crate::names::definitions::items::{item_names, label_names, metavariable_names, rename_names};
use crate::names::definitions::matches::match_arm_names;
use crate::names::definitions::parameters::function_parameter_names;
use crate::names::definitions::patterns::{for_loop_names, let_names};
use crate::syntax::ParsedSource;

/// The indices of the tokens that define a name, sorted and without
/// duplicates.
#[must_use]
pub fn defined_names(parsed: &ParsedSource) -> Vec<usize> {
  let tokens = &parsed.tokens;
  let delimiters = &parsed.delimiters;
  let mut indices = item_names(tokens);
  indices.extend(rename_names(tokens));
  indices.extend(label_names(tokens));
  indices.extend(metavariable_names(tokens));
  indices.extend(generic_parameter_names(tokens));
  indices.extend(field_and_variant_names(tokens, delimiters));
  indices.extend(function_parameter_names(tokens, delimiters));
  indices.extend(let_names(tokens));
  indices.extend(for_loop_names(tokens));
  indices.extend(match_arm_names(tokens, delimiters));
  indices.extend(closure_parameter_names(tokens));
  indices.sort_unstable();
  indices.dedup();
  indices
}

#[cfg(test)]
mod tests {
  use super::defined_names;
  use crate::syntax::ParsedSource;

  fn names(source: &str) -> Vec<String> {
    let parsed = ParsedSource::parse(source).unwrap();
    defined_names(&parsed)
      .into_iter()
      .map(|index| parsed.tokens[index].text.clone())
      .collect()
  }

  #[test]
  fn collects_every_kind_of_definition() {
    let source = "use std::io::Read as Reading;\nstruct Packet<'input, Source> { payload: &'input [u8], source: Source }\nfn parse<Item>(first: u8, (second, third): (u8, u8)) -> u8 {\n    let total = first;\n    'outer: for value in 0..total { break 'outer; }\n    match second { Some(found) => found, other => other }\n    let doubled = |number: u8| number;\n    total\n}\nmacro_rules! repeat { ($count:expr) => {} }\n";
    let mut found = names(source);
    found.sort();
    let mut expected = vec![
      "Reading", "Packet", "'input", "Source", "payload", "source", "parse", "Item", "first", "second", "third",
      "total", "'outer", "value", "found", "other", "doubled", "number", "repeat", "count",
    ];
    expected.sort_unstable();
    assert_eq!(found, expected);
  }
}
