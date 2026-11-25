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

//! `Attachment`, a file attached to the media (`api.md` section 6.10; field guide section 3.9).

use super::AttachmentRole;
use crate::model::{Identifier, MediaType, Provenance};

/// A file attached to the media, such as a font or cover art, from `MediaReport::attachments()`. Cover art whose
/// layout an approved source gives is always an attachment, whatever its carriage, and the tag lists do not repeat
/// it (F8).
///
/// ```
/// use cc_media_probe::MediaReport;
///
/// fn attachments(report: &MediaReport) {
///     for attachment in report.attachments() {
///         println!(
///             "{} ({:?}, {:?} from {:?}), {:?} bytes, kept {}, link {:?}",
///             attachment.name().unwrap_or("unnamed"),
///             attachment.description(),
///             attachment.media_type().map(|media_type| media_type.as_str()),
///             attachment.media_type_provenance(),
///             attachment.size(),
///             attachment.data().is_some(),
///             attachment.link(),
///         );
///         println!(
///             "  {:?}x{:?} ({:?}, {:?}), role {:?}, picture type {:?}, identifier {:?}",
///             attachment.width(),
///             attachment.height(),
///             attachment.width_provenance(),
///             attachment.height_provenance(),
///             attachment.role().map(|role| role.as_str()),
///             attachment.picture_type(),
///             attachment.unique_identifier().map(|identifier| identifier.as_str()),
///         );
///     }
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Attachment {
  unique_identifier: Option<Identifier>,
  name: Option<String>,
  description: Option<String>,
  media_type: Option<MediaType>,
  media_type_provenance: Option<Provenance>,
  size: Option<u64>,
  link: Option<String>,
  data: Option<Vec<u8>>,
  width: Option<u64>,
  width_provenance: Option<Provenance>,
  height: Option<u64>,
  height_provenance: Option<Provenance>,
  role: Option<AttachmentRole>,
  picture_type: Option<String>,
}

impl Attachment {
  /// The attachment's unique identifier, such as a Matroska `FileUID`.
  #[must_use]
  pub fn unique_identifier(&self) -> Option<&Identifier> {
    self.unique_identifier.as_ref()
  }

  /// The file name.
  #[must_use]
  pub fn name(&self) -> Option<&str> {
    self.name.as_deref()
  }

  /// The description.
  #[must_use]
  pub fn description(&self) -> Option<&str> {
    self.description.as_deref()
  }

  /// The media type.
  #[must_use]
  pub fn media_type(&self) -> Option<&MediaType> {
    self.media_type.as_ref()
  }

  /// Where the media type came from.
  #[must_use]
  pub fn media_type_provenance(&self) -> Option<&Provenance> {
    self.media_type_provenance.as_ref()
  }

  /// The size of embedded data, in bytes.
  #[must_use]
  pub fn size(&self) -> Option<u64> {
    self.size
  }

  /// A linked picture's URI as written, never fetched; also listed as an external reference.
  #[must_use]
  pub fn link(&self) -> Option<&str> {
    self.link.as_deref()
  }

  /// The payload, only with `AttachmentHandling::WithData` and within the retained-payload limit. An empty
  /// payload gives an empty slice; a payload that a limit stopped is absent, and the attachments section is then
  /// incomplete.
  #[must_use]
  pub fn data(&self) -> Option<&[u8]> {
    self.data.as_deref()
  }

  /// The width of a picture, in pixels.
  #[must_use]
  pub fn width(&self) -> Option<u64> {
    self.width
  }

  /// Where the width came from.
  #[must_use]
  pub fn width_provenance(&self) -> Option<&Provenance> {
    self.width_provenance.as_ref()
  }

  /// The height of a picture, in pixels.
  #[must_use]
  pub fn height(&self) -> Option<u64> {
    self.height
  }

  /// Where the height came from.
  #[must_use]
  pub fn height_provenance(&self) -> Option<&Provenance> {
    self.height_provenance.as_ref()
  }

  /// The role (F8).
  #[must_use]
  pub fn role(&self) -> Option<&AttachmentRole> {
    self.role.as_ref()
  }

  /// The native picture type as written, such as FLAC's `3` for a front cover.
  #[must_use]
  pub fn picture_type(&self) -> Option<&str> {
    self.picture_type.as_deref()
  }
}

#[cfg(test)]
mod tests {
  use super::Attachment;
  use crate::model::{AttachmentRole, Identifier, MediaType, Provenance};

  impl Attachment {
    /// A front cover of 600 by 600 pixels, kept with its data.
    pub(crate) fn sample() -> Attachment {
      Attachment {
        unique_identifier: Some(Identifier::sample("99")),
        name: Some("cover.png".to_owned()),
        description: Some("Front".to_owned()),
        media_type: Some(MediaType::sample("image/png")),
        media_type_provenance: Some(Provenance::Declared),
        size: Some(4),
        link: None,
        data: Some(vec![0x89, 0x50, 0x4e, 0x47]),
        width: Some(600),
        width_provenance: Some(Provenance::Bitstream),
        height: Some(600),
        height_provenance: Some(Provenance::Bitstream),
        role: Some(AttachmentRole::CoverArt),
        picture_type: Some("3".to_owned()),
      }
    }
  }

  #[test]
  fn gives_each_field() {
    let attachment = Attachment::sample();
    assert_eq!(attachment.unique_identifier().map(Identifier::as_str), Some("99"));
    assert_eq!(attachment.name(), Some("cover.png"));
    assert_eq!(attachment.description(), Some("Front"));
    assert_eq!(attachment.media_type().map(MediaType::as_str), Some("image/png"));
    assert_eq!(attachment.media_type_provenance(), Some(&Provenance::Declared));
    assert_eq!(attachment.size(), Some(4));
    assert_eq!(attachment.link(), None);
    assert_eq!(attachment.data().map(<[u8]>::len), Some(4));
    assert_eq!(attachment.width(), Some(600));
    assert_eq!(attachment.width_provenance(), Some(&Provenance::Bitstream));
    assert_eq!(attachment.height(), Some(600));
    assert_eq!(attachment.height_provenance(), Some(&Provenance::Bitstream));
    assert_eq!(attachment.role(), Some(&AttachmentRole::CoverArt));
    assert_eq!(attachment.picture_type(), Some("3"));
  }
}
