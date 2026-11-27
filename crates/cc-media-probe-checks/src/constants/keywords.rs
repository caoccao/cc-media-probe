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

//! Rust keywords and fixed names that the checks recognize.

/// The keyword `as`.
pub const AS_KEYWORD: &str = "as";
/// The keyword `async`.
pub const ASYNCHRONOUS_KEYWORD: &str = "async";
/// The contextual keyword `auto`.
pub const AUTO_KEYWORD: &str = "auto";
/// The reserved keyword `box`.
pub const BOX_KEYWORD: &str = "box";
/// The keyword `const`.
pub const CONSTANT_KEYWORD: &str = "const";
/// The keyword `crate`.
pub const CRATE_KEYWORD: &str = "crate";
/// The contextual keyword `default`.
pub const DEFAULT_KEYWORD: &str = "default";
/// The keyword `else`.
pub const ELSE_KEYWORD: &str = "else";
/// The keyword `enum`.
pub const ENUMERATION_KEYWORD: &str = "enum";
/// The keyword `extern`.
pub const EXTERN_KEYWORD: &str = "extern";
/// The keyword `fn`.
pub const FUNCTION_KEYWORD: &str = "fn";
/// The keyword `for`.
pub const FOR_KEYWORD: &str = "for";
/// The keyword `if`.
pub const IF_KEYWORD: &str = "if";
/// The keyword `impl`.
pub const IMPLEMENTATION_KEYWORD: &str = "impl";
/// The keyword `in`.
pub const IN_KEYWORD: &str = "in";
/// The keyword `let`.
pub const LET_KEYWORD: &str = "let";
/// The keyword `loop`.
pub const LOOP_KEYWORD: &str = "loop";
/// The name of the macro that defines declarative macros.
pub const MACRO_RULES_NAME: &str = "macro_rules";
/// The keyword `match`.
pub const MATCH_KEYWORD: &str = "match";
/// The keyword `mod`.
pub const MODULE_KEYWORD: &str = "mod";
/// The keyword `move`.
pub const MOVE_KEYWORD: &str = "move";
/// The keyword `mut`.
pub const MUTABLE_KEYWORD: &str = "mut";
/// The keyword `pub`.
pub const PUBLIC_KEYWORD: &str = "pub";
/// The keyword `ref`.
pub const REFERENCE_KEYWORD: &str = "ref";
/// The keyword `return`.
pub const RETURN_KEYWORD: &str = "return";
/// The contextual keyword `safe`.
pub const SAFE_KEYWORD: &str = "safe";
/// The keyword `self`.
pub const SELF_VALUE_KEYWORD: &str = "self";
/// The keyword `static`.
pub const STATIC_KEYWORD: &str = "static";
/// The keyword `struct`.
pub const STRUCTURE_KEYWORD: &str = "struct";
/// The keyword `trait`.
pub const TRAIT_KEYWORD: &str = "trait";
/// The keyword `type`.
pub const TYPE_KEYWORD: &str = "type";
/// The contextual keyword `union`.
pub const UNION_KEYWORD: &str = "union";
/// The keyword `unsafe`.
pub const UNSAFE_KEYWORD: &str = "unsafe";
/// The keyword `use`.
pub const USE_KEYWORD: &str = "use";
/// The keyword `where`.
pub const WHERE_KEYWORD: &str = "where";
/// The keyword `while`.
pub const WHILE_KEYWORD: &str = "while";
/// The placeholder pattern `_`.
pub const UNDERSCORE_NAME: &str = "_";
/// The configuration predicate of test builds.
pub const TEST_CONFIGURATION: &str = "test";
/// The attribute that compiles an item conditionally.
pub const CONFIGURATION_ATTRIBUTE: &str = "cfg";
/// The attribute that applies other attributes conditionally.
pub const CONDITIONAL_ATTRIBUTE: &str = "cfg_attr";
/// The attribute that sets the file of a module.
pub const PATH_ATTRIBUTE: &str = "path";
/// The macro that includes Rust source from another file.
pub const INCLUDE_MACRO: &str = "include";
/// The name of the embedded test module (R31).
pub const TESTS_MODULE_NAME: &str = "tests";
/// The name of a binary's entry function.
pub const MAIN_FUNCTION_NAME: &str = "main";

/// The keywords after `const` that make it a qualifier of a function
/// rather than the start of a constant item.
pub const CONSTANT_FUNCTION_KEYWORDS: &[&str] =
  &[FUNCTION_KEYWORD, UNSAFE_KEYWORD, ASYNCHRONOUS_KEYWORD, EXTERN_KEYWORD];
/// The keywords that start an item after `pub`, so a `pub` before anything else but the parenthesis of a
/// restriction marks a field.
pub const VISIBLE_ITEM_KEYWORDS: &[&str] = &[
  ASYNCHRONOUS_KEYWORD,
  AUTO_KEYWORD,
  CONSTANT_KEYWORD,
  ENUMERATION_KEYWORD,
  EXTERN_KEYWORD,
  FUNCTION_KEYWORD,
  MODULE_KEYWORD,
  SAFE_KEYWORD,
  STATIC_KEYWORD,
  STRUCTURE_KEYWORD,
  TRAIT_KEYWORD,
  TYPE_KEYWORD,
  UNION_KEYWORD,
  UNSAFE_KEYWORD,
  USE_KEYWORD,
];
/// Keywords that never name a binding in a pattern.
pub const PATTERN_KEYWORDS: &[&str] = &[
  "ref", "mut", "box", "_", "true", "false", "self", "Self", "crate", "super", "in", "if", "else", "as", "const",
];
