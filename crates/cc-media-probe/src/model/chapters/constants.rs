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

//! The serialized names of `ChapterOrigin` (`json-output.md` section 6.1), and the message of an edition that
//! contradicts itself.

/// The serialized name of `ChapterOrigin::MatroskaEdition`.
pub const MATROSKA_EDITION_NAME: &str = "matroska_edition";
/// The serialized name of `ChapterOrigin::QuickTimeChapterTrack`.
pub const QUICKTIME_CHAPTER_TRACK_NAME: &str = "quicktime_chapter_track";

/// The message of an edition whose playback values or link flag its order and its chapters rule out (`api.md`
/// section 6.8).
#[cfg(feature = "serde")]
pub const EDITION_MESSAGE: &str = "an unordered edition has playback values, or an edition said to link no segment \
                                   has a chapter that links one";
