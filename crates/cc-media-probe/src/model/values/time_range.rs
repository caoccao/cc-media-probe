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

//! `TimeRange`, a start and an end (`api.md` section 6.1).

use super::Seconds;
use crate::form_record::form_record;

form_record! {
  /// A start and an end on a presentation timeline, such as the time that the events of a subtitle stream span.
  ///
  /// ```
  /// use cc_media_probe::Stream;
  ///
  /// fn subtitle_span(stream: &Stream) -> Option<String> {
  ///     let range = stream.subtitle()?.time_range()?;
  ///     Some(format!("{} to {}", range.start(), range.end()))
  /// }
  /// ```
  #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
  pub struct TimeRange {
    start: Seconds,
    end: Seconds,
  }
}

impl TimeRange {
  /// The start.
  ///
  /// ```
  /// use cc_media_probe::model::TimeRange;
  ///
  /// fn starts_at_zero(range: TimeRange) -> bool {
  ///     range.start().as_rational().numerator() == 0
  /// }
  /// ```
  #[must_use]
  pub fn start(&self) -> Seconds {
    self.start
  }

  /// The end.
  ///
  /// ```
  /// use cc_media_probe::model::TimeRange;
  ///
  /// fn is_empty(range: TimeRange) -> bool {
  ///     range.end() <= range.start()
  /// }
  /// ```
  #[must_use]
  pub fn end(&self) -> Seconds {
    self.end
  }
}

#[cfg(test)]
mod tests {
  use super::TimeRange;
  use crate::Seconds;

  impl TimeRange {
    /// The range from `start` to `end`.
    pub(crate) fn sample(start: Seconds, end: Seconds) -> TimeRange {
      TimeRange { start, end }
    }
  }

  #[test]
  fn keeps_its_ends() {
    let range = TimeRange::sample(Seconds::sample(1, 2), Seconds::sample(90, 1));
    assert_eq!(range.start(), Seconds::sample(1, 2));
    assert_eq!(range.end(), Seconds::sample(90, 1));
  }
}
