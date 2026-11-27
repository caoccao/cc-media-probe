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

//! The toy container's walker, written once as `async` code over the internal source (D8): one traversal in file
//! order with an explicit stack, which offers every packet it reaches to its stream's assembler (plan section 7.4,
//! "One traversal feeds every stream"), and the probe around it, with detection and the companions.

mod catalog;
mod constants;
mod cursor;
mod element_action;
mod identifier_set;
mod identities;
mod packet;
mod probe;
mod state;
mod walk;

pub(crate) use cursor::{Action, Cursor, Header};
pub(crate) use element_action::title_target;
pub(crate) use packet::Packet;
pub(crate) use probe::{Primary, probe};
pub(crate) use state::{PRIMARY, Walk};
