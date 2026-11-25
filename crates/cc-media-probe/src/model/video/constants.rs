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

//! The serialized names of the video types (`json-output.md` sections 6.1 and 6.2).

/// The serialized name of `FrameRateMode::Constant`.
pub const CONSTANT_NAME: &str = "constant";
/// The serialized name of `FrameRateMode::Variable`.
pub const VARIABLE_NAME: &str = "variable";
/// The serialized name of `FrameRateMode::Unknown`.
pub const UNKNOWN_NAME: &str = "unknown";

/// The serialized name of `Component::Luma`.
pub const LUMA_NAME: &str = "luma";
/// The serialized name of `Component::Chroma`.
pub const CHROMA_NAME: &str = "chroma";
/// The serialized name of `Component::Red`.
pub const RED_NAME: &str = "red";
/// The serialized name of `Component::Green`.
pub const GREEN_NAME: &str = "green";
/// The serialized name of `Component::Blue`.
pub const BLUE_NAME: &str = "blue";
/// The serialized name of `Component::Alpha`.
pub const ALPHA_NAME: &str = "alpha";

/// The serialized kind of `ChromaFormat::Monochrome`.
pub const MONOCHROME_KIND: &str = "monochrome";
/// The serialized kind of `ChromaFormat::Chroma420`.
pub const CHROMA_420_KIND: &str = "chroma_420";
/// The serialized kind of `ChromaFormat::Chroma422`.
pub const CHROMA_422_KIND: &str = "chroma_422";
/// The serialized kind of `ChromaFormat::Chroma444`.
pub const CHROMA_444_KIND: &str = "chroma_444";
/// The serialized kind of `ChromaFormat::Chroma411`.
pub const CHROMA_411_KIND: &str = "chroma_411";
/// The serialized kind of `ChromaFormat::Other`, also the name of the member that holds the subsampling.
pub const OTHER_KIND: &str = "other";

/// The serialized name of `ScanType::Progressive`.
pub const PROGRESSIVE_NAME: &str = "progressive";
/// The serialized name of `ScanType::Interlaced`.
pub const INTERLACED_NAME: &str = "interlaced";
/// The serialized name of `FieldOrder::TopFieldFirst`.
pub const TOP_FIELD_FIRST_NAME: &str = "top_field_first";
/// The serialized name of `FieldOrder::BottomFieldFirst`.
pub const BOTTOM_FIELD_FIRST_NAME: &str = "bottom_field_first";

/// The serialized name of `ColorRange::Limited`.
pub const LIMITED_NAME: &str = "limited";
/// The serialized name of `ColorRange::Full`.
pub const FULL_NAME: &str = "full";

/// The serialized name of `DynamicRangeTransfer::Standard`.
pub const STANDARD_NAME: &str = "standard";
/// The serialized name of `DynamicRangeTransfer::PerceptualQuantizer`.
pub const PERCEPTUAL_QUANTIZER_NAME: &str = "perceptual_quantizer";
/// The serialized name of `DynamicRangeTransfer::HybridLogGamma`.
pub const HYBRID_LOG_GAMMA_NAME: &str = "hybrid_log_gamma";
/// The serialized name of `DynamicRangeMark::DolbyVision`.
pub const DOLBY_VISION_NAME: &str = "dolby_vision";

/// The serialized name of `Rotation::Upright`.
pub const UPRIGHT_NAME: &str = "upright";
/// The serialized name of `Rotation::Clockwise90`.
pub const CLOCKWISE_90_NAME: &str = "clockwise_90";
/// The serialized name of `Rotation::Clockwise180`.
pub const CLOCKWISE_180_NAME: &str = "clockwise_180";
/// The serialized name of `Rotation::Clockwise270`.
pub const CLOCKWISE_270_NAME: &str = "clockwise_270";
/// The clockwise angle of `Rotation::Clockwise90`, in degrees.
pub const CLOCKWISE_90_DEGREES: u16 = 90;
/// The clockwise angle of `Rotation::Clockwise180`, in degrees.
pub const CLOCKWISE_180_DEGREES: u16 = 180;
/// The clockwise angle of `Rotation::Clockwise270`, in degrees.
pub const CLOCKWISE_270_DEGREES: u16 = 270;

/// The serialized name of `ProjectionKind::Rectangular`.
pub const RECTANGULAR_NAME: &str = "rectangular";
/// The serialized name of `ProjectionKind::Equirectangular`.
pub const EQUIRECTANGULAR_NAME: &str = "equirectangular";
/// The serialized name of `ProjectionKind::Cubemap`.
pub const CUBEMAP_NAME: &str = "cubemap";
/// The serialized name of `ProjectionKind::Mesh`.
pub const MESH_NAME: &str = "mesh";
