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

//! The constants of the document and its root records (R33; `json-output.md` section 4.1).

/// The major version of the form that this release writes and reads (`stability.md` section 5.2).
pub const SCHEMA_MAJOR: u32 = 1;

/// The minor version of the form that this release writes.
pub const SCHEMA_MINOR: u32 = 0;

/// The separator of a version's two numbers.
pub const VERSION_SEPARATOR: char = '.';

/// The serialized name of the outcome `ok` (`json-output.md` section 3).
pub const OK_NAME: &str = "ok";

/// The serialized name of the outcome `partial`.
pub const PARTIAL_NAME: &str = "partial";

/// The serialized name of the outcome `error`.
pub const ERROR_NAME: &str = "error";

/// The member of an entry that holds its outcome, which decides whether the entry keeps the members that it does not
/// read.
pub const OUTCOME_MEMBER: &str = "outcome";

/// The member of a document that names its generator.
pub const GENERATOR_MEMBER: &str = "generator";

/// The member of a document that holds its usage error.
pub const USAGE_ERROR_MEMBER: &str = "usage_error";

/// The member of a document that holds its entries.
pub const RESULTS_MEMBER: &str = "results";

/// The name of a document's record, which a format that names records gives it.
pub const DOCUMENT_NAME: &str = "Document";

/// The members of a document: its version, its generator, its usage error and its results.
pub const DOCUMENT_MEMBER_COUNT: usize = 4;

/// What a document's results are read as.
pub const RESULTS_EXPECTING: &str = "the entries of a document, an array";

/// The message of a document that holds a usage error beside results (`api.md` section 12).
pub const USAGE_ERROR_WITH_RESULTS_MESSAGE: &str = "a document with a usage error holds no results";

/// The message of an entry whose index is not its position.
pub const POSITION_MESSAGE: &str = "an entry's index is its position in the results";

/// The message of an entry of a known outcome that holds no record or several.
pub const ENTRY_WITHOUT_ONE_RECORD_MESSAGE: &str =
  "an entry of a known outcome holds exactly one of a report, a detection and an error record";

/// The message of an entry whose record does not allow its outcome.
pub const OUTCOME_DISAGREES_MESSAGE: &str = "an entry's outcome disagrees with the record it holds";

/// The message of a writer asked for a position beyond 2^53 - 1, the limit of every number of the form, beyond which
/// JSON implementations need not agree on an integer's value (RFC 8259 section 6; RFC 7493 section 2.2).
pub const POSITION_RANGE_MESSAGE: &str = "a document's last index is 2^53 - 1";

/// The message of a writer whose entries were taken already, which a serializer that writes a value twice meets.
pub const ENTRIES_TAKEN_MESSAGE: &str = "the entries of a document written as it goes were already written";

/// The display of `DeriveOutcomeError`.
pub const DERIVE_OUTCOME_MESSAGE: &str =
  "the record holds a value of a newer version that decides the entry's outcome, so the outcome must be given";

/// The members of a document of results, which has no usage error.
pub const RESULTS_DOCUMENT_MEMBER_COUNT: usize = 3;
