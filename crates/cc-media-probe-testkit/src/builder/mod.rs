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

//! Builders (plan section 9.1, `docs/design/test-kit.md` section 5): a byte writer, a bit writer in either order of
//! the bits in a byte, and the escape of a NAL unit's payload against start-code emulation. The container
//! builders, which compute each size, length, offset and checksum at `build()`, arrive with their formats, and the
//! toy format's with plan Phase 3, task 5.

mod bit_writer;
mod byte_writer;
mod constants;
mod emulation_prevention;
mod error;

pub use bit_writer::{BitOrder, BitWriter};
pub use byte_writer::ByteWriter;
pub use emulation_prevention::insert_emulation_prevention;
pub use error::BuildError;
