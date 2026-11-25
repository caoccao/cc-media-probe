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

//! What a video stream says about its pictures (`api.md` section 6.5).

mod bit_depth;
mod chroma_format;
mod color;
mod constants;
mod content_light_level;
mod dynamic_range;
mod frame_rate_mode;
mod information;
mod mastering_display;
mod projection;
mod rotation;
mod scan;
mod stereo_mode;

pub use bit_depth::{BitDepth, Component, ComponentDepth};
pub use chroma_format::{ChromaFormat, ChromaSubsampling};
pub use color::{ColorDescription, ColorRange};
pub use content_light_level::ContentLightLevel;
pub use dynamic_range::{DynamicRange, DynamicRangeMark, DynamicRangeTransfer};
pub use frame_rate_mode::FrameRateMode;
pub use information::VideoInformation;
pub use mastering_display::{Chromaticity, MasteringDisplay};
pub use projection::{Projection, ProjectionKind};
pub use rotation::Rotation;
pub use scan::{FieldOrder, ScanType};
pub use stereo_mode::{StereoLayout, StereoMode};
