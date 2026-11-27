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

//! The traits of the public types that `api.md` section 3.3 states, asserted at compile time as a crate that
//! depends on the library sees them (C-COMMON-TRAITS, C-SEND-SYNC, C-GOOD-ERR, C-CTOR, C-DEBUG). A bound that a
//! type lacks stops this test from building.

use std::error::Error as StandardError;
use std::fmt::{Debug, Display};
use std::hash::Hash;

use cc_media_probe::{
  Codec, Error, Format, Limit, Limits, Prober, completeness, details, diagnostics, error, model, source,
};

/// A type of the report model: cloned, compared exactly, hashed, debugged, and shared between threads, which a
/// spawned thread can own since it borrows nothing.
fn model_type<Value: Clone + PartialEq + Eq + Hash + Debug + Send + Sync + 'static>() {}

/// A value type that an accessor returns by value.
fn copied<Value: Copy>() {}

/// A type with one order that a caller can rely on.
fn ordered<Value: Ord>() {}

/// A type with a line of text for people.
fn displayed<Value: Display>() {}

/// An error that a caller can pass on with `?` into any error type and send between threads.
fn error_type<Value: StandardError + Send + Sync + 'static>() {}

/// A type that is cloned, debugged and shared, without the comparisons of a model type.
fn shared<Value: Clone + Debug + Send + Sync>() {}

/// A type that a spawned thread can own.
fn owned<Value: Send + 'static>() {}

/// Asserts that each type is a model type.
macro_rules! model_types {
  ($($type:ty),* $(,)?) => {
    $(model_type::<$type>();)*
  };
}

#[test]
fn every_model_type_is_cloned_compared_hashed_and_shared() {
  use details::{FormatDetails, StoredValue, UnrecognizedDetails};
  use model::{Estimate, Seconds, StreamKind, TagValue, UnrecognizedKind, UnrecognizedValue};

  model_types! {
    Codec, Format, Limit, Limits,
  }
  model_types! {
    completeness::CompanionStatus, completeness::Completeness, completeness::DetectionStatus,
    completeness::DiscoveryStatus, completeness::Feature, completeness::HeaderState, completeness::HeaderStatus,
    completeness::HeadersStatus, completeness::Identification, completeness::LimitRecord, completeness::Part,
    completeness::PartReason, completeness::Reason, completeness::ReasonKind, completeness::SectionStatus,
    completeness::TimingStatus,
  }
  model_types! {
    details::CodecDetails, details::Conflict, details::ConflictingValue, details::ExactInput, details::ExactValue,
    details::FormatDetails, details::ObservedBound, details::ObservedValue, StoredValue<bool>,
    UnrecognizedDetails<FormatDetails>,
  }
  model_types! {
    diagnostics::Diagnostic, diagnostics::DiagnosticCode, diagnostics::Severity,
  }
  model_types! {
    error::CandidateCheck, error::CheckState, error::ErrorKind, error::ErrorRecord, error::Observations,
  }
  model_types! {
    model::AmbisonicChannelOrdering, model::AmbisonicNormalization, model::Ambisonics, model::Attachment,
    model::AttachmentHandling, model::AttachmentRole, model::AudioInformation, model::BinaryValue, model::BitDepth,
    model::ByteOrder, model::Candidate, model::ChannelLayout, model::ChannelPosition, model::Chapter,
    model::ChapterEdition, model::ChapterOrigin, model::ChromaFormat, model::ChromaSubsampling, model::Chromaticity,
    model::CodecCandidate, model::CodecInformation, model::CodecNamespace, model::ColorDescription, model::ColorRange,
    model::Component, model::ComponentDepth, model::Confidence, model::ContentLightLevel, model::DeclarationRange,
    model::Detection, model::DetectionOutcome, model::Disposition, model::DynamicRange, model::DynamicRangeMark,
    model::DynamicRangeTransfer, model::EncodingName, model::EntityKind, model::EntityLocation, Estimate<Seconds>,
    model::EstimateMethod, model::ExternalReference, model::FieldOrder, model::FloatingPointValue,
    model::FloatingPointWidth, model::FrameRateMode, model::Guid, model::Hint, model::HintOrigin, model::Identifier,
    model::IdentifierNamespace, model::ImmersiveAudio, model::ImmersiveKind, model::IntegerValue, model::InventoryClass,
    model::Language, model::LanguageCodeSystem, model::LanguageRole, model::Level, model::LocalizedText,
    model::MapState, model::MasteringDisplay, model::MediaReport, model::MediaType, model::NativeScope, model::Profile,
    model::Program, model::Projection, model::ProjectionKind, model::Protection, model::ProtectionDeclaration,
    model::ProtectionScheme, model::ProtectionSchemeNamespace, model::ProtectionScope, model::Provenance,
    model::Rational, model::RawCodecIdentifier, model::ReferenceKind, model::ReferenceOwner, model::ReferenceRole,
    model::Rotation, model::SampleDescriptionProtection, model::SampleDescriptionState, model::SampleEncoding,
    model::SampleFormat, model::ScanType, model::Seconds, model::SectionRequest, model::SegmentReference, model::Size,
    model::Speaker, model::StereoLayout, model::StereoMode, model::Stream, model::StreamKind, model::StreamLanguage,
    model::SubtitleInformation, model::SubtitleKind, model::SupportLevel, model::Tag, model::TagKey, model::TagName,
    model::TagSet, model::TagValue, model::TargetLevel, model::TargetReference, model::TargetReferenceKind,
    model::TargetScope, model::TextEncoding, model::Tier, model::TimeRange, model::Timecode, model::TimecodeLabel,
    UnrecognizedKind<TagValue>, UnrecognizedValue<StreamKind>, model::Usage, model::UtcTime, model::VideoInformation,
  }
  model_types! {
    source::AttemptedIdentity, source::PortablePath, source::RawPath, source::RawPathEncoding, source::SourceEntry,
    source::SourceIdentifier, source::SourceOffset, source::SourceOutcome, source::SourceRole,
  }
}

