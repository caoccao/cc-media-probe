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

//! An optional member as `api.md` section 12 reads it. The form writes no null, so a member that holds null fails as
//! malformed input, as the maintainer chose at the seventh review: only a member that is absent reads as `None`. A
//! field uses it with `#[serde(default, deserialize_with = "crate::presence::present")]`, where the default gives the
//! absent member.

use serde::{Deserialize, Deserializer};

/// The member's value, read as a present value of its type, so that null is a value of the wrong type.
pub(crate) fn present<'input, Input, Value>(deserializer: Input) -> Result<Option<Value>, Input::Error>
where
  Input: Deserializer<'input>,
  Value: Deserialize<'input>,
{
  Value::deserialize(deserializer).map(Some)
}
