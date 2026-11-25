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

//! The serialized names of the reference types (`json-output.md` section 6.1).

/// The serialized name of `ReferenceKind::LinkedSegment`.
pub const LINKED_SEGMENT_NAME: &str = "linked_segment";
/// The serialized name of `ReferenceKind::DataReference`.
pub const DATA_REFERENCE_NAME: &str = "data_reference";
/// The serialized name of `ReferenceKind::LinkedPicture`.
pub const LINKED_PICTURE_NAME: &str = "linked_picture";
/// The serialized name of `ReferenceKind::ExpectedCompanion`.
pub const EXPECTED_COMPANION_NAME: &str = "expected_companion";
/// The serialized name of `ReferenceKind::PlaylistEntry`.
pub const PLAYLIST_ENTRY_NAME: &str = "playlist_entry";

/// The serialized name of `ReferenceRole::PreviousSegment`.
pub const PREVIOUS_SEGMENT_NAME: &str = "previous_segment";
/// The serialized name of `ReferenceRole::NextSegment`.
pub const NEXT_SEGMENT_NAME: &str = "next_segment";
/// The serialized name of `ReferenceRole::TrackData`.
pub const TRACK_DATA_NAME: &str = "track_data";
