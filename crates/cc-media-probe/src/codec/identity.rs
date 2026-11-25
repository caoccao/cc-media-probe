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

//! `Codec`, one variant per codec of plan sections 4.2 and 4.3 (`api.md` section 6.4).

use std::fmt;

use super::constants::{
  AAC_NAME, AAC_TITLE, AC3_NAME, AC3_TITLE, AC4_NAME, AC4_TITLE, AV1_NAME, AV1_TITLE, DTS_NAME, DTS_TITLE,
  ENHANCED_AC3_NAME, ENHANCED_AC3_TITLE, FLAC_NAME, FLAC_TITLE, H264_NAME, H264_TITLE, H265_NAME, H265_TITLE,
  H266_NAME, H266_TITLE, MERIDIAN_LOSSLESS_PACKING_NAME, MERIDIAN_LOSSLESS_PACKING_TITLE, MPEG_AUDIO_NAME,
  MPEG_AUDIO_TITLE, MPEG1_VIDEO_NAME, MPEG1_VIDEO_TITLE, MPEG2_VIDEO_NAME, MPEG2_VIDEO_TITLE, OPUS_NAME, OPUS_TITLE,
  PCM_NAME, PCM_TITLE, PGS_NAME, PGS_TITLE, TRUEHD_NAME, TRUEHD_TITLE, UNKNOWN_CODEC_TEXT, UNKNOWN_NAME, VORBIS_NAME,
  VORBIS_TITLE, VP8_NAME, VP8_TITLE, VP9_NAME, VP9_TITLE, WEBVTT_NAME, WEBVTT_TITLE,
};
use crate::StreamKind;
use crate::string_enumeration::string_enumeration;

string_enumeration! {
  /// The identity of a codec, one variant per codec, named as commonly written. The codec registry of Phase 4
  /// fixes the list for `0.1.0`, and each later phase adds its codecs.
  ///
  /// Two variants are special. `Unknown` says that the file named a codec that this version does not know; the
  /// stream's raw identifiers keep what the file wrote. `Unrecognized` keeps a serialized name that a newer version
  /// wrote; the library never produces it itself.
  ///
  /// ```
  /// use cc_media_probe::{Codec, StreamKind};
  ///
  /// assert_eq!(Codec::H264.name(), Some("H.264"));
  /// assert_eq!(Codec::H264.as_str(), "h264");
  /// assert_eq!(Codec::Opus.kind(), Some(StreamKind::Audio));
  /// assert_eq!(Codec::Unknown.name(), None);
  /// assert_eq!(Codec::EnhancedAc3.to_string(), "E-AC-3");
  /// ```
  Codec {
    /// H.264, Advanced Video Coding (ITU-T H.264).
    H264 => H264_NAME,
    /// H.265, High Efficiency Video Coding (ITU-T H.265).
    H265 => H265_NAME,
    /// H.266, Versatile Video Coding (ITU-T H.266).
    H266 => H266_NAME,
    /// AV1.
    Av1 => AV1_NAME,
    /// VP8 (RFC 6386).
    Vp8 => VP8_NAME,
    /// VP9.
    Vp9 => VP9_NAME,
    /// MPEG-1 Video (ISO/IEC 11172-2).
    Mpeg1Video => MPEG1_VIDEO_NAME,
    /// MPEG-2 Video (ITU-T H.262).
    Mpeg2Video => MPEG2_VIDEO_NAME,
    /// AAC, Advanced Audio Coding.
    Aac => AAC_NAME,
    /// MPEG audio of layers I to III.
    MpegAudio => MPEG_AUDIO_NAME,
    /// AC-3.
    Ac3 => AC3_NAME,
    /// Enhanced AC-3.
    EnhancedAc3 => ENHANCED_AC3_NAME,
    /// AC-4.
    Ac4 => AC4_NAME,
    /// DTS.
    Dts => DTS_NAME,
    /// Dolby TrueHD.
    TrueHd => TRUEHD_NAME,
    /// Meridian Lossless Packing.
    MeridianLosslessPacking => MERIDIAN_LOSSLESS_PACKING_NAME,
    /// FLAC (RFC 9639).
    Flac => FLAC_NAME,
    /// Opus (RFC 6716).
    Opus => OPUS_NAME,
    /// Vorbis.
    Vorbis => VORBIS_NAME,
    /// Uncompressed pulse-code modulation.
    Pcm => PCM_NAME,
    /// WebVTT text.
    WebVtt => WEBVTT_NAME,
    /// Presentation Graphic Stream subtitles.
    Pgs => PGS_NAME,
    /// A codec that the file names and this version does not know. The raw identifiers keep what the file wrote.
    Unknown => UNKNOWN_NAME,
  }
}

