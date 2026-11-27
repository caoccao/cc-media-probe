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

//! The spike's own report of a toy file, which stands in for `MediaReport`, whose constructors are the library's:
//! the detection, the streams with their header states, the titles, the diagnostics, the source table and the
//! usage.

mod error;
mod header_status;
mod toy_report;

pub use error::ProbeError;
pub use header_status::{HeaderStatus, Reason};
pub use toy_report::{
  DetectionOutcome, Diagnostic, DiagnosticKind, SourceEntry, SourceOutcome, StreamReport, Title, ToyReport,
};
