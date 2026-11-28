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

//! The golden tests of the library (plan section 9.2, `docs/design/test-kit.md` section 4): each hex fixture of the
//! test kit in each of its cases through each adapter, blocking and async, compared with its expectations in the
//! library's JSON form, or written when `CC_MEDIA_PROBE_WRITE_EXPECTATIONS` is set; with the tests of the
//! protocol's own parts: the conditions of a case, the test's sources and readers, the outcomes that adapters share
//! or keep apart, the usage of each record against the limits, the paths of a record, the round trip of each result
//! and the strict reading of an expectation's text, the writing of an expectation file, the split of a record, the
//! values that a recipe states, and the smallest records that those tests build. The target requires every feature
//! of the library, so that each expectation has one feature set (section 4.3).

#[cfg(test)]
mod adapters;
#[cfg(test)]
mod common;
#[cfg(test)]
mod conditions;
#[cfg(test)]
mod expectations;
#[cfg(test)]
mod folder;
#[cfg(test)]
mod outcomes;
#[cfg(test)]
mod probes;
#[cfg(test)]
mod recipes;
#[cfg(test)]
mod records;
#[cfg(test)]
mod round_trip;
#[cfg(test)]
mod sources;
#[cfg(test)]
mod split;
#[cfg(test)]
mod usage;
#[cfg(test)]
mod writing;
