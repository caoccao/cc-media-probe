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

//! The resolvers' surface, after `api.md` section 13: the request, the five results, the blocking and async
//! resolver traits, and the option that turns lookup on.

use std::future::Future;
use std::io;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::Arc;

use cc_media_probe::Limit;
use cc_media_probe::asynchronous::AsyncPositionalSource;
use cc_media_probe::source::PositionalSource;

/// A request for one companion, which carries a validated identifier and never a path taken from the media.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CompanionRequest {
  /// The source identifier, assigned before anything is opened.
  pub source_identifier: u64,
  /// The validated identifier.
  pub identifier: String,
  /// The name built from it, with the fixed extension.
  pub name: String,
  /// The path that the caller gave to `probe_file`, absent for every other input.
  pub primary_path: Option<PathBuf>,
}

/// A resolver's answer to one request: one of the five results of plan section 7.7.
pub enum Answer<Source> {
  /// The companion, with an identity token of the resolver's choosing where it knows one.
  Found {
    /// The source that serves the companion.
    source: Source,
    /// Bytes that name the underlying source: two equal tokens are one source.
    identity: Option<Vec<u8>>,
  },
  /// Nothing there.
  NotFound,
  /// Refused, with the resolver's reason.
  Declined(String),
  /// Not allowed by this limit, by the resolver's own account.
  LimitReached(Limit),
  /// Opening failed with this error, which ends the probe (R19).
  Failed(io::Error),
}

/// The answer of a blocking resolver.
pub type Resolution = Answer<Box<dyn PositionalSource + Send>>;
/// The answer of an async resolver.
pub type AsyncResolution = Answer<Pin<Box<dyn AsyncPositionalSource + Send>>>;

/// A blocking resolver, shared by the probes of one set of options.
pub trait CompanionResolver: Send + Sync {
  /// Answers one request.
  fn resolve(&self, request: &CompanionRequest) -> Resolution;
}

/// An async resolver. Each request keeps its state in its own future, so one resolver serves concurrent probes.
pub trait AsyncCompanionResolver: Send + Sync {
  /// Answers one request.
  fn resolve<'request>(
    &'request self,
    request: &'request CompanionRequest,
  ) -> Pin<Box<dyn Future<Output = AsyncResolution> + Send + 'request>>;
}

/// Lookup for the blocking entry points.
#[derive(Clone, Default)]
pub enum CompanionLookup {
  /// No other file is opened, the default.
  #[default]
  Off,
  /// The built-in resolver, whose trusted root is the folder of the file that `probe_file` probes. Another input
  /// has no folder, so it declines each request.
  BuiltIn,
  /// The caller's own resolver.
  Custom(Arc<dyn CompanionResolver>),
}
