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

//! Scenario 4: dump every tag with the entities it describes.

use cc_media_probe::model::{Tag, TagKey, TagSet, TagValue, TargetReference, TargetReferenceKind, TargetScope};
use cc_media_probe::{Error, MediaReport, probe_file};

fn main() -> Result<(), Error> {
  let report = probe_file("concert.mka")?;
  for tag_set in report.tag_sets() {
    println!("{}", describe_targets(&report, tag_set));
    print_tags(tag_set.tags());
  }
  // The flat view: the file-level title, whatever name the format gives it.
  if let Some(TagValue::Text(title)) = report.tag(&TagKey::Title).and_then(Tag::value) {
    println!("title: {title}");
  }
  Ok(())
}

fn describe_targets(report: &MediaReport, tag_set: &TagSet) -> String {
  let level = tag_set
    .target_level()
    .and_then(|level| level.type_name())
    .unwrap_or("no level");
  // Empty lists mean the whole file only once the probe read the targets in full.
  let note = match tag_set.target_scope() {
    TargetScope::WholeFile => return format!("the whole file ({level})"),
    TargetScope::Entities => "",
    _ => ", and targets that were not read",
  };
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
  format!("{}{note} ({level})", targets.join(", "))
}

/// One reference: resolved to an entity of the report, a wildcard, or an identifier that matches nothing the report
/// holds. Such an entity can still be in the file: chapters that were skipped or cut short are not in the report.
fn describe(reference: &TargetReference, entity: &str, name: impl Fn(usize, &[usize]) -> Option<String>) -> String {
  match (reference.kind(), reference.location()) {
    (TargetReferenceKind::Resolved, Some(location)) => {
      let name = name(location.index(), location.chapter_path()).unwrap_or_default();
      format!("{entity} {} {name}", location.index())
    }
    (TargetReferenceKind::Wildcard, _) => format!("every {entity}"),
    _ => {
      let written = reference.identifier().map_or("?", |identifier| identifier.as_str());
      format!("{entity} {written}, unresolved in this report")
    }
  }
}

/// Prints a tree of tags in depth-first order with a stack of its own, since a tree can be as deep as its input
/// (section 3.3): a call per level could exhaust the call stack.
fn print_tags(tags: &[Tag]) {
  // Each node waits with its depth; the children go on in reverse, so the first comes off first.
  let mut waiting: Vec<(&Tag, usize)> = tags.iter().rev().map(|tag| (tag, 1)).collect();
  while let Some((tag, depth)) = waiting.pop() {
    let value = match tag.value() {
      Some(TagValue::Text(text)) => text.clone(),
      Some(TagValue::Integer(integer)) => integer.to_string(),
      Some(TagValue::Binary(binary)) => format!("{} bytes", binary.size()),
      Some(other) => format!("{other:?}"),
      // A node that only groups its children.
      None => String::new(),
    };
    let key = tag.key().map(|key| format!(" ({})", key.as_str())).unwrap_or_default();
    println!(
      "{:indent$}{}{key} = {value}",
      "",
      tag.name().as_written(),
      indent = depth * 2
    );
    waiting.extend(tag.children().iter().rev().map(|child| (child, depth + 1)));
  }
}
