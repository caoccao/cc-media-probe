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

//! `cc-media-probe` inspects multimedia files and reports what they contain:
//! container format, streams, codec parameters, chapters, tags and
//! attachments. It never decodes audio or video.
//!
//! The crate has no public API yet. The API is designed and approved in
//! Phase 2 of the development plan.

#[cfg(test)]
mod tests;
