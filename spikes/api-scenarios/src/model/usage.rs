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

//! The usage summary, `api.md` section 6.14, with two of its counters.

/// What the budget accounted for. Each counter is a quantity that the record cannot do without, so beyond 2^53 - 1
/// it is its twin `_exact` (`json-output.md` section 5.5; fourteenth review).
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(into = "UsageData"))]
pub struct Usage {
  bytes_read: u64,
  peak_working_memory: u64,
}

#[cfg(feature = "serde")]
crate::neutral::checked_record_traits!(Usage from UsageData);

/// The counters as written, each as its number or its twin.
#[cfg(feature = "serde")]
#[derive(serde::Serialize, serde::Deserialize)]
struct UsageData {
  #[serde(default, skip_serializing_if = "Option::is_none")]
  bytes_read: Option<u64>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  bytes_read_exact: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  peak_working_memory: Option<u64>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  peak_working_memory_exact: Option<String>,
}

#[cfg(feature = "serde")]
impl TryFrom<UsageData> for Usage {
  type Error = crate::document::InvalidDocument;

  /// Each counter from exactly one of its number and its twin, each in its own range.
  fn try_from(read: UsageData) -> Result<Usage, crate::document::InvalidDocument> {
    let counter = |number, twin| {
      crate::twin::unsigned_value(number, twin)?
        .ok_or(crate::document::InvalidDocument::new(crate::constants::TWIN_MESSAGE))
    };
    Ok(Usage {
      bytes_read: counter(read.bytes_read, read.bytes_read_exact)?,
      peak_working_memory: counter(read.peak_working_memory, read.peak_working_memory_exact)?,
    })
  }
}

#[cfg(feature = "serde")]
impl From<Usage> for UsageData {
  fn from(usage: Usage) -> UsageData {
    let (bytes_read, bytes_read_exact) = crate::twin::unsigned_members(Some(usage.bytes_read));
    let (peak_working_memory, peak_working_memory_exact) =
      crate::twin::unsigned_members(Some(usage.peak_working_memory));
    UsageData {
      bytes_read,
      bytes_read_exact,
      peak_working_memory,
      peak_working_memory_exact,
    }
  }
}

impl Usage {
  /// The bytes read from every source.
  pub fn bytes_read(&self) -> u64 {
    self.bytes_read
  }

  /// The highest working memory in use at one time.
  pub fn peak_working_memory(&self) -> u64 {
    self.peak_working_memory
  }
}
