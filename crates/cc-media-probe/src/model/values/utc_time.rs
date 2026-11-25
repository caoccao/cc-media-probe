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

//! `UtcTime`, an instant in UTC (`api.md` section 6.1).

use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// An instant in UTC, from the start of year 0 to the end of year 9999, which RFC 3339 can write with a four-digit
/// year. A stated instant outside that range is implausible, so the normalized time is absent and the raw value
/// stays in the details (`api.md` section 7). `Ord` compares by instant.
///
/// ```
/// use cc_media_probe::MediaReport;
///
/// fn created(report: &MediaReport) -> Option<std::time::SystemTime> {
///     report.creation_time()?.to_system_time()
/// }
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UtcTime {
  seconds_since_unix_epoch: i64,
  nanoseconds: u32,
}

impl UtcTime {
  /// The whole seconds since 1970-01-01T00:00:00Z, negative before it.
  ///
  /// ```
  /// use cc_media_probe::model::UtcTime;
  ///
  /// fn before_1970(time: UtcTime) -> bool {
  ///     time.seconds_since_unix_epoch() < 0
  /// }
  /// ```
  #[must_use]
  pub fn seconds_since_unix_epoch(&self) -> i64 {
    self.seconds_since_unix_epoch
  }

  /// The nanoseconds after those seconds, below one billion.
  ///
  /// ```
  /// use cc_media_probe::model::UtcTime;
  ///
  /// fn on_the_second(time: UtcTime) -> bool {
  ///     time.nanoseconds() == 0
  /// }
  /// ```
  #[must_use]
  pub fn nanoseconds(&self) -> u32 {
    self.nanoseconds
  }

  /// The instant as a `SystemTime`, or `None` where the platform's `SystemTime` cannot hold it exactly: beyond its
  /// range, or finer than its precision, such as one nanosecond after a whole second on Windows, whose
  /// `SystemTime` counts in 100-nanosecond intervals. A result is therefore always the instant itself, never a
  /// neighbor of it.
  ///
  /// ```
  /// use cc_media_probe::model::UtcTime;
  ///
  /// fn is_in_the_past(time: UtcTime) -> bool {
  ///     time.to_system_time().is_some_and(|instant| instant <= std::time::SystemTime::now())
  /// }
  /// ```
  #[must_use]
  pub fn to_system_time(&self) -> Option<SystemTime> {
    let whole = Duration::from_secs(self.seconds_since_unix_epoch.unsigned_abs());
    let fraction = Duration::from_nanos(u64::from(self.nanoseconds));
    let before_epoch = self.seconds_since_unix_epoch < 0;
    let distance = if before_epoch {
      whole.checked_sub(fraction)?
    } else {
      whole.checked_add(fraction)?
    };
    let instant = if before_epoch {
      UNIX_EPOCH.checked_sub(distance)?
    } else {
      UNIX_EPOCH.checked_add(distance)?
    };
    let kept = if before_epoch {
      UNIX_EPOCH.duration_since(instant)
    } else {
      instant.duration_since(UNIX_EPOCH)
    };
    (kept.ok()? == distance).then_some(instant)
  }
}

#[cfg(test)]
mod tests {
  use std::time::{Duration, UNIX_EPOCH};

  use super::UtcTime;

  impl UtcTime {
    /// The instant `seconds` and `nanoseconds` after the Unix epoch.
    pub(crate) fn sample(seconds_since_unix_epoch: i64, nanoseconds: u32) -> UtcTime {
      UtcTime {
        seconds_since_unix_epoch,
        nanoseconds,
      }
    }
  }

  #[test]
  fn converts_to_a_system_time() {
    let time = UtcTime::sample(1_795_422_600, 250_000_000);
    assert_eq!(time.seconds_since_unix_epoch(), 1_795_422_600);
    assert_eq!(time.nanoseconds(), 250_000_000);
    assert_eq!(
      time.to_system_time(),
      UNIX_EPOCH.checked_add(Duration::new(1_795_422_600, 250_000_000))
    );
    let before = UtcTime::sample(-1, 500_000_000);
    assert_eq!(
      before.to_system_time(),
      UNIX_EPOCH.checked_sub(Duration::from_millis(500))
    );
    assert_eq!(UtcTime::sample(0, 0).to_system_time(), Some(UNIX_EPOCH));
  }

  /// One nanosecond after the epoch is finer than the 100-nanosecond intervals of Windows, and as fine as the
  /// `SystemTime` of the other platforms that CI runs.
  #[test]
  fn refuses_an_instant_finer_than_the_platform() {
    let finest = UtcTime::sample(0, 1);
    let before = UtcTime::sample(-1, 999_999_999);
    if cfg!(windows) {
      assert_eq!(finest.to_system_time(), None);
      assert_eq!(before.to_system_time(), None);
    } else {
      assert_eq!(finest.to_system_time(), UNIX_EPOCH.checked_add(Duration::from_nanos(1)));
      assert_eq!(before.to_system_time(), UNIX_EPOCH.checked_sub(Duration::from_nanos(1)));
    }
    let hundred = UtcTime::sample(0, 100).to_system_time();
    assert_eq!(hundred, UNIX_EPOCH.checked_add(Duration::from_nanos(100)));
  }

  #[test]
  fn orders_by_instant() {
    assert!(UtcTime::sample(-1, 999_999_999) < UtcTime::sample(0, 0));
    assert!(UtcTime::sample(5, 1) > UtcTime::sample(5, 0));
  }
}
