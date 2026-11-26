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

//! An integer that a reader hands over as 128 bits, as the nineteenth review asked: the integer is a value of the
//! form when it fits within 64 bits, whatever type the reader gives it in, so it reads as the integer of 64 bits that
//! JSON's reader gives, and only one beyond 64 bits is a value that JSON does not carry. The review read the rational
//! `1/2` from Serde's `MapDeserializer` with its parts as `u64` and refused with them as `u128`. A CBOR bignum that
//! ciborium gives for a value beyond 64 bits still fails.

/// An integer within 64 bits, as JSON's reader gives it: one that is not negative as unsigned.
#[derive(Clone, Copy)]
pub(crate) enum Narrowed {
  /// An integer that is not negative.
  Unsigned(u64),
  /// A negative integer.
  Signed(i64),
}

/// The integer within 64 bits that the value holds, if it fits.
pub(crate) fn narrow_signed(value: i128) -> Option<Narrowed> {
  match u64::try_from(value) {
    Ok(value) => Some(Narrowed::Unsigned(value)),
    Err(_) => i64::try_from(value).ok().map(Narrowed::Signed),
  }
}

/// The integer within 64 bits that the value holds, if it fits.
pub(crate) fn narrow_unsigned(value: u128) -> Option<Narrowed> {
  u64::try_from(value).ok().map(Narrowed::Unsigned)
}
