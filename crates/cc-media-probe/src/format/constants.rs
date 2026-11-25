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

//! The serialized names and the library's names of the formats (`json-output.md` section 6.1).

/// The serialized name of `Format::Matroska`.
pub const MATROSKA_NAME: &str = "matroska";
/// The serialized name of `Format::WebM`.
pub const WEBM_NAME: &str = "webm";
/// The serialized name of `Format::Mp4`.
pub const MP4_NAME: &str = "mp4";
/// The serialized name of `Format::ThreeGpp`.
pub const THREE_GPP_NAME: &str = "three_gpp";
/// The serialized name of `Format::ThreeGpp2`.
pub const THREE_GPP2_NAME: &str = "three_gpp2";
/// The serialized name of `Format::F4v`.
pub const F4V_NAME: &str = "f4v";
/// The serialized name of `Format::QuickTime`.
pub const QUICKTIME_NAME: &str = "quicktime";
/// The serialized name of `Format::Avi`.
pub const AVI_NAME: &str = "avi";
/// The serialized name of `Format::IsoBaseMedia`.
pub const ISO_BASE_MEDIA_NAME: &str = "iso_base_media";
/// The serialized name of `Format::Riff`.
pub const RIFF_NAME: &str = "riff";

/// The library's name of `Format::Matroska`.
pub const MATROSKA_TITLE: &str = "Matroska";
/// The library's name of `Format::WebM`.
pub const WEBM_TITLE: &str = "WebM";
/// The library's name of `Format::Mp4`.
pub const MP4_TITLE: &str = "MP4";
/// The library's name of `Format::ThreeGpp`.
pub const THREE_GPP_TITLE: &str = "3GPP";
/// The library's name of `Format::ThreeGpp2`.
pub const THREE_GPP2_TITLE: &str = "3GPP2";
/// The library's name of `Format::F4v`.
pub const F4V_TITLE: &str = "F4V";
/// The library's name of `Format::QuickTime`.
pub const QUICKTIME_TITLE: &str = "QuickTime";
/// The library's name of `Format::Avi`.
pub const AVI_TITLE: &str = "AVI";
/// The library's name of `Format::IsoBaseMedia`.
pub const ISO_BASE_MEDIA_TITLE: &str = "ISO base media";
/// The library's name of `Format::Riff`.
pub const RIFF_TITLE: &str = "RIFF";

/// The serialized name of `SupportLevel::Probed`, which the CLI's detection block shows (`cli.md` section 8.5).
pub const PROBED_NAME: &str = "probed";
/// The serialized name of `SupportLevel::DetectOnly`.
pub const DETECT_ONLY_NAME: &str = "detect_only";
