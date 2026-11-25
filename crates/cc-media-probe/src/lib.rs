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

//! `cc-media-probe` inspects multimedia files and reports what they contain: container format, streams, codec
//! parameters, chapters, tags and attachments. It never decodes audio or video. One universal API serves every
//! supported format.
//!
//! The library is in development, and nothing is published yet. This build holds the public data model of the
//! API, whose [specification](https://github.com/caoccao/cc-media-probe/blob/main/docs/design/api.md) the
//! development plan approved; the entry points, the options and the serialized form follow.
//!
//! # What a probe reports
//!
//! - [`MediaReport`]: the [`Format`], the [`Stream`]s with their [`Codec`] and the video, audio and subtitle
//!   information, the chapters, tags and attachments, the diagnostics, and the [`completeness`] that says what was
//!   established, what was not, and why (R23).
//! - [`Detection`]: what detection found, one of six outcomes.
//! - [`Error`]: an ending of the ordered procedure without a report, with what the probe established on the way;
//!   [`error::ErrorRecord`] is its serializable description.
//!
//! Every value that a file may not state is an `Option`, never a zero or an empty string. Every public enum is
//! non-exhaustive, and every enum that is serialized keeps a value that a newer version wrote in its catch-all,
//! `Unrecognized`, whose text `as_str()` returns.
//!
//! # Modules
//!
//! - [`model`]: every other type of the report model, and the value types such as [`Rational`] and [`Seconds`].
//! - [`completeness`]: the statuses, parts and reasons, and the fixed-size [`completeness::LimitRecord`].
//! - [`diagnostics`]: what a probe noticed on the way.
//! - [`details`]: the typed details of formats and codecs.
//! - [`source`]: the source table and the types that name a source.
//! - [`error`]: the error record and the parts of an error.

#![deny(missing_debug_implementations)]

mod codec;
pub mod completeness;
mod constants;
mod data_enumeration;
pub mod details;
pub mod diagnostics;
pub mod error;
mod format;
pub mod model;
mod prober;
pub mod source;
mod string_enumeration;
#[cfg(test)]
mod tests;
mod utilities;

pub use codec::Codec;
pub use error::{Error, ErrorKind};
pub use format::Format;
pub use model::{Detection, MediaReport, Rational, Seconds, Stream, StreamKind};
pub use prober::Limit;
