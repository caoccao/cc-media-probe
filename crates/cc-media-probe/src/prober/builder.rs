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

//! `Prober`, the builder that carries the options of a probe (`api.md` sections 4.2 and 4.3).

use std::ffi::{OsStr, OsString};

use super::{AttachmentHandling, Limits, SectionRequest};
use crate::Format;

/// Carries the options of a probe. It is built once and used for any number of probes, from any number of
/// threads: every probing method takes `&self`, and `Prober` is `Clone`, `Send` and `Sync`.
///
/// `Prober::new()` and `Prober::default()` give the default options. Every setter consumes the builder and
/// returns it, and every option has a getter with the option's name. The ten blocking methods have the
/// signatures of the functions at the crate root, such as [`probe_file`](crate::probe_file); the async methods
/// are reached through the handles of the [`asynchronous`](crate::asynchronous) module.
///
/// ```
/// use cc_media_probe::model::{AttachmentHandling, SectionRequest};
/// use cc_media_probe::{Format, Limits, Prober};
///
/// let prober = Prober::new()
///     .with_limits(Limits::default().with_bytes_read(16 * 1024 * 1024))
///     .with_format_hint(Format::Matroska)
///     .with_file_name_hint("upload.mkv")
///     .with_chapters(SectionRequest::Skip)
///     .with_attachments(AttachmentHandling::WithData);
/// assert_eq!(prober.limits().bytes_read(), 16 * 1024 * 1024);
/// assert_eq!(prober.format_hint(), Some(&Format::Matroska));
/// assert_eq!(prober.file_name_hint().and_then(|name| name.to_str()), Some("upload.mkv"));
/// assert_eq!(prober.tags(), &SectionRequest::Read);
/// assert_eq!(prober.chapters(), &SectionRequest::Skip);
/// assert_eq!(prober.attachments(), &AttachmentHandling::WithData);
/// assert_eq!(prober.cover_art(), &AttachmentHandling::MetadataOnly);
/// ```
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Prober {
  format_hint: Option<Format>,
  file_name_hint: Option<OsString>,
  limits: Limits,
  tags: SectionRequest,
  chapters: SectionRequest,
  attachments: AttachmentHandling,
  cover_art: AttachmentHandling,
}

impl Prober {
  /// The default options: no hint, the default limits, the tags and the chapters read, and the attachments and
  /// the cover art listed without their data. The same value as `Prober::default()`.
  #[must_use]
  pub fn new() -> Prober {
    Prober {
      format_hint: None,
      file_name_hint: None,
      limits: Limits::new(),
      tags: SectionRequest::Read,
      chapters: SectionRequest::Read,
      attachments: AttachmentHandling::MetadataOnly,
      cover_art: AttachmentHandling::MetadataOnly,
    }
  }

  /// The builder with the format hint `format`. The hint orders the candidates that the bytes allow, ahead of the
  /// file-name hint. It never overrides a conclusive signature, and never selects a format for bytes that matched
  /// nothing: detection then gives the outcome `HintOnly`, and a probe the error `UnknownFormat` with the hint.
  #[must_use]
  pub fn with_format_hint(mut self, format: Format) -> Prober {
    self.format_hint = Some(format);
    self
  }

  /// The builder with the file-name hint `name`, which orders the candidates as the format hint does, after it.
  /// The name is never opened. Without it, `probe_file` and `detect_file` take the final component of their path.
  ///
  /// The detection inventory of plan gate G3.5 fixes the role of file-name hints. Until then, detection keeps this
  /// hint but consults neither it nor the final component of a path, so only the format hint gives the outcome
  /// `HintOnly`.
  #[must_use]
  pub fn with_file_name_hint(mut self, name: impl Into<OsString>) -> Prober {
    self.file_name_hint = Some(name.into());
    self
  }

  /// The builder with the limits of the shared budget.
  #[must_use]
  pub fn with_limits(mut self, limits: Limits) -> Prober {
    self.limits = limits;
    self
  }

  /// The builder with the request for the tags; `Skip` leaves the tags section "not requested".
  #[must_use]
  pub fn with_tags(mut self, request: SectionRequest) -> Prober {
    self.tags = request;
    self
  }

