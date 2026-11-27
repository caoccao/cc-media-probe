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

//! `Companions`, how the probe resolves a request, written once for the blocking and the async resolvers: each
//! wraps its found source in a block cache, so the walker reads a companion as it reads the primary input.

use std::future::{Future, ready};
use std::io;
use std::pin::Pin;
use std::sync::Arc;

use cc_media_probe::asynchronous::AsyncPositionalSource;
use cc_media_probe::source::PositionalSource;

use super::{Answer, AsyncCompanionResolver, CompanionRequest, CompanionResolver};
use crate::options::Options;
use crate::report::SourceOutcome;
use crate::source::{AsyncCustom, BlockCache, BlockingPositional, InternalSource};

/// What the probe makes of an answer.
pub(crate) enum Resolved<Source> {
  /// A source to read, with its identity token.
  Found(Source, Option<Vec<u8>>),
  /// No source, with the outcome for the source table.
  Other(SourceOutcome),
  /// Opening failed.
  Failed(io::Error),
}

/// How a probe resolves its requests.
pub(crate) trait Companions {
  /// The internal source of a found companion.
  type Source: InternalSource;

  /// Whether lookup is on.
  fn enabled(&self) -> bool;

  /// Resolves one request.
  async fn resolve(&mut self, request: &CompanionRequest, options: &Options) -> Resolved<Self::Source>;
}

/// The blocking resolver of a probe, if lookup is on.
pub(crate) struct BlockingLookup(pub(crate) Option<Arc<dyn CompanionResolver>>);

impl Companions for BlockingLookup {
  type Source = BlockCache<BlockingPositional<Box<dyn PositionalSource + Send>>>;

  fn enabled(&self) -> bool {
    self.0.is_some()
  }

  fn resolve(&mut self, request: &CompanionRequest, options: &Options) -> impl Future<Output = Resolved<Self::Source>> {
    ready(match &self.0 {
      Some(resolver) => convert(resolver.resolve(request), |source| {
        BlockCache::new(
          BlockingPositional(source),
          options.block_size.get(),
          options.cached_blocks.get(),
        )
      }),
      None => Resolved::Other(SourceOutcome::NotFound),
    })
  }
}

/// The async resolver of a probe, if lookup is on.
pub(crate) struct AsyncLookup(pub(crate) Option<Arc<dyn AsyncCompanionResolver>>);

impl Companions for AsyncLookup {
  type Source = BlockCache<AsyncCustom<Pin<Box<dyn AsyncPositionalSource + Send>>>>;

  fn enabled(&self) -> bool {
    self.0.is_some()
  }

  async fn resolve(&mut self, request: &CompanionRequest, options: &Options) -> Resolved<Self::Source> {
    match &self.0 {
      Some(resolver) => convert(resolver.resolve(request).await, |source| {
        BlockCache::new(
          AsyncCustom(source),
          options.block_size.get(),
          options.cached_blocks.get(),
        )
      }),
      None => Resolved::Other(SourceOutcome::NotFound),
    }
  }
}

/// The probe's view of an answer, with a found source wrapped by `wrap`.
fn convert<Source, Wrapped>(answer: Answer<Source>, wrap: impl FnOnce(Source) -> Wrapped) -> Resolved<Wrapped> {
  match answer {
    Answer::Found { source, identity } => Resolved::Found(wrap(source), identity),
    Answer::NotFound => Resolved::Other(SourceOutcome::NotFound),
    Answer::Declined(reason) => Resolved::Other(SourceOutcome::Declined(reason)),
    Answer::LimitReached(limit) => Resolved::Other(SourceOutcome::LimitReached(limit)),
    Answer::Failed(error) => Resolved::Failed(error),
  }
}
