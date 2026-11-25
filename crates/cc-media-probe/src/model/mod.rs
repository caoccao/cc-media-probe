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

//! The report model (`api.md` section 6): every type of a report and of a detection, and the value types, except
//! those that the crate root holds.

mod attachments;
mod audio;
mod chapters;
mod codec_information;
mod detection;
mod entity;
mod language;
mod programs;
mod protection;
mod references;
mod report;
mod stream;
mod subtitle;
mod tags;
mod tree;
mod unrecognized;
mod usage;
mod values;
mod video;

pub use crate::format::SupportLevel;
pub use attachments::{Attachment, AttachmentRole};
pub use audio::{
  AmbisonicChannelOrdering, AmbisonicNormalization, Ambisonics, AudioInformation, ByteOrder, ChannelLayout,
  ChannelPosition, ImmersiveAudio, ImmersiveKind, SampleEncoding, SampleFormat, Speaker,
};
pub use chapters::{Chapter, ChapterEdition, ChapterOrigin, SegmentReference};
pub use codec_information::{
  CodecCandidate, CodecInformation, CodecNamespace, Level, Profile, RawCodecIdentifier, Tier,
};
pub use detection::{Candidate, Confidence, Detection, DetectionOutcome, Hint, HintOrigin};
pub use entity::{EntityKind, EntityLocation};
pub use language::{Language, LanguageCodeSystem, LocalizedText};
pub use programs::{MapState, Program};
pub use protection::{
  Protection, ProtectionDeclaration, ProtectionScheme, ProtectionSchemeNamespace, ProtectionScope,
  SampleDescriptionProtection, SampleDescriptionState,
};
pub use references::{ExternalReference, ReferenceKind, ReferenceOwner, ReferenceRole};
pub use report::{FileTags, InventoryClass, MediaReport, StreamTags, TagsWithKey};
pub use stream::{
  DeclarationRange, Disposition, LanguageRole, Stream, StreamKind, StreamLanguage, Timecode, TimecodeLabel,
};
pub use subtitle::{EncodingName, SubtitleInformation, SubtitleKind, TextEncoding};
pub use tags::{
  BinaryValue, IdentifierNamespace, IntegerValue, NativeScope, Tag, TagKey, TagName, TagSet, TagValue, TargetLevel,
  TargetReference, TargetReferenceKind, TargetScope,
};
pub(crate) use unrecognized::OwnedText;
pub use unrecognized::{UnrecognizedKind, UnrecognizedValue};
pub use usage::Usage;
pub use values::{
  Estimate, EstimateMethod, FloatingPointValue, FloatingPointWidth, Guid, Identifier, MediaType, Provenance, Rational,
  Seconds, Size, TimeRange, UtcTime,
};
pub use video::{
  BitDepth, ChromaFormat, ChromaSubsampling, Chromaticity, ColorDescription, ColorRange, Component, ComponentDepth,
  ContentLightLevel, DynamicRange, DynamicRangeMark, DynamicRangeTransfer, FieldOrder, FrameRateMode, MasteringDisplay,
  Projection, ProjectionKind, Rotation, ScanType, StereoLayout, StereoMode, VideoInformation,
};
