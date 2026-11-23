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
/// Between the candidates in the display of an unresolved codec.
pub const CANDIDATE_SEPARATOR: &str = " or ";
/// The display of a candidate that the probe could not read or keep.
pub const MISSING_CANDIDATE_TEXT: &str = "a description that was not read";

/// The serialized names of `DetectionOutcome`, with `UNKNOWN_NAME` below.
pub const IDENTIFIED_NAME: &str = "identified";
/// The serialized names of `DetectionOutcome`.
pub const FAMILY_ONLY_NAME: &str = "family_only";
/// The serialized names of `DetectionOutcome`.
pub const AMBIGUOUS_NAME: &str = "ambiguous";
/// The serialized names of `DetectionOutcome`.
pub const HINT_ONLY_NAME: &str = "hint_only";
/// The serialized names of `DetectionOutcome`.
pub const INSUFFICIENT_DATA_NAME: &str = "insufficient_data";

/// The serialized names of `EntityKind`.
pub const STREAM_NAME: &str = "stream";
/// The serialized names of `EntityKind`.
pub const EDITION_NAME: &str = "edition";
/// The serialized names of `EntityKind`.
pub const CHAPTER_NAME: &str = "chapter";
/// The serialized names of `EntityKind`.
pub const ATTACHMENT_NAME: &str = "attachment";
/// The serialized names of `EntityKind`.
pub const PROGRAM_NAME: &str = "program";

/// The serialized names of `TargetScope`.
pub const WHOLE_FILE_NAME: &str = "whole_file";
/// The serialized names of `TargetScope`.
pub const ENTITIES_NAME: &str = "entities";
/// The serialized names of `TargetScope`.
pub const INCOMPLETE_NAME: &str = "incomplete";

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

/// The serialized kinds of `TagValue`, each also the name of the member that holds the payload.
pub const TEXT_KIND: &str = "text";
/// The serialized kinds of `TagValue`.
pub const BINARY_KIND: &str = "binary";
/// The serialized kinds of `TagValue`.
pub const INTEGER_KIND: &str = "integer";
/// The serialized kinds of `TagValue`.
pub const BOOLEAN_KIND: &str = "boolean";
