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

//! The program's own message for each error that the parser reports (dependency dossier: "Usage errors are
//! turned into the CLI's own messages"). The argument comes from the error's context, which needs the
//! `error-context` feature.

use clap::error::{ContextKind, ContextValue, Error, ErrorKind};

use crate::constants::{TAKES_NO_VALUE, UNKNOWN_OPTION, UNREADABLE};

/// The message of a parser error.
pub fn message(error: &Error) -> String {
  let argument = error.get(ContextKind::InvalidArg).map(context_text);
  match (error.kind(), argument) {
    (ErrorKind::UnknownArgument, Some(argument)) => format!("{UNKNOWN_OPTION}: {argument}"),
    (ErrorKind::TooManyValues, Some(argument)) => format!("{argument} {TAKES_NO_VALUE}"),
    _ => UNREADABLE.to_owned(),
  }
}

fn context_text(value: &ContextValue) -> String {
  value.to_string()
}