impl Codec {
  /// The library's name for the codec, such as "H.264", or `None` for `Unknown` and `Unrecognized`.
  ///
  /// ```
  /// use cc_media_probe::Codec;
  ///
  /// fn label(codec: &Codec) -> &str {
  ///     codec.name().unwrap_or("another codec")
  /// }
  /// ```
  #[must_use]
  pub fn name(&self) -> Option<&'static str> {
    match self {
      Codec::H264 => Some(H264_TITLE),
      Codec::H265 => Some(H265_TITLE),
      Codec::H266 => Some(H266_TITLE),
      Codec::Av1 => Some(AV1_TITLE),
      Codec::Vp8 => Some(VP8_TITLE),
      Codec::Vp9 => Some(VP9_TITLE),
      Codec::Mpeg1Video => Some(MPEG1_VIDEO_TITLE),
      Codec::Mpeg2Video => Some(MPEG2_VIDEO_TITLE),
      Codec::Aac => Some(AAC_TITLE),
      Codec::MpegAudio => Some(MPEG_AUDIO_TITLE),
      Codec::Ac3 => Some(AC3_TITLE),
      Codec::EnhancedAc3 => Some(ENHANCED_AC3_TITLE),
      Codec::Ac4 => Some(AC4_TITLE),
      Codec::Dts => Some(DTS_TITLE),
      Codec::TrueHd => Some(TRUEHD_TITLE),
      Codec::MeridianLosslessPacking => Some(MERIDIAN_LOSSLESS_PACKING_TITLE),
      Codec::Flac => Some(FLAC_TITLE),
      Codec::Opus => Some(OPUS_TITLE),
      Codec::Vorbis => Some(VORBIS_TITLE),
      Codec::Pcm => Some(PCM_TITLE),
      Codec::WebVtt => Some(WEBVTT_TITLE),
      Codec::Pgs => Some(PGS_TITLE),
      Codec::Unknown | Codec::Unrecognized(_) => None,
    }
  }

  /// The kind of stream that the codec implies when the container states none, or `None` for `Unknown` and
  /// `Unrecognized`.
  ///
  /// ```
  /// use cc_media_probe::{Codec, StreamKind};
  ///
  /// fn is_video_codec(codec: &Codec) -> bool {
  ///     codec.kind() == Some(StreamKind::Video)
  /// }
  /// ```
  #[must_use]
  pub fn kind(&self) -> Option<StreamKind> {
    match self {
      Codec::H264
      | Codec::H265
      | Codec::H266
      | Codec::Av1
      | Codec::Vp8
      | Codec::Vp9
      | Codec::Mpeg1Video
      | Codec::Mpeg2Video => Some(StreamKind::Video),
      Codec::Aac
      | Codec::MpegAudio
      | Codec::Ac3
      | Codec::EnhancedAc3
      | Codec::Ac4
      | Codec::Dts
      | Codec::TrueHd
      | Codec::MeridianLosslessPacking
      | Codec::Flac
      | Codec::Opus
      | Codec::Vorbis
      | Codec::Pcm => Some(StreamKind::Audio),
      Codec::WebVtt | Codec::Pgs => Some(StreamKind::Subtitle),
      Codec::Unknown | Codec::Unrecognized(_) => None,
    }
  }

  /// What a display writes for the codec itself: the library's name, or the serialized name that a newer version
  /// wrote (`api.md` section 6.4, twelfth review).
  pub(crate) fn display_name(&self) -> Option<&str> {
    match self {
      Codec::Unrecognized(value) => Some(value.as_str()),
      known => known.name(),
    }
  }
}

