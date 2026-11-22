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

//! The names and texts of the model's types.

/// The unit after a time in `Seconds`'s display.
pub const SECONDS_SYMBOL: &str = "s";
/// Between the width and the height in `Size`'s display.
pub const SIZE_SEPARATOR: &str = "x";
/// The display of a codec without a name or a raw identifier.
pub const UNKNOWN_CODEC_TEXT: &str = "unknown codec";

/// Format names.
pub const MATROSKA_NAME: &str = "Matroska";
/// Format names.
pub const WEBM_NAME: &str = "WebM";
/// Format names.
pub const MP4_NAME: &str = "MP4";
/// Format names.
pub const QUICKTIME_NAME: &str = "QuickTime";
/// Format names.
pub const AVI_NAME: &str = "AVI";
/// Format names.
pub const ISO_BASE_MEDIA_NAME: &str = "ISO base media";
/// Format names.
pub const RIFF_NAME: &str = "RIFF";

/// Codec names.
pub const H264_NAME: &str = "H.264";
/// Codec names.
pub const AV1_NAME: &str = "AV1";
/// Codec names.
pub const VP9_NAME: &str = "VP9";
/// Codec names.
pub const AAC_NAME: &str = "AAC";
/// Codec names.
pub const OPUS_NAME: &str = "Opus";
/// Codec names.
pub const WEBVTT_NAME: &str = "WebVTT";

/// The serialized names of `StreamKind`.
pub const VIDEO_NAME: &str = "video";
/// The serialized names of `StreamKind`.
pub const AUDIO_NAME: &str = "audio";
/// The serialized names of `StreamKind`.
pub const SUBTITLE_NAME: &str = "subtitle";
/// The serialized names of `StreamKind`.
pub const DATA_NAME: &str = "data";
/// The serialized names of `StreamKind`.
pub const UNKNOWN_NAME: &str = "unknown";

/// The serialized names of `Provenance`.
pub const DECLARED_NAME: &str = "declared";
/// The serialized names of `Provenance`.
pub const BITSTREAM_NAME: &str = "bitstream";
/// The serialized names of `Provenance`.
pub const DERIVED_NAME: &str = "derived";

/// The serialized names of `EstimateMethod`.
pub const CONSTANT_BIT_RATE_OVER_SIZE_NAME: &str = "constant_bit_rate_over_size";
/// The serialized names of `EstimateMethod`.
pub const DELIVERY_CLOCK_SPAN_NAME: &str = "delivery_clock_span";
/// The serialized names of `EstimateMethod`.
pub const ENDPOINT_TIMESTAMPS_NAME: &str = "endpoint_timestamps";
/// The serialized names of `EstimateMethod`.
pub const LAST_UNIT_START_NAME: &str = "last_unit_start";
/// The serialized names of `EstimateMethod`.
pub const INDEX_SIZE_TOTALS_NAME: &str = "index_size_totals";

/// The serialized names of `TagKey`.
pub const TITLE_NAME: &str = "title";
/// The serialized names of `TagKey`.
pub const ARTIST_NAME: &str = "artist";
/// The serialized names of `TagKey`.
pub const ALBUM_NAME: &str = "album";
/// The serialized names of `TagKey`.
pub const DATE_NAME: &str = "date";
/// The serialized names of `TagKey`.
pub const GENRE_NAME: &str = "genre";
/// The serialized names of `TagKey`.
pub const COMMENT_NAME: &str = "comment";
/// The serialized names of `TagKey`.
pub const COPYRIGHT_NAME: &str = "copyright";
/// The serialized names of `TagKey`.
pub const ENCODER_NAME: &str = "encoder";
/// The serialized names of `TagKey`.
pub const TRACK_NUMBER_NAME: &str = "track_number";
/// The serialized names of `TagKey`.
pub const DISC_NUMBER_NAME: &str = "disc_number";

/// The angles of `Rotation`, in degrees.
pub const UPRIGHT_DEGREES: u16 = 0;
/// The angles of `Rotation`, in degrees.
pub const CLOCKWISE_90_DEGREES: u16 = 90;
/// The angles of `Rotation`, in degrees.
pub const CLOCKWISE_180_DEGREES: u16 = 180;
/// The angles of `Rotation`, in degrees.
pub const CLOCKWISE_270_DEGREES: u16 = 270;
