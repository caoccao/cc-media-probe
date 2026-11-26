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

//! The owner of an entry of the typed details and its stream (`api.md` section 11.3; `json-output.md` section 8):
//! `stream` is the owner's index exactly when the owner is a stream, so a conflict, an observed bound and an exact
//! value never name two entities, as the review of the third slice's first part found.

use crate::model::{EntityKind, EntityLocation};

/// The stream that an entry names: its owner's index when the owner is a stream, and none otherwise.
fn owner_stream(owner: Option<&EntityLocation>) -> Option<usize> {
  owner
    .filter(|owner| matches!(owner.kind(), EntityKind::Stream))
    .map(EntityLocation::index)
}

/// Whether the entry's `stream` is its owner's index exactly when the owner is a stream.
pub(super) fn check_owner(owner: Option<&EntityLocation>, stream: Option<usize>) -> Result<(), &'static str> {
  if stream == owner_stream(owner) {
    Ok(())
  } else {
    Err(super::constants::OWNER_STREAM_MESSAGE)
  }
}
