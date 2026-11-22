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

//! The report model of `api.md` section 6, as far as the scenarios read it.

mod attachments;
mod audio;
mod chapters;
mod codec;
mod constants;
mod detection;
mod format;
mod language;
mod lookups;
mod report;
mod stream;
mod tags;
mod usage;
mod values;
mod video;

pub use attachments::{Attachment, AttachmentHandling};
pub use audio::AudioInformation;
pub use chapters::{Chapter, ChapterEdition};
pub use codec::{Codec, CodecCandidate, CodecInformation, CodecNamespace, Profile, RawCodecIdentifier};
pub use detection::{Detection, DetectionOutcome};
pub use format::{Format, SupportLevel};
pub use language::{Language, LocalizedText};
pub use lookups::TagsOf;
pub use report::MediaReport;
pub use stream::{Disposition, Stream, StreamKind};
pub use tags::{
  BinaryValue, EntityKind, EntityLocation, IntegerValue, Tag, TagKey, TagName, TagSet, TagValue, TargetLevel,
  TargetReference, TargetReferenceKind, TargetScope,
};
pub use usage::Usage;
pub use values::{
  Estimate, EstimateMethod, Identifier, MediaType, Provenance, Rational, Seconds, Size, UnrecognizedValue,
};
pub use video::{FrameRateMode, Rotation, VideoInformation};
