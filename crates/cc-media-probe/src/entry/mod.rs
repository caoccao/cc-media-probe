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

//! The ten blocking entry points at the crate root (`api.md` section 4.1): each is the method of the same name on
//! `Prober` with the default options.

mod detection;
mod probing;

pub use detection::{detect_bytes, detect_file, detect_reader, detect_source, detect_stream};
pub use probing::{probe_bytes, probe_file, probe_reader, probe_source, probe_stream};
