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

//! Values kept as written, `api.md` section 12. A reader holds the members of an enum that carries data before it
//! looks at the kind, so that the order of the members never matters and the kind alone decides how they are read. A
//! kind that a newer version wrote keeps its other members unread and writes them back unchanged, as the maintainer
//! chose at the seventh review, so the version that knows the kind reads its value again. A known kind reads its own
//! members from the buffer into their types.

mod kinds;
mod members;
mod reader;
mod value;

pub(crate) use kinds::{read_kind, refuse_other_kinds, write_kept};
pub(crate) use members::KeptMembers;
