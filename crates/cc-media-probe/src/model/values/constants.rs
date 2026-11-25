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
