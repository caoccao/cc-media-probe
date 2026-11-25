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

//! A stream of the report and the types that describe it (`api.md` section 6.3).

mod constants;
mod declaration_range;
mod disposition;
mod kind;
mod stream_entry;
mod stream_language;
mod timecode;

pub use declaration_range::DeclarationRange;
pub use disposition::Disposition;
pub use kind::StreamKind;
pub use stream_entry::Stream;
pub use stream_language::{LanguageRole, StreamLanguage};
pub use timecode::{Timecode, TimecodeLabel};