#[cfg(feature = "serde")]
#[test]
fn every_type_of_the_document_is_a_model_type() {
  use cc_media_probe::document;

  model_types! {
    document::DeriveOutcomeError, document::Document, document::DocumentEntry, document::EntryOutcome,
    document::Generator, document::SchemaVersion, document::UsageError,
  }
}

#[test]
fn the_value_types_that_accessors_return_by_value_are_copy() {
  copied::<model::Rational>();
  copied::<model::Seconds>();
  copied::<model::Size>();
  copied::<model::TimeRange>();
  copied::<model::UtcTime>();
  copied::<model::Guid>();
  copied::<model::Confidence>();
  copied::<model::TimecodeLabel>();
  copied::<source::SourceIdentifier>();
}

#[cfg(feature = "serde")]
#[test]
fn the_schema_version_is_copy_and_ordered() {
  copied::<cc_media_probe::document::SchemaVersion>();
  ordered::<cc_media_probe::document::SchemaVersion>();
}

#[test]
fn the_values_with_one_order_are_ordered() {
  ordered::<model::Rational>();
  ordered::<model::Seconds>();
  ordered::<model::UtcTime>();
  ordered::<model::Confidence>();
  ordered::<source::SourceIdentifier>();
}

#[test]
fn the_options_have_a_default_that_new_gives() {
  assert_eq!(Limits::new(), Limits::default());
  // `Prober` implements `Clone` only, so its debug text stands in for a comparison.
  assert_eq!(format!("{:?}", Prober::new()), format!("{:?}", Prober::default()));
}

