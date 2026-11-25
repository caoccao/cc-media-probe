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

//! The greatest common divisor of two unsigned integers, by Euclid's algorithm, without any operation that can panic.

/// The greatest common divisor of `first` and `second`; 0 only when both are 0, and `first` when `second` is 0.
pub(crate) fn greatest_common_divisor(mut first: u64, mut second: u64) -> u64 {
  while let Some(remainder) = first.checked_rem(second) {
    first = second;
    second = remainder;
  }
  first
}

#[cfg(test)]
mod tests {
  use super::greatest_common_divisor;

  #[test]
  fn finds_the_greatest_common_divisor() {
    assert_eq!(greatest_common_divisor(48_000, 2_002), 2);
    assert_eq!(greatest_common_divisor(2_002, 48_000), 2);
    assert_eq!(greatest_common_divisor(7, 13), 1);
    assert_eq!(greatest_common_divisor(0, 5), 5);
    assert_eq!(greatest_common_divisor(5, 0), 5);
    assert_eq!(greatest_common_divisor(0, 0), 0);
    assert_eq!(greatest_common_divisor(1 << 63, 1 << 62), 1 << 62);
  }
}
