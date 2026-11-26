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

//! `AttachmentHandling`, which attachments a probe lists and whether it keeps their data (`api.md` section 4.3).

/// Which attachments of a role a probe lists, and whether it keeps their data. The attachments option applies to
/// every attachment whose role is not cover art, and the cover-art option to every attachment whose role is cover
/// art, whatever its carriage, including a picture inside a tag structure while the tags option is `Skip`. The
/// default of both is `MetadataOnly`.
///
/// ```
/// use cc_media_probe::Prober;
/// use cc_media_probe::model::AttachmentHandling;
///
/// let prober = Prober::new()
///     .with_attachments(AttachmentHandling::Skip)
///     .with_cover_art(AttachmentHandling::WithData);
/// assert_eq!(prober.attachments(), &AttachmentHandling::Skip);
/// assert_eq!(prober.cover_art(), &AttachmentHandling::WithData);
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum AttachmentHandling {
  /// The attachments of the role are left out. The probe reads their metadata only as far as it needs to learn
  /// their role, and never lists them.
  Skip,
  /// The attachments of the role are listed without their data. An embedded picture that leaves out its pixel
  /// size, its media type or both has its image header read for what is missing, within the per-stream header
  /// limit.
  MetadataOnly,
  /// The attachments of the role are listed, and their payloads kept within the retained-payload limit. A payload
  /// that a limit stopped leaves the attachment's data absent and the attachments section incomplete.
  WithData,
}

#[cfg(test)]
mod tests {
  use super::AttachmentHandling;

  #[test]
  fn tells_the_three_handlings_apart() {
    let handlings = [
      AttachmentHandling::Skip,
      AttachmentHandling::MetadataOnly,
      AttachmentHandling::WithData,
    ];
    for (position, handling) in handlings.iter().enumerate() {
      assert_eq!(handlings.iter().position(|other| other == handling), Some(position));
    }
  }
}