#[test]
fn the_types_for_people_display_a_line() {
  displayed::<Format>();
  displayed::<Codec>();
  displayed::<model::CodecInformation>();
  displayed::<model::CodecCandidate>();
  displayed::<model::Confidence>();
  displayed::<model::IntegerValue>();
  displayed::<model::Rational>();
  displayed::<model::Seconds>();
  displayed::<model::Size>();
  displayed::<model::MediaType>();
  displayed::<model::Language>();
  displayed::<model::Identifier>();
  displayed::<model::Profile>();
  displayed::<model::Level>();
  displayed::<model::Tier>();
  displayed::<model::Timecode>();
  displayed::<model::Guid>();
  displayed::<source::SourceIdentifier>();
  displayed::<completeness::HeaderStatus>();
  displayed::<completeness::Completeness>();
  displayed::<diagnostics::DiagnosticCode>();
  displayed::<Error>();
  displayed::<error::ErrorRecord>();
}

#[test]
fn the_errors_are_errors_that_threads_share() {
  error_type::<Error>();
}

#[cfg(feature = "serde")]
#[test]
fn a_refused_outcome_is_an_error_that_threads_share() {
  error_type::<cc_media_probe::document::DeriveOutcomeError>();
  displayed::<cc_media_probe::document::DeriveOutcomeError>();
}

#[cfg(feature = "serde")]
#[test]
fn the_completeness_statuses_are_returned_by_reference() -> Result<(), serde_json::Error> {
  use completeness::{Completeness, DetectionStatus, DiscoveryStatus, SectionStatus};

  // Each status is a stored enum with a catch-all, so its getter borrows it (`api.md` section 1, rule 7), and a
  // match on what it returns needs no dereference.
  let completeness: Completeness = serde_json::from_str(
    r#"{"detection": "complete", "discovery": "complete", "headers": "complete", "tags": "complete",
    "chapters": "complete", "attachments": "complete", "timing": "established", "companions": "not_applicable",
    "reasons": [], "record": {"exhausted": [], "first_stopped": [], "first_reasons": [],
    "omitted_diagnostics": "0", "omitted_declarations": "0", "omitted_reasons": "0", "omitted_references": "0"}}"#,
  )?;
  let detection: &DetectionStatus = completeness.detection();
  let discovery: &DiscoveryStatus = completeness.discovery();
  let sections: [&SectionStatus; 3] = [completeness.tags(), completeness.chapters(), completeness.attachments()];
  assert!(matches!(detection, DetectionStatus::Complete));
  assert!(matches!(discovery, DiscoveryStatus::Complete));
  assert!(sections.iter().all(|status| matches!(status, SectionStatus::Complete)));
  Ok(())
}

#[test]
fn the_prober_its_handles_and_the_lookups_are_shared() {
  owned::<Prober>();
  shared::<Prober>();
  shared::<cc_media_probe::asynchronous::AsynchronousProber<'static>>();
  shared::<model::FileTags<'static>>();
  shared::<model::StreamTags<'static>>();
  shared::<model::TagsWithKey<'static>>();
}

#[cfg(feature = "futures-io")]
#[test]
fn the_futures_handle_is_shared() {
  shared::<cc_media_probe::asynchronous::futures::FuturesProber<'static>>();
}

#[cfg(feature = "tokio")]
#[test]
fn the_tokio_handle_is_shared() {
  shared::<cc_media_probe::asynchronous::tokio::TokioProber<'static>>();
}

/// Asserts that each type has a serialized form of its own.
#[cfg(feature = "serde")]
macro_rules! serialized_types {
  ($($type:ty),* $(,)?) => {
    $(serialized::<$type>();)*
  };
}

/// A type with a serialized form of its own, which reads by the strict rules of the JSON contract.
#[cfg(feature = "serde")]
fn serialized<Value: serde::Serialize + serde::de::DeserializeOwned>() {}

