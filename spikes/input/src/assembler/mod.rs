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

//! The bounded per-stream header assembler of plan section 7.4 (D17), which feeds the toy codec's parser with pieces
//! of packets or of a byte stream.

mod constants;
mod piece;
mod stream_assembler;

pub(crate) use piece::Piece;
pub(crate) use stream_assembler::Assembler;
