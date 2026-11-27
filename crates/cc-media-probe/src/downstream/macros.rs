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

//! The two macros of the downstream doc tests. For every type of its list, each writes a doc test, as a crate that
//! depends on the library writes it, that must not compile, and its twin, which must: the same match with a
//! wildcard arm, or the type named alone. A name of the list that is wrong fails the twin, which the compile-fail
//! test alone would not show, since stable Rust does not check the error code of a compile-fail test. Each test
//! stands on an item named after its type, so a failure names the type.

/// For each enum, a match that names every variant and no wildcard, which must not compile outside the crate, and
/// the same match with a wildcard arm, which must. The crate's own tests match each list exhaustively inside the
/// crate, where `#[non_exhaustive]` does not apply, so a variant that a list leaves out stops them from building:
/// the compile-fail test then still fails for `#[non_exhaustive]` alone.
///
/// The input names the public module in brackets, then each enum with its variants, one that carries data followed
/// by `(..)` or `{ .. }`.
macro_rules! no_exhaustive_match {
  (in $module:tt; $($name:ident $variants:tt)*) => {
    $(no_exhaustive_match_of! { $module $name $variants })*
  };
}

/// The two doc tests of one enum, and its match inside the crate.
macro_rules! no_exhaustive_match_of {
  (
    [$($module:ident)::+]
    $name:ident { $($variant:ident $(($($tuple:tt)*))? $({$($fields:tt)*})?),* $(,)? }
  ) => {
    #[cfg_attr(doctest, doc = concat!(
      "```compile_fail\n",
      "use ", stringify!($($module)::+), "::", stringify!($name), ";\n\n",
      "fn name(value: &", stringify!($name), ") -> &'static str {\n",
      "    match value {\n",
      $(
        "        ", stringify!($name), "::", stringify!($variant $(($($tuple)*))? $({$($fields)*})?),
        " => \"", stringify!($variant), "\",\n",
      )*
      "    }\n",
      "}\n",
      "```\n\n",
      "```\n",
      "use ", stringify!($($module)::+), "::", stringify!($name), ";\n\n",
      "fn name(value: &", stringify!($name), ") -> &'static str {\n",
      "    match value {\n",
      $(
        "        ", stringify!($name), "::", stringify!($variant $(($($tuple)*))? $({$($fields)*})?),
        " => \"", stringify!($variant), "\",\n",
      )*
      "        _ => \"newer\",\n",
      "    }\n",
      "}\n",
      "```\n",
    ))]
    #[cfg(doctest)]
    pub struct $name;

    #[cfg(test)]
    const _: Option<&str> = {
      type Matched = $($module)::+::$name;
      match None::<&Matched> {
        $(Some(Matched::$variant $(($($tuple)*))? $({$($fields)*})?) => Some(stringify!($variant)),)*
        None => None,
      }
    };
  };
}

/// For each struct, a struct literal that takes every field from another value, which must not compile outside the
/// crate, since a public struct's fields are private or the struct is `#[non_exhaustive]`, and the type named alone,
/// which must. A struct whose fields were all public, and which was not `#[non_exhaustive]`, would build.
///
/// The input names the public module in brackets, then each struct, a generic one with type arguments.
macro_rules! no_struct_literal {
  (in $module:tt; $($name:ident $(::<$($argument:ty),+>)?),* $(,)?) => {
    $(no_struct_literal_of! { $module $name $(::<$($argument),+>)? })*
  };
}

/// The two doc tests of one struct.
macro_rules! no_struct_literal_of {
  ([$($module:ident)::+] $name:ident $(::<$($argument:ty),+>)?) => {
    #[cfg_attr(doctest, doc = concat!(
      "```compile_fail\n",
      "use ", stringify!($($module)::+), "::", stringify!($name), ";\n\n",
      "let _built = ", stringify!($name $(::<$($argument),+>)?), " { ..todo!() };\n",
      "```\n\n",
      "```\n",
      "use ", stringify!($($module)::+), "::", stringify!($name), ";\n\n",
      "let _named: Option<", stringify!($name $(::<$($argument),+>)?), "> = None;\n",
      "```\n",
    ))]
    #[cfg(doctest)]
    pub struct $name;
  };
}