  /// The builder with the request for the chapters; `Skip` leaves the chapters section "not requested".
  #[must_use]
  pub fn with_chapters(mut self, request: SectionRequest) -> Prober {
    self.chapters = request;
    self
  }

  /// The builder with the handling of every attachment whose role is not cover art.
  #[must_use]
  pub fn with_attachments(mut self, handling: AttachmentHandling) -> Prober {
    self.attachments = handling;
    self
  }

  /// The builder with the handling of every attachment whose role is cover art, whatever its carriage.
  #[must_use]
  pub fn with_cover_art(mut self, handling: AttachmentHandling) -> Prober {
    self.cover_art = handling;
    self
  }

  /// The format hint, absent by default.
  #[must_use]
  pub fn format_hint(&self) -> Option<&Format> {
    self.format_hint.as_ref()
  }

  /// The file-name hint that the builder sets, absent by default.
  #[must_use]
  pub fn file_name_hint(&self) -> Option<&OsStr> {
    self.file_name_hint.as_deref()
  }

  /// The limits of the shared budget, `Limits::default()` by default.
  #[must_use]
  pub fn limits(&self) -> &Limits {
    &self.limits
  }

  /// The request for the tags, `Read` by default.
  #[must_use]
  pub fn tags(&self) -> &SectionRequest {
    &self.tags
  }

  /// The request for the chapters, `Read` by default.
  #[must_use]
  pub fn chapters(&self) -> &SectionRequest {
    &self.chapters
  }

  /// The handling of every attachment whose role is not cover art, `MetadataOnly` by default.
  #[must_use]
  pub fn attachments(&self) -> &AttachmentHandling {
    &self.attachments
  }

  /// The handling of every attachment whose role is cover art, `MetadataOnly` by default.
  #[must_use]
  pub fn cover_art(&self) -> &AttachmentHandling {
    &self.cover_art
  }
}

impl Default for Prober {
  /// The default options, the same value as `Prober::new()`.
  fn default() -> Prober {
    Prober::new()
  }
}

#[cfg(test)]
mod tests {
  use std::ffi::OsStr;

  use super::Prober;
  use crate::model::{AttachmentHandling, SectionRequest};
  use crate::{Format, Limits};

  fn shared<Value: Send + Sync + Clone>(value: &Value) -> Value {
    value.clone()
  }

  #[test]
  fn gives_the_default_options() {
    for prober in [Prober::new(), Prober::default()] {
      assert_eq!(prober.format_hint(), None);
      assert_eq!(prober.file_name_hint(), None);
      assert_eq!(prober.limits(), &Limits::default());
      assert_eq!(prober.tags(), &SectionRequest::Read);
      assert_eq!(prober.chapters(), &SectionRequest::Read);
      assert_eq!(prober.attachments(), &AttachmentHandling::MetadataOnly);
      assert_eq!(prober.cover_art(), &AttachmentHandling::MetadataOnly);
    }
  }

  #[test]
  fn keeps_each_option_that_a_setter_gives() {
    let prober = Prober::new()
      .with_format_hint(Format::Avi)
      .with_file_name_hint("movie.avi")
      .with_limits(Limits::new().with_streams(2))
      .with_tags(SectionRequest::Skip)
      .with_chapters(SectionRequest::Skip)
      .with_attachments(AttachmentHandling::WithData)
      .with_cover_art(AttachmentHandling::Skip);
    let copy = shared(&prober);
    assert_eq!(copy.format_hint(), Some(&Format::Avi));
    assert_eq!(copy.file_name_hint(), Some(OsStr::new("movie.avi")));
    assert_eq!(copy.limits().streams(), 2);
    assert_eq!(copy.tags(), &SectionRequest::Skip);
    assert_eq!(copy.chapters(), &SectionRequest::Skip);
    assert_eq!(copy.attachments(), &AttachmentHandling::WithData);
    assert_eq!(copy.cover_art(), &AttachmentHandling::Skip);
    assert!(format!("{prober:?}").starts_with("Prober {"));
  }
}
