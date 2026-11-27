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

//! Scenario 4: dump every tag with the entities it describes. `print_tag_sets` is its rendering half, which the
//! tests below call with tag sets read from JSON (plan section 9.7).

use std::io::{self, Write};

use cc_media_probe::model::{
  Tag, TagKey, TagSet, TagValue, TargetLevel, TargetReference, TargetReferenceKind, TargetScope,
};
use cc_media_probe::{MediaReport, probe_file};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let report = probe_file("concert.mka")?;
  Ok(print_tag_sets(&mut io::stdout().lock(), &report)?)
}

/// Writes each tag set's targets and its tree of tags, then the file-level title. The file's own text, a name, a
/// value or a title, is written as a quoted string: its quotes and control characters escaped, it stays on its line.
fn print_tag_sets(output: &mut impl Write, report: &MediaReport) -> io::Result<()> {
  for tag_set in report.tag_sets() {
    writeln!(output, "{}", describe_targets(report, tag_set))?;
    print_tags(output, tag_set.tags())?;
  }
  // The flat view: the file-level title, whatever name the format gives it.
  if let Some(TagValue::Text(title)) = report.tag(&TagKey::Title).and_then(Tag::value) {
    writeln!(output, "title: {title:?}")?;
  }
  Ok(())
}

fn describe_targets(report: &MediaReport, tag_set: &TagSet) -> String {
  let level = describe_level(tag_set.target_level());
  let mut targets = Vec::new();
  for reference in tag_set.stream_targets() {
    targets.push(describe(reference, "stream", |index, _| {
      report.stream(index).map(|stream| stream.codec().to_string())
    }));
  }
  for reference in tag_set.edition_targets() {
    targets.push(describe(reference, "edition", |index, _| {
      let edition = report.chapter_editions().get(index)?;
      edition
        .unique_identifier()
        .map(|identifier| identifier.as_str().to_owned())
    }));
  }
  for reference in tag_set.chapter_targets() {
    targets.push(describe(reference, "chapter", |edition, path| {
      let chapter = report.chapter_editions().get(edition)?.chapter_at(path)?;
      chapter.titles().first().map(|title| title.text().to_owned())
    }));
  }
  for reference in tag_set.attachment_targets() {
    targets.push(describe(reference, "attachment", |index, _| {
      report.attachments().get(index)?.name().map(str::to_owned)
    }));
  }
  let listed = if targets.is_empty() {
    "no target listed".to_owned()
  } else {
    targets.join(", ")
  };
  // Empty lists mean the whole file only once the probe read the targets in full.
  match tag_set.target_scope() {
    TargetScope::WholeFile => format!("the whole file ({level})"),
    TargetScope::Entities => format!("{listed} ({level})"),
    // The probe stopped before it read every target, or could not keep every target that it read.
    TargetScope::Incomplete => format!("{listed}; the targets were not all read or kept ({level})"),
    // A scope of a newer version, whose lists this version cannot interpret, is named as written.
    other => format!(
      "{listed}; the target scope {} is of a newer version ({level})",
      other.as_str()
    ),
  }
}

/// The level: its number and its name are each optional, and the number often comes alone, as Matroska's
/// `TargetTypeValue` does without a `TargetType`.
fn describe_level(level: Option<&TargetLevel>) -> String {
  match (
    level.and_then(TargetLevel::value),
    level.and_then(TargetLevel::type_name),
  ) {
    (Some(value), Some(name)) => format!("level {value} {name:?}"),
    (Some(value), None) => format!("level {value}"),
    (None, Some(name)) => format!("level {name:?}"),
    (None, None) => "no level".to_owned(),
  }
}

/// One reference: resolved to an entity of the report, a wildcard, an identifier that matches nothing the report
/// holds, or a reference of a kind of a newer version. An unmatched entity can still be in the file: chapters that
/// were skipped or cut short are not in the report.
fn describe(reference: &TargetReference, entity: &str, name: impl Fn(usize, &[usize]) -> Option<String>) -> String {
  let written = reference.identifier().map_or("?", |identifier| identifier.as_str());
  match (reference.kind(), reference.location()) {
    (TargetReferenceKind::Resolved, Some(location)) => {
      let name = name(location.index(), location.chapter_path()).map_or_default(|name| format!(" {name:?}"));
      // A chapter is found by its edition and its path of child indexes, since titles can be missing or repeated.
      match location.chapter_path() {
        [] => format!("{entity} {}{name}", location.index()),
        path => {
          let path: Vec<String> = path.iter().map(ToString::to_string).collect();
          format!("{entity} {} of edition {}{name}", path.join("."), location.index())
        }
      }
    }
    (TargetReferenceKind::Wildcard, _) => format!("every {entity}"),
    (TargetReferenceKind::Unmatched, _) => format!("{entity} {written}, unresolved in this report"),
    // A kind of a newer version, whose meaning this version does not know, is named as written.
    (kind, _) => format!("{entity} {written}, a reference of the kind {}", kind.as_str()),
  }
}

