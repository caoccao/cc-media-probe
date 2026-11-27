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

//! `poll_once`, the executor: one poll with a waker that does nothing.

use std::future::Future;
use std::pin::pin;
use std::task::{Context, Poll, Waker};

/// Polls `future` once with a waker that does nothing, and gives its output, or `None` when it was not ready.
/// Every read of a blocking input returns before its future does, so a probe over blocking inputs is ready on its
/// first poll (D8); `Waker::noop` is stable since Rust 1.85, so the executor needs no `unsafe` code.
pub(crate) fn poll_once<Output>(future: impl Future<Output = Output>) -> Option<Output> {
  let mut future = pin!(future);
  let mut context = Context::from_waker(Waker::noop());
  match future.as_mut().poll(&mut context) {
    Poll::Ready(output) => Some(output),
    Poll::Pending => None,
  }
}
