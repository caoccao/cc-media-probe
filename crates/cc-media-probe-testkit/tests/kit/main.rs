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

//! The test kit's own tests: the text format and what it refuses, the layout of the fixtures' folder, the two
//! sample fixtures against second transcriptions of their bytes, the writers against the specifications' vectors,
//! the readers with their faults, and the paths of generated test files, with a folder of its own for each test
//! that writes files.

#[cfg(test)]
mod generated;
#[cfg(test)]
mod layout;
#[cfg(test)]
mod readers;
#[cfg(test)]
mod samples;
#[cfg(test)]
mod scratch;
#[cfg(test)]
mod syntax;
#[cfg(test)]
mod writers;
