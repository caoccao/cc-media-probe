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

//! What a target's entry point reached: the parser that the seed check asks for, its outcome, and for the toy
//! container each stream's header state.

use super::constants::REJECTED;

/// A parser that an entry point can reach.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Parser {
  /// The toy container's walker.
  ToyContainer,
  /// The toy codec's parser.
  ToyCodec,
  /// Detection, through the public `detect_bytes`.
  Detection,
  /// The full public probe, through `probe_bytes`.
  PublicProbe,
}

/// The parser that an input reached, if any, and what it gave.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reached {
  /// The parser, or `None` for an input rejected before any parser.
  pub parser: Option<Parser>,
  /// The outcome, as fixtures record it: `rejected` for an input that its encoding rejects, and detection's
  /// outcome for one that a container target's detection rejects.
  pub outcome: String,
  /// For the toy container, each declared stream's header state.
  pub streams: Vec<&'static str>,
}

impl Reached {
  /// An input rejected before any parser, with the outcome that rejected it.
  pub(crate) fn rejected(outcome: &str) -> Reached {
    Reached {
      parser: None,
      outcome: outcome.to_owned(),
      streams: Vec::new(),
    }
  }

  /// An input that its encoding rejects.
  pub(crate) fn malformed_input() -> Reached {
    Reached::rejected(REJECTED)
  }

  /// An input that reached `parser` with `outcome`.
  pub(crate) fn at(parser: Parser, outcome: &str) -> Reached {
    Reached {
      parser: Some(parser),
      outcome: outcome.to_owned(),
      streams: Vec::new(),
    }
  }
}
