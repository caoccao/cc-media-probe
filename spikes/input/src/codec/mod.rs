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

//! The toy codec `TCOD` of `README.md` section 3: a pure parser over byte slices that follows the parsing contract
//! of plan section 7.4 (D17), with a header of two units in which the second needs a fact of the first, in two
//! representations.

mod constants;
mod parser;
mod types;

pub use constants::{
  CONTINUATION_SIZE, KIND_LENGTH, NEEDS_DECODING_FLAG, ONE_BYTE_LENGTH_SIZE, PARAMETER_LENGTH, PARAMETERS_KIND,
  PAYLOAD_KIND, SETUP_KIND, SETUP_LENGTH, TWO_BYTE_LENGTH_SIZE,
};
pub use parser::{length_size, parse};
pub use types::{
  Consumption, Context, Continuation, Framing, Header, Input, Outcome, ParseResult, Representation, Setup,
};
