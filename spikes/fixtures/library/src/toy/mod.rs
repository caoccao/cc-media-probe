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

//! The toy format's walker and its codec's parser, private to the library and compiled only in repository builds
//! and for fuzzing (plan section 7.2), as the permanent `toy/` is. The parser is the I/O spike's, which G3.1
//! accepted; the walk is reduced from its walker.

mod codec;
mod constants;
mod detect;
mod representation;
mod streams;
mod walk;

pub(crate) use codec::{Context, Continuation, Framing, Input, parse};
pub(crate) use constants::{INSUFFICIENT_DATA, PACKET_MODE, STREAM_MODE, UNKNOWN_FORMAT};
pub(crate) use representation::Representation;
pub(crate) use walk::walk;
