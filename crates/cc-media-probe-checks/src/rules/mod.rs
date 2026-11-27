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

//! The structure rules that need no knowledge of names or literals: the
//! copyright header, module roots (R30), the line limit and inclusions
//! (R31), module files beside directories (R30), and the library's private fields (plan section 8.1).

mod constants;
mod header;
mod inclusion;
mod line_limit;
mod module_files;
mod module_roots;
mod public_fields;

pub use header::check_header;
pub use inclusion::check_inclusion;
pub use line_limit::{check_line_limit, counted_lines};
pub use module_files::check_module_files;
pub use module_roots::check_module_root;
pub use public_fields::check_public_fields;
