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

//! `AudioInformation`, what an audio stream says about its samples (`api.md` section 6.6; field guide section 3.5).

use super::{ChannelLayout, ImmersiveAudio, SampleFormat};
use crate::Rational;
use crate::form_record::form_record;
use crate::model::Provenance;

form_record! {
  /// What an audio stream says about its samples, from `Stream::audio()`. Present exactly when the stream's kind is
  /// audio; every field inside may still be absent.
  ///
  /// ```
  /// use cc_media_probe::Stream;
  ///
  /// fn audio_summary(stream: &Stream) -> Option<String> {
  ///     let audio = stream.audio()?;
  ///     Some(format!(
  ///         "{:?} Hz ({:?}; output {:?} {:?}), {:?} channels ({:?}), layout of {:?} ({:?}), {:?} bits ({:?}), \
  ///          format {:?}, delay {:?} ({:?}), padding {:?} ({:?}), immersive {:?}",
  ///         audio.sample_rate(),
  ///         audio.sample_rate_provenance(),
  ///         audio.output_sample_rate(),
  ///         audio.output_sample_rate_provenance(),
  ///         audio.channel_count(),
  ///         audio.channel_count_provenance(),
  ///         audio.channel_layout().map(|layout| layout.channels().len()),
  ///         audio.channel_layout_provenance(),
  ///         audio.bit_depth(),
  ///         audio.bit_depth_provenance(),
  ///         audio.sample_format().map(|format| format.encoding().as_str()),
  ///         audio.encoder_delay(),
  ///         audio.encoder_delay_provenance(),
  ///         audio.padding(),
  ///         audio.padding_provenance(),
  ///         audio.immersive().map(|immersive| immersive.kinds().len()),
  ///     ))
  /// }
  /// ```
  #[derive(Clone, Debug, PartialEq, Eq, Hash)]
  pub struct AudioInformation {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    sample_rate: Option<Rational>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    sample_rate_provenance: Option<Provenance>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    output_sample_rate: Option<Rational>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    output_sample_rate_provenance: Option<Provenance>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    channel_count: Option<u64>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    channel_count_provenance: Option<Provenance>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    channel_layout: Option<ChannelLayout>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    channel_layout_provenance: Option<Provenance>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    bit_depth: Option<u64>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    bit_depth_provenance: Option<Provenance>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    sample_format: Option<SampleFormat>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    encoder_delay: Option<u64>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    encoder_delay_provenance: Option<Provenance>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    padding: Option<u64>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    padding_provenance: Option<Provenance>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    immersive: Option<ImmersiveAudio>,
  }
  pairs = [
    sample_rate with sample_rate_provenance,
    output_sample_rate with output_sample_rate_provenance,
    channel_count with channel_count_provenance,
    channel_layout with channel_layout_provenance,
    bit_depth with bit_depth_provenance,
    encoder_delay with encoder_delay_provenance,
    padding with padding_provenance,
  ];
  check = AudioInformation::check_form;
}

impl AudioInformation {
  /// The sample rate, in hertz.
  #[must_use]
  pub fn sample_rate(&self) -> Option<Rational> {
    self.sample_rate
  }

  /// Where the sample rate came from.
  #[must_use]
  pub fn sample_rate_provenance(&self) -> Option<&Provenance> {
    self.sample_rate_provenance.as_ref()
  }

  /// The rate at which the decoder outputs samples, in hertz, where it differs from the coded rate, as with
  /// spectral-band replication.
  #[must_use]
  pub fn output_sample_rate(&self) -> Option<Rational> {
    self.output_sample_rate
  }

  /// Where the output sample rate came from.
  #[must_use]
  pub fn output_sample_rate_provenance(&self) -> Option<&Provenance> {
    self.output_sample_rate_provenance.as_ref()
  }

  /// The channels that the decoder outputs.
  #[must_use]
  pub fn channel_count(&self) -> Option<u64> {
    self.channel_count
  }

  /// Where the channel count came from.
  #[must_use]
  pub fn channel_count_provenance(&self) -> Option<&Provenance> {
    self.channel_count_provenance.as_ref()
  }

  /// Where the channels play.
  #[must_use]
  pub fn channel_layout(&self) -> Option<&ChannelLayout> {
    self.channel_layout.as_ref()
  }

  /// Where the channel layout came from.
  #[must_use]
  pub fn channel_layout_provenance(&self) -> Option<&Provenance> {
    self.channel_layout_provenance.as_ref()
  }

  /// The bits per sample, for uncompressed and floating-point audio.
  #[must_use]
  pub fn bit_depth(&self) -> Option<u64> {
    self.bit_depth
  }

