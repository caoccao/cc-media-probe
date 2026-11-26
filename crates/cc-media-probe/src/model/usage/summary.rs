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

//! `Usage`, what the budget accounted for (`api.md` section 6.14; plan section 7.5).

use crate::Limit;

/// What the budget accounted for during a probe, so that a caller can tune the limits from real numbers, from
/// `MediaReport::usage()`, `Detection::usage()` and `Error::usage()`.
///
/// ```
/// use cc_media_probe::{Limit, MediaReport};
///
/// fn usage(report: &MediaReport) {
///     let usage = report.usage();
///     println!(
///         "read {} bytes, {} bytes of memory at most, kept {} + {} bytes, inflated {} bytes, {} work units",
///         usage.bytes_read(),
///         usage.peak_working_memory(),
///         usage.retained_data(),
///         usage.retained_payloads(),
///         usage.decompressed_output(),
///         usage.work_units(),
///     );
///     println!("{} companion requests, {} sources", usage.companion_requests(), usage.sources_opened());
///     assert_eq!(usage.get(&Limit::BytesRead), Some(usage.bytes_read()));
///     assert_eq!(usage.get(&Limit::NestingDepth), None);
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Usage {
  bytes_read: u64,
  peak_working_memory: u64,
  retained_data: u64,
  retained_payloads: u64,
  decompressed_output: u64,
  work_units: u64,
  companion_requests: u64,
  sources_opened: u64,
}

impl Usage {
  /// The bytes read from every source together.
  #[must_use]
  pub fn bytes_read(&self) -> u64 {
    self.bytes_read
  }

  /// The most working memory that the probe held at any moment, in bytes.
  #[must_use]
  pub fn peak_working_memory(&self) -> u64 {
    self.peak_working_memory
  }

  /// The data that the report keeps, other than attachment payloads, in bytes.
  #[must_use]
  pub fn retained_data(&self) -> u64 {
    self.retained_data
  }

  /// The attachment payloads that the report keeps, in bytes.
  #[must_use]
  pub fn retained_payloads(&self) -> u64 {
    self.retained_payloads
  }

  /// The output of decompression, in bytes.
  #[must_use]
  pub fn decompressed_output(&self) -> u64 {
    self.decompressed_output
  }

  /// The units of work.
  #[must_use]
  pub fn work_units(&self) -> u64 {
    self.work_units
  }

  /// The requests for companions.
  #[must_use]
  pub fn companion_requests(&self) -> u64 {
    self.companion_requests
  }

  /// The sources opened.
  #[must_use]
  pub fn sources_opened(&self) -> u64 {
    self.sources_opened
  }

  /// The counter of `limit`, or `None` for a limit that has no usage value, such as the nesting depth, and for a
  /// limit that a newer version wrote. The working-memory limit gives the peak.
  #[must_use]
  pub fn get(&self, limit: &Limit) -> Option<u64> {
    match limit {
      Limit::BytesRead => Some(self.bytes_read),
      Limit::WorkingMemory => Some(self.peak_working_memory),
      Limit::RetainedData => Some(self.retained_data),
      Limit::RetainedPayloads => Some(self.retained_payloads),
      Limit::DecompressedOutput => Some(self.decompressed_output),
      Limit::WorkUnits => Some(self.work_units),
      Limit::CompanionRequests => Some(self.companion_requests),
      Limit::SourcesOpened => Some(self.sources_opened),
      _ => None,
    }
  }

  /// The usage of a probe that read nothing and held nothing.
  pub(crate) fn unused() -> Usage {
    Usage {
      bytes_read: 0,
      peak_working_memory: 0,
      retained_data: 0,
      retained_payloads: 0,
      decompressed_output: 0,
      work_units: 0,
      companion_requests: 0,
      sources_opened: 0,
    }
  }
}

/// A usage as written: each counter as a number, or beyond 2^53 - 1 as its twin (`json-output.md` sections 5.5 and 7.13).
#[cfg(feature = "serde")]
#[derive(serde::Serialize, serde::Deserialize)]
struct UsageForm {
  #[serde(skip_serializing_if = "Option::is_none")]
  bytes_read: Option<u64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  bytes_read_exact: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  peak_working_memory: Option<u64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  peak_working_memory_exact: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  retained_data: Option<u64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  retained_data_exact: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  retained_payloads: Option<u64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  retained_payloads_exact: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  decompressed_output: Option<u64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  decompressed_output_exact: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  work_units: Option<u64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  work_units_exact: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  companion_requests: Option<u64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  companion_requests_exact: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  sources_opened: Option<u64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  sources_opened_exact: Option<String>,
}

