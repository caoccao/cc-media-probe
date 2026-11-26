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

//! The serialized names of the audio types (`json-output.md` section 6.1), and the messages of a layout that
//! contradicts its channels.

/// The serialized name of `SampleEncoding::Integer`.
pub const INTEGER_NAME: &str = "integer";
/// The serialized name of `SampleEncoding::FloatingPoint`.
pub const FLOATING_POINT_NAME: &str = "floating_point";
/// The serialized name of `ByteOrder::LittleEndian`.
pub const LITTLE_ENDIAN_NAME: &str = "little_endian";
/// The serialized name of `ByteOrder::BigEndian`.
pub const BIG_ENDIAN_NAME: &str = "big_endian";

/// The serialized name of `ImmersiveKind::Objects`.
pub const OBJECTS_NAME: &str = "objects";
/// The serialized name of `ImmersiveKind::IntermediateSpatialFormat`.
pub const INTERMEDIATE_SPATIAL_FORMAT_NAME: &str = "intermediate_spatial_format";
/// The serialized name of `ImmersiveKind::HeightChannels`.
pub const HEIGHT_CHANNELS_NAME: &str = "height_channels";

/// The message of a channel without an assigned position that names one (`api.md` section 6.6).
#[cfg(feature = "serde")]
pub const CHANNEL_POSITION_MESSAGE: &str = "an unassigned channel names a speaker or coordinates";

/// The message of a layout whose positions do not number the channels that the decoder outputs (`api.md` section
/// 6.6).
#[cfg(feature = "serde")]
pub const CHANNEL_COUNT_MESSAGE: &str = "a channel layout does not have one position for each output channel";