/// The types that have no form of their own are the seven of `api.md` section 3.3, the options and the refusal of
/// an outcome; every other model type, and every root record, reads and writes its form (C-SERDE).
#[cfg(feature = "serde")]
#[test]
fn every_type_with_a_form_reads_and_writes_it() {
  use cc_media_probe::document;
  use details::StoredValue;
  use model::{Estimate, Seconds};

  serialized_types! {
    Codec, Format, Limit, completeness::CompanionStatus, completeness::Completeness, completeness::DetectionStatus,
    completeness::DiscoveryStatus, completeness::Feature, completeness::HeaderState, completeness::HeaderStatus,
    completeness::HeadersStatus, completeness::Identification, completeness::LimitRecord, completeness::Part,
    completeness::PartReason, completeness::Reason, completeness::ReasonKind, completeness::SectionStatus,
    completeness::TimingStatus, details::CodecDetails, details::Conflict, details::ConflictingValue,
    details::ExactInput, details::ExactValue, details::FormatDetails, details::ObservedBound, details::ObservedValue,
    StoredValue<bool>, diagnostics::Diagnostic, diagnostics::DiagnosticCode, diagnostics::Severity, document::Document,
    document::EntryOutcome, document::Generator, document::SchemaVersion, document::UsageError, error::CandidateCheck,
    error::CheckState, error::ErrorKind, error::ErrorRecord, error::Observations, model::AmbisonicChannelOrdering,
    model::AmbisonicNormalization, model::Ambisonics, model::Attachment, model::AttachmentRole, model::AudioInformation,
    model::BinaryValue, model::BitDepth, model::ByteOrder, model::Candidate, model::ChannelLayout,
    model::ChannelPosition, model::Chapter, model::ChapterEdition, model::ChapterOrigin, model::ChromaFormat,
    model::ChromaSubsampling, model::Chromaticity, model::CodecCandidate, model::CodecInformation,
    model::CodecNamespace, model::ColorDescription, model::ColorRange, model::Component, model::ComponentDepth,
    model::Confidence, model::ContentLightLevel, model::DeclarationRange, model::Detection, model::DetectionOutcome,
    model::Disposition, model::DynamicRange, model::DynamicRangeMark, model::DynamicRangeTransfer, model::EncodingName,
    model::EntityKind, model::EntityLocation, Estimate<Seconds>, model::EstimateMethod, model::ExternalReference,
    model::FieldOrder, model::FloatingPointValue, model::FloatingPointWidth, model::FrameRateMode, model::Guid,
    model::Hint, model::HintOrigin, model::Identifier, model::IdentifierNamespace, model::ImmersiveAudio,
    model::ImmersiveKind, model::InventoryClass, model::Language, model::LanguageCodeSystem, model::LanguageRole,
    model::Level, model::LocalizedText, model::MapState, model::MasteringDisplay, model::MediaReport, model::MediaType,
    model::NativeScope, model::Profile, model::Program, model::Projection, model::ProjectionKind, model::Protection,
    model::ProtectionDeclaration, model::ProtectionScheme, model::ProtectionSchemeNamespace, model::ProtectionScope,
    model::Provenance, model::Rational, model::RawCodecIdentifier, model::ReferenceKind, model::ReferenceOwner,
    model::ReferenceRole, model::Rotation, model::SampleDescriptionProtection, model::SampleDescriptionState,
    model::SampleEncoding, model::SampleFormat, model::ScanType, model::Seconds, model::SegmentReference, model::Size,
    model::Speaker, model::StereoLayout, model::StereoMode, model::Stream, model::StreamKind, model::StreamLanguage,
    model::SubtitleInformation, model::SubtitleKind, model::SupportLevel, model::Tag, model::TagKey, model::TagName,
    model::TagSet, model::TagValue, model::TargetLevel, model::TargetReference, model::TargetReferenceKind,
    model::TargetScope, model::TextEncoding, model::Tier, model::TimeRange, model::Timecode, model::Usage,
    model::UtcTime, model::VideoInformation, source::AttemptedIdentity, source::RawPath, source::RawPathEncoding,
    source::SourceEntry, source::SourceIdentifier, source::SourceOffset, source::SourceOutcome, source::SourceRole,
  }
}
