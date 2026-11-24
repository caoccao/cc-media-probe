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

//! A tag's normalized key, `api.md` section 6.9 (F4), which moved out of `tags.rs` when the eighteenth review's
//! strict reading of every enum read alone lengthened that file.

use super::constants::{
  ALBUM_NAME, ARTIST_NAME, COMMENT_NAME, COPYRIGHT_NAME, DATE_NAME, DISC_NUMBER_NAME, ENCODER_NAME, GENRE_NAME,
  TITLE_NAME, TRACK_NUMBER_NAME,
};
use super::unrecognized_value::UnrecognizedValue;

crate::form::strict_form! {
  #[form(remote = "TagKey")]
  /// A normalized tag key (F4).
  #[derive(Clone, Debug, PartialEq, Eq, Hash)]
  #[non_exhaustive]
  pub enum TagKey {
    /// The title.
    Title,
    /// The artist.
    Artist,
    /// The album.
    Album,
    /// The date of the content's creation.
    Date,
    /// The genre.
    Genre,
    /// A comment.
    Comment,
    /// The copyright.
    Copyright,
    /// The encoder.
    Encoder,
    /// The track number.
    TrackNumber,
    /// The disc number.
    DiscNumber,
    /// A value that a newer version wrote.
    #[cfg_attr(feature = "serde", serde(with = "crate::model::unrecognized_text"))]
    Unrecognized(UnrecognizedValue<TagKey>),
  }
}

impl TagKey {
  /// The serialized name, or the text of an unrecognized value.
  pub fn as_str(&self) -> &str {
    match self {
      TagKey::Title => TITLE_NAME,
      TagKey::Artist => ARTIST_NAME,
      TagKey::Album => ALBUM_NAME,
      TagKey::Date => DATE_NAME,
      TagKey::Genre => GENRE_NAME,
      TagKey::Comment => COMMENT_NAME,
      TagKey::Copyright => COPYRIGHT_NAME,
      TagKey::Encoder => ENCODER_NAME,
      TagKey::TrackNumber => TRACK_NUMBER_NAME,
      TagKey::DiscNumber => DISC_NUMBER_NAME,
      TagKey::Unrecognized(value) => value.as_str(),
    }
  }
}
