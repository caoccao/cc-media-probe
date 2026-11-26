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

//! Whether an exact rational is reduced (`json-output.md` section 8; plan section 8.8, R24): its numerator and its
//! denominator have no common factor but 1, so a whole number has the denominator 1 and every exact value has one
//! form, as the second review of the third slice's first part asked, which read 18014398509481986/2 for
//! 9007199254740993 seconds. The parts are canonical decimal strings of at most `MAXIMUM_EXACT_DIGITS` digits, which
//! the caller checks, so each is read into limbs of 64 bits and compared by the binary GCD algorithm: Stein's
//! algorithm, which needs only shifts, comparisons and subtractions, and takes a number of steps linear in the bits
//! and a time quadratic in them, which the bound on the digits bounds.

use std::cmp::Ordering;

use super::constants::{DECIMAL_RADIX, DIGITS_PER_STEP};

/// A natural number of any width: its limbs of 64 bits, the least significant first, with no zero limb at the top,
/// so that 0 has no limb.
#[derive(PartialEq, Eq)]
struct Natural {
  limbs: Vec<u64>,
}

/// The low and the high limb of a 128-bit value.
fn split(value: u128) -> (u64, u64) {
  let low = u64::try_from(value & u128::from(u64::MAX)).unwrap_or(0);
  let high = u64::try_from(value.wrapping_shr(u64::BITS)).unwrap_or(0);
  (low, high)
}

impl Natural {
  /// The number that canonical decimal digits spell, read `DIGITS_PER_STEP` digits a step.
  fn from_digits(digits: &str) -> Natural {
    let radix = u64::from(DECIMAL_RADIX);
    let mut number = Natural { limbs: Vec::new() };
    for step in digits.as_bytes().chunks(DIGITS_PER_STEP) {
      let (scale, value) = step.iter().fold((1, 0), |(scale, value): (u64, u64), digit| {
        let digit = char::from(*digit).to_digit(DECIMAL_RADIX).map_or(0, u64::from);
        (scale.wrapping_mul(radix), value.wrapping_mul(radix).wrapping_add(digit))
      });
      number.multiply_add(scale, value);
    }
    number
  }

  /// Multiplies the number by `factor` and adds `addend`.
  fn multiply_add(&mut self, factor: u64, addend: u64) {
    let mut carry = addend;
    for limb in &mut self.limbs {
      let product = u128::from(*limb)
        .wrapping_mul(u128::from(factor))
        .wrapping_add(u128::from(carry));
      (*limb, carry) = split(product);
    }
    if carry != 0 {
      self.limbs.push(carry);
    }
  }

  /// Whether the number is 0.
  fn is_zero(&self) -> bool {
    self.limbs.is_empty()
  }

  /// Whether the number is 1.
  fn is_one(&self) -> bool {
    self.limbs == [1]
  }

  /// Whether the number is even; 0 is.
  fn is_even(&self) -> bool {
    self.limbs.first().is_none_or(|limb| limb & 1 == 0)
  }

  /// Divides the number by 2 until it is odd; it is not 0.
  fn remove_twos(&mut self) {
    let zero_limbs = self.limbs.iter().take_while(|limb| **limb == 0).count();
    self.limbs.drain(..zero_limbs);
    let Some(shift) = self.limbs.first().map(|limb| limb.trailing_zeros()) else {
      return;
    };
    if shift == 0 {
      return;
    }
    let mut higher = 0;
    for limb in self.limbs.iter_mut().rev() {
      let shifted = limb.wrapping_shr(shift) | higher;
      higher = limb.wrapping_shl(u64::BITS.wrapping_sub(shift));
      *limb = shifted;
    }
    self.trim();
  }

  /// Subtracts `smaller`, which is not greater than the number.
  fn subtract(&mut self, smaller: &Natural) {
    let mut borrow = false;
    for (position, limb) in self.limbs.iter_mut().enumerate() {
      let other = smaller.limbs.get(position).copied().unwrap_or(0);
      let (difference, first_borrow) = limb.overflowing_sub(other);
      let (difference, second_borrow) = difference.overflowing_sub(u64::from(borrow));
      *limb = difference;
      borrow = first_borrow || second_borrow;
    }
    self.trim();
  }

  /// Removes the zero limbs at the top.
  fn trim(&mut self) {
    while self.limbs.last() == Some(&0) {
      self.limbs.pop();
    }
  }
}

impl PartialOrd for Natural {
  fn partial_cmp(&self, other: &Natural) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Natural {
  /// By the number of limbs, then limb by limb from the most significant.
  fn cmp(&self, other: &Natural) -> Ordering {
    self
      .limbs
      .len()
      .cmp(&other.limbs.len())
      .then_with(|| self.limbs.iter().rev().cmp(other.limbs.iter().rev()))
  }
}

/// Whether the two canonical decimal numbers, without a sign, have no common factor but 1: whether a rational of
/// these parts is reduced. A part of 0 is reduced only over 1.
pub(super) fn is_reduced(numerator: &str, denominator: &str) -> bool {
  let (mut first, mut second) = (Natural::from_digits(numerator), Natural::from_digits(denominator));
  if first.is_one() || second.is_one() {
    return true;
  }
  if first.is_zero() || second.is_zero() || (first.is_even() && second.is_even()) {
    return false;
  }
  first.remove_twos();
  loop {
    second.remove_twos();
    if first > second {
      std::mem::swap(&mut first, &mut second);
    }
    second.subtract(&first);
    if second.is_zero() {
      return first.is_one();
    }
  }
}

#[cfg(test)]
mod tests {
  use super::{Natural, is_reduced};

  #[test]
  fn reads_digits_of_any_width_into_limbs() {
    assert!(Natural::from_digits("0").is_zero());
    assert!(Natural::from_digits("1").is_one());
    assert_eq!(Natural::from_digits("18446744073709551615").limbs, [u64::MAX]);
    assert_eq!(Natural::from_digits("18446744073709551616").limbs, [0, 1]);
    assert_eq!(
      Natural::from_digits("170141183460469231731687303715884105728").limbs,
      [0, 1 << 63]
    );
  }

  #[test]
  fn takes_a_rational_without_a_common_factor_as_reduced() {
    assert!(is_reduced("3689348814741910323", "200000000"));
    assert!(is_reduced("9007199254740993", "1"));
    assert!(is_reduced("0", "1"));
    assert!(is_reduced("1", "18446744073709551616"));
    assert!(is_reduced(
      "170141183460469231731687303715884105727",
      "170141183460469231731687303715884105728"
    ));
    assert!(is_reduced("340282366920938463463374607431768211457", "3"));
  }

  #[test]
  fn refuses_a_rational_with_a_common_factor() {
    assert!(!is_reduced("18014398509481986", "2"));
    assert!(!is_reduced("6", "4"));
    assert!(!is_reduced("0", "5"));
    assert!(!is_reduced("9", "3"));
    assert!(!is_reduced(
      "340282366920938463463374607431768211456",
      "18446744073709551616"
    ));
    assert!(!is_reduced("340282366920938463463374607431768211455", "3"));
  }
}
