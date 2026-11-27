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

//! The constants of the toy codec, which `README.md` section 3 defines.

use std::mem::size_of;

use super::types::{Consumption, Continuation};

/// The consumption of a packet that the parser needs nothing more of.
pub const PACKET_FINISHED: Consumption = Consumption::Packet { finished: true };
/// The fixed size of the continuation value, which counts toward the stream's header limit and its working memory
/// (plan sections 7.4, "Continuation", and 7.5).
pub const CONTINUATION_SIZE: usize = size_of::<Continuation>();

/// The length of a unit's kind byte, which comes first: a payload unit is known by it alone.
pub const KIND_LENGTH: usize = 1;
/// The length field of a unit in the representation with one-byte lengths.
pub const ONE_BYTE_LENGTH_SIZE: usize = 1;
/// The length field of a unit in the representation with two-byte lengths.
pub const TWO_BYTE_LENGTH_SIZE: usize = 2;
/// A unit of payload, which carries no header.
pub const PAYLOAD_KIND: u8 = 0;
/// The setup unit: width, height, parameter count and flags.
pub const SETUP_KIND: u8 = 1;
/// The parameters unit, whose length only the setup's parameter count tells.
pub const PARAMETERS_KIND: u8 = 2;
/// The length of the setup unit's body.
pub const SETUP_LENGTH: usize = 6;
/// The length of one parameter.
pub const PARAMETER_LENGTH: usize = 2;
/// The flag of the setup unit that says the parameters cannot be read without decoding.
pub const NEEDS_DECODING_FLAG: u8 = 0b0000_0001;
