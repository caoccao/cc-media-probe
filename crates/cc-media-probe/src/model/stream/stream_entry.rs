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

//! `Stream`, one stream of the report (`api.md` section 6.3; field guide section 3.2).

use super::{DeclarationRange, Disposition, StreamKind, StreamLanguage, Timecode};
use crate::Seconds;
use crate::completeness::HeaderStatus;
use crate::details::CodecDetails;
use crate::model::{
  AudioInformation, CodecInformation, Estimate, Identifier, Language, LocalizedText, Protection, Provenance, Rational,
  SubtitleInformation, VideoInformation,
};

/// One stream of the report, from `MediaReport::streams()`, in index order.
///
/// ```
/// use cc_media_probe::MediaReport;
///
/// fn list_streams(report: &MediaReport) {
///     for stream in report.streams() {
///         let language = stream.language().map_or("und", |language| language.as_written());
///         println!(
///             "#{} {}: {} [{language}] {}",
///             stream.index(),
///             stream.kind().as_str(),
///             stream.codec(),
///             stream.title().unwrap_or(""),
///         );
///         println!("  header {}", stream.header_status());
///         if let Some(duration) = stream.duration() {
///             println!("  duration {duration} ({:?})", stream.duration_provenance());
///         }
///         if let Some(start) = stream.start_time() {
///             println!("  starts at {start} ({:?})", stream.start_time_provenance());
///         }
///         if let Some(rate) = stream.bit_rate() {
///             println!("  {rate} bit/s ({:?})", stream.bit_rate_provenance());
///         }
///         if let Some(estimate) = stream.estimated_duration() {
///             println!("  about {}", estimate.value());
///         }
///         if let Some(estimate) = stream.estimated_bit_rate() {
///             println!("  about {} bit/s", estimate.value());
///         }
///         println!(
///             "  {:?} packets ({:?}), {:?} frames ({:?}), {:?} samples ({:?})",
///             stream.packet_count(),
///             stream.packet_count_provenance(),
///             stream.frame_count(),
///             stream.frame_count_provenance(),
///             stream.sample_count(),
///             stream.sample_count_provenance(),
///         );
///         println!(
///             "  identifiers {:?} {:?}, embedded in {:?}, {} languages, {} titles, default {:?}",
///             stream.native_identifier().map(|identifier| identifier.as_str()),
///             stream.unique_identifier().map(|identifier| identifier.as_str()),
///             stream.embedded_in(),
///             stream.languages().len(),
///             stream.titles().len(),
///             stream.disposition().is_default(),
///         );
///         println!(
///             "  protected {}, declared {:?}, timecode {:?}, details {}",
///             stream.protection().is_some(),
///             stream.declaration_range().and_then(|range| range.start()),
///             stream.start_timecode().map(|timecode| timecode.to_string()),
///             stream.codec_details().is_some(),
///         );
///         println!(
///             "  video {}, audio {}, subtitle {}",
///             stream.video().is_some(),
///             stream.audio().is_some(),
///             stream.subtitle().is_some(),
///         );
///     }
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Stream {
  index: usize,
  native_identifier: Option<Identifier>,
  unique_identifier: Option<Identifier>,
  kind: StreamKind,
  codec: CodecInformation,
  header_status: HeaderStatus,
  protection: Option<Protection>,
  language: Option<Language>,
  languages: Vec<StreamLanguage>,
  title: Option<String>,
  titles: Vec<LocalizedText>,
  duration: Option<Seconds>,
  duration_provenance: Option<Provenance>,
  start_time: Option<Seconds>,
  start_time_provenance: Option<Provenance>,
  bit_rate: Option<Rational>,
  bit_rate_provenance: Option<Provenance>,
  estimated_duration: Option<Estimate<Seconds>>,
  estimated_bit_rate: Option<Estimate<Rational>>,
  packet_count: Option<u64>,
  packet_count_provenance: Option<Provenance>,
  frame_count: Option<u64>,
  frame_count_provenance: Option<Provenance>,
  sample_count: Option<u64>,
  sample_count_provenance: Option<Provenance>,
  disposition: Disposition,
  declaration_range: Option<DeclarationRange>,
  embedded_in: Option<usize>,
  start_timecode: Option<Timecode>,
  video: Option<VideoInformation>,
  audio: Option<AudioInformation>,
  subtitle: Option<SubtitleInformation>,
  codec_details: Option<CodecDetails>,
}

impl Stream {
  /// The stream's index, its position in `MediaReport::streams()`.
  #[must_use]
  pub fn index(&self) -> usize {
    self.index
  }

  /// The identifier that the container gives the stream, such as a track number; absent for an embedded stream
  /// (F14).
  #[must_use]
  pub fn native_identifier(&self) -> Option<&Identifier> {
    self.native_identifier.as_ref()
  }

