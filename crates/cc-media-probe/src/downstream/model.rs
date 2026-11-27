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

//! The public enums and structs of `model`, which a crate that depends on the library can neither match
//! exhaustively nor build with a struct literal.

#[cfg(test)]
use crate as cc_media_probe;

no_exhaustive_match! {
  in [cc_media_probe::model];
  AmbisonicChannelOrdering { Unrecognized(..) }
  AmbisonicNormalization { Unrecognized(..) }
  AttachmentHandling { Skip, MetadataOnly, WithData }
  AttachmentRole { CoverArt, Other, Unrecognized(..) }
  ByteOrder { LittleEndian, BigEndian, Unrecognized(..) }
  ChapterOrigin { MatroskaEdition, QuickTimeChapterTrack, Unrecognized(..) }
  ChromaFormat { Monochrome, Chroma420, Chroma422, Chroma444, Chroma411, Other(..), Unrecognized(..) }
  CodecNamespace { MatroskaCodecIdentifier, RiffFourCharacterCode, WaveFormatTag, WaveSubformat, IsoSampleEntry,
    Mpeg4ObjectType, QuickTimeCode, Unrecognized(..) }
  ColorRange { Limited, Full, Unrecognized(..) }
  Component { Luma, Chroma, Red, Green, Blue, Alpha, Unrecognized(..) }
  DetectionOutcome { Identified, FamilyOnly, Ambiguous, HintOnly, InsufficientData, Unknown, Unrecognized(..) }
  DynamicRangeMark { DolbyVision, Unrecognized(..) }
  DynamicRangeTransfer { Standard, PerceptualQuantizer, HybridLogGamma, Unrecognized(..) }
  EntityKind { Stream, Edition, Chapter, Attachment, Program, Unrecognized(..) }
  EstimateMethod { ConstantBitRateOverSize, DeliveryClockSpan, EndpointTimestamps, LastUnitStart, IndexSizeTotals,
    Unrecognized(..) }
  FieldOrder { TopFieldFirst, BottomFieldFirst, Unrecognized(..) }
  FloatingPointWidth { Single, Double, Unrecognized(..) }
  FrameRateMode { Constant, Variable, Unknown, Unrecognized(..) }
  HintOrigin { FileName, FormatOption, Unrecognized(..) }
  IdentifierNamespace { MatroskaUniqueIdentifier, Unrecognized(..) }
  ImmersiveKind { Objects, IntermediateSpatialFormat, HeightChannels, Unrecognized(..) }
  InventoryClass { Declared, SingleStream, Observed, Unrecognized(..) }
  LanguageCodeSystem { Bcp47, Iso639Alpha3, MatroskaLegacy, QuickTimePacked, Unrecognized(..) }
  LanguageRole { CleanEffects, HearingImpaired, VisuallyImpairedCommentary, Unrecognized(..) }
  MapState { Complete, Incomplete, Unrecognized(..) }
  ProjectionKind { Rectangular, Equirectangular, Cubemap, Mesh, Unrecognized(..) }
  ProtectionSchemeNamespace { IsoSchemeType, MatroskaContentEncryptionAlgorithm, Unrecognized(..) }
  ProtectionScope { Packets, CodecPrivateData, SampleDescription, EncodingSettings, Other, Unread, Unrecognized(..) }
  Provenance { Declared, Bitstream, Derived, Unrecognized(..) }
  ReferenceKind { LinkedSegment, DataReference, LinkedPicture, ExpectedCompanion, PlaylistEntry, Unrecognized(..) }
  ReferenceRole { PreviousSegment, NextSegment, TrackData, Unrecognized(..) }
  Rotation { Upright, Clockwise90, Clockwise180, Clockwise270, Unrecognized(..) }
  SampleDescriptionState { Clear, Declared, Unknown, Unrecognized(..) }
  SampleEncoding { Integer, FloatingPoint, Unrecognized(..) }
  ScanType { Progressive, Interlaced, Unrecognized(..) }
  SectionRequest { Read, Skip }
  Speaker { Unrecognized(..) }
  StereoLayout { Unrecognized(..) }
  StreamKind { Video, Audio, Subtitle, Buttons, Data, Unknown, Unrecognized(..) }
  SubtitleKind { Text, Bitmap, Unrecognized(..) }
  SupportLevel { Probed, DetectOnly, Unrecognized(..) }
  TagKey { Title, Artist, Album, Date, Genre, Comment, Copyright, Encoder, TrackNumber, DiscNumber, Unrecognized(..) }
  TagValue { Text(..), Binary(..), Integer(..), FloatingPoint(..), Boolean(..), Guid(..), Unrecognized(..) }
  TargetReferenceKind { Resolved, Unmatched, Wildcard, Unrecognized(..) }
  TargetScope { WholeFile, Entities, Incomplete, Unrecognized(..) }
  TextEncoding { Utf8, Utf16LittleEndian, Utf16BigEndian, Utf32LittleEndian, Utf32BigEndian, Iso8859Part1,
    Windows1252, MacRoman, Other(..), Unrecognized(..) }
}

no_struct_literal! {
  in [cc_media_probe::model];
  Ambisonics,
  Attachment,
  AudioInformation,
  BinaryValue,
  BitDepth,
  Candidate,
  ChannelLayout,
  ChannelPosition,
  Chapter,
  ChapterEdition,
  ChromaSubsampling,
  Chromaticity,
  CodecCandidate,
  CodecInformation,
  ColorDescription,
  ComponentDepth,
  Confidence,
  ContentLightLevel,
  DeclarationRange,
  Detection,
  Disposition,
  DynamicRange,
  EncodingName,
  EntityLocation,
  Estimate::<cc_media_probe::Seconds>,
  ExternalReference,
  FileTags,
  FloatingPointValue,
  Guid,
  Hint,
  Identifier,
  ImmersiveAudio,
  IntegerValue,
  Language,
  Level,
  LocalizedText,
  MasteringDisplay,
  MediaReport,
  MediaType,
  NativeScope,
  Profile,
  Program,
  Projection,
  Protection,
  ProtectionDeclaration,
  ProtectionScheme,
  Rational,
  RawCodecIdentifier,
  ReferenceOwner,
  SampleDescriptionProtection,
  SampleFormat,
  Seconds,
  SegmentReference,
  Size,
  StereoMode,
  Stream,
  StreamLanguage,
  StreamTags,
  SubtitleInformation,
  Tag,
  TagName,
  TagSet,
  TagsWithKey,
  TargetLevel,
  TargetReference,
  Tier,
  TimeRange,
  Timecode,
  TimecodeLabel,
  UnrecognizedKind::<cc_media_probe::model::TagValue>,
  UnrecognizedValue::<cc_media_probe::StreamKind>,
  Usage,
  UtcTime,
  VideoInformation,
}