/// Writes a tree of tags in depth-first order with a stack of its own, since a tree can be as deep as its input
/// (section 3.3): a call per level could exhaust the call stack.
fn print_tags(output: &mut impl Write, tags: &[Tag]) -> io::Result<()> {
  // Each node waits with its depth; the children go on in reverse, so the first comes off first.
  let mut waiting: Vec<(&Tag, usize)> = tags.iter().rev().map(|tag| (tag, 1)).collect();
  while let Some((tag, depth)) = waiting.pop() {
    let value = match tag.value() {
      Some(TagValue::Text(text)) => format!(" = {text:?}"),
      Some(TagValue::Integer(integer)) => format!(" = {integer}"),
      Some(TagValue::Binary(binary)) => format!(" = {} bytes", binary.size()),
      Some(other) => format!(" = {other:?}"),
      // A node that only groups its children.
      None => String::new(),
    };
    let key = tag.key().map_or_default(|key| format!(" ({})", key.as_str()));
    writeln!(
      output,
      "{:indent$}{:?}{key}{value}",
      "",
      tag.name().as_written(),
      indent = depth.saturating_mul(2)
    )?;
    let child_depth = depth.saturating_add(1);
    waiting.extend(tag.children().iter().rev().map(|child| (child, child_depth)));
  }
  Ok(())
}

#[cfg(test)]
#[cfg(feature = "serde")]
mod tests {
  use cc_media_probe::MediaReport;
  use serde_json::{Value, json};

  use super::print_tag_sets;

  /// What `print_tag_sets` writes for the report.
  fn rendered(report: &Value) -> String {
    let report: MediaReport = serde_json::from_value(report.clone()).unwrap();
    let mut output = Vec::new();
    print_tag_sets(&mut output, &report).unwrap();
    String::from_utf8(output).unwrap()
  }

  /// A report of one Opus stream, an edition of two chapters, the second with a child, an attachment, and the tag
  /// set given, whose tags are complete. Each entity has the unique identifier that its resolved targets hold.
  fn report(tag_set: &Value) -> Value {
    json!({
      "schema_version": "1.0",
      "format": "matroska",
      "streams": [{
        "index": 0,
        "unique_identifier": "11",
        "kind": "audio",
        "codec": { "codec": "opus", "candidates": [], "has_every_candidate": true, "raw_identifiers": [] },
        "header_status": { "state": "parsed" },
        "languages": [],
        "titles": [],
        "disposition": {},
        "audio": {},
      }],
      "programs": [],
      "chapter_editions": [{
        "unique_identifier": "9001",
        "origin": "matroska_edition",
        "chapters": [
          { "string_identifier": "1", "titles": [{ "text": "Opening", "languages": [], "countries": [] }] },
          { "unique_identifier": "2", "string_identifier": "2", "titles": [] },
          {
            "parent": 1,
            "unique_identifier": "3",
            "string_identifier": "3",
            "titles": [{ "text": "Encore", "languages": [], "countries": [] }],
          },
        ],
      }],
      "tag_sets": [tag_set],
      "attachments": [{ "unique_identifier": "21", "name": "cover.jpg" }],
      "external_references": [],
      "diagnostics": [],
      "completeness": completeness("complete", &json!([])),
      "inventory_class": "declared",
      "usage": {
        "bytes_read": 4096,
        "peak_working_memory": 0,
        "retained_data": 0,
        "retained_payloads": 0,
        "decompressed_output": 0,
        "work_units": 0,
        "companion_requests": 0,
        "sources_opened": 0,
      },
      "sources": [{ "identifier": "0", "role": "primary", "outcome": "opened" }],
    })
  }

  /// A completeness whose tags have that status, with those reasons, the first of which the record keeps.
  fn completeness(tags: &str, reasons: &Value) -> Value {
    let first_reasons = reasons.get(0).map_or_else(|| json!([]), |first| json!([first]));
    json!({
      "detection": "complete",
      "discovery": "complete",
      "headers": "complete",
      "tags": tags,
      "chapters": "complete",
      "attachments": "complete",
      "timing": "established",
      "companions": "not_applicable",
      "reasons": reasons,
      "record": {
        "exhausted": [],
        "first_stopped": [],
        "first_reasons": first_reasons,
        "omitted_diagnostics": "0",
        "omitted_declarations": "0",
        "omitted_references": "0",
        "omitted_reasons": "0",
      },
    })
  }

