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

//! The serialized kinds of the shared detail types (`json-output.md` sections 6.2 and 8).

/// The serialized kind of `ObservedValue::Count`, also the name of the member that holds the count.
pub const COUNT_KIND: &str = "count";
/// The serialized kind of `ObservedValue::Time`, also the name of the member that holds the time.
pub const TIME_KIND: &str = "time";
