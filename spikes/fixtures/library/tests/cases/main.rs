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

//! The cases of the fixture spike's stand-in for the library: the seed check over every fixture, which also writes
//! the corpus, the configuration names of a repository build, the expectations in the library's JSON form in each
//! case through each blocking adapter, the conditions of a case, the split of a record, the values that a recipe
//! states, and the toy walker's and parser's outcomes.

#[cfg(test)]
mod common;
#[cfg(test)]
mod conditions;
#[cfg(test)]
mod configuration;
#[cfg(test)]
mod expectations;
#[cfg(test)]
mod recipes;
#[cfg(test)]
mod seeds;
#[cfg(test)]
mod split;
#[cfg(test)]
mod toy;
