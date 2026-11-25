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

//! `StreamKind`, what a stream carries (`api.md` section 6.3).

use super::constants::{AUDIO_NAME, BUTTONS_NAME, DATA_NAME, SUBTITLE_NAME, UNKNOWN_NAME, VIDEO_NAME};
use crate::string_enumeration::string_enumeration;

string_enumeration! {
  /// What a stream carries.
  ///
  /// ```
  /// use cc_media_probe::{MediaReport, StreamKind};
  ///
  /// fn audio_streams(report: &MediaReport) -> usize {
  ///     report.streams().iter().filter(|stream| matches!(stream.kind(), StreamKind::Audio)).count()
  /// }
  ///
  /// assert_eq!(StreamKind::Subtitle.as_str(), "subtitle");
  /// ```
  StreamKind {
    /// Video.
    Video => VIDEO_NAME,
    /// Audio.
    Audio => AUDIO_NAME,
    /// Subtitles and captions.
    Subtitle => SUBTITLE_NAME,
    /// Interactive buttons, as of a DVD menu.
    Buttons => BUTTONS_NAME,
    /// Timed data that is none of the above, such as timecodes or metadata.
    Data => DATA_NAME,
    /// A stream whose kind the file does not state and its codec does not imply.
    Unknown => UNKNOWN_NAME,
  }
}

#[cfg(test)]
mod tests {
  use super::StreamKind;
  use crate::model::UnrecognizedValue;

  #[test]
  fn names_each_kind() {
    for (kind, name) in [
      (StreamKind::Video, "video"),
      (StreamKind::Audio, "audio"),
      (StreamKind::Subtitle, "subtitle"),
      (StreamKind::Buttons, "buttons"),
      (StreamKind::Data, "data"),
      (StreamKind::Unknown, "unknown"),
      (
        StreamKind::Unrecognized(UnrecognizedValue::sample("hologram")),
        "hologram",
      ),
    ] {
      assert_eq!(kind.as_str(), name);
    }
  }
}
