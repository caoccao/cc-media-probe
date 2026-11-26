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

//! `Timecode` and `TimecodeLabel`, a stream's start timecode (`api.md` section 6.3; F23).

use std::fmt;

use super::constants::{
  DROP_FRAME_BASE_RATE, DROP_FRAME_SEPARATOR, DROPPED_PER_MINUTE_AT_BASE_RATE, FRAME_SEPARATOR, MINUTES_PER_DROP_CYCLE,
  NEGATIVE_SIGN, NO_SIGN, SECONDS_PER_DAY, SIXTY,
};
use crate::Rational;
use crate::form_record::form_record;

form_record! {
  /// A stream's start timecode (F23): a frame number counted at a nominal whole rate of at least 1 frame a second,
  /// with the counting rules that give its label.
  ///
  /// `Display` writes the label, such as `01:00:00:00`, with `;` before the frames of a drop-frame timecode.
  ///
  /// ```
  /// use cc_media_probe::Stream;
  ///
  /// fn start_timecode(stream: &Stream) -> Option<String> {
  ///     let timecode = stream.start_timecode()?;
  ///     let label = timecode.label();
  ///     Some(format!(
  ///         "{timecode}: frame {} at {} fps ({}), {}{} h {} min {} s {} frames{}",
  ///         timecode.frame_number(),
  ///         timecode.rate(),
  ///         timecode.nominal_frames_per_second(),
  ///         if label.is_negative() { "-" } else { "" },
  ///         label.hours(),
  ///         label.minutes(),
  ///         label.seconds(),
  ///         label.frames(),
  ///         if timecode.is_drop_frame() { ", drop frame" } else { "" },
  ///     ))
  /// }
  ///
  /// fn wraps(stream: &Stream) -> bool {
  ///     stream.start_timecode().is_some_and(|timecode| timecode.wraps_at_24_hours())
  /// }
  /// ```
  #[derive(Clone, Debug, PartialEq, Eq, Hash)]
  pub struct Timecode {
    frame_number: i64,
    rate: Rational,
    nominal_frames_per_second: u64,
    drop_frame: bool,
    wraps_at_24_hours: bool,
  }
  check = Timecode::check_form;
}

impl Timecode {
  /// The frame number that the label counts, negative where the format allows negative timecodes.
  #[must_use]
  pub fn frame_number(&self) -> i64 {
    self.frame_number
  }

  /// The real rate of the frames, such as 30000/1001.
  #[must_use]
  pub fn rate(&self) -> Rational {
    self.rate
  }

  /// The whole rate at which the label counts frames, such as 30 for 30000/1001. It is at least 1: a source that
  /// declares a nominal rate of 0 has no start timecode, and reading refuses a timecode whose rate is 0.
  #[must_use]
  pub fn nominal_frames_per_second(&self) -> u64 {
    self.nominal_frames_per_second
  }

  /// Whether the label skips frame numbers to keep pace with a rate below the nominal one: two each minute at 30
  /// frames per second, and as many more at a multiple of it, except in every tenth minute.
  #[must_use]
  pub fn is_drop_frame(&self) -> bool {
    self.drop_frame
  }

  /// Whether the label starts again from zero after 24 hours.
  #[must_use]
  pub fn wraps_at_24_hours(&self) -> bool {
    self.wraps_at_24_hours
  }

  /// The label of the frame number: hours, minutes, seconds and frames, after the drop-frame and wrapping rules.
  #[must_use]
  pub fn label(&self) -> TimecodeLabel {
    let nominal = self.nominal_frames_per_second;
    let mut count = self.frame_number.unsigned_abs();
    if self.drop_frame {
      count = with_dropped_numbers(count, nominal);
    }
    if self.wraps_at_24_hours {
      count = count
        .checked_rem(nominal.saturating_mul(SECONDS_PER_DAY))
        .unwrap_or(count);
    }
    let total_seconds = count.checked_div(nominal).unwrap_or_default();
    let minutes = total_seconds.checked_div(SIXTY).unwrap_or_default();
    TimecodeLabel {
      is_negative: self.frame_number < 0,
      hours: minutes.checked_div(SIXTY).unwrap_or_default(),
      minutes: below_sixty(minutes),
      seconds: below_sixty(total_seconds),
      frames: count.checked_rem(nominal).unwrap_or_default(),
    }
  }

  /// Whether the nominal rate is at least 1, since a source that declares a rate of 0 has no start timecode
  /// (`json-output.md` section 7.3).
  #[cfg(feature = "serde")]
  fn check_form(&self) -> Result<(), &'static str> {
    if self.nominal_frames_per_second >= 1 {
      Ok(())
    } else {
      Err(super::constants::TIMECODE_MESSAGE)
    }
  }
}

/// The frame count with the numbers that drop-frame counting skips added back, so that the label counts as if no
/// number were skipped.
fn with_dropped_numbers(count: u64, nominal: u64) -> u64 {
  if !nominal.is_multiple_of(DROP_FRAME_BASE_RATE) {
    return count;
  }
  let dropped = nominal
    .checked_div(DROP_FRAME_BASE_RATE)
    .unwrap_or_default()
    .saturating_mul(DROPPED_PER_MINUTE_AT_BASE_RATE);
  let per_minute = nominal.saturating_mul(SIXTY).saturating_sub(dropped);
  let full_minutes = MINUTES_PER_DROP_CYCLE.saturating_sub(1);
  let per_cycle = per_minute
    .saturating_mul(MINUTES_PER_DROP_CYCLE)
    .saturating_add(dropped);
  let cycles = count.checked_div(per_cycle).unwrap_or_default();
  let rest = count.checked_rem(per_cycle).unwrap_or_default();
  let skipped_in_rest = if rest > dropped {
    rest
      .saturating_sub(dropped)
      .checked_div(per_minute)
      .unwrap_or_default()
      .saturating_mul(dropped)
  } else {
    0
  };
  count
    .saturating_add(cycles.saturating_mul(full_minutes).saturating_mul(dropped))
    .saturating_add(skipped_in_rest)
}

