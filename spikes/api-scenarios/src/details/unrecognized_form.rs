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

//! The catch-all of a typed detail, written and read by the enum that owns it, for `#[serde(with)]` on its
//! variant: a map of its `kind`. The stub keeps Serde's default tagging of the detail enums (`api.md` section 12).

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::unrecognized_details::UnrecognizedDetails;

/// The members of a catch-all detail as written.
#[derive(Serialize, Deserialize)]
struct DetailsData<'kind> {
  kind: std::borrow::Cow<'kind, str>,
}

/// Writes the map of the kind.
pub(super) fn serialize<Owner, Output: Serializer>(
  details: &UnrecognizedDetails<Owner>,
  serializer: Output,
) -> Result<Output::Ok, Output::Error> {
  DetailsData {
    kind: details.kind().into(),
  }
  .serialize(serializer)
}

/// Reads the map of a kind that the enum `Owner` does not know.
pub(super) fn deserialize<'input, Owner, Input: Deserializer<'input>>(
  deserializer: Input,
) -> Result<UnrecognizedDetails<Owner>, Input::Error> {
  DetailsData::deserialize(deserializer).map(|data| UnrecognizedDetails::new(data.kind.into_owned()))
}
