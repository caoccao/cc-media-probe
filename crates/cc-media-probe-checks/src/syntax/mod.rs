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

//! Structural analysis over tokens: matching delimiters, attribute ranges,
//! the top-level items of a file and its embedded test block.

mod attributes;
pub mod constants;
mod delimiters;
mod item;
mod outline;
mod parsed_source;
mod sequences;
mod test_block;

pub use attributes::attribute_ranges;
pub use delimiters::{Delimiters, match_delimiters};
pub use item::{Item, ItemKind};
pub use outline::outline;
pub use parsed_source::ParsedSource;
pub use sequences::{
  is_fat_arrow, is_path_separator, is_path_separator_before, is_thin_arrow, previous_is_path_separator,
};
pub use test_block::{TestBlock, find_test_block};
