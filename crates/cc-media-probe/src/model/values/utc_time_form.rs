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

//! The serialized form of `UtcTime` (`json-output.md` section 5.6): RFC 3339 text in UTC with the suffix `Z` and a
//! four-digit year from 0000 to 9999, as in `"2026-11-23T08:30:00.25Z"`. The fraction of a second has as many digits
//! as it needs, up to nine, and none when it is zero, so an instant has one text, and reading takes only that text.
//! The calendar is the proleptic Gregorian one of RFC 3339 section 5.6, counted in days from 1970-01-01 by the
//! standard conversion of a civil date through eras of 400 years.

use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::UtcTime;
use super::constants::{
  DATE_SEPARATOR, DAYS_PER_CENTURY, DAYS_PER_ERA, DAYS_PER_FOUR_YEARS, DAYS_PER_YEAR, DAYS_TO_UNIX_EPOCH,
  FRACTION_DIGITS, FRACTION_SEPARATOR, LARGEST_MONTH, LARGEST_MONTH_DAY, LARGEST_YEAR, LAST_DAY_OF_ERA,
  LEAP_YEAR_CYCLE, MARCH, MONTH_LENGTH_DIVISOR, MONTH_LENGTH_FACTOR, MONTH_LENGTH_ROUNDING, MONTHS_FROM_MARCH,
  SECONDS_PER_DAY, SECONDS_PER_HOUR, SECONDS_PER_MINUTE, TIME_MARKER, TIME_SEPARATOR, UTC_SUFFIX, UTC_TIME_MESSAGE,
  YEARS_PER_CENTURY, YEARS_PER_ERA, ZERO_DIGIT,
};
use crate::form::{ReadAlone, read_alone};

/// The days before the start of the month, counted from March, of a month index from 0 for March.
fn days_before_month(month_index: i64) -> Option<i64> {
  month_index
    .checked_mul(MONTH_LENGTH_FACTOR)?
    .checked_add(MONTH_LENGTH_ROUNDING)?
    .checked_div(MONTH_LENGTH_DIVISOR)
}

/// The days of an era before the start of its year of that index, counted from March.
fn days_before_year(year_of_era: i64) -> Option<i64> {
  year_of_era
    .checked_mul(DAYS_PER_YEAR)?
    .checked_add(year_of_era.checked_div(LEAP_YEAR_CYCLE)?)?
    .checked_sub(year_of_era.checked_div(YEARS_PER_CENTURY)?)
}

/// The days from 1970-01-01 to a civil date.
fn days_from_civil(year: i64, month: i64, day: i64) -> Option<i64> {
  let (year, month_index) = match month.checked_sub(MARCH)? {
    index if index < 0 => (year.checked_sub(1)?, index.checked_add(MONTHS_FROM_MARCH)?),
    index => (year, index),
  };
  let era = year.div_euclid(YEARS_PER_ERA);
  let day_of_year = days_before_month(month_index)?.checked_add(day)?.checked_sub(1)?;
  let day_of_era = days_before_year(year.rem_euclid(YEARS_PER_ERA))?.checked_add(day_of_year)?;
  era
    .checked_mul(DAYS_PER_ERA)?
    .checked_add(day_of_era)?
    .checked_sub(DAYS_TO_UNIX_EPOCH)
}

/// The civil date of the day that many days from 1970-01-01.
fn civil_from_days(days: i64) -> Option<(i64, i64, i64)> {
  let shifted = days.checked_add(DAYS_TO_UNIX_EPOCH)?;
  let era = shifted.div_euclid(DAYS_PER_ERA);
  let day_of_era = shifted.rem_euclid(DAYS_PER_ERA);
  let year_of_era = day_of_era
    .checked_sub(day_of_era.checked_div(DAYS_PER_FOUR_YEARS)?)?
    .checked_add(day_of_era.checked_div(DAYS_PER_CENTURY)?)?
    .checked_sub(day_of_era.checked_div(LAST_DAY_OF_ERA)?)?
    .checked_div(DAYS_PER_YEAR)?;
  let day_of_year = day_of_era.checked_sub(days_before_year(year_of_era)?)?;
  let month_index = day_of_year
    .checked_mul(MONTH_LENGTH_DIVISOR)?
    .checked_add(MONTH_LENGTH_ROUNDING)?
    .checked_div(MONTH_LENGTH_FACTOR)?;
  let day = day_of_year
    .checked_sub(days_before_month(month_index)?)?
    .checked_add(1)?;
  let year = era.checked_mul(YEARS_PER_ERA)?.checked_add(year_of_era)?;
  match month_index.checked_add(MARCH)? {
    month if month > LARGEST_MONTH => Some((year.checked_add(1)?, month.checked_sub(LARGEST_MONTH)?, day)),
    month => Some((year, month, day)),
  }
}

/// The instant as its one RFC 3339 text, or `None` for an instant outside the years 0000 to 9999.
fn to_text(time: UtcTime) -> Option<String> {
  let seconds = time.seconds_since_unix_epoch();
  let (year, month, day) = civil_from_days(seconds.div_euclid(SECONDS_PER_DAY))?;
  if !(0..=LARGEST_YEAR).contains(&year) {
    return None;
  }
  let second_of_day = seconds.rem_euclid(SECONDS_PER_DAY);
  let hour = second_of_day.checked_div(SECONDS_PER_HOUR)?;
  let minute = second_of_day
    .rem_euclid(SECONDS_PER_HOUR)
    .checked_div(SECONDS_PER_MINUTE)?;
  let second = second_of_day.rem_euclid(SECONDS_PER_MINUTE);
  let mut text = format!(
    "{year:04}{DATE_SEPARATOR}{month:02}{DATE_SEPARATOR}{day:02}{TIME_MARKER}{hour:02}{TIME_SEPARATOR}{minute:02}\
     {TIME_SEPARATOR}{second:02}"
  );
  if time.nanoseconds() > 0 {
    let fraction = format!("{:0FRACTION_DIGITS$}", time.nanoseconds());
    text.push(FRACTION_SEPARATOR);
    text.push_str(fraction.trim_end_matches(ZERO_DIGIT));
  }
  text.push(UTC_SUFFIX);
  Some(text)
}