impl fmt::Display for Codec {
  /// Writes the library's name, the serialized name of a codec that a newer version wrote, or "an unknown codec".
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(self.display_name().unwrap_or(UNKNOWN_CODEC_TEXT))
  }
}

#[cfg(test)]
mod tests {
  use super::Codec;
  use crate::StreamKind;
  use crate::model::UnrecognizedValue;

  #[test]
  fn names_each_codec() {
    let codecs = [
      (Codec::H264, "h264", "H.264", StreamKind::Video),
      (Codec::H265, "h265", "H.265", StreamKind::Video),
      (Codec::H266, "h266", "H.266", StreamKind::Video),
      (Codec::Av1, "av1", "AV1", StreamKind::Video),
      (Codec::Vp8, "vp8", "VP8", StreamKind::Video),
      (Codec::Vp9, "vp9", "VP9", StreamKind::Video),
      (Codec::Mpeg1Video, "mpeg1_video", "MPEG-1 Video", StreamKind::Video),
      (Codec::Mpeg2Video, "mpeg2_video", "MPEG-2 Video", StreamKind::Video),
      (Codec::Aac, "aac", "AAC", StreamKind::Audio),
      (Codec::MpegAudio, "mpeg_audio", "MPEG Audio", StreamKind::Audio),
      (Codec::Ac3, "ac3", "AC-3", StreamKind::Audio),
      (Codec::EnhancedAc3, "enhanced_ac3", "E-AC-3", StreamKind::Audio),
      (Codec::Ac4, "ac4", "AC-4", StreamKind::Audio),
      (Codec::Dts, "dts", "DTS", StreamKind::Audio),
      (Codec::TrueHd, "truehd", "TrueHD", StreamKind::Audio),
      (
        Codec::MeridianLosslessPacking,
        "meridian_lossless_packing",
        "MLP",
        StreamKind::Audio,
      ),
      (Codec::Flac, "flac", "FLAC", StreamKind::Audio),
      (Codec::Opus, "opus", "Opus", StreamKind::Audio),
      (Codec::Vorbis, "vorbis", "Vorbis", StreamKind::Audio),
      (Codec::Pcm, "pcm", "PCM", StreamKind::Audio),
      (Codec::WebVtt, "webvtt", "WebVTT", StreamKind::Subtitle),
      (Codec::Pgs, "pgs", "PGS", StreamKind::Subtitle),
    ];
    for (codec, serialized, name, kind) in codecs {
      assert_eq!(codec.as_str(), serialized);
      assert_eq!(codec.name(), Some(name));
      assert_eq!(codec.to_string(), name);
      assert_eq!(codec.kind(), Some(kind));
    }
  }

  #[test]
  fn has_no_name_for_an_unknown_or_newer_codec() {
    assert_eq!(Codec::Unknown.as_str(), "unknown");
    assert_eq!(Codec::Unknown.name(), None);
    assert_eq!(Codec::Unknown.kind(), None);
    assert_eq!(Codec::Unknown.to_string(), "an unknown codec");
    let newer = Codec::Unrecognized(UnrecognizedValue::sample("future_codec"));
    assert_eq!(newer.as_str(), "future_codec");
    assert_eq!(newer.name(), None);
    assert_eq!(newer.kind(), None);
    assert_eq!(newer.to_string(), "future_codec");
  }
}
