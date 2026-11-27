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
//! Scenario 10: read format-specific and codec-specific details.
//!
//! Each format and codec adds its own variant of `FormatDetails` and `CodecDetails` in the release that reads it,
//! and a caller matches the variants it knows. What every kind shares is read without matching on a variant: its
//! stable name, and the three lists that each kind keeps beside its own fields. A kind that a newer version wrote
//! keeps its name, and keeps its members as written but unread, its lists among them: its three lists are always
//! empty, which says nothing about what it recorded.

use cc_media_probe::details::{CodecDetails, Conflict, ExactValue, FormatDetails, ObservedBound, ObservedValue};
use cc_media_probe::{Error, probe_file};

fn main() -> Result<(), Error> {
  let report = probe_file("movie.mkv")?;
  // The container's own facts.
  if let Some(details) = report.format_details() {
    println!("format details: {}", details.as_str());
    if matches!(details, FormatDetails::Unrecognized(_)) {
      println!("  a kind of a newer version, whose lists this version does not read");
    } else {
      print_lists(
        details.conflicts(),
        details.observed_lower_bounds(),
        details.exact_values(),
      );
    }
  }
  // The coded values of each stream's codec.
  for stream in report.streams() {
    if let Some(details) = stream.codec_details() {
      println!("#{}: codec details {}", stream.index(), details.as_str());
      if matches!(details, CodecDetails::Unrecognized(_)) {
        println!("  a kind of a newer version, whose lists this version does not read");
      } else {
        print_lists(
          details.conflicts(),
          details.observed_lower_bounds(),
          details.exact_values(),
        );
      }
    }
  }
  Ok(())
}

/// The three lists that every kind of details keeps, the same for formats and codecs.
fn print_lists(conflicts: &[Conflict], bounds: &[ObservedBound], exact_values: &[ExactValue]) {
  // Two sources that disagree keep their raw values.
  for conflict in conflicts {
    let values: Vec<String> = conflict
      .values()
      .iter()
      .map(|value| format!("{} says {}", value.source(), value.value()))
      .collect();
    println!("  {} disagrees: {}", conflict.field(), values.join("; "));
  }
  // A count or a time observed while its total was not established: a partial count is never a total.
  for bound in bounds {
    match bound.value() {
      ObservedValue::Count(count) => println!("  {} is at least {count}", bound.field()),
      ObservedValue::Time(time) => println!("  {} is at least {time}", bound.field()),
      // A value of a kind that a newer version wrote: its name is known, its value is not read.
      other => println!("  {} has a lower bound of the kind {}", bound.field(), other.as_str()),
    }
  }
  // A value beyond the exact range of the serialized form, kept as decimal text.
  for value in exact_values {
    match value.denominator() {
      Some(denominator) => println!("  {} is exactly {}/{denominator}", value.field(), value.numerator()),
      None => println!("  {} is exactly {}", value.field(), value.numerator()),
    }
  }
}
