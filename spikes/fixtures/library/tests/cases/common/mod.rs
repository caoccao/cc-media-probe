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

//! What the cases share, as the library's `tests/common/` would hold it: the conditions of a case, the places of
//! source entries and their paths, the split of a record into its semantic and adapter parts, the comparison of an
//! expectation with its file, the outcomes that adapters share or keep apart where a limit explains it, the usage
//! of each record against the case's limits and the measured reads, the I/O error kinds of a failed companion, the
//! match of a recipe's stated values, JSON pointers, and the corpus that the seed check writes.

pub mod conditions;
pub mod corpus;
pub mod expectation;
pub mod failures;
pub mod matcher;
pub mod outcomes;
pub mod pointer;
pub mod sources;
pub mod split;
pub mod usage;
