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

//! The places of a record that hold source entries with paths, and what the test does with them before the split
//! (`docs/design/test-kit.md` section 4.2):
//! - a path below the folder that the test wrote its files into, by its components, must be one that the test handed
//!   the library, written exactly as its form gives it: its display string, and the flag and the raw form only for a
//!   lossy path. It is then written with the placeholder `{folder}` and `/`, so that no expectation holds the
//!   machine's folder, and a serialization that marked a Unicode path as lossy fails rather than collapsing away;
//! - a record's retained data is taken net of the handed paths that it lists, so that `retained_data` stays
//!   semantic. An error's usage covers its partial report's (`api.md` section 6.14), so a path that the partial
//!   report lists is taken out of both. The figure is read from its number or its twin and written net in its
//!   canonical form;
//! - each source table of a record lists the primary input with exactly the path that its adapter was given, the
//!   probed file's for a file adapter and none for the others, before the placeholder replaces it.
//!
//! A path is charged, as plan section 7.5 charges retained data by capacity, at the strings that the report keeps
//! for it, each held at exactly its length: its display string, and a lossy path's raw form's data.

use std::path::Path;

use cc_media_probe::model::Usage;
use cc_media_probe::source::{PortablePath, RawPath};
use serde_json::{Map, Value};

/// The placeholder of the folder that a test wrote a fixture's files into.
pub const FOLDER: &str = "{folder}";
/// A path's display string.
const PATH: &str = "path";
/// The flag of a lossy path.
const PATH_LOSSY: &str = "path_lossy";
/// The raw form of a lossy path.
const PATH_RAW: &str = "path_raw";
/// The members of a path, in the order that the form writes them.
const PATH_MEMBERS: [&str; 3] = [PATH, PATH_LOSSY, PATH_RAW];
/// The separator that an expectation's path uses on every platform.
const SEPARATOR: &str = "/";
/// The usage's retained data as a number.
const RETAINED_DATA: &str = "retained_data";
/// Its twin, a decimal string beyond 2^53 − 1.
const RETAINED_DATA_EXACT: &str = "retained_data_exact";
/// The largest quantity that the form writes as a number, 2^53 − 1 (`json-output.md` section 5.5).
const LARGEST_EXACT: u64 = 9_007_199_254_740_991;
/// The member that only an error record has.
const KIND: &str = "kind";
/// A source entry's identifier.
const IDENTIFIER: &str = "identifier";
/// The identifier of the primary input (`api.md` section 6.15).
const PRIMARY: &str = "0";
/// The member that holds a source table.
const SOURCES: &str = "sources";
/// The place of a report, which holds its source table: the record itself.
const REPORT_HOLDER: &str = "";
/// The places of an error's records that hold a source table: its observations and its partial report, each a
/// record of `format_details`, `diagnostics` and `sources` as a report has them (`json-output.md` section 4.4).
const ERROR_HOLDERS: [&str; 2] = ["/observations", "/partial_report"];
/// The message of a record without a source table where it needs one.
const NO_TABLE: &str = "the record has no source table at";
/// The message of a source table without the primary input.
const NO_PRIMARY: &str = "the source table lists no primary input at";
/// The message of a primary input whose path is not the one that its adapter was given.
const OTHER_PATH: &str = "the primary input's path is not the one that its adapter was given at";

/// A path that the test handed the library: its form with the placeholder, the members that a source entry writes
/// for it, and its charge to retained data.
pub struct Handed {
  /// The path with the placeholder, as an expectation writes it.
  pub placeholder: String,
  /// The members `path`, and for a lossy path `path_lossy` and `path_raw`, as the record must write them.
  pub members: Map<String, Value>,
  /// The charge to retained data.
  pub length: u64,
}

impl Handed {
  /// The path `file_name` below `folder`.
  pub fn new(folder: &Path, file_name: &str) -> Handed {
    let portable = PortablePath::from_path(&folder.join(file_name));
    let mut members = Map::new();
    members.insert(PATH.to_owned(), Value::from(portable.display()));
    if portable.is_lossy() {
      members.insert(PATH_LOSSY.to_owned(), Value::Bool(true));
      if let Some(raw) = portable.raw() {
        members.insert(PATH_RAW.to_owned(), serde_json::to_value(raw).unwrap());
      }
    }
    Handed {
      placeholder: format!("{FOLDER}{SEPARATOR}{file_name}"),
      members,
      length: charge(&portable),
    }
  }
}

/// The charge of a path to retained data: the lengths of its display string and, for a lossy path, of its raw
/// form's data, the strings that the report keeps for it.
fn charge(portable: &PortablePath) -> u64 {
  let raw = portable.raw().map_or(0, |raw: &RawPath| raw.data().len());
  u64::try_from(portable.display().len().checked_add(raw).unwrap()).unwrap()
}

