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

//! Reading a value that a reader kept before it knew the value's type, once the type is known: the members of an
//! enum that carries data, kept until its kind says how to read them, and a tree's node, kept until its `parent` is
//! taken out (`crate::kept`). The value was read as a value of no known type, so the rules that a type brings are
//! checked now, through `Strict`, as for a value that the type reads from the input: the name of `serde_json`'s private
//! map in an object of a known type, and an object where a number is required. The eighteenth review read the map
//! as a tag's integer of 60, a chapter's parent and a binary value's size, since the kept value had already become
//! the number, and a tag's value beside a member of that name in a report, which dropped it.

use serde::{Deserialize, Deserializer};

use crate::form::strict::Strict;

/// Reads a kept value into its type, strictly. A kept value holds no root record (`json-output.md` section 9), so it
/// is read within the body of none.
pub(crate) fn read_kept<'input, Value: Deserialize<'input>, Input: Deserializer<'input>>(
  deserializer: Input,
) -> Result<Value, Input::Error> {
  Value::deserialize(Strict::value(deserializer, None))
}
