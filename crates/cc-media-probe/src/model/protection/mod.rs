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

//! What a stream declares protected, and where (`api.md` section 6.3; F22).

mod constants;
mod declaration;
mod sample_description;
mod scheme;
mod stream_protection;

pub use declaration::{ProtectionDeclaration, ProtectionScope};
pub use sample_description::{SampleDescriptionProtection, SampleDescriptionState};
pub use scheme::{ProtectionScheme, ProtectionSchemeNamespace};
pub use stream_protection::Protection;
