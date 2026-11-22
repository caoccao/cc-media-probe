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

use cc_media_probe::details::{CodecDetails, FormatDetails};
use cc_media_probe::{Error, probe_file};

fn main() -> Result<(), Error> {
  let report = probe_file("movie.mkv")?;
  // The container's own facts. Its per-track details are found by stream index.
  if let Some(FormatDetails::Matroska(matroska)) = report.format_details() {
    let writer = matroska.writing_application().unwrap_or("not stated");
    println!("DocType {}, WritingApp {writer}", matroska.document_type());
    for stream in report.streams() {
      if let Some(track) = matroska.track(stream.index()) {
        let flag = track.flag_default();
        let origin = if flag.is_stored() { "stored" } else { "the default" };
        println!(
          "#{}: TrackNumber {}, FlagDefault {} ({origin})",
          stream.index(),
          track.track_number(),
          flag.value()
        );
      }
    }
  }
  for stream in report.streams() {
    // The coded values of the set that supplied the normalized profile and level, which are in `stream.codec()`.
    let Some(CodecDetails::H264(h264)) = stream.codec_details() else {
      continue;
    };
    if let Some(set) = h264.primary_sequence_parameter_set() {
      // A set cut short keeps the fields read before the cut.
      match (set.profile_indication(), set.level_indication()) {
        (Some(profile), Some(level)) => println!("#{}: profile_idc {profile}, level_idc {level}", stream.index()),
        (Some(profile), None) => println!("#{}: profile_idc {profile}, cut before level_idc", stream.index()),
        _ => println!("#{}: cut before profile_idc", stream.index()),
      }
    }
  }
  // Conflicts are reached the same way for every format, without matching on its variant.
  for conflict in report.format_details().into_iter().flat_map(FormatDetails::conflicts) {
    let values: Vec<String> = conflict
      .values()
      .iter()
      .map(|value| format!("{} says {}", value.source(), value.value()))
      .collect();
    println!("{} disagrees: {}", conflict.field(), values.join("; "));
  }
  Ok(())
}