/// The number that a field of ASCII digits spells.
fn field(text: &str) -> Option<i64> {
  (!text.is_empty() && text.bytes().all(|byte| byte.is_ascii_digit()))
    .then(|| text.parse().ok())
    .flatten()
}

/// The instant that an RFC 3339 text spells, when the text is the one that the instant writes.
fn from_text(text: &str) -> Option<UtcTime> {
  let (date, time) = text.strip_suffix(UTC_SUFFIX)?.split_once(TIME_MARKER)?;
  let mut date = date.split(DATE_SEPARATOR).map(field);
  let (year, month, day) = (date.next()??, date.next()??, date.next()??);
  let (clock, fraction) = match time.split_once(FRACTION_SEPARATOR) {
    Some((clock, fraction)) => (clock, Some(fraction)),
    None => (time, None),
  };
  let mut clock = clock.split(TIME_SEPARATOR).map(field);
  let (hour, minute, second) = (clock.next()??, clock.next()??, clock.next()??);
  if date.next().is_some()
    || clock.next().is_some()
    || !(1..=LARGEST_MONTH).contains(&month)
    || !(1..=LARGEST_MONTH_DAY).contains(&day)
  {
    return None;
  }
  let nanoseconds = match fraction {
    Some(fraction) if fraction.len() <= FRACTION_DIGITS => {
      let padded = format!("{fraction:0<FRACTION_DIGITS$}");
      u32::try_from(field(&padded)?).ok()?
    }
    Some(_) => return None,
    None => 0,
  };
  let seconds = days_from_civil(year, month, day)?
    .checked_mul(SECONDS_PER_DAY)?
    .checked_add(hour.checked_mul(SECONDS_PER_HOUR)?)?
    .checked_add(minute.checked_mul(SECONDS_PER_MINUTE)?)?
    .checked_add(second)?;
  let instant = UtcTime::from_parts(seconds, nanoseconds)?;
  (to_text(instant)? == text).then_some(instant)
}

impl Serialize for UtcTime {
  fn serialize<Output: Serializer>(&self, serializer: Output) -> Result<Output::Ok, Output::Error> {
    let text = to_text(*self).ok_or_else(|| <Output::Error as serde::ser::Error>::custom(UTC_TIME_MESSAGE))?;
    serializer.serialize_str(&text)
  }
}

impl<'input> ReadAlone<'input> for UtcTime {
  fn read_members<Input: Deserializer<'input>>(deserializer: Input) -> Result<UtcTime, Input::Error> {
    let text = String::deserialize(deserializer)?;
    from_text(&text).ok_or_else(|| Input::Error::custom(UTC_TIME_MESSAGE))
  }
}

impl<'input> Deserialize<'input> for UtcTime {
  fn deserialize<Input: Deserializer<'input>>(deserializer: Input) -> Result<UtcTime, Input::Error> {
    read_alone(deserializer)
  }
}

#[cfg(test)]
mod tests {
  use super::{from_text, to_text};
  use crate::model::UtcTime;

  #[test]
  fn writes_one_text_for_each_instant() {
    let cases = [
      (UtcTime::sample(1_795_422_600, 250_000_000), "2026-11-23T08:30:00.25Z"),
      (UtcTime::sample(0, 0), "1970-01-01T00:00:00Z"),
      (UtcTime::sample(-1, 999_999_999), "1969-12-31T23:59:59.999999999Z"),
      (UtcTime::sample(951_782_400, 1), "2000-02-29T00:00:00.000000001Z"),
      (UtcTime::sample(-62_167_219_200, 0), "0000-01-01T00:00:00Z"),
      (UtcTime::sample(253_402_300_799, 0), "9999-12-31T23:59:59Z"),
    ];
    for (time, text) in cases {
      assert_eq!(to_text(time).as_deref(), Some(text));
      assert_eq!(from_text(text), Some(time));
    }
    assert_eq!(to_text(UtcTime::sample(253_402_300_800, 0)), None);
    assert_eq!(to_text(UtcTime::sample(-62_167_219_201, 0)), None);
    assert_eq!(to_text(UtcTime::sample(i64::MIN, 0)), None);
  }

  #[test]
  fn reads_only_the_text_that_an_instant_writes() {
    for text in [
      "2026-11-23T08:30:00.250Z",
      "2026-11-23T08:30:00.Z",
      "2026-11-23T08:30:00.1234567891Z",
      "2026-11-23T08:30:00z",
      "2026-11-23t08:30:00Z",
      "2026-11-23T08:30:00+00:00",
      "2026-11-23T08:30:60Z",
      "2026-02-30T00:00:00Z",
      "2026-13-01T00:00:00Z",
      "2026-00-01T00:00:00Z",
      "2026-01-00T00:00:00Z",
      "2026-01-32T00:00:00Z",
      "26-01-01T00:00:00Z",
      "+2026-01-01T00:00:00Z",
      "2026-01-01T24:00:00Z",
      "2026-01-01T00:00Z",
      "2026-01-01-01T00:00:00Z",
      "2026-01-01T00:00:00:00Z",
      "10000-01-01T00:00:00Z",
      "2026-1-01T00:00:00Z",
      "",
    ] {
      assert_eq!(from_text(text), None, "{text}");
    }
  }
}
