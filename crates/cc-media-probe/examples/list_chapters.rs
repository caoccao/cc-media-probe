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

//! Scenario 5: list the chapters of every edition, with both timelines.

use cc_media_probe::completeness::SectionStatus;
use cc_media_probe::model::Chapter;
use cc_media_probe::{Error, Seconds, probe_file};

fn main() -> Result<(), Error> {
  let report = probe_file("movie.mkv")?;
  for (index, edition) in report.chapter_editions().iter().enumerate() {
    let ordered = edition.is_ordered() == Some(true);
    print!("edition {index}");
    // Playback time exists only for an ordered edition. The report's duration stays the segment's.
    if ordered {
      match edition.playback_duration() {
        Some(duration) => print!(", ordered, plays for {duration}"),
        None => print!(", ordered, playback length unknown"),
      }
    }
    println!();
    print_chapters(edition.chapters(), ordered);
  }
  if !matches!(report.completeness().chapters(), SectionStatus::Complete) {
    println!("chapters: {}", report.completeness().chapters().as_str());
  }
  Ok(())
}

/// Prints a tree of chapters in depth-first order with a stack of its own, since a tree can be as deep as its input
/// (section 3.3): a call per level could exhaust the call stack.
fn print_chapters(chapters: &[Chapter], ordered: bool) {
  // Each chapter waits with its depth; the children go on in reverse, so the first comes off first.
  let mut waiting: Vec<(&Chapter, usize)> = chapters.iter().rev().map(|chapter| (chapter, 1)).collect();
  while let Some((chapter, depth)) = waiting.pop() {
    let title = chapter.titles().first().map_or("(untitled)", |title| title.text());
    // Segment time, as written; a chapter that marks a point has no end.
    print!(
      "{:indent$}{title}: {} to {}",
      "",
      time(chapter.start()),
      time(chapter.end()),
      indent = depth.saturating_mul(2)
    );
    if ordered && let Some(position) = chapter.playback_position() {
      print!(", at {position} of playback");
    }
    if chapter.is_hidden() == Some(true) {
      print!(" (hidden)");
    }
    println!();
    let child_depth = depth.saturating_add(1);
    waiting.extend(chapter.children().iter().rev().map(|child| (child, child_depth)));
  }
}

fn time(value: Option<Seconds>) -> String {
  value.map_or_else(|| "?".to_owned(), |seconds| seconds.to_string())
}
