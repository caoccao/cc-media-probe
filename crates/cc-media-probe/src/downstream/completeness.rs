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

//! The public enums and structs of `completeness`, which a crate that depends on the library can neither match
//! exhaustively nor build with a struct literal.

#[cfg(test)]
use crate as cc_media_probe;

no_exhaustive_match! {
  in [cc_media_probe::completeness];
  CompanionStatus { NotApplicable, Complete, NotEnabled, CutShort, Unrecognized(..) }
  DetectionStatus { Complete, Incomplete, Unrecognized(..) }
  DiscoveryStatus { Complete, Incomplete, Unrecognized(..) }
  Feature { Zlib, Unrecognized(..) }
  HeaderState { Parsed, PartiallyParsedByDesign, Identified, Protected, NotFoundWithinBudget,
    PartiallyParsedForLackOfBytes, Malformed, CompressionUnavailable, NotAttempted, Unrecognized(..) }
  HeadersStatus { Complete, Incomplete, Unrecognized(..) }
  Identification { IdentificationOnly, UnknownCodec, PendingParser, UnsupportedCompression, Unrecognized(..) }
  Part { Detection, Discovery, Headers, Tags, Chapters, Attachments, Timing, Companions, Unrecognized(..) }
  ReasonKind { RangeUnavailable, TruncatedInput, LimitExhausted, EndOfInputNotObserved, FeatureDisabled,
    CompanionsNotEnabled, CompanionNotFound, CompanionDeclined, CompanionLimited, CompanionNameUnavailable,
    UnsupportedStructure, DamagedStructure, FurtherSegmentsNotExamined, MediaSegmentWithoutInitializationData,
    SourceFailed, Unrecognized(..) }
  SectionStatus { Complete, Incomplete, NotRequested, NotApplicable, Unrecognized(..) }
  TimingStatus { Established, NotAvailableByDesign, CutShort, NotApplicable, Unrecognized(..) }
}

no_struct_literal! {
  in [cc_media_probe::completeness];
  Completeness,
  HeaderStatus,
  LimitRecord,
  PartReason,
  Reason,
}