  /// An identifier that the file states as unique, such as a Matroska `TrackUID`.
  #[must_use]
  pub fn unique_identifier(&self) -> Option<&Identifier> {
    self.unique_identifier.as_ref()
  }

  /// What the stream carries.
  #[must_use]
  pub fn kind(&self) -> &StreamKind {
    &self.kind
  }

  /// What the stream says about its codec.
  #[must_use]
  pub fn codec(&self) -> &CodecInformation {
    &self.codec
  }

  /// The outcome of header parsing.
  #[must_use]
  pub fn header_status(&self) -> &HeaderStatus {
    &self.header_status
  }

  /// What the stream declares protected (F22); absent only when every structure that can declare protection was
  /// examined and none does.
  #[must_use]
  pub fn protection(&self) -> Option<&Protection> {
    self.protection.as_ref()
  }

  /// The primary language.
  #[must_use]
  pub fn language(&self) -> Option<&Language> {
    self.language.as_ref()
  }

  /// Every language of the stream, with its roles (F24).
  #[must_use]
  pub fn languages(&self) -> &[StreamLanguage] {
    &self.languages
  }

  /// The title.
  #[must_use]
  pub fn title(&self) -> Option<&str> {
    self.title.as_deref()
  }

  /// Every title, with the languages and countries it applies to.
  #[must_use]
  pub fn titles(&self) -> &[LocalizedText] {
    &self.titles
  }

  /// The duration.
  #[must_use]
  pub fn duration(&self) -> Option<Seconds> {
    self.duration
  }

  /// Where the duration came from.
  #[must_use]
  pub fn duration_provenance(&self) -> Option<&Provenance> {
    self.duration_provenance.as_ref()
  }

  /// The start time, an offset from the report's zero.
  #[must_use]
  pub fn start_time(&self) -> Option<Seconds> {
    self.start_time
  }

  /// Where the start time came from.
  #[must_use]
  pub fn start_time_provenance(&self) -> Option<&Provenance> {
    self.start_time_provenance.as_ref()
  }

  /// The bit rate, in bits per second.
  #[must_use]
  pub fn bit_rate(&self) -> Option<Rational> {
    self.bit_rate
  }

  /// Where the bit rate came from.
  #[must_use]
  pub fn bit_rate_provenance(&self) -> Option<&Provenance> {
    self.bit_rate_provenance.as_ref()
  }

  /// An estimate of the duration, apart from `duration()` (R25).
  #[must_use]
  pub fn estimated_duration(&self) -> Option<&Estimate<Seconds>> {
    self.estimated_duration.as_ref()
  }

  /// An estimate of the bit rate, apart from `bit_rate()` (R25).
  #[must_use]
  pub fn estimated_bit_rate(&self) -> Option<&Estimate<Rational>> {
    self.estimated_bit_rate.as_ref()
  }

  /// The number of packets, established for the whole stream.
  #[must_use]
  pub fn packet_count(&self) -> Option<u64> {
    self.packet_count
  }

  /// Where the packet count came from.
  #[must_use]
  pub fn packet_count_provenance(&self) -> Option<&Provenance> {
    self.packet_count_provenance.as_ref()
  }

  /// The number of frames, established for the whole stream.
  #[must_use]
  pub fn frame_count(&self) -> Option<u64> {
    self.frame_count
  }

  /// Where the frame count came from.
  #[must_use]
  pub fn frame_count_provenance(&self) -> Option<&Provenance> {
    self.frame_count_provenance.as_ref()
  }

  /// The number of samples per channel, established for the whole stream.
  #[must_use]
  pub fn sample_count(&self) -> Option<u64> {
    self.sample_count
  }

  /// Where the sample count came from.
  #[must_use]
  pub fn sample_count_provenance(&self) -> Option<&Provenance> {
    self.sample_count_provenance.as_ref()
  }

  /// The stream's flags.
  #[must_use]
  pub fn disposition(&self) -> &Disposition {
    &self.disposition
  }

  /// Where the stream's declaration lies.
  #[must_use]
  pub fn declaration_range(&self) -> Option<&DeclarationRange> {
    self.declaration_range.as_ref()
  }

  /// The index of the stream that carries this one, for an embedded stream (F14).
  #[must_use]
  pub fn embedded_in(&self) -> Option<usize> {
    self.embedded_in
  }

  /// The start timecode (F23).
  #[must_use]
  pub fn start_timecode(&self) -> Option<&Timecode> {
    self.start_timecode.as_ref()
  }

  /// What a video stream says about its pictures; present exactly when the kind is video.
  #[must_use]
  pub fn video(&self) -> Option<&VideoInformation> {
    self.video.as_ref()
  }

