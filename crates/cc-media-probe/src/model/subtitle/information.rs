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

//! `SubtitleInformation`, what a subtitle stream says about its events (`api.md` section 6.7; field guide section
//! 3.6).

use super::{SubtitleKind, TextEncoding};
use crate::form_record::form_record;
use crate::model::{Provenance, Size, TimeRange};

form_record! {
  /// What a subtitle stream says about its events, from `Stream::subtitle()`. Present exactly when the stream's
  /// kind is subtitle; every field inside may still be absent.
  ///
  /// ```
  /// use cc_media_probe::Stream;
  ///
  /// fn subtitle_summary(stream: &Stream) -> Option<String> {
  ///     let subtitle = stream.subtitle()?;
  ///     Some(format!(
  ///         "{:?} in {:?}, canvas {:?} ({:?}), {:?} events over {:?}",
  ///         subtitle.subtitle_kind().map(|kind| kind.as_str()),
  ///         subtitle.text_encoding().map(|encoding| encoding.as_str()),
  ///         subtitle.canvas_size(),
  ///         subtitle.canvas_size_provenance(),
  ///         subtitle.event_count(),
  ///         subtitle.time_range(),
  ///     ))
  /// }
  /// ```
  #[derive(Clone, Debug, PartialEq, Eq, Hash)]
  pub struct SubtitleInformation {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    subtitle_kind: Option<SubtitleKind>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    text_encoding: Option<TextEncoding>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    canvas_size: Option<Size>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    canvas_size_provenance: Option<Provenance>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    event_count: Option<u64>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    time_range: Option<TimeRange>,
  }
  pairs = [
    canvas_size with canvas_size_provenance,
  ];
}

impl SubtitleInformation {
  /// Whether the events are text or pictures.
  #[must_use]
  pub fn subtitle_kind(&self) -> Option<&SubtitleKind> {
    self.subtitle_kind.as_ref()
  }

  /// The encoding of text events.
  #[must_use]
  pub fn text_encoding(&self) -> Option<&TextEncoding> {
    self.text_encoding.as_ref()
  }

  /// The size of the canvas that the events are placed on, in pixels.
  #[must_use]
  pub fn canvas_size(&self) -> Option<Size> {
    self.canvas_size
  }

  /// Where the canvas size came from.
  #[must_use]
  pub fn canvas_size_provenance(&self) -> Option<&Provenance> {
    self.canvas_size_provenance.as_ref()
  }

  /// The number of events, only when every event was examined.
  #[must_use]
  pub fn event_count(&self) -> Option<u64> {
    self.event_count
  }

  /// The time that the events span, only when every event's end is established (F20).
  #[must_use]
  pub fn time_range(&self) -> Option<TimeRange> {
    self.time_range
  }
}

#[cfg(test)]
mod tests {
  use super::SubtitleInformation;
  use crate::Seconds;
  use crate::model::{Provenance, Size, SubtitleKind, TextEncoding, TimeRange};

  impl SubtitleInformation {
    /// A stream of 12 text events in UTF-8 over the first minute.
    pub(crate) fn sample() -> SubtitleInformation {
      SubtitleInformation {
        subtitle_kind: Some(SubtitleKind::Text),
        text_encoding: Some(TextEncoding::Utf8),
        canvas_size: Size::new(1_920, 1_080),
        canvas_size_provenance: Some(Provenance::Declared),
        event_count: Some(12),
        time_range: Some(TimeRange::sample(Seconds::sample(1, 1), Seconds::sample(60, 1))),
      }
    }
  }

  #[test]
  fn gives_each_field() {
    let subtitle = SubtitleInformation::sample();
    assert_eq!(subtitle.subtitle_kind(), Some(&SubtitleKind::Text));
    assert_eq!(subtitle.text_encoding(), Some(&TextEncoding::Utf8));
    assert_eq!(subtitle.canvas_size(), Size::new(1_920, 1_080));
    assert_eq!(subtitle.canvas_size_provenance(), Some(&Provenance::Declared));
    assert_eq!(subtitle.event_count(), Some(12));
    assert_eq!(
      subtitle.time_range().map(|range| range.end()),
      Some(Seconds::sample(60, 1))
    );
  }
}
