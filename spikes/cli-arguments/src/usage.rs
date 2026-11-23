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
//! turned into the CLI's own messages"). The parser's context names the option, which needs the `error-context`
//! feature. For a long option, the message names the argument as written, value and all, so `--bogus=value` and
//! `--=x` are named whole (`docs/design/cli.md` section 3).

use std::ffi::OsString;

use clap::error::{ContextKind, ContextValue, Error, ErrorKind};

use crate::constants::{SEPARATOR, TAKES_NO_VALUE, UNKNOWN_OPTION, UNREADABLE, VALUE_SIGN};

/// The message of a parser error about these arguments.
pub fn message(error: &Error, arguments: &[OsString]) -> String {
  let argument = error.get(ContextKind::InvalidArg).map(context_text);
  match (error.kind(), argument) {
    (ErrorKind::UnknownArgument, Some(argument)) => {
      format!("{UNKNOWN_OPTION}: {}", as_written(&argument, arguments))
    }
    (ErrorKind::TooManyValues, Some(argument)) => format!("{argument} {TAKES_NO_VALUE}"),
    _ => UNREADABLE.to_owned(),
  }
}

fn context_text(value: &ContextValue) -> String {
  value.to_string()
}

/// The first argument before the separator that the parser's name for a long option stands for: the name itself, or
/// the name with a value after `=`. A short letter of a group, such as `-j` of `-json`, stays as the parser names it.
fn as_written(name: &str, arguments: &[OsString]) -> String {
  if !name.starts_with(SEPARATOR) {
    return name.to_owned();
  }
  arguments
    .iter()
    .map(|argument| argument.to_string_lossy())
    .take_while(|argument| argument != SEPARATOR)
    .find(|argument| {
      argument
        .strip_prefix(name)
        .is_some_and(|rest| rest.is_empty() || rest.starts_with(VALUE_SIGN))
    })
    .map_or_else(|| name.to_owned(), std::borrow::Cow::into_owned)
}
