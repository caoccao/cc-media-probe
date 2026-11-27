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

//! The companions that a primary input names (plan section 7.7), after the second review of the spike: each
//! identifier is listed once, in the order found, and found again among the earlier ones in a bounded number of
//! steps, so a large catalog costs no quadratic work. With lookup on, the queue of requests holds only the offset
//! of each identifier's first element, charged to working memory like every other buffer of the walk.

use std::sync::Arc;

use cc_media_probe::{Limit, Limits};
use input::blocking;
use input::builder::ToyFile;
use input::companion::{Answer, CompanionLookup, CompanionRequest, CompanionResolver, Resolution};
use input::report::DiagnosticKind;

use crate::support::small_options;

/// The scratch buffers of `small_options`: a piece and the longest text.
const SCRATCH: u64 = 16 + 255;

/// A resolver that finds nothing.
struct Nothing;

impl CompanionResolver for Nothing {
  fn resolve(&self, _request: &CompanionRequest) -> Resolution {
    Answer::NotFound
  }
}

/// A primary input that names `count` distinct companions, each twice.
fn catalog(count: usize) -> Vec<u8> {
  let identifiers: Vec<String> = (0..count).map(|index| format!("c{index}")).collect();
  identifiers
    .iter()
    .chain(identifiers.iter())
    .fold(ToyFile::new(), |file, identifier| file.companion(identifier))
    .build()
}

/// Options with lookup through `Nothing` or off, no companion requests, and `memory` bytes of working memory.
fn options(lookup: bool, memory: u64) -> input::Options {
  input::Options {
    limits: Limits::default().with_companion_requests(0).with_working_memory(memory),
    companions: if lookup {
      CompanionLookup::Custom(Arc::new(Nothing))
    } else {
      CompanionLookup::Off
    },
    ..small_options()
  }
}

#[test]
fn the_queue_of_named_companions_is_charged_to_working_memory() {
  let count = 200;
  let file = catalog(count);
  let off = blocking::probe_bytes(&file, &options(false, u64::MAX)).unwrap();
  let on = blocking::probe_bytes(&file, &options(true, u64::MAX)).unwrap();
  assert_eq!(off.expected_companions.len(), count);
  assert_eq!(on.expected_companions, off.expected_companions);
  let limited = DiagnosticKind::CompanionLimited(Limit::CompanionRequests);
  assert_eq!(on.diagnostics.len(), count);
  assert!(on.diagnostics.iter().all(|diagnostic| diagnostic.kind == limited));
  assert!(off.usage.peak_working_memory > SCRATCH, "{:?}", off.usage);
  assert!(
    on.usage.peak_working_memory > off.usage.peak_working_memory,
    "{:?} {:?}",
    on.usage,
    off.usage
  );
  assert_eq!(on.usage.working_memory_at_end, 0);
  let tight = blocking::probe_bytes(&file, &options(true, off.usage.peak_working_memory)).unwrap();
  assert_eq!(tight.exhausted, [Limit::WorkingMemory]);
  assert_eq!(tight.usage.working_memory_at_end, 0);
}

#[test]
fn many_companions_are_listed_once_each_in_the_order_found() {
  let count = 20_000;
  let report = blocking::probe_bytes(&catalog(count), &options(false, u64::MAX)).unwrap();
  let expected: Vec<String> = (0..count).map(|index| format!("c{index}")).collect();
  assert_eq!(report.expected_companions, expected);
  assert_eq!(report.usage.working_memory_at_end, 0);
}
