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

//! Compile-time checks of the auto traits that `api.md` sections 3.3 and 9.1 promise, and of matching on an enum
//! that an accessor returns by reference.

use std::pin::Pin;

use cc_media_probe::asynchronous::{AsyncPositionalSource, probe_source};
use cc_media_probe::model::{FrameRateMode, Provenance};

fn assert_send<Value: Send>(_value: &Value) {}

fn assert_thread_safe_and_static<Value: Send + Sync + 'static>() {}

#[test]
fn model_error_and_prober_are_thread_safe_and_static() {
  assert_thread_safe_and_static::<cc_media_probe::MediaReport>();
  assert_thread_safe_and_static::<cc_media_probe::Error>();
  assert_thread_safe_and_static::<cc_media_probe::Prober>();
}

#[test]
fn the_probe_of_a_boxed_send_source_is_send() {
  let source: Option<Pin<Box<dyn AsyncPositionalSource + Send>>> = None;
  if let Some(source) = source {
    assert_send(&probe_source(source));
  }
}

#[test]
fn an_enum_returned_by_reference_matches_without_dereferencing() {
  let provenance: Option<&Provenance> = Some(&Provenance::Declared);
  assert!(matches!(provenance, Some(Provenance::Declared)));
  let mode: &FrameRateMode = &FrameRateMode::Unknown;
  assert!(matches!((None::<u64>, mode), (None, FrameRateMode::Unknown)));
}

#[cfg(feature = "tokio")]
#[test]
fn the_tokio_futures_are_send() {
  assert_send(&cc_media_probe::asynchronous::tokio::probe_file("movie.mkv"));
  let prober = cc_media_probe::Prober::new();
  assert_send(&prober.tokio().probe_file("movie.mkv"));
  let file: Option<tokio::fs::File> = None;
  if let Some(file) = file {
    assert_send(&prober.tokio().probe_reader(file));
  }
}
