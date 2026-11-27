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

//! `Delivery`, what one read of the internal source gave.

use cc_media_probe::Limit;

/// What one read of the internal source gave.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Delivery {
  /// This many bytes from the requested position on. 0 answers a nonempty request only at or past the end.
  Bytes(usize),
  /// The range lies behind the forward-only window, or outside the pinned prefix while detection reads, so it can
  /// no longer be read (D9). The walker treats the element as unreadable and goes on.
  Unavailable,
  /// A limit kept the read from being issued: the read allowance, or the working memory of a buffer.
  Exhausted(Limit),
}
