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

//! The document of the serialized form, its entries and its writer, and the version of its root records (`api.md`
//! section 12; `json-output.md` sections 3 and 4), with the `serde` feature. A document is the output of `--json`
//! that the command-line contract specifies, which the command-line tool does not write yet: it writes it from its
//! skeleton on (development plan, Phase 3, task 21). A library user writes the same document with
//! [`Document::from_results`] or, one entry at a time, [`Document::serialize_results`].

mod constants;
mod contents;
mod derive_outcome_error;
mod document_form;
mod entry;
mod entry_form;
mod generator;
mod outcome;
mod schema_version;
mod usage_error;
mod warranted;
mod writer;

pub use contents::Document;
pub use derive_outcome_error::DeriveOutcomeError;
pub use entry::DocumentEntry;
pub use generator::Generator;
pub use outcome::EntryOutcome;
pub use schema_version::SchemaVersion;
pub use usage_error::UsageError;
