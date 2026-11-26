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

//! `MediaReport`, what a probe found (`api.md` section 6.2), with its stream inventory class (section 5.2) and the
//! tag lookups (section 6.9). The crate root re-exports `MediaReport`.

mod constants;
#[cfg(feature = "serde")]
mod header_reasons;
mod inventory_class;
mod lookups;
mod media_report;
#[cfg(feature = "serde")]
mod references;
#[cfg(feature = "serde")]
mod report_form;
mod selection;
#[cfg(feature = "serde")]
mod shortfalls;
#[cfg(feature = "serde")]
mod skipped;

pub use inventory_class::InventoryClass;
pub use lookups::{FileTags, StreamTags, TagsWithKey};
pub use media_report::MediaReport;
