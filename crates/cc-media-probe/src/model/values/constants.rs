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

//! The names and texts of the value types.

/// Between the width and the height in the display of a `Size`.
pub const SIZE_SEPARATOR: &str = "x";
/// The unit after a time in the display of `Seconds`.
pub const SECONDS_SYMBOL: &str = "s";
/// The nanoseconds in a second, which also bounds the decimal display of `Seconds` to nine digits.
pub const NANOSECONDS_PER_SECOND: u64 = 1_000_000_000;
/// The digits of the fraction in the decimal display of `Seconds`, before trailing zeros are removed.
pub const FRACTION_DIGITS: usize = 9;
/// The digit that ends no fraction in the decimal display of `Seconds`.
pub const ZERO_DIGIT: char = '0';

/// The serialized name of `Provenance::Declared`.
pub const DECLARED_NAME: &str = "declared";
/// The serialized name of `Provenance::Bitstream`.
pub const BITSTREAM_NAME: &str = "bitstream";
/// The serialized name of `Provenance::Derived`.
pub const DERIVED_NAME: &str = "derived";

/// The serialized name of `EstimateMethod::ConstantBitRateOverSize`.
pub const CONSTANT_BIT_RATE_OVER_SIZE_NAME: &str = "constant_bit_rate_over_size";
/// The serialized name of `EstimateMethod::DeliveryClockSpan`.
pub const DELIVERY_CLOCK_SPAN_NAME: &str = "delivery_clock_span";
/// The serialized name of `EstimateMethod::EndpointTimestamps`.
pub const ENDPOINT_TIMESTAMPS_NAME: &str = "endpoint_timestamps";
/// The serialized name of `EstimateMethod::LastUnitStart`.
pub const LAST_UNIT_START_NAME: &str = "last_unit_start";
/// The serialized name of `EstimateMethod::IndexSizeTotals`.
pub const INDEX_SIZE_TOTALS_NAME: &str = "index_size_totals";

/// The serialized name of `FloatingPointWidth::Single`.
pub const SINGLE_NAME: &str = "single";
/// The serialized name of `FloatingPointWidth::Double`.
pub const DOUBLE_NAME: &str = "double";

/// The bits of an IEEE 754 double's significand that are stored, without the implicit leading bit.
pub const DOUBLE_FRACTION_BITS: u32 = 52;
/// The mask of those bits.
pub const DOUBLE_FRACTION_MASK: u64 = 0x000f_ffff_ffff_ffff;
/// The mask of a double's biased exponent once shifted down.
pub const DOUBLE_EXPONENT_MASK: u64 = 0x7ff;
/// The bias of a double's exponent, together with the fraction bits: a significand `m` with the biased exponent `e`
/// is the value `m * 2^(e - DOUBLE_EXPONENT_OFFSET)`.
pub const DOUBLE_EXPONENT_OFFSET: i64 = 1_075;
/// The implicit leading bit of a normal double's significand.
pub const DOUBLE_IMPLICIT_BIT: u64 = 0x0010_0000_0000_0000;

/// The bytes of a GUID's first field.
pub const GUID_FIRST_FIELD: usize = 4;
/// The bytes of a GUID's second and third fields.
pub const GUID_SHORT_FIELD: usize = 2;
/// Between the fields of a GUID's display.
pub const GUID_SEPARATOR: char = '-';

/// The sign before a negative time in the display of `Seconds`.
pub const MINUS_SIGN: &str = "-";
/// The base of a binary floating-point value's exponent.
pub const BINARY_BASE: u64 = 2;
/// The bytes of a GUID.
pub const GUID_BYTES: usize = 16;
/// The sign before a time that is not negative in the display of `Seconds`: none.
pub const NO_SIGN: &str = "";

/// The message of a rational that is not reduced, has a denominator of 0 or a part beyond 2^53 - 1
/// (`json-output.md` section 5.6).
#[cfg(feature = "serde")]
pub const RATIONAL_MESSAGE: &str = "a rational is not reduced, has a denominator of 0 or a part beyond 2^53 - 1";

/// The hexadecimal digits of the bits of a single-precision value (`json-output.md` section 5.6).
#[cfg(feature = "serde")]
pub const SINGLE_BIT_DIGITS: usize = 8;

/// The hexadecimal digits of the bits of a double-precision value.
#[cfg(feature = "serde")]
pub const DOUBLE_BIT_DIGITS: usize = 16;

/// The radix of hexadecimal text.
#[cfg(feature = "serde")]
pub const HEXADECIMAL_RADIX: u32 = 16;

