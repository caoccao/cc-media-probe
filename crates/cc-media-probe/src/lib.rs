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
//! development plan approved, its entry points, options, limits and source traits, blocking and async, and, with
//! the `serde` feature, the serialized form of the report model and its root records; the document of the command
//! line follows. No format is detected yet: until the detection inventory arrives, the bytes of every
//! input match nothing, so detection gives the outcome `Unknown`, or `HintOnly` for a format hint, and a probe the
//! error `UnknownFormat`.
//!
//! # Probing
//!
//! ```no_run
//! # fn main() -> Result<(), cc_media_probe::Error> {
//! use cc_media_probe::{Limits, Prober};
//!
//! let report = cc_media_probe::probe_file("movie.mkv")?;
//! println!("{}: {} streams", report.format(), report.streams().len());
//!
//! let prober = Prober::new().with_limits(Limits::default().with_bytes_read(16 * 1024 * 1024));
//! let detection = prober.detect_file("unknown.bin")?;
//! println!("{}", detection.outcome().as_str());
//! # Ok(())
//! # }
//! ```
//!
//! - Ten functions at the crate root probe or detect with the default options: [`probe_file`], [`probe_reader`]
//!   for a `Read + Seek`, [`probe_stream`] for a forward-only `Read` such as standard input, [`probe_bytes`] and
//!   [`probe_source`] for the caller's own [`source::PositionalSource`], and the five `detect_*` functions.
//! - [`Prober`] carries the options, among them the [`Limits`] of the shared budget, and has the same ten methods.
//! - [`asynchronous`] holds the async surface: a caller's own async source needs no feature, and the features
//!   `tokio` and `futures-io` add one module each for the reader traits of their runtime.
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
//! - [`model`]: every other type of the report model, the value types such as [`Rational`] and [`Seconds`], and
//!   the enums of the options.
//! - [`completeness`]: the statuses, parts and reasons, and the fixed-size [`completeness::LimitRecord`].
//! - [`diagnostics`]: what a probe noticed on the way.
//! - [`details`]: the typed details of formats and codecs.
//! - [`source`]: the source table, the types that name a source, and the trait of a caller's own source.
//! - [`error`]: the error record and the parts of an error.
//! - [`asynchronous`]: the async source trait, its functions and handles, and the modules of the two runtimes.
#![cfg_attr(
  feature = "serde",
  doc = "- [`document`]: with the `serde` feature, the version of the serialized form, `SchemaVersion`."
)]
#![cfg_attr(
  not(feature = "serde"),
  doc = "- `document`: with the `serde` feature, the version of the serialized form, `SchemaVersion`."
)]
//!
//! # Features
//!
#![cfg_attr(
  feature = "serde",
  doc = "- `serde`, off by default: `Serialize` and `Deserialize` for the report model, read by the strict rules of the
  JSON contract, and [`document`]."
)]
#![cfg_attr(
  not(feature = "serde"),
  doc = "- `serde`, off by default: `Serialize` and `Deserialize` for the report model, read by the strict rules of the
  JSON contract, and `document`."
)]
#![cfg_attr(
  feature = "tokio",
  doc = "- `tokio`, off by default: [`asynchronous::tokio`], with Tokio's `AsyncRead`, `AsyncSeek` and file type."
)]
#![cfg_attr(
  not(feature = "tokio"),
  doc = "- `tokio`, off by default: `asynchronous::tokio`, with Tokio's `AsyncRead`, `AsyncSeek` and file type."
)]
#![cfg_attr(
  feature = "futures-io",
  doc = "- `futures-io`, off by default: [`asynchronous::futures`], with the `AsyncRead` and `AsyncSeek` of futures-io."
)]
#![cfg_attr(
  not(feature = "futures-io"),
  doc = "- `futures-io`, off by default: `asynchronous::futures`, with the `AsyncRead` and `AsyncSeek` of futures-io."
)]
//!
//! Features are additive: enabling one never removes or changes an item.

#![deny(missing_debug_implementations)]

pub mod asynchronous;
mod codec;
pub mod completeness;
mod constants;
mod data_enumeration;
pub mod details;
mod detect;
pub mod diagnostics;
#[cfg(feature = "serde")]
pub mod document;
mod entry;
pub mod error;
#[cfg(feature = "serde")]
mod form;
mod form_record;
mod format;
pub mod model;
mod prober;
pub mod source;
mod string_enumeration;
#[cfg(test)]
mod tests;
mod utilities;

pub use codec::Codec;
pub use entry::{
  detect_bytes, detect_file, detect_reader, detect_source, detect_stream, probe_bytes, probe_file, probe_reader,
  probe_source, probe_stream,
};
pub use error::{Error, ErrorKind};
pub use format::Format;
pub use model::{Detection, MediaReport, Rational, Seconds, Stream, StreamKind};
pub use prober::{Limit, Limits, Prober};
