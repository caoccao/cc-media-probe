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

//! The fuzz targets and their entry points, each of which decodes its target's input encoding and runs a private
//! parser or the public API, and the one prober that the `detection` and `probe` targets and the golden tests' cases
//! start from (`docs/design/test-kit.md` section 6.3).

use std::ffi::OsString;

use cc_media_probe::Prober;

use super::constants::{DETECTION, PROBE, REPORT, TOY, TOY_CODEC};
use super::encoding::{decode_codec, decode_named};
use super::reached::{Parser, Reached};
use crate::toy::{Continuation, INSUFFICIENT_DATA, UNKNOWN_FORMAT, parse, walk};

/// A fuzz target.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Target {
  /// The toy container: a whole file.
  Toy,
  /// The toy codec: a representation, a context and the bytes.
  ToyCodec,
  /// Detection: a file-name hint and the bytes.
  Detection,
  /// The full public probe: a file-name hint and the bytes. Every fixture is its seed.
  Probe,
}

impl Target {
  /// The target that a fixture names, or `None` for one that this build does not have.
  #[must_use]
  pub fn from_name(name: &str) -> Option<Target> {
    match name {
      TOY => Some(Target::Toy),
      TOY_CODEC => Some(Target::ToyCodec),
      DETECTION => Some(Target::Detection),
      PROBE => Some(Target::Probe),
      _ => None,
    }
  }

  /// The target's name, which is also its folder in `fuzz/corpus/` and its binary in `fuzz/`.
  #[must_use]
  pub fn name(self) -> &'static str {
    match self {
      Target::Toy => TOY,
      Target::ToyCodec => TOY_CODEC,
      Target::Detection => DETECTION,
      Target::Probe => PROBE,
    }
  }

  /// The parser that a seed of the target is written for.
  #[must_use]
  pub fn parser(self) -> Parser {
    match self {
      Target::Toy => Parser::ToyContainer,
      Target::ToyCodec => Parser::ToyCodec,
      Target::Detection => Parser::Detection,
      Target::Probe => Parser::PublicProbe,
    }
  }
}

/// Runs `input`, in `target`'s input encoding, through the target's entry point.
#[must_use]
pub fn run(target: Target, input: &[u8]) -> Reached {
  match target {
    Target::Toy => toy(input),
    Target::ToyCodec => toy_codec(input),
    Target::Detection => detection(input),
    Target::Probe => probe(input),
  }
}

/// The toy container's entry point: the input is a whole file, which detection must take for a toy file.
#[must_use]
pub fn toy(input: &[u8]) -> Reached {
  let walked = walk(input);
  if walked.outcome == UNKNOWN_FORMAT || walked.outcome == INSUFFICIENT_DATA {
    return Reached::rejected(walked.outcome);
  }
  Reached {
    streams: walked.streams,
    ..Reached::at(Parser::ToyContainer, walked.outcome)
  }
}

/// The toy codec's entry point: the input is a representation's code, the context, and the bytes of one whole
/// packet, of the length that the context states, or of a stream to its end, whose context states 0.
#[must_use]
pub fn toy_codec(input: &[u8]) -> Reached {
  let Some((codec_input, context, payload)) = decode_codec(input) else {
    return Reached::malformed_input();
  };
  let result = parse(codec_input, context, payload, Continuation::default(), true);
  Reached::at(Parser::ToyCodec, result.outcome.name())
}

/// The detection target's entry point: the input is a file-name hint and the bytes.
#[must_use]
pub fn detection(input: &[u8]) -> Reached {
  let Some((hint, payload)) = decode_named(input) else {
    return Reached::malformed_input();
  };
  let outcome = match prober(hint).detect_bytes(payload) {
    Ok(detection) => detection.outcome().as_str().to_owned(),
    Err(error) => error.kind().as_str().to_owned(),
  };
  Reached::at(Parser::Detection, &outcome)
}

/// The full public probe's entry point: the input is a file-name hint and the bytes; the outcome is `report`, or
/// the error's kind.
#[must_use]
pub fn probe(input: &[u8]) -> Reached {
  let Some((hint, payload)) = decode_named(input) else {
    return Reached::malformed_input();
  };
  let outcome = match prober(hint).probe_bytes(payload) {
    Ok(_) => REPORT.to_owned(),
    Err(error) => error.kind().as_str().to_owned(),
  };
  Reached::at(Parser::PublicProbe, &outcome)
}

/// The prober of the default case: the default prober, with its lookup off, and the hint where one is given. The
/// `detection` and `probe` targets run it, so the seed check does too, and the golden tests start each case's
/// prober from it, so that a default case's expectation and the `probe` target's outcome come from one setup. The
/// permanent library adds the toy format here, through the one private function that `with_toy_format` calls,
/// in every build that compiles this module.
#[must_use]
pub fn prober(hint: Option<OsString>) -> Prober {
  let prober = Prober::new();
  match hint {
    Some(hint) => prober.with_file_name_hint(hint),
    None => prober,
  }
}