  /// What an audio stream says about its samples; present exactly when the kind is audio.
  #[must_use]
  pub fn audio(&self) -> Option<&AudioInformation> {
    self.audio.as_ref()
  }

  /// What a subtitle stream says about its events; present exactly when the kind is subtitle.
  #[must_use]
  pub fn subtitle(&self) -> Option<&SubtitleInformation> {
    self.subtitle.as_ref()
  }

  /// The codec header's typed details (`api.md` section 11).
  #[must_use]
  pub fn codec_details(&self) -> Option<&CodecDetails> {
    self.codec_details.as_ref()
  }
}

#[cfg(test)]
mod tests {
  use super::Stream;
  use crate::completeness::HeaderStatus;
  use crate::details::{CodecDetails, UnrecognizedDetails};
  use crate::model::{
    AudioInformation, CodecInformation, DeclarationRange, Disposition, Estimate, EstimateMethod, Identifier, Language,
    LanguageCodeSystem, LocalizedText, Protection, Provenance, Rational, StreamKind, StreamLanguage,
    SubtitleInformation, Timecode, VideoInformation,
  };
  use crate::{Codec, Seconds};

  impl Stream {
    /// An H.264 stream of the first track of a Matroska file, at this index, with every field stated.
    pub(crate) fn video_sample(index: usize) -> Stream {
      let english = Language::sample("eng", Some("en"), LanguageCodeSystem::Iso639Alpha3);
      Stream {
        index,
        native_identifier: Some(Identifier::sample("1")),
        unique_identifier: Some(Identifier::sample("12345")),
        kind: StreamKind::Video,
        codec: CodecInformation::sample(),
        header_status: HeaderStatus::parsed(),
        protection: Some(Protection::matroska_sample()),
        language: Some(english.clone()),
        languages: vec![StreamLanguage::sample(english.clone(), Vec::new())],
        title: Some("Main".to_owned()),
        titles: vec![LocalizedText::sample("Main", vec![english], &[])],
        duration: Some(Seconds::sample(5_400, 1)),
        duration_provenance: Some(Provenance::Declared),
        start_time: Some(Seconds::sample(0, 1)),
        start_time_provenance: Some(Provenance::Declared),
        bit_rate: Rational::new(5_000_000, 1),
        bit_rate_provenance: Some(Provenance::Derived),
        estimated_duration: Some(Estimate::sample(
          Seconds::sample(5_401, 1),
          EstimateMethod::EndpointTimestamps,
        )),
        estimated_bit_rate: Some(Estimate::sample(
          Rational::new(4_999_000, 1).unwrap(),
          EstimateMethod::ConstantBitRateOverSize,
        )),
        packet_count: Some(129_600),
        packet_count_provenance: Some(Provenance::Declared),
        frame_count: Some(129_600),
        frame_count_provenance: Some(Provenance::Derived),
        sample_count: None,
        sample_count_provenance: None,
        disposition: Disposition::default_stream(),
        declaration_range: Some(DeclarationRange::sample()),
        embedded_in: None,
        start_timecode: Some(Timecode::sample(90_000, 25, false, false)),
        video: Some(VideoInformation::sample()),
        audio: None,
        subtitle: None,
        codec_details: Some(CodecDetails::Unrecognized(UnrecognizedDetails::sample("h264"))),
      }
    }

    /// An Opus stream of the second track of the same Matroska file, at this index, with its own track number and
    /// `TrackUID`, and without a timecode or a frame count.
    pub(crate) fn audio_sample(index: usize) -> Stream {
      Stream {
        native_identifier: Some(Identifier::sample("2")),
        unique_identifier: Some(Identifier::sample("67890")),
        kind: StreamKind::Audio,
        codec: CodecInformation::opus_sample(),
        bit_rate: Rational::new(256_000, 1),
        estimated_bit_rate: None,
        packet_count: Some(270_000),
        frame_count: None,
        frame_count_provenance: None,
        sample_count: Some(259_200_000),
        sample_count_provenance: Some(Provenance::Declared),
        start_timecode: None,
        video: None,
        audio: Some(AudioInformation::sample()),
        codec_details: Some(CodecDetails::Unrecognized(UnrecognizedDetails::sample("opus"))),
        ..Stream::video_sample(index)
      }
    }

    /// A WebVTT stream of the third track of the same Matroska file, at this index, with its own track number and
    /// `TrackUID`.
    pub(crate) fn subtitle_sample(index: usize) -> Stream {
      Stream {
        native_identifier: Some(Identifier::sample("3")),
        unique_identifier: Some(Identifier::sample("13579")),
        kind: StreamKind::Subtitle,
        codec: CodecInformation::webvtt_sample(),
        bit_rate: None,
        bit_rate_provenance: None,
        estimated_bit_rate: None,
        packet_count: Some(12),
        frame_count: None,
        frame_count_provenance: None,
        start_timecode: None,
        video: None,
        subtitle: Some(SubtitleInformation::sample()),
        codec_details: None,
        ..Stream::video_sample(index)
      }
    }
  }

