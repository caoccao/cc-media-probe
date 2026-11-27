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

//! The public structs of `asynchronous` and of the modules of its two runtimes, each behind its feature, which a
//! crate that depends on the library cannot build with a struct literal.

no_struct_literal! {
  in [cc_media_probe::asynchronous];
  AsynchronousProber,
}

#[cfg(feature = "futures-io")]
no_struct_literal! {
  in [cc_media_probe::asynchronous::futures];
  FuturesProber,
}

#[cfg(feature = "tokio")]
no_struct_literal! {
  in [cc_media_probe::asynchronous::tokio];
  TokioProber,
}