  /// Where the bit depth came from.
  #[must_use]
  pub fn bit_depth_provenance(&self) -> Option<&Provenance> {
    self.bit_depth_provenance.as_ref()
  }

  /// How the samples are stored.
  #[must_use]
  pub fn sample_format(&self) -> Option<&SampleFormat> {
    self.sample_format.as_ref()
  }

  /// The samples per channel that the decoder drops at the start (plan section 8.4).
  #[must_use]
  pub fn encoder_delay(&self) -> Option<u64> {
    self.encoder_delay
  }

  /// Where the encoder delay came from.
  #[must_use]
  pub fn encoder_delay_provenance(&self) -> Option<&Provenance> {
    self.encoder_delay_provenance.as_ref()
  }

  /// The samples per channel that the decoder drops at the end.
  #[must_use]
  pub fn padding(&self) -> Option<u64> {
    self.padding
  }

  /// Where the padding came from.
  #[must_use]
  pub fn padding_provenance(&self) -> Option<&Provenance> {
    self.padding_provenance.as_ref()
  }

  /// The immersive signaling.
  #[must_use]
  pub fn immersive(&self) -> Option<&ImmersiveAudio> {
    self.immersive.as_ref()
  }

  /// Whether the layout has one position for each channel that the decoder outputs, the unassigned ones included,
  /// where both are stated (`api.md` section 6.6), as the fourth review of the third slice's first part asked, which
  /// read a two-channel stream with a layout of one position.
  #[cfg(feature = "serde")]
  fn check_form(&self) -> Result<(), &'static str> {
    let fits = match (self.channel_count, &self.channel_layout) {
      (Some(count), Some(layout)) => u64::try_from(layout.channels().len()).is_ok_and(|positions| positions == count),
      _ => true,
    };
    if fits {
      Ok(())
    } else {
      Err(super::constants::CHANNEL_COUNT_MESSAGE)
    }
  }
}

#[cfg(test)]
mod tests {
  use super::AudioInformation;
  use crate::Rational;
  use crate::model::{ChannelLayout, ImmersiveAudio, Provenance, SampleFormat};

  impl AudioInformation {
    /// A 48 kHz stream of six channels, with every field stated.
    pub(crate) fn sample() -> AudioInformation {
      AudioInformation {
        sample_rate: Rational::new(48_000, 1),
        sample_rate_provenance: Some(Provenance::Declared),
        output_sample_rate: Rational::new(48_000, 1),
        output_sample_rate_provenance: Some(Provenance::Bitstream),
        channel_count: Some(6),
        channel_count_provenance: Some(Provenance::Bitstream),
        channel_layout: Some(ChannelLayout::sample()),
        channel_layout_provenance: Some(Provenance::Bitstream),
        bit_depth: Some(24),
        bit_depth_provenance: Some(Provenance::Declared),
        sample_format: Some(SampleFormat::sample()),
        encoder_delay: Some(312),
        encoder_delay_provenance: Some(Provenance::Bitstream),
        padding: Some(1_024),
        padding_provenance: Some(Provenance::Derived),
        immersive: Some(ImmersiveAudio::sample()),
      }
    }
  }

  #[test]
  fn gives_each_field() {
    let audio = AudioInformation::sample();
    assert_eq!(audio.sample_rate(), Rational::new(48_000, 1));
    assert_eq!(audio.sample_rate_provenance(), Some(&Provenance::Declared));
    assert_eq!(audio.output_sample_rate(), Rational::new(48_000, 1));
    assert_eq!(audio.output_sample_rate_provenance(), Some(&Provenance::Bitstream));
    assert_eq!(audio.channel_count(), Some(6));
    assert_eq!(audio.channel_count_provenance(), Some(&Provenance::Bitstream));
    assert_eq!(audio.channel_layout(), Some(&ChannelLayout::sample()));
    assert_eq!(audio.channel_layout_provenance(), Some(&Provenance::Bitstream));
    assert_eq!(audio.bit_depth(), Some(24));
    assert_eq!(audio.bit_depth_provenance(), Some(&Provenance::Declared));
    assert_eq!(audio.sample_format(), Some(&SampleFormat::sample()));
    assert_eq!(audio.encoder_delay(), Some(312));
    assert_eq!(audio.encoder_delay_provenance(), Some(&Provenance::Bitstream));
    assert_eq!(audio.padding(), Some(1_024));
    assert_eq!(audio.padding_provenance(), Some(&Provenance::Derived));
    assert_eq!(audio.immersive(), Some(&ImmersiveAudio::sample()));
  }
}
