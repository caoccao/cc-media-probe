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

//! Covered and total lines, and the bar of R28.

use crate::coverage::constants::{BASIS_POINTS, HUNDREDTHS, THRESHOLD_DENOMINATOR, THRESHOLD_NUMERATOR};

/// Covered lines out of all executable lines.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Tally {
  /// Lines executed at least once.
  pub covered: u64,
  /// Executable lines.
  pub total: u64,
}

impl Tally {
  /// Counts one executable line with `hits` executions.
  pub fn add_line(&mut self, hits: u64) {
    self.total = self.total.saturating_add(1);
    if hits > 0 {
      self.covered = self.covered.saturating_add(1);
    }
  }

  /// Adds the lines of another tally.
  pub fn merge(&mut self, other: Tally) {
    self.total = self.total.saturating_add(other.total);
    self.covered = self.covered.saturating_add(other.covered);
  }

  /// Whether coverage is strictly above 90%. A tally without executable
  /// lines is not held to the bar.
  #[must_use]
  pub fn passes(self) -> bool {
    if self.total == 0 {
      return true;
    }
    let scaled_covered = u128::from(self.covered).checked_mul(THRESHOLD_DENOMINATOR);
    let scaled_total = u128::from(self.total).checked_mul(THRESHOLD_NUMERATOR);
    scaled_covered
      .zip(scaled_total)
      .is_some_and(|(covered, total)| covered > total)
  }

  /// The percentage with two decimals, rounded down, such as `90.00%`.
  #[must_use]
  pub fn percentage(self) -> String {
    let basis_points = u128::from(self.covered)
      .checked_mul(BASIS_POINTS)
      .and_then(|scaled| scaled.checked_div(u128::from(self.total)))
      .unwrap_or_default();
    let whole = basis_points.checked_div(HUNDREDTHS).unwrap_or_default();
    let fraction = basis_points.checked_rem(HUNDREDTHS).unwrap_or_default();
    format!("{whole}.{fraction:02}%")
  }
}

#[cfg(test)]
mod tests {
  use super::Tally;

  #[test]
  fn exactly_ninety_percent_fails() {
    assert!(!Tally { covered: 9, total: 10 }.passes());
    assert!(
      !Tally {
        covered: 900,
        total: 1000
      }
      .passes()
    );
    assert!(
      Tally {
        covered: 901,
        total: 1000
      }
      .passes()
    );
    assert!(Tally { covered: 0, total: 0 }.passes());
    assert!(!Tally { covered: 0, total: 1 }.passes());
  }

  #[test]
  fn counts_lines_and_formats_percentages() {
    let mut tally = Tally::default();
    tally.add_line(3);
    tally.add_line(0);
    tally.add_line(1);
    assert_eq!(tally, Tally { covered: 2, total: 3 });
    assert_eq!(tally.percentage(), "66.66%");
    tally.merge(Tally { covered: 1, total: 1 });
    assert_eq!(tally.percentage(), "75.00%");
    assert_eq!(Tally::default().percentage(), "0.00%");
    assert_eq!(
      Tally {
        covered: 901,
        total: 1000
      }
      .percentage(),
      "90.10%"
    );
  }
}
