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

//! `InternalSource`, the positional source that the walker reads (D8).

use std::io;

use super::{Delivery, Holdings};
use crate::budget::Budget;

/// The positional source that the walker reads, the same for every input (D8). It has two paths: `read_now` serves
/// what the source can give at once, as plain code, and `read_at` makes a read that may wait. Over a blocking input
/// every future completes at once, so the blocking entry points poll the walker once with a waker that does nothing.
pub(crate) trait InternalSource: Holdings {
  /// Reads at most `buffer.len()` bytes at `position`, clamped to the read allowance, and never asks its input for
  /// zero bytes. A short read is allowed; `fill` asks again for the rest.
  async fn read_at(&mut self, budget: &mut Budget, position: u64, buffer: &mut [u8]) -> io::Result<Delivery>;

  /// Reads what the source can give at once, without a future and without asking its input: `None` where the
  /// read needs the input, which `read_at` then asks.
  fn read_now(&mut self, budget: &mut Budget, position: u64, buffer: &mut [u8]) -> Option<Delivery>;

  /// Ends detection. A forward-only input keeps the bytes that detection read for the walker, which reads them
  /// first; a seekable one has nothing to do.
  fn end_detection(&mut self);

  /// The bytes that the input delivered, for the source table.
  fn delivered(&self) -> u64;

  /// Releases the source's buffers and returns their charges.
  fn release(&mut self, budget: &mut Budget);
}
