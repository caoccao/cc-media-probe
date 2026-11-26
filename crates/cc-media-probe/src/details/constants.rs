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

//! The serialized kinds of the shared detail types (`json-output.md` sections 6.2 and 8).

/// The serialized kind of `ObservedValue::Count`, also the name of the member that holds the count.
pub const COUNT_KIND: &str = "count";
/// The serialized kind of `ObservedValue::Time`, also the name of the member that holds the time.
pub const TIME_KIND: &str = "time";

/// The message of an exact value or input whose parts are not canonical decimal strings (`json-output.md`
/// section 8).
#[cfg(feature = "serde")]
pub const EXACT_VALUE_MESSAGE: &str = "an exact value's parts are not canonical decimal strings";

/// The message of an exact rational that is not reduced, or whose part has more digits than an exact value holds
/// (`json-output.md` section 8).
#[cfg(feature = "serde")]
pub const EXACT_REDUCED_MESSAGE: &str = "an exact value is not reduced, or a part has more than 10000 digits";

/// The most digits of a part of an exact value, its sign not counted, as the maintainer chose at the second review of
/// the third slice's first part (`json-output.md` section 8). It holds the exact rational of every binary
/// floating-point value up to the 80-bit extended format, whose least positive value, 2^-16445, has a denominator of
/// 4951 digits, with room for the factors that a computation adds, and it bounds the time that checking a part takes.
#[cfg(feature = "serde")]
pub const MAXIMUM_EXACT_DIGITS: usize = 10_000;

/// The decimal digits that one step of reading a part takes in, so that each step's value fits 64 bits: 10^19 is
/// below 2^64.
#[cfg(feature = "serde")]
pub const DIGITS_PER_STEP: usize = 19;

/// The radix of a decimal digit.
#[cfg(feature = "serde")]
pub const DECIMAL_RADIX: u32 = 10;

/// The message of an exact value that does not fit its field: a rational field of the field guide without a
/// denominator, an integer field with one or negative, or a value within the range that the normalized field holds
/// (`json-output.md` section 8).
#[cfg(feature = "serde")]
pub const EXACT_FIELD_MESSAGE: &str = "an exact value does not fit its field, or its normalized field holds it";

/// The fields that are rationals, which an exact value writes with a denominator, `1` for a whole one
/// (`docs/guide/fields.md` section 3; `json-output.md` section 8): every rational quantity of the report model that a
/// probe can leave absent, by its accessor's name, as the second review of the third slice's first part found the
/// angles of a projection and of a channel position, the luminance and chromaticities of a mastering display and the
/// estimates missing. Times, durations, rates, ratios and angles.
#[cfg(feature = "serde")]
pub const RATIONAL_FIELDS: [&str; 23] = [
  "duration",
  "start_time",
  "bit_rate",
  "pixel_aspect_ratio",
  "display_aspect_ratio",
  "frame_rate",
  "sample_rate",
  "output_sample_rate",
  "start",
  "end",
  "playback_duration",
  "playback_position",
  "estimated_duration",
  "estimated_bit_rate",
  "yaw",
  "pitch",
  "roll",
  "azimuth",
  "elevation",
  "maximum_luminance",
  "minimum_luminance",
  "x_coordinate",
  "y_coordinate",
];

/// The fields that are integers, which an exact value writes without a denominator (`docs/guide/fields.md` section 3;
/// `json-output.md` section 8): every integer quantity of the report model that a probe can leave absent, by its
/// accessor's name, as the second review of the third slice's first part found a bit depth and the light levels
/// missing. Sizes in bytes or pixels, counts, depths and levels.
#[cfg(feature = "serde")]
pub const INTEGER_FIELDS: [&str; 14] = [
  "size",
  "packet_count",
  "frame_count",
  "sample_count",
  "event_count",
  "channel_count",
  "encoder_delay",
  "padding",
  "width",
  "height",
  "bit_depth",
  "ambisonic_component",
  "maximum_content",
  "maximum_frame_average",
];

/// The message of a typed detail's entry whose `stream` is not its owner's index, given exactly when the owner is a
/// stream (`json-output.md` section 8).
#[cfg(feature = "serde")]
pub const OWNER_STREAM_MESSAGE: &str = "an entry's stream is not the index of its owner stream";

/// The message of a value of a typed detail that is not in the form of its type, such as a 64-bit integer that is no
/// decimal string in canonical form (`json-output.md` section 5.5).
#[cfg(feature = "serde")]
pub const DETAIL_VALUE_MESSAGE: &str = "a value of a typed detail is not in the form of its type";
