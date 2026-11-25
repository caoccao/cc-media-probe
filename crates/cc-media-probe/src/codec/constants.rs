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

//! The serialized names and the library's names of the codecs (`json-output.md` section 6.1).

/// The display of a codec that the file named and this version does not know, without a raw identifier to show.
pub const UNKNOWN_CODEC_TEXT: &str = "an unknown codec";

/// The serialized name of `Codec::H264`.
pub const H264_NAME: &str = "h264";
/// The serialized name of `Codec::H265`.
pub const H265_NAME: &str = "h265";
/// The serialized name of `Codec::H266`.
pub const H266_NAME: &str = "h266";
/// The serialized name of `Codec::Av1`.
pub const AV1_NAME: &str = "av1";
/// The serialized name of `Codec::Vp8`.
pub const VP8_NAME: &str = "vp8";
/// The serialized name of `Codec::Vp9`.
pub const VP9_NAME: &str = "vp9";
/// The serialized name of `Codec::Mpeg1Video`.
pub const MPEG1_VIDEO_NAME: &str = "mpeg1_video";
/// The serialized name of `Codec::Mpeg2Video`.
pub const MPEG2_VIDEO_NAME: &str = "mpeg2_video";
/// The serialized name of `Codec::Aac`.
pub const AAC_NAME: &str = "aac";
/// The serialized name of `Codec::MpegAudio`.
pub const MPEG_AUDIO_NAME: &str = "mpeg_audio";
/// The serialized name of `Codec::Ac3`.
pub const AC3_NAME: &str = "ac3";
/// The serialized name of `Codec::EnhancedAc3`.
pub const ENHANCED_AC3_NAME: &str = "enhanced_ac3";
/// The serialized name of `Codec::Ac4`.
pub const AC4_NAME: &str = "ac4";
/// The serialized name of `Codec::Dts`.
pub const DTS_NAME: &str = "dts";
/// The serialized name of `Codec::TrueHd`.
pub const TRUEHD_NAME: &str = "truehd";
/// The serialized name of `Codec::MeridianLosslessPacking`.
pub const MERIDIAN_LOSSLESS_PACKING_NAME: &str = "meridian_lossless_packing";
/// The serialized name of `Codec::Flac`.
pub const FLAC_NAME: &str = "flac";
/// The serialized name of `Codec::Opus`.
pub const OPUS_NAME: &str = "opus";
/// The serialized name of `Codec::Vorbis`.
pub const VORBIS_NAME: &str = "vorbis";
/// The serialized name of `Codec::Pcm`.
pub const PCM_NAME: &str = "pcm";
/// The serialized name of `Codec::WebVtt`.
pub const WEBVTT_NAME: &str = "webvtt";
/// The serialized name of `Codec::Pgs`.
pub const PGS_NAME: &str = "pgs";
/// The serialized name of `Codec::Unknown`.
pub const UNKNOWN_NAME: &str = "unknown";

/// The library's name of `Codec::H264`.
pub const H264_TITLE: &str = "H.264";
/// The library's name of `Codec::H265`.
pub const H265_TITLE: &str = "H.265";
/// The library's name of `Codec::H266`.
pub const H266_TITLE: &str = "H.266";
/// The library's name of `Codec::Av1`.
pub const AV1_TITLE: &str = "AV1";
/// The library's name of `Codec::Vp8`.
pub const VP8_TITLE: &str = "VP8";
/// The library's name of `Codec::Vp9`.
pub const VP9_TITLE: &str = "VP9";
/// The library's name of `Codec::Mpeg1Video`.
pub const MPEG1_VIDEO_TITLE: &str = "MPEG-1 Video";
/// The library's name of `Codec::Mpeg2Video`.
pub const MPEG2_VIDEO_TITLE: &str = "MPEG-2 Video";
/// The library's name of `Codec::Aac`.
pub const AAC_TITLE: &str = "AAC";
/// The library's name of `Codec::MpegAudio`.
pub const MPEG_AUDIO_TITLE: &str = "MPEG Audio";
/// The library's name of `Codec::Ac3`.
pub const AC3_TITLE: &str = "AC-3";
/// The library's name of `Codec::EnhancedAc3`.
pub const ENHANCED_AC3_TITLE: &str = "E-AC-3";
/// The library's name of `Codec::Ac4`.
pub const AC4_TITLE: &str = "AC-4";
/// The library's name of `Codec::Dts`.
pub const DTS_TITLE: &str = "DTS";
/// The library's name of `Codec::TrueHd`.
pub const TRUEHD_TITLE: &str = "TrueHD";
/// The library's name of `Codec::MeridianLosslessPacking`.
pub const MERIDIAN_LOSSLESS_PACKING_TITLE: &str = "MLP";
/// The library's name of `Codec::Flac`.
pub const FLAC_TITLE: &str = "FLAC";
/// The library's name of `Codec::Opus`.
pub const OPUS_TITLE: &str = "Opus";
/// The library's name of `Codec::Vorbis`.
pub const VORBIS_TITLE: &str = "Vorbis";
/// The library's name of `Codec::Pcm`.
pub const PCM_TITLE: &str = "PCM";
/// The library's name of `Codec::WebVtt`.
pub const WEBVTT_TITLE: &str = "WebVTT";
/// The library's name of `Codec::Pgs`.
pub const PGS_TITLE: &str = "PGS";
