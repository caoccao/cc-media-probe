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

//! The comparison of a value with its expectation file, exactly, or its writing when the environment variable
//! `CC_MEDIA_PROBE_WRITE_EXPECTATIONS` asks for it: the explicit action that regenerates expectations, whose diff is
//! then reviewed (plan section 9.1).
//!
//! A file holds exactly the text that the writer writes for its value: the members of each object sorted, by an
//! explicit sort, whichever order `serde_json::Map` keeps, in pretty JSON with a final line feed. Its text is read
//! for a member named twice in one object, which the value would not show (`common::duplicates`), and once parsed
//! must be the text that its value is written as, so that a value spelled in any other way fails, in every build.
//! With `serde_json`'s `arbitrary_precision` on, an object of the one member `$serde_json::private::Number` parses as
//! the number that it spells, so a file that held one where a number belongs passed in that build alone, before the
//! third review of plan Phase 3, task 4, found it and the maintainer chose the rule. White space and the order of
//! the members therefore count, and a file that passes is the text that writing it again would give.
//!
//! A file is written whole: into a file of its own below Cargo's folder for integration tests, outside the fixtures'
//! folder that `layout` lists, which a rename then moves into its place, as the maintainer chose at that review. A
//! concurrent reader so meets the old text or the new one, never a truncated file, which `fs::write` had shown to 3
//! of the review's 60 concurrent runs. A file that holds the text already is left as it is.

use std::fs;
use std::path::Path;
use std::sync::atomic::{AtomicU32, Ordering};

use cc_media_probe_testkit::fixture::expectation_path;
use serde_json::{Map, Value};

use super::duplicates::distinct_members;

/// The files that this process has written before moving them into place.
static WRITTEN: AtomicU32 = AtomicU32::new(0);

/// The variable that makes the comparison write the expectation instead.
pub const WRITE: &str = "CC_MEDIA_PROBE_WRITE_EXPECTATIONS";

/// Whether expectations are written in this run.
pub fn writing() -> bool {
  std::env::var_os(WRITE).is_some()
}

/// `value` with the members of each object in sorted order.
pub fn sorted(value: &Value) -> Value {
  match value {
    Value::Object(members) => {
      let mut names: Vec<&String> = members.keys().collect();
      names.sort();
      let mut ordered = Map::new();
      for name in names {
        ordered.insert(name.clone(), sorted(&members[name]));
      }
      Value::Object(ordered)
    }
    Value::Array(entries) => Value::Array(entries.iter().map(sorted).collect()),
    other => other.clone(),
  }
}

/// The text of an expectation: the value with sorted members, in pretty JSON with a final line feed.
pub fn written(value: &Value) -> String {
  format!("{}\n", serde_json::to_string_pretty(&sorted(value)).unwrap())
}

/// The value stored at `path`, or the failure that names the variable.
pub fn stored(path: &Path) -> Value {
  let text =
    fs::read_to_string(path).unwrap_or_else(|error| panic!("{}: {error}; set {WRITE} to write it", path.display()));
  parsed(&text).unwrap_or_else(|error| panic!("{}: {error}; set {WRITE} to write it", path.display()))
}

/// The value of an expectation's `text`, which names each member of an object once and is the text that its value
/// is written as, or why it is not: the first line that differs, as the file has it and as the writer writes it.
pub fn parsed(text: &str) -> Result<Value, String> {
  distinct_members(text).map_err(|error| error.to_string())?;
  let value = serde_json::from_str(text).map_err(|error| error.to_string())?;
  match first_difference(text, &written(&value)) {
    None => Ok(value),
    Some((line, stored, rewritten)) => Err(format!(
      "line {line} is {stored}, where the writer writes {rewritten} for the value that the file holds"
    )),
  }
}

/// The first line, counted from 1, where `stored` and `rewritten` differ, with each one's line quoted, or the end of
/// the text where it has none.
fn first_difference(stored: &str, rewritten: &str) -> Option<(usize, String, String)> {
  let (stored, rewritten): (Vec<&str>, Vec<&str>) = (stored.split('\n').collect(), rewritten.split('\n').collect());
  let shown = |line: Option<&&str>| line.map_or_else(|| "the end of the text".to_owned(), |line| format!("{line:?}"));
  (0..stored.len().max(rewritten.len()))
    .zip(1..)
    .find(|&(index, _)| stored.get(index) != rewritten.get(index))
    .map(|(index, line)| (line, shown(stored.get(index)), shown(rewritten.get(index))))
}

/// Writes `text` at `path` whole, unless the file there holds it already: into a file of its own first, which a
/// rename then moves over the old one, so that a concurrent reader meets either text and never a truncated file. A
/// rename stays on one file system, so a target folder on another file system than the repository's stops the run
/// with a message that says so.
pub fn replace(path: &Path, text: &str) {
  if fs::read(path).is_ok_and(|stored| stored == text.as_bytes()) {
    return;
  }
  let folder = Path::new(env!("CARGO_TARGET_TMPDIR")).join("written");
  fs::create_dir_all(&folder).unwrap_or_else(|error| panic!("{}: {error}", folder.display()));
  let number = WRITTEN.fetch_add(1, Ordering::Relaxed);
  let temporary = folder.join(format!("{:08x}-{number:08x}.json", std::process::id()));
  fs::write(&temporary, text).unwrap_or_else(|error| panic!("{}: {error}", temporary.display()));
  if let Err(error) = fs::rename(&temporary, path) {
    let _ = fs::remove_file(&temporary);
    panic!(
      "{}: {error}: an expectation is written below {} first and then moved into place, which needs both on one file \
       system; set CARGO_TARGET_DIR to a folder on the repository's file system",
      path.display(),
      folder.display()
    );
  }
}

/// Checks `value` against the expectation `part` of fixture `name` in `case`, or writes it.
pub fn check(name: &str, case: Option<&str>, part: &str, value: &Value) {
  let path = expectation_path(name, case, part);
  if writing() {
    replace(&path, &written(value));
    return;
  }
  assert_eq!(
    value,
    &stored(&path),
    "{name}: the {part} part differs from {}; set {WRITE} to write it",
    path.display()
  );
}

/// Checks that fixture `name` has no expectation `part` in `case`, or removes it when expectations are written.
pub fn check_absent(name: &str, case: Option<&str>, part: &str) {
  let path = expectation_path(name, case, part);
  if !path.exists() {
    return;
  }
  if writing() {
    fs::remove_file(&path).unwrap();
    return;
  }
  panic!(
    "{name}: {} is not needed any more; set {WRITE} to remove it",
    path.display()
  );
}
