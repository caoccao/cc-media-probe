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

//! `Seconds`, a time on a presentation timeline or a length of time (`api.md` section 6.1).

use std::fmt;
use std::time::Duration;

use super::Rational;
use super::constants::{FRACTION_DIGITS, MINUS_SIGN, NANOSECONDS_PER_SECOND, NO_SIGN, SECONDS_SYMBOL, ZERO_DIGIT};

/// A time on a presentation timeline, such as a start time, or a length of time, such as a duration, as an exact
/// number of seconds.
///
/// `Ord` compares by value. `Display` writes the exact decimal when it ends within nine digits, and the fraction
/// otherwise, followed by " s":
///
/// ```
/// use cc_media_probe::{MediaReport, Seconds};
///
/// fn describe(report: &MediaReport) -> Option<String> {
///     let duration: Seconds = report.duration()?;
///     // "5400 s", "10.5 s" or "1001/30000 s"; and as a `Duration`, rounded to the nanosecond.
///     Some(format!("{duration} ({:?})", duration.to_duration()?))
/// }
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Seconds(Rational);

impl Seconds {
  /// The exact number of seconds.
  ///
  /// ```
  /// use cc_media_probe::Stream;
  ///
  /// fn duration_in_milliseconds(stream: &Stream) -> Option<f64> {
  ///     Some(stream.duration()?.as_rational().to_f64() * 1_000.0)
  /// }
  /// ```
  #[must_use]
  pub fn as_rational(&self) -> Rational {
    self.0
  }

  /// The time as a `Duration`, rounded to the nanosecond with ties away from zero, or `None` when it is negative.
  ///
  /// ```
  /// use std::time::Duration;
  /// use cc_media_probe::MediaReport;
  ///
  /// fn longer_than_an_hour(report: &MediaReport) -> Option<bool> {
  ///     Some(report.duration()?.to_duration()? > Duration::from_secs(3_600))
  /// }
  /// ```
  #[must_use]
  pub fn to_duration(&self) -> Option<Duration> {
    let numerator = u64::try_from(self.0.numerator()).ok()?;
    let denominator = self.0.denominator();
    let whole = numerator.checked_div(denominator)?;
    let remainder = numerator.checked_rem(denominator)?;
    let scaled = u128::from(remainder).checked_mul(u128::from(NANOSECONDS_PER_SECOND))?;
    let divisor = u128::from(denominator);
    let truncated = scaled.checked_div(divisor)?;
    let rest = scaled.checked_rem(divisor)?;
    let rounded = if rest >= divisor.checked_sub(rest)? {
      truncated.checked_add(1)?
    } else {
      truncated
    };
    Some(Duration::new(whole, u32::try_from(rounded).ok()?))
  }

  /// The exact decimal of the value when it ends within nine digits, without the unit.
  fn decimal(&self) -> Option<String> {
    let denominator = self.0.denominator();
    if NANOSECONDS_PER_SECOND.checked_rem(denominator)? != 0 {
      return None;
    }
    let factor = NANOSECONDS_PER_SECOND.checked_div(denominator)?;
    let nanoseconds = u128::from(self.0.numerator().unsigned_abs()).checked_mul(u128::from(factor))?;
    let whole = nanoseconds.checked_div(u128::from(NANOSECONDS_PER_SECOND))?;
    let fraction = nanoseconds.checked_rem(u128::from(NANOSECONDS_PER_SECOND))?;
    let sign = if self.0.numerator() < 0 { MINUS_SIGN } else { NO_SIGN };
    if fraction == 0 {
      return Some(format!("{sign}{whole}"));
    }
    let digits = format!("{fraction:0FRACTION_DIGITS$}");
    Some(format!("{sign}{whole}.{}", digits.trim_end_matches(ZERO_DIGIT)))
  }
}

impl fmt::Display for Seconds {
  /// Writes `5400 s`, `-0.5 s` or `1001/30000 s`.
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self.decimal() {
      Some(decimal) => write!(formatter, "{decimal} {SECONDS_SYMBOL}"),
      None => write!(formatter, "{} {SECONDS_SYMBOL}", self.0),
    }
  }
}

#[cfg(test)]
mod tests {
  use std::time::Duration;

  use super::Seconds;
  use crate::Rational;

  impl Seconds {
    /// The time `numerator / denominator` seconds.
    pub(crate) fn sample(numerator: i64, denominator: u64) -> Seconds {
      Seconds(Rational::new(numerator, denominator).unwrap())
    }
  }

  #[test]
  fn converts_to_a_duration() {
    assert_eq!(Seconds::sample(5_400, 1).to_duration(), Some(Duration::from_mins(90)));
    assert_eq!(
      Seconds::sample(21, 2).to_duration(),
      Some(Duration::from_millis(10_500))
    );
    assert_eq!(
      Seconds::sample(1_001, 30_000).to_duration(),
      Some(Duration::from_nanos(33_366_667))
    );
    assert_eq!(
      Seconds::sample(1, 3).to_duration(),
      Some(Duration::from_nanos(333_333_333))
    );
    assert_eq!(
      Seconds::sample(1, 2_000_000_000).to_duration(),
      Some(Duration::from_nanos(1))
    );
    assert_eq!(Seconds::sample(1, 2_000_000_001).to_duration(), Some(Duration::ZERO));
    assert_eq!(
      Seconds::sample(2_999_999_999, 3_000_000_000).to_duration(),
      Some(Duration::from_secs(1))
    );
    assert_eq!(Seconds::sample(-1, 2).to_duration(), None);
    assert_eq!(Seconds::sample(0, 1).to_duration(), Some(Duration::ZERO));
  }

  #[test]
  fn displays_its_value() {
    assert_eq!(Seconds::sample(5_400, 1).to_string(), "5400 s");
    assert_eq!(Seconds::sample(21, 2).to_string(), "10.5 s");
    assert_eq!(Seconds::sample(-1, 2).to_string(), "-0.5 s");
    assert_eq!(Seconds::sample(1, 1_000_000_000).to_string(), "0.000000001 s");
    assert_eq!(Seconds::sample(1, 3).to_string(), "1/3 s");
    assert_eq!(Seconds::sample(1, 1_024).to_string(), "1/1024 s");
    assert_eq!(Seconds::sample(0, 1).to_string(), "0 s");
  }

  #[test]
  fn compares_by_value() {
    assert!(Seconds::sample(1, 3) < Seconds::sample(1, 2));
    assert_eq!(Seconds::sample(3, 6).as_rational(), Rational::new(1, 2).unwrap());
  }
}