#[cfg(feature = "serde")]
impl serde::Serialize for Usage {
  fn serialize<Output: serde::Serializer>(&self, serializer: Output) -> Result<Output::Ok, Output::Error> {
    let twin = |counter| crate::form::unsigned_members(Some(counter));
    let (bytes_read, bytes_read_exact) = twin(self.bytes_read);
    let (peak_working_memory, peak_working_memory_exact) = twin(self.peak_working_memory);
    let (retained_data, retained_data_exact) = twin(self.retained_data);
    let (retained_payloads, retained_payloads_exact) = twin(self.retained_payloads);
    let (decompressed_output, decompressed_output_exact) = twin(self.decompressed_output);
    let (work_units, work_units_exact) = twin(self.work_units);
    let (companion_requests, companion_requests_exact) = twin(self.companion_requests);
    let (sources_opened, sources_opened_exact) = twin(self.sources_opened);
    let form = UsageForm {
      bytes_read,
      bytes_read_exact,
      peak_working_memory,
      peak_working_memory_exact,
      retained_data,
      retained_data_exact,
      retained_payloads,
      retained_payloads_exact,
      decompressed_output,
      decompressed_output_exact,
      work_units,
      work_units_exact,
      companion_requests,
      companion_requests_exact,
      sources_opened,
      sources_opened_exact,
    };
    serde::Serialize::serialize(&form, serializer)
  }
}

#[cfg(feature = "serde")]
impl TryFrom<UsageForm> for Usage {
  type Error = &'static str;

  /// Each counter from exactly one of its two members, each in its own range; every counter is required.
  fn try_from(form: UsageForm) -> Result<Usage, &'static str> {
    use crate::form::required_unsigned as counter;
    Ok(Usage {
      bytes_read: counter(form.bytes_read, form.bytes_read_exact)?,
      peak_working_memory: counter(form.peak_working_memory, form.peak_working_memory_exact)?,
      retained_data: counter(form.retained_data, form.retained_data_exact)?,
      retained_payloads: counter(form.retained_payloads, form.retained_payloads_exact)?,
      decompressed_output: counter(form.decompressed_output, form.decompressed_output_exact)?,
      work_units: counter(form.work_units, form.work_units_exact)?,
      companion_requests: counter(form.companion_requests, form.companion_requests_exact)?,
      sources_opened: counter(form.sources_opened, form.sources_opened_exact)?,
    })
  }
}

#[cfg(feature = "serde")]
crate::form::checked_form!(Usage from UsageForm);

#[cfg(test)]
mod tests {
  use super::Usage;
  use crate::Limit;
  use crate::model::UnrecognizedValue;

  impl Usage {
    /// The usage of a probe that read 4096 bytes.
    pub(crate) fn sample() -> Usage {
      Usage {
        bytes_read: 4_096,
        peak_working_memory: 65_536,
        retained_data: 1_024,
        retained_payloads: 0,
        decompressed_output: 0,
        work_units: 120,
        companion_requests: 0,
        sources_opened: 1,
      }
    }

    /// The usage of the sample with `bytes_read` bytes read.
    pub(crate) fn reading(bytes_read: u64) -> Usage {
      Usage {
        bytes_read,
        ..Usage::sample()
      }
    }
  }

  #[test]
  fn gives_each_counter_by_accessor_and_by_name() {
    let usage = Usage::sample();
    let counters = [
      (Limit::BytesRead, usage.bytes_read(), 4_096),
      (Limit::WorkingMemory, usage.peak_working_memory(), 65_536),
      (Limit::RetainedData, usage.retained_data(), 1_024),
      (Limit::RetainedPayloads, usage.retained_payloads(), 0),
      (Limit::DecompressedOutput, usage.decompressed_output(), 0),
      (Limit::WorkUnits, usage.work_units(), 120),
      (Limit::CompanionRequests, usage.companion_requests(), 0),
      (Limit::SourcesOpened, usage.sources_opened(), 1),
    ];
    for (limit, accessor, expected) in counters {
      assert_eq!(accessor, expected);
      assert_eq!(usage.get(&limit), Some(expected));
    }
    for limit in [
      Limit::HeaderBytesPerStream,
      Limit::NestingDepth,
      Limit::Streams,
      Limit::Diagnostics,
      Limit::Unrecognized(UnrecognizedValue::sample("threads")),
    ] {
      assert_eq!(usage.get(&limit), None);
    }
  }
}
