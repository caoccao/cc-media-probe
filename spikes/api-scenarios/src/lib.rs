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

//! A stub of the public surface that `docs/design/api.md` specifies, as far as its usage scenarios use it. The
//! types, accessors and entry points have the specified signatures. Nothing is parsed: every probe opens or
//! touches its input and then returns the error of kind "unsupported format". The scenarios are the programs in
//! `examples/`; they only need to compile.

#![forbid(unsafe_code)]

pub mod asynchronous;
pub mod completeness;
mod constants;
#[cfg(feature = "serde")]
mod decimal;
pub mod details;
pub mod diagnostics;
#[cfg(feature = "serde")]
pub mod document;
mod entry_points;
pub mod error;
#[cfg(feature = "serde")]
mod kept;
mod limits;
pub mod model;
#[cfg(feature = "serde")]
mod presence;
mod prober;
pub mod source;
#[cfg(feature = "serde")]
mod twin;

pub use entry_points::{probe_bytes, probe_file, probe_reader, probe_source, probe_stream};
pub use error::{Error, ErrorKind};
pub use limits::{Limit, Limits};
pub use model::{Codec, Detection, Format, MediaReport, Rational, Seconds, Stream, StreamKind};
pub use prober::Prober;