  /// A tag set of that scope and those tags, whose lists hold no target.
  fn tag_set(scope: &str, tags: &Value) -> Value {
    json!({
      "stream_targets": [],
      "edition_targets": [],
      "chapter_targets": [],
      "attachment_targets": [],
      "target_scope": scope,
      "superseded": false,
      "tags": tags,
    })
  }

  /// A tag of that name, value and parent, without a key.
  fn tag(name: &str, value: Option<&str>, parent: Option<usize>) -> Value {
    let mut tag = json!({ "name": { "as_written": name }, "languages": [], "countries": [] });
    if let Some(value) = value {
      tag["value"] = json!({ "kind": "text", "text": value });
    }
    if let Some(parent) = parent {
      tag["parent"] = json!(parent);
    }
    tag
  }

  /// A reference of that kind and Matroska identifier, resolved to that location when one is given.
  fn reference(kind: &str, identifier: &str, location: Option<(&str, usize, &[usize])>) -> Value {
    let mut reference = json!({ "kind": kind, "identifier": identifier, "namespace": "matroska_unique_identifier" });
    if let Some((entity, index, path)) = location {
      reference["location"] = json!({ "kind": entity, "index": index, "chapter_path": path });
    }
    reference
  }

  #[test]
  fn a_numeric_level_without_its_name_and_a_tree_render_on_their_lines() {
    let mut title = tag("TITLE", Some("Live at the \"Hall\"\nSecond night"), None);
    title["key"] = json!("title");
    let tags = json!([
      title,
      tag("ORIGINAL", None, None),
      tag("ARTIST", Some("The Band"), Some(1))
    ]);
    let mut set = tag_set("whole_file", &tags);
    set["target_level"] = json!({ "value": 50 });
    let expected = "\
the whole file (level 50)
  \"TITLE\" (title) = \"Live at the \\\"Hall\\\"\\nSecond night\"
  \"ORIGINAL\"
    \"ARTIST\" = \"The Band\"
title: \"Live at the \\\"Hall\\\"\\nSecond night\"
";
    assert_eq!(rendered(&report(&set)), expected);
  }

  #[test]
  fn each_entity_renders_with_its_name_and_a_chapter_with_its_path() {
    let mut set = tag_set("entities", &json!([tag("COMMENT", Some("Encore"), None)]));
    set["target_level"] = json!({ "value": 30, "type_name": "TRACK" });
    set["stream_targets"] = json!([reference("resolved", "11", Some(("stream", 0, &[])))]);
    set["edition_targets"] = json!([reference("wildcard", "0", None)]);
    set["chapter_targets"] = json!([
      reference("resolved", "3", Some(("chapter", 0, &[1, 0]))),
      reference("resolved", "2", Some(("chapter", 0, &[1]))),
    ]);
    set["attachment_targets"] = json!([
      reference("resolved", "21", Some(("attachment", 0, &[]))),
      reference("unmatched", "77", None),
    ]);
    let expected = "\
stream 0 \"Opus\", every edition, chapter 1.0 of edition 0 \"Encore\", chapter 1 of edition 0, attachment 0 \
\"cover.jpg\", attachment 77, unresolved in this report (level 30 \"TRACK\")
  \"COMMENT\" = \"Encore\"
";
    assert_eq!(rendered(&report(&set)), expected);
  }

  #[test]
  fn an_incomplete_scope_renders_the_targets_kept_and_a_level_of_its_name_alone() {
    let mut set = tag_set("incomplete", &json!([]));
    set["target_level"] = json!({ "type_name": "ALBUM" });
    set["stream_targets"] = json!([reference("resolved", "11", Some(("stream", 0, &[])))]);
    let truncated = json!({ "part": "tags", "reason": { "kind": "truncated_input" } });
    let mut cut = report(&set);
    cut["completeness"] = completeness("incomplete", &json!([truncated]));
    assert_eq!(
      rendered(&cut),
      "stream 0 \"Opus\"; the targets were not all read or kept (level \"ALBUM\")\n"
    );
  }

  #[test]
  fn a_scope_and_a_reference_kind_of_a_newer_version_render_with_their_names() {
    let mut set = tag_set("partial", &json!([]));
    set["stream_targets"] = json!([reference("borrowed", "5", None)]);
    assert_eq!(
      rendered(&report(&set)),
      "stream 5, a reference of the kind borrowed; the target scope partial is of a newer version (no level)\n"
    );
  }
}
