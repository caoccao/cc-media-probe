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

//! The serialized names and texts of a stream's types (`json-output.md` section 6.1).

/// The serialized name of `StreamKind::Video`.
pub const VIDEO_NAME: &str = "video";
/// The serialized name of `StreamKind::Audio`.
pub const AUDIO_NAME: &str = "audio";
/// The serialized name of `StreamKind::Subtitle`.
pub const SUBTITLE_NAME: &str = "subtitle";
/// The serialized name of `StreamKind::Buttons`.
pub const BUTTONS_NAME: &str = "buttons";
/// The serialized name of `StreamKind::Data`.
pub const DATA_NAME: &str = "data";
/// The serialized name of `StreamKind::Unknown`.
pub const UNKNOWN_NAME: &str = "unknown";

/// The serialized name of `LanguageRole::CleanEffects`.
pub const CLEAN_EFFECTS_NAME: &str = "clean_effects";
/// The serialized name of `LanguageRole::HearingImpaired`.
pub const HEARING_IMPAIRED_NAME: &str = "hearing_impaired";
/// The serialized name of `LanguageRole::VisuallyImpairedCommentary`.
pub const VISUALLY_IMPAIRED_COMMENTARY_NAME: &str = "visually_impaired_commentary";

/// The seconds of a minute, and the minutes of an hour.
pub const SIXTY: u64 = 60;
/// The seconds of a day, at which a timecode that wraps at 24 hours starts again.
pub const SECONDS_PER_DAY: u64 = 86_400;
/// The nominal rate that drop-frame counting is defined for, and of which other drop-frame rates are multiples.
pub const DROP_FRAME_BASE_RATE: u64 = 30;
/// The frame numbers that drop-frame counting skips each minute at the base rate.
pub const DROPPED_PER_MINUTE_AT_BASE_RATE: u64 = 2;
/// Every tenth minute, drop-frame counting skips no frame number.
pub const MINUTES_PER_DROP_CYCLE: u64 = 10;
/// The separator before the frames of a drop-frame timecode.
pub const DROP_FRAME_SEPARATOR: char = ';';
/// The separator before the frames of any other timecode.
pub const FRAME_SEPARATOR: char = ':';
/// The sign before a negative timecode.
pub const NEGATIVE_SIGN: &str = "-";
/// The sign before a timecode that is not negative: none.
pub const NO_SIGN: &str = "";

/// The message of a timecode whose nominal rate is 0 (`json-output.md` section 7.3).
#[cfg(feature = "serde")]
pub const TIMECODE_MESSAGE: &str = "a timecode's nominal rate is 0";

/// The message of a stream whose information does not stand beside its kind (`json-output.md` section 7.3).
#[cfg(feature = "serde")]
pub const KIND_INFORMATION_MESSAGE: &str = "a stream's video, audio or subtitle information does not match its kind";

/// The message of a stream whose header state is final while a description that its codec needs is missing, or is
/// protected beside no protection (`api.md` sections 6.3 and 6.4).
#[cfg(feature = "serde")]
pub const HEADER_STATE_MESSAGE: &str = "a stream's header state is final beside a missing description, or protected \
                                        beside no protection";