/// The value modulo sixty, which always fits a `u8`.
fn below_sixty(value: u64) -> u8 {
  u8::try_from(value.checked_rem(SIXTY).unwrap_or_default()).unwrap_or_default()
}

impl fmt::Display for Timecode {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    let label = self.label();
    let sign = if label.is_negative { NEGATIVE_SIGN } else { NO_SIGN };
    let separator = if self.drop_frame {
      DROP_FRAME_SEPARATOR
    } else {
      FRAME_SEPARATOR
    };
    write!(
      formatter,
      "{sign}{:02}:{:02}:{:02}{separator}{:02}",
      label.hours, label.minutes, label.seconds, label.frames
    )
  }
}

/// The label of a timecode, as `Timecode::label()` computes it. It has no serialized form of its own (`api.md` section
/// 3.3), as the maintainer chose at the second review of the third slice's first part: it is computed from a
/// `Timecode`, which has one, and no record holds a label.
///
/// ```
/// use cc_media_probe::model::Timecode;
///
/// fn is_on_the_hour(timecode: &Timecode) -> bool {
///     let label = timecode.label();
///     !label.is_negative() && label.minutes() == 0 && label.seconds() == 0 && label.frames() == 0
/// }
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TimecodeLabel {
  is_negative: bool,
  hours: u64,
  minutes: u8,
  seconds: u8,
  frames: u64,
}

impl TimecodeLabel {
  /// Whether the label is negative.
  #[must_use]
  pub fn is_negative(&self) -> bool {
    self.is_negative
  }

  /// The hours, below 24 for a timecode that wraps at 24 hours.
  #[must_use]
  pub fn hours(&self) -> u64 {
    self.hours
  }

  /// The minutes, below 60.
  #[must_use]
  pub fn minutes(&self) -> u8 {
    self.minutes
  }

  /// The seconds, below 60.
  #[must_use]
  pub fn seconds(&self) -> u8 {
    self.seconds
  }

  /// The frames, below the nominal rate.
  #[must_use]
  pub fn frames(&self) -> u64 {
    self.frames
  }
}

#[cfg(test)]
mod tests {
  use super::Timecode;
  use crate::Rational;

  impl Timecode {
    /// A timecode at this frame number and nominal rate, which is at least 1, as every timecode's is.
    pub(crate) fn sample(frame_number: i64, nominal: u64, drop_frame: bool, wraps_at_24_hours: bool) -> Timecode {
      assert!(nominal >= 1, "a timecode's nominal rate is at least 1");
      Timecode {
        frame_number,
        rate: if drop_frame {
          Rational::new(i64::try_from(nominal).unwrap().checked_mul(1_000).unwrap(), 1_001).unwrap()
        } else {
          Rational::new(i64::try_from(nominal).unwrap(), 1).unwrap()
        },
        nominal_frames_per_second: nominal,
        drop_frame,
        wraps_at_24_hours,
      }
    }
  }

  #[test]
  fn labels_a_count_of_whole_frames() {
    let hour = Timecode::sample(90_000, 25, false, false);
    assert_eq!(hour.to_string(), "01:00:00:00");
    assert_eq!(hour.frame_number(), 90_000);
    assert_eq!(hour.rate(), Rational::new(25, 1).unwrap());
    assert_eq!(hour.nominal_frames_per_second(), 25);
    assert!(!hour.is_drop_frame());
    assert!(!hour.wraps_at_24_hours());
    let label = Timecode::sample(90_061 * 25 + 7, 25, false, false).label();
    assert_eq!(
      (label.hours(), label.minutes(), label.seconds(), label.frames()),
      (25, 1, 1, 7)
    );
    assert!(!label.is_negative());
    assert_eq!(Timecode::sample(-25, 25, false, false).to_string(), "-00:00:01:00");
  }

  #[test]
  fn counts_at_the_lowest_nominal_rate() {
    let slowest = Timecode::sample(1, 1, false, false);
    assert_eq!(slowest.nominal_frames_per_second(), 1);
    assert_eq!(slowest.to_string(), "00:00:01:00");
    assert_eq!(slowest.label().frames(), 0);
  }

  #[test]
  fn skips_the_frame_numbers_of_drop_frame_counting() {
    assert_eq!(Timecode::sample(1_799, 30, true, false).to_string(), "00:00:59;29");
    assert_eq!(Timecode::sample(1_800, 30, true, false).to_string(), "00:01:00;02");
    assert_eq!(Timecode::sample(17_982, 30, true, false).to_string(), "00:10:00;00");
    assert_eq!(Timecode::sample(107_892, 30, true, false).to_string(), "01:00:00;00");
    assert_eq!(Timecode::sample(3_600, 60, true, false).to_string(), "00:01:00;04");
    assert_eq!(Timecode::sample(1_440, 24, true, false).to_string(), "00:01:00;00");
  }

  #[test]
  fn wraps_at_24_hours_when_the_file_says_so() {
    assert_eq!(
      Timecode::sample(25 * 86_400 + 25, 25, false, true).to_string(),
      "00:00:01:00"
    );
    assert_eq!(
      Timecode::sample(25 * 86_400 + 25, 25, false, false).to_string(),
      "24:00:01:00"
    );
  }
}
