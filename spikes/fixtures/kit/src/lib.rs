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

//! The stand-in for the test kit (plan section 7.1) in the fixture spike of plan Phase 3, task 3, for gate G3.2: the
//! text format of hand-annotated hex fixtures and its assembler, the fixtures with the routes of their seeds and the
//! places of their expectations, a counting reader, and a byte writer with a builder of toy files. It uses the
//! standard library only and never depends on the library, as the test kit must not. See `../README.md`.

#![forbid(unsafe_code)]

pub mod builder;
pub mod fixture;
pub mod readers;
pub mod syntax;
