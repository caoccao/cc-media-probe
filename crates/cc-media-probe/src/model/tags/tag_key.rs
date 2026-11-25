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

//! `TagKey`, the normalized key of a tag (`api.md` section 6.9; F4).

use super::constants::{
  ALBUM_NAME, ARTIST_NAME, COMMENT_NAME, COPYRIGHT_NAME, DATE_NAME, DISC_NUMBER_NAME, ENCODER_NAME, GENRE_NAME,
  TITLE_NAME, TRACK_NUMBER_NAME,
};
use crate::string_enumeration::string_enumeration;

string_enumeration! {
  /// The normalized key of a tag (F4), given by its name alone, never by its target level. A name with no key
  /// keeps its name and is reached through `Tag::name()`. Each format phase lists its names in `api.md` section
  /// 6.9.
  ///
  /// ```
  /// use cc_media_probe::MediaReport;
  /// use cc_media_probe::model::{TagKey, TagValue};
  ///
  /// fn title(report: &MediaReport) -> Option<&str> {
  ///     match report.tag(&TagKey::Title)?.value()? {
  ///         TagValue::Text(text) => Some(text),
  ///         _ => None,
  ///     }
  /// }
  ///
  /// assert_eq!(TagKey::TrackNumber.as_str(), "track_number");
  /// ```
  TagKey {
    /// The title of the entity that the tag set describes.
    Title => TITLE_NAME,
    /// The artist.
    Artist => ARTIST_NAME,
    /// The album.
    Album => ALBUM_NAME,
    /// The date.
    Date => DATE_NAME,
    /// The genre.
    Genre => GENRE_NAME,
    /// A comment.
    Comment => COMMENT_NAME,
    /// The copyright notice.
    Copyright => COPYRIGHT_NAME,
    /// The software that encoded the content.
    Encoder => ENCODER_NAME,
    /// The track number.
    TrackNumber => TRACK_NUMBER_NAME,
    /// The disc number.
    DiscNumber => DISC_NUMBER_NAME,
  }
}

#[cfg(test)]
mod tests {
  use super::TagKey;
  use crate::model::UnrecognizedValue;

  #[test]
  fn names_each_key() {
    for (key, name) in [
      (TagKey::Title, "title"),
      (TagKey::Artist, "artist"),
      (TagKey::Album, "album"),
      (TagKey::Date, "date"),
      (TagKey::Genre, "genre"),
      (TagKey::Comment, "comment"),
      (TagKey::Copyright, "copyright"),
      (TagKey::Encoder, "encoder"),
      (TagKey::TrackNumber, "track_number"),
      (TagKey::DiscNumber, "disc_number"),
      (TagKey::Unrecognized(UnrecognizedValue::sample("composer")), "composer"),
    ] {
      assert_eq!(key.as_str(), name);
    }
  }
}
