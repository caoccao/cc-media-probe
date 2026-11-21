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

//! Uses each normal dependency once, so that the linked binary holds code
//! from all of them: the argument parser, the inflater, the derived
//! serialization and the JSON writer.

use std::io::{self, Write};
use std::process::ExitCode;

use cc_media_probe::miniz_oxide::inflate::decompress_to_vec_zlib;
use clap::{Arg, ArgAction, Command};

use crate::constants::{INPUT_ARGUMENT, PROGRAM_NAME};

/// What the binary writes: its inputs, and the inflated size of the first.
#[derive(cc_media_probe::serde::Serialize)]
#[serde(crate = "cc_media_probe::serde")]
struct Summary {
  inputs: Vec<String>,
  inflated_size: Option<usize>,
}

/// Parses the arguments, inflates the first one's bytes as a zlib stream and
/// writes a JSON summary to standard output.
pub fn run() -> ExitCode {
  let command = Command::new(PROGRAM_NAME).arg(Arg::new(INPUT_ARGUMENT).action(ArgAction::Append));
  let Ok(matches) = command.try_get_matches() else {
    return ExitCode::FAILURE;
  };
  let inputs: Vec<String> = matches
    .get_many::<String>(INPUT_ARGUMENT)
    .map_or_default(|values| values.cloned().collect());
  let inflated_size = inputs
    .first()
    .and_then(|input| decompress_to_vec_zlib(input.as_bytes()).ok())
    .map(|bytes| bytes.len());
  let summary = Summary { inputs, inflated_size };
  let Ok(text) = serde_json::to_string(&summary) else {
    return ExitCode::FAILURE;
  };
  if writeln!(io::stdout().lock(), "{text}").is_err() {
    return ExitCode::FAILURE;
  }
  ExitCode::SUCCESS
}
