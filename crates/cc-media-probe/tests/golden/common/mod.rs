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

//! What the golden tests share: the conditions of a case; the test's own custom sources, blocking and async, and
//! its async reader for both runtimes, which implement the library's traits or a runtime's (plan section 7.1); the
//! places of source entries and their paths; each result's text read back by the library's strict reader; the split
//! of a record into its semantic and adapter parts; the comparison of an expectation with its file, whose text names
//! each member of an object once; the outcomes that adapters share or keep apart where a limit explains it; the
//! usage of each record against the case's limits and the measured reads; the match of a recipe's stated values;
//! and JSON pointers. A test target reaches only its own folder's modules without `#[path]`, which R31 forbids, so a
//! later test that needs these helpers is a module of this target or keeps its own (the maintainer's choice on
//! 2026-11-28).

pub mod conditions;
pub mod duplicates;
pub mod expectation;
pub mod matcher;
pub mod memory;
pub mod outcomes;
pub mod pending;
pub mod pointer;
pub mod round_trip;
pub mod sources;
pub mod split;
pub mod usage;
