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

//! The cases of the I/O spike, `README.md` section 4: the same report from every input, the header assembly with
//! its limits and the parser's inputs, the forward-only window, the traversal's edges, the source protocol, the
//! budget, failing sources, and companions through both resolvers with their limits and identities.

#[cfg(test)]
mod assembly;
#[cfg(test)]
mod budget;
#[cfg(test)]
mod catalog;
#[cfg(test)]
mod companions;
#[cfg(test)]
mod failures;
#[cfg(test)]
mod forward;
#[cfg(test)]
mod limits;
#[cfg(test)]
mod lookup;
#[cfg(test)]
mod machine;
#[cfg(test)]
mod memory;
#[cfg(test)]
mod parity;
#[cfg(test)]
mod protocol;
#[cfg(test)]
mod representation;
#[cfg(test)]
mod support;
#[cfg(test)]
mod traversal;
#[cfg(test)]
mod units;
