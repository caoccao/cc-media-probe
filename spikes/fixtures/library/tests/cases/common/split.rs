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

//! The split of a record's JSON form into its semantic part and its adapter part (plan section 9.2,
//! `docs/design/test-kit.md` section 4.2), at the places that the form defines and nowhere else, so that a member
//! of a typed detail that happens to be named `usage` stays semantic:
//! - a root record, a report, an error record or a detection, and an error's partial report: `size`, the usage's
//!   bytes read and peak working memory, and each source entry's bytes read and path;
//! - an error's observations: each source entry's bytes read and path.
//!
//! The adapter part is keyed by JSON pointer (RFC 6901), so each value keeps its place. The native details of an
//! error record, its operating system's code and the messages of its causes, differ by platform and stay in the
//! semantic part as placeholders: whether each is there, and how many messages, is compared, and their text is
//! not.

use serde_json::{Map, Value};

use super::pointer::{entry, member};

/// The placeholder of a native detail's value.
pub const NATIVE: &str = "{native}";
/// The members of a source entry that belong to the adapter part.
const SOURCE_MEMBERS: [&str; 5] = ["bytes_read", "bytes_read_exact", "path", "path_lossy", "path_raw"];
/// The members of a usage that belong to the adapter part.
const USAGE_MEMBERS: [&str; 4] = [
  "bytes_read",
  "bytes_read_exact",
  "peak_working_memory",
  "peak_working_memory_exact",
];
/// The members of a record that belong to the adapter part.
const RECORD_MEMBERS: [&str; 2] = ["size", "size_exact"];
/// The member that only an error record has.
const KIND: &str = "kind";
/// An error record's operating system's code.
const CODE: &str = "operating_system_error_code";
/// An error record's messages of its causes.
const CAUSES: &str = "cause_messages";

/// The semantic part of `record` and its adapter part.
pub fn split(record: &Value) -> (Value, Map<String, Value>) {
  let mut semantic = record.clone();
  let mut adapter = Map::new();
  if let Value::Object(root) = &mut semantic {
    let error = root.contains_key(KIND);
    take_record(root, "", &mut adapter);
    if error {
      if let Some(Value::Object(partial)) = root.get_mut("partial_report") {
        take_record(partial, "/partial_report", &mut adapter);
      }
      if let Some(Value::Object(observations)) = root.get_mut("observations") {
        take_sources(observations, "/observations", &mut adapter);
      }
      mark_native(root);
    }
  }
  (semantic, adapter)
}

/// Moves the adapter's members of a record at `pointer`, of its usage and of its source entries.
fn take_record(record: &mut Map<String, Value>, pointer: &str, adapter: &mut Map<String, Value>) {
  move_members(record, &RECORD_MEMBERS, pointer, adapter);
  if let Some(Value::Object(usage)) = record.get_mut("usage") {
    move_members(usage, &USAGE_MEMBERS, &member(pointer, "usage"), adapter);
  }
  take_sources(record, pointer, adapter);
}

/// Moves the adapter's members of each entry of the source table that `holder`, at `pointer`, has.
fn take_sources(holder: &mut Map<String, Value>, pointer: &str, adapter: &mut Map<String, Value>) {
  if let Some(Value::Array(sources)) = holder.get_mut("sources") {
    let table = member(pointer, "sources");
    for (index, source) in sources.iter_mut().enumerate() {
      if let Value::Object(source) = source {
        move_members(source, &SOURCE_MEMBERS, &entry(&table, index), adapter);
      }
    }
  }
}

/// Moves `names` of `members`, which stand at `pointer`, into `adapter`.
fn move_members(members: &mut Map<String, Value>, names: &[&str], pointer: &str, adapter: &mut Map<String, Value>) {
  for name in names {
    if let Some(value) = members.remove(*name) {
      adapter.insert(member(pointer, name), value);
    }
  }
}

/// Puts the placeholder in place of each native detail of an error record.
fn mark_native(root: &mut Map<String, Value>) {
  if let Some(code) = root.get_mut(CODE) {
    *code = Value::from(NATIVE);
  }
  if let Some(Value::Array(messages)) = root.get_mut(CAUSES) {
    for message in messages {
      *message = Value::from(NATIVE);
    }
  }
}