  #[test]
  fn gives_each_field_of_a_video_stream() {
    let stream = Stream::video_sample(0);
    assert_eq!(stream.index(), 0);
    assert_eq!(stream.native_identifier().map(Identifier::as_str), Some("1"));
    assert_eq!(stream.unique_identifier().map(Identifier::as_str), Some("12345"));
    assert_eq!(stream.kind(), &StreamKind::Video);
    assert_eq!(stream.codec(), &CodecInformation::sample());
    assert!(stream.header_status().is_parsed());
    assert!(stream.protection().is_some());
    assert_eq!(stream.language().and_then(Language::tag), Some("en"));
    assert_eq!(stream.languages().len(), 1);
    assert_eq!(stream.title(), Some("Main"));
    assert_eq!(stream.titles().len(), 1);
    assert_eq!(stream.duration(), Some(Seconds::sample(5_400, 1)));
    assert_eq!(stream.duration_provenance(), Some(&Provenance::Declared));
    assert_eq!(stream.start_time(), Some(Seconds::sample(0, 1)));
    assert_eq!(stream.start_time_provenance(), Some(&Provenance::Declared));
    assert_eq!(stream.bit_rate(), Rational::new(5_000_000, 1));
    assert_eq!(stream.bit_rate_provenance(), Some(&Provenance::Derived));
    assert_eq!(
      stream.estimated_duration().map(Estimate::value),
      Some(Seconds::sample(5_401, 1))
    );
    assert_eq!(
      stream.estimated_bit_rate().map(Estimate::value),
      Rational::new(4_999_000, 1)
    );
    assert_eq!(stream.packet_count(), Some(129_600));
    assert_eq!(stream.packet_count_provenance(), Some(&Provenance::Declared));
    assert_eq!(stream.frame_count(), Some(129_600));
    assert_eq!(stream.frame_count_provenance(), Some(&Provenance::Derived));
    assert_eq!(stream.sample_count(), None);
    assert_eq!(stream.sample_count_provenance(), None);
    assert_eq!(stream.disposition().is_default(), Some(true));
    assert!(stream.declaration_range().is_some());
    assert_eq!(stream.embedded_in(), None);
    assert_eq!(
      stream.start_timecode().map(ToString::to_string),
      Some("01:00:00:00".to_owned())
    );
    assert!(stream.video().is_some());
    assert!(stream.audio().is_none());
    assert!(stream.subtitle().is_none());
    assert!(stream.codec_details().is_some());
  }

  #[test]
  fn carries_the_information_of_its_kind() {
    let audio = Stream::audio_sample(1);
    assert_eq!(audio.kind(), &StreamKind::Audio);
    assert!(audio.audio().is_some() && audio.video().is_none());
    assert_eq!(audio.codec().codec(), Some(&Codec::Opus));
    assert_eq!(audio.sample_count(), Some(259_200_000));
    assert_eq!((audio.frame_count(), audio.start_timecode()), (None, None));
    let subtitle = Stream::subtitle_sample(2);
    assert!(subtitle.subtitle().is_some());
    assert_eq!(subtitle.codec().codec(), Some(&Codec::WebVtt));
    assert_eq!(subtitle.embedded_in(), None);
  }

  #[test]
  fn gives_each_track_its_own_identifiers() {
    let streams = [
      Stream::video_sample(0),
      Stream::audio_sample(1),
      Stream::subtitle_sample(2),
    ];
    let numbers: Vec<_> = streams.iter().filter_map(Stream::native_identifier).collect();
    let unique: Vec<_> = streams.iter().filter_map(Stream::unique_identifier).collect();
    assert_eq!(
      numbers.iter().map(|number| number.as_str()).collect::<Vec<_>>(),
      ["1", "2", "3"]
    );
    assert_eq!(
      unique.iter().map(|identifier| identifier.as_str()).collect::<Vec<_>>(),
      ["12345", "67890", "13579"]
    );
    for stream in &streams {
      assert_eq!(
        stream.codec().codec().and_then(Codec::kind),
        Some(stream.kind().clone())
      );
    }
  }

  #[test]
  fn names_the_stream_that_carries_an_embedded_one() {
    // The audio of a type-1 DV stream in AVI is carried in the video stream and has no identifier of its own (F14).
    let embedded = Stream {
      embedded_in: Some(0),
      native_identifier: None,
      unique_identifier: None,
      ..Stream::audio_sample(1)
    };
    assert_eq!(embedded.embedded_in(), Some(0));
    assert_eq!(embedded.native_identifier(), None);
  }
}
