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

//! The serialized names of the tag types (`json-output.md` sections 6.1 and 6.2).

/// The serialized name of `TargetScope::WholeFile`.
pub const WHOLE_FILE_NAME: &str = "whole_file";
/// The serialized name of `TargetScope::Entities`.
pub const ENTITIES_NAME: &str = "entities";
/// The serialized name of `TargetScope::Incomplete`.
pub const INCOMPLETE_NAME: &str = "incomplete";

/// The serialized name of `TargetReferenceKind::Resolved`.
pub const RESOLVED_NAME: &str = "resolved";
/// The serialized name of `TargetReferenceKind::Unmatched`.
pub const UNMATCHED_NAME: &str = "unmatched";
/// The serialized name of `TargetReferenceKind::Wildcard`.
pub const WILDCARD_NAME: &str = "wildcard";

/// The serialized name of `IdentifierNamespace::MatroskaUniqueIdentifier`.
pub const MATROSKA_UNIQUE_IDENTIFIER_NAME: &str = "matroska_unique_identifier";

/// The serialized name of `TagKey::Title`.
pub const TITLE_NAME: &str = "title";
/// The serialized name of `TagKey::Artist`.
pub const ARTIST_NAME: &str = "artist";
/// The serialized name of `TagKey::Album`.
pub const ALBUM_NAME: &str = "album";
/// The serialized name of `TagKey::Date`.
pub const DATE_NAME: &str = "date";
/// The serialized name of `TagKey::Genre`.
pub const GENRE_NAME: &str = "genre";
/// The serialized name of `TagKey::Comment`.
pub const COMMENT_NAME: &str = "comment";
/// The serialized name of `TagKey::Copyright`.
pub const COPYRIGHT_NAME: &str = "copyright";
/// The serialized name of `TagKey::Encoder`.
pub const ENCODER_NAME: &str = "encoder";
/// The serialized name of `TagKey::TrackNumber`.
pub const TRACK_NUMBER_NAME: &str = "track_number";
/// The serialized name of `TagKey::DiscNumber`.
pub const DISC_NUMBER_NAME: &str = "disc_number";

/// The serialized kind of `TagValue::Text`, also the name of the member that holds the value.
pub const TEXT_KIND: &str = "text";
/// The serialized kind of `TagValue::Binary`.
pub const BINARY_KIND: &str = "binary";
/// The serialized kind of `TagValue::Integer`.
pub const INTEGER_KIND: &str = "integer";
/// The serialized kind of `TagValue::FloatingPoint`.
pub const FLOATING_POINT_KIND: &str = "floating_point";
/// The serialized kind of `TagValue::Boolean`.
pub const BOOLEAN_KIND: &str = "boolean";
/// The serialized kind of `TagValue::Guid`.
pub const GUID_KIND: &str = "guid";
