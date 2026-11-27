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

//! Working memory between the cache and what the walk requires (plan section 7.5), after the second review of the
//! spike. A cache block beyond the first is optional: one that does not fit marks nothing, and a required buffer
//! that does not fit takes the optional blocks back before it names the limit. A required buffer that the
//! allocator refuses names the limit too, so a larger cache never gives less than a smaller one.

use std::io::Cursor;

use cc_media_probe::{Limit, Limits};
use input::builder::ToyFile;
use input::report::{DetectionOutcome, ToyReport};
use input::{blocking, handwritten, machine};

use crate::support::{size, small_options};

/// The scratch buffers of `small_options`: a piece and the longest text.
const SCRATCH: u64 = 16 + 255;
/// The block size of `small_options`.
const BLOCK: u64 = 64;
/// The record of one title element, its source and position, which references are checked against.
const TITLE_RECORD: u64 = 16;

/// The report of `file` read through a cache of `blocks` blocks under a working memory of `memory` bytes, the same
/// from the three walkers.
fn cached(file: &[u8], blocks: usize, memory: u64) -> ToyReport {
  let options = input::Options {
    limits: Limits::default().with_working_memory(memory),
    cached_blocks: size(blocks),
    ..small_options()
  };
  let report = blocking::probe_reader(Cursor::new(file), &options).unwrap();
  assert_eq!(handwritten::probe_reader(Cursor::new(file), &options).unwrap(), report);
  assert_eq!(machine::probe_reader(Cursor::new(file), &options).unwrap(), report);
  assert_eq!(report.usage.working_memory_at_end, 0);
  report
}

/// The texts of a report's titles.
fn titles(report: &ToyReport) -> Vec<&str> {
  report.titles.iter().map(|title| title.text.as_str()).collect()
}

#[test]
fn a_block_taken_during_detection_gives_way_to_the_scratch_buffers() {
  let file = ToyFile::with_first_element_at(80).title("x").build();
  for blocks in [1, 2] {
    let report = cached(&file, blocks, BLOCK + SCRATCH + TITLE_RECORD);
    assert_eq!(titles(&report), ["x"], "{blocks} blocks");
    assert_eq!(report.exhausted, [], "{blocks} blocks");
  }
}

#[test]
fn an_optional_block_that_does_not_fit_marks_nothing() {
  let long = "a".repeat(60);
  let file = ToyFile::new().title(&long).title("b").build();
  for blocks in [1, 2] {
    let growth = TITLE_RECORD + 2 * TITLE_RECORD;
    let report = cached(&file, blocks, BLOCK + SCRATCH + growth + 10);
    assert_eq!(titles(&report), [long.as_str(), "b"], "{blocks} blocks");
    assert_eq!(report.exhausted, [], "{blocks} blocks");
  }
}

#[test]
fn a_scratch_buffer_that_the_allocator_refuses_names_working_memory() {
  let file = ToyFile::new().title("x").build();
  let options = input::Options {
    limits: Limits::default().with_working_memory(u64::MAX),
    piece: size(usize::MAX / 2 + 1),
    ..small_options()
  };
  let report = blocking::probe_bytes(&file, &options).unwrap();
  assert_eq!(handwritten::probe_bytes(&file, &options).unwrap(), report);
  assert_eq!(machine::probe_bytes(&file, &options).unwrap(), report);
  assert_eq!(report.detection, DetectionOutcome::Identified);
  assert_eq!(report.titles, []);
  assert_eq!(report.exhausted, [Limit::WorkingMemory]);
  assert_eq!(report.usage.working_memory_at_end, 0);
}

#[test]
fn the_stack_of_open_lists_is_charged_to_working_memory() {
  let depth = 100;
  let file = (0..depth).fold(ToyFile::new(), |file, _| file.open_list()).title("x");
  let file = (0..depth).fold(file, |file, _| file.close_list()).build();
  let with = |memory: u64| {
    let options = input::Options {
      limits: Limits::default()
        .with_nesting_depth(u64::MAX)
        .with_working_memory(memory),
      ..small_options()
    };
    let report = blocking::probe_bytes(&file, &options).unwrap();
    assert_eq!(handwritten::probe_bytes(&file, &options).unwrap(), report);
    assert_eq!(machine::probe_bytes(&file, &options).unwrap(), report);
    assert_eq!(report.usage.working_memory_at_end, 0);
    report
  };
  let free = with(u64::MAX);
  assert_eq!(titles(&free), ["x"]);
  let stack = 128 * 16;
  assert!(free.usage.peak_working_memory >= SCRATCH + stack, "{:?}", free.usage);
  let tight = with(SCRATCH + stack / 2);
  assert_eq!(tight.titles, []);
  assert_eq!(tight.exhausted, [Limit::WorkingMemory]);
}