/// Checks that each source table of `record` lists the primary input with exactly the path that its adapter was
/// given, written as its form gives it: `given`, the probed file's, for a file adapter, and none for the others, since
/// a name hint is never a path (`api.md` section 4.3). A report has its table, and an error one in its observations
/// and one in its partial report wherever they stand: an error may lack both, but neither without its table, which
/// passed before the review of plan Phase 3, task 4, found it. A file adapter's retained-data limit is raised by its
/// path's charge, which is the probe's first, so the path always fits and is never left out (`api.md` section 6.15).
/// A file report without it passed, with the whole raised limit left to the rest of the probe, as the seventh review
/// of the proposal found.
///
/// # Errors
///
/// Returns the table that is missing, that lacks the primary input or that gives it another path, with `adapter`.
pub fn primary_path(record: &Value, given: Option<&Handed>, adapter: &str) -> Result<(), String> {
  let holders: &[&str] = if record.get(KIND).is_some() {
    &ERROR_HOLDERS
  } else {
    &[REPORT_HOLDER]
  };
  let expected = given.map_or_default(|path| path.members.clone());
  for holder in holders {
    let Some(held) = record.pointer(holder) else {
      continue;
    };
    let place = format!("{holder}/{SOURCES}");
    let Some(table) = held.get(SOURCES) else {
      return Err(format!("{NO_TABLE} {place} of {adapter}"));
    };
    let primary = table
      .as_array()
      .and_then(|entries| {
        entries
          .iter()
          .filter_map(Value::as_object)
          .find(|entry| entry.get(IDENTIFIER).and_then(Value::as_str) == Some(PRIMARY))
      })
      .ok_or_else(|| format!("{NO_PRIMARY} {place} of {adapter}"))?;
    let written = path_members(primary);
    if written != expected {
      return Err(format!(
        "{OTHER_PATH} {place} of {adapter}: {} != {}",
        Value::Object(written),
        Value::Object(expected)
      ));
    }
  }
  Ok(())
}

/// The members of `source` that write its path, in the order that the form writes them.
fn path_members(source: &Map<String, Value>) -> Map<String, Value> {
  PATH_MEMBERS
    .iter()
    .filter_map(|name| source.get(*name).map(|value| ((*name).to_owned(), value.clone())))
    .collect()
}

/// Writes each path below `folder` with the placeholder, and takes the retained data of each record net of the
/// paths in `handed` that its source tables list: the record's own, its observations' or its partial report's.
///
/// # Panics
///
/// Panics at a path below `folder` that is no handed path written as its form gives it.
pub fn normalize(record: &mut Value, folder: &Path, handed: &[Handed]) {
  let Value::Object(root) = record else {
    return;
  };
  let mut root_charge = 0_u64;
  let mut partial_charge = 0_u64;
  for (name, value) in root.iter_mut() {
    let (holder, partial) = match name.as_str() {
      "sources" => (Some(&mut *value), false),
      "observations" => (value.get_mut("sources"), false),
      "partial_report" => (value.get_mut("sources"), true),
      _ => continue,
    };
    if let Some(Value::Array(entries)) = holder {
      for source in entries.iter_mut().filter_map(Value::as_object_mut) {
        let charge = normalize_entry(source, folder, handed);
        root_charge = root_charge.saturating_add(charge);
        if partial {
          partial_charge = partial_charge.saturating_add(charge);
        }
      }
    }
  }
  subtract(root.get_mut("usage"), root_charge);
  subtract(
    root
      .get_mut("partial_report")
      .and_then(|partial| partial.get_mut("usage")),
    partial_charge,
  );
}

/// Writes the path of `source` with the placeholder where it lies below `folder`, by its components, and gives the
/// charge of the handed path that it is.
fn normalize_entry(source: &mut Map<String, Value>, folder: &Path, handed: &[Handed]) -> u64 {
  let Some(display) = source.get(PATH).and_then(Value::as_str) else {
    return 0;
  };
  if !Path::new(display).starts_with(folder) {
    return 0;
  }
  let written = path_members(source);
  let path = handed.iter().find(|path| path.members == written).unwrap_or_else(|| {
    panic!(
      "{}: a path below the test's folder is one that the test handed the library, written as its form gives it",
      Value::Object(written.clone())
    )
  });
  source.remove(PATH_LOSSY);
  source.remove(PATH_RAW);
  source.insert(PATH.to_owned(), Value::from(path.placeholder.as_str()));
  path.length
}

/// Takes `charge` from the retained data of `usage`, read from its number or its twin, and writes the net figure
/// in its canonical form: a number up to 2^53 − 1, its twin beyond, as the fifth review of the proposal found the
/// twin left gross. The check of its limits comes first and refuses a usage that is not the form's, so one that
/// reaches this is left as it is.
fn subtract(usage: Option<&mut Value>, charge: u64) {
  let Some(members) = usage.and_then(Value::as_object_mut) else {
    return;
  };
  if charge == 0 {
    return;
  }
  let Ok(read) = serde_json::from_value::<Usage>(Value::Object(members.clone())) else {
    return;
  };
  let net = read.retained_data().checked_sub(charge).unwrap_or_else(|| {
    panic!(
      "retained data of {} is below the charge {charge} of its paths",
      read.retained_data()
    )
  });
  members.remove(RETAINED_DATA);
  members.remove(RETAINED_DATA_EXACT);
  if net <= LARGEST_EXACT {
    members.insert(RETAINED_DATA.to_owned(), Value::from(net));
  } else {
    members.insert(RETAINED_DATA_EXACT.to_owned(), Value::from(net.to_string()));
  }
}
