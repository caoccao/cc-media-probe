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

//! `ProbeError`, how a probe ends without a report.

use std::io;

use super::ToyReport;

/// How a probe ends without a report.
#[derive(Debug)]
pub enum ProbeError {
  /// A source failed (R19): its identifier, the original cause, and what the probe had established.
  SourceFailed {
    /// The identifier of the source that failed.
    source: u64,
    /// The error that it failed with.
    cause: io::Error,
    /// The report built so far, with the failed source's entry.
    partial: Box<ToyReport>,
  },
  /// A blocking probe's future was not ready on its first poll, which no blocking input causes (D8).
  Stalled,
}