/// The message of a floating-point value whose bits are not the lowercase hexadecimal digits of its width.
#[cfg(feature = "serde")]
pub const FLOATING_POINT_MESSAGE: &str = "a floating-point value's bits are not the lowercase digits of its width";

/// The message of a GUID that is not 32 lowercase hexadecimal digits (`json-output.md` section 5.6).
#[cfg(feature = "serde")]
pub const GUID_MESSAGE: &str = "a GUID is not 32 lowercase hexadecimal digits";

/// The separator of the parts of an RFC 3339 date (RFC 3339 section 5.6).
#[cfg(feature = "serde")]
pub const DATE_SEPARATOR: char = '-';

/// The marker between the date and the time of an RFC 3339 text, which the form writes in uppercase.
#[cfg(feature = "serde")]
pub const TIME_MARKER: char = 'T';

/// The separator of the parts of an RFC 3339 time.
#[cfg(feature = "serde")]
pub const TIME_SEPARATOR: char = ':';

/// The separator of the fraction of a second.
#[cfg(feature = "serde")]
pub const FRACTION_SEPARATOR: char = '.';

/// The suffix of a time in UTC, which the form writes in uppercase.
#[cfg(feature = "serde")]
pub const UTC_SUFFIX: char = 'Z';

/// The largest year of the form, which writes four digits.
#[cfg(feature = "serde")]
pub const LARGEST_YEAR: i64 = 9_999;

/// The number of the last month of a year.
#[cfg(feature = "serde")]
pub const LARGEST_MONTH: i64 = 12;

/// The largest day of any month.
#[cfg(feature = "serde")]
pub const LARGEST_MONTH_DAY: i64 = 31;

/// The number of March, the first month of the year in the conversion of a civil date.
#[cfg(feature = "serde")]
pub const MARCH: i64 = 3;

/// The months from March to the end of a year, which move January and February to the year before.
#[cfg(feature = "serde")]
pub const MONTHS_FROM_MARCH: i64 = 12;

/// The seconds of a day.
#[cfg(feature = "serde")]
pub const SECONDS_PER_DAY: i64 = 86_400;

/// The seconds of an hour.
#[cfg(feature = "serde")]
pub const SECONDS_PER_HOUR: i64 = 3_600;

/// The seconds of a minute.
#[cfg(feature = "serde")]
pub const SECONDS_PER_MINUTE: i64 = 60;

/// The years of an era of the Gregorian calendar, which repeats every 400 years.
#[cfg(feature = "serde")]
pub const YEARS_PER_ERA: i64 = 400;

/// The days of an era.
#[cfg(feature = "serde")]
pub const DAYS_PER_ERA: i64 = 146_097;

/// The last day of an era, counted from 0.
#[cfg(feature = "serde")]
pub const LAST_DAY_OF_ERA: i64 = 146_096;

/// The days of a common year.
#[cfg(feature = "serde")]
pub const DAYS_PER_YEAR: i64 = 365;

/// The years of the cycle of leap years.
#[cfg(feature = "serde")]
pub const LEAP_YEAR_CYCLE: i64 = 4;

/// The years of a century, whose first year is no leap year unless the era begins with it.
#[cfg(feature = "serde")]
pub const YEARS_PER_CENTURY: i64 = 100;

/// The days of four years, the last of which is a leap year.
#[cfg(feature = "serde")]
pub const DAYS_PER_FOUR_YEARS: i64 = 1_460;

/// The days of a century of an era, with its leap years.
#[cfg(feature = "serde")]
pub const DAYS_PER_CENTURY: i64 = 36_524;

/// The days from 0000-03-01 to 1970-01-01.
#[cfg(feature = "serde")]
pub const DAYS_TO_UNIX_EPOCH: i64 = 719_468;

/// The factor of the month lengths from March, which alternate 31 and 30 days in groups of five months.
#[cfg(feature = "serde")]
pub const MONTH_LENGTH_FACTOR: i64 = 153;

/// The divisor of the month lengths.
#[cfg(feature = "serde")]
pub const MONTH_LENGTH_DIVISOR: i64 = 5;

/// The rounding of the month lengths.
#[cfg(feature = "serde")]
pub const MONTH_LENGTH_ROUNDING: i64 = 2;

/// The message of a time that is not one RFC 3339 text in UTC of the years 0000 to 9999 (`json-output.md`
/// section 5.6).
#[cfg(feature = "serde")]
pub const UTC_TIME_MESSAGE: &str = "a UTC time is not the one RFC 3339 text of an instant of the years 0000 to 9999";
