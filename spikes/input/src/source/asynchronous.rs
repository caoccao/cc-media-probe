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

//! A caller's own async positional source (`api.md` section 9.3) as a raw input. A request that is pending is
//! repeated with the same position and buffer, since the future holds the request until it is ready.

use std::future::poll_fn;
use std::io;
use std::pin::Pin;

use cc_media_probe::asynchronous::AsyncPositionalSource;

use super::RawPositional;

/// A caller's own async positional source.
pub(crate) struct AsyncCustom<Source>(pub(crate) Source);

impl<Source: AsyncPositionalSource + Unpin> RawPositional for AsyncCustom<Source> {
  async fn read_at(&mut self, position: u64, buffer: &mut [u8]) -> io::Result<usize> {
    poll_fn(|context| Pin::new(&mut self.0).poll_read_at(context, position, buffer)).await
  }
}
