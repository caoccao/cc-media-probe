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

//! Scenario 1, the quick start: list the streams of a file. `print_report` is its rendering half, which the tests
//! below call with a complete and an incomplete report (plan section 9.7).

use std::io::{self, Write};

use cc_media_probe::{MediaReport, probe_file};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let report = probe_file("movie.mkv")?;
  print_report(&mut io::stdout().lock(), &report)?;
  Ok(())
}

/// Writes what the report holds. Every value that a file may not state is an `Option`, so each one says what an
/// absent value means here.
fn print_report(output: &mut impl Write, report: &MediaReport) -> io::Result<()> {
  writeln!(output, "{}", report.format())?;
  if let Some(media_type) = report.media_type() {
    writeln!(output, "media type: {media_type}")?;
  }
  match report.duration() {
    Some(duration) => writeln!(output, "duration: {duration}")?,
    None => writeln!(output, "duration: unknown")?,
  }
  for stream in report.streams() {
    // The codec displays its name, or for an unknown codec the identifier that the file wrote.
    write!(
      output,
      "#{} {} {}",
      stream.index(),
      stream.kind().as_str(),
      stream.codec()
    )?;
    if let Some(tag) = stream.language().and_then(|language| language.tag()) {
      write!(output, " [{tag}]")?;
    }
    // A title is the file's own text: written as a quoted string, its quotes and control characters escaped, it
    // stays on its line.
    if let Some(title) = stream.title() {
      write!(output, " {title:?}")?;
    }
    // `video()` and `audio()` are present exactly for their kinds, but every field inside them may be absent.
    if let Some(video) = stream.video() {
      match video.display_size() {
        Some(size) => write!(output, ", {size}")?,
        None => write!(output, ", size unknown")?,
      }
    }
    if let Some(audio) = stream.audio() {
      match audio.sample_rate() {
        Some(rate) => write!(output, ", {rate} Hz")?,
        None => write!(output, ", sample rate unknown")?,
      }
      match audio.channel_count() {
        Some(channels) => write!(output, ", {channels} channels")?,
        None => write!(output, ", channels unknown")?,
      }
    }
    writeln!(output)?;
    // A header that was not parsed says why.
    if !stream.header_status().is_parsed() {
      writeln!(output, "    headers: {}", stream.header_status())?;
    }
  }
  // An incomplete report names what stopped it.
  if !report.completeness().is_complete() {
    writeln!(output, "note: {}", report.completeness())?;
  }
  Ok(())
}

#[cfg(test)]
#[cfg(feature = "serde")]
mod tests {
  use cc_media_probe::MediaReport;
  use serde_json::{Value, json};

  use super::print_report;

  /// What `print_report` writes for the report.
  fn rendered(report: &Value) -> String {
    let report: MediaReport = serde_json::from_value(report.clone()).unwrap();
    let mut output = Vec::new();
    print_report(&mut output, &report).unwrap();
    String::from_utf8(output).unwrap()
  }

  /// A report of that media type, duration, streams and completeness.
  fn report(head: Value, streams: &Value, completeness: &Value) -> Value {
    let mut report = json!({
      "schema_version": "1.0",
      "format": "matroska",
      "streams": streams,
      "programs": [],
      "chapter_editions": [],
      "tag_sets": [],
      "attachments": [],
      "external_references": [],
      "diagnostics": [],
      "completeness": completeness,
      "inventory_class": "declared",
      "usage": {
        "bytes_read": 4096,
        "peak_working_memory": 0,
        "retained_data": 0,
        "retained_payloads": 0,
        "decompressed_output": 0,
        "work_units": 0,
        "companion_requests": 0,
        "sources_opened": 0,
      },
      "sources": [{ "identifier": "0", "role": "primary", "outcome": "opened" }],
    });
    if let (Value::Object(report), Value::Object(head)) = (&mut report, head) {
      report.extend(head);
    }
    report
  }

  /// A completeness whose headers have that status, with those reasons, the first of which the record keeps.
  fn completeness(headers: &str, reasons: &Value) -> Value {
    let first_reasons = reasons.get(0).map_or_else(|| json!([]), |first| json!([first]));
    json!({
      "detection": "complete",
      "discovery": "complete",
      "headers": headers,
      "tags": "complete",
      "chapters": "complete",
      "attachments": "complete",
      "timing": "established",
      "companions": "not_applicable",
      "reasons": reasons,
      "record": {
        "exhausted": [],
        "first_stopped": [],
        "first_reasons": first_reasons,
        "omitted_diagnostics": "0",
        "omitted_declarations": "0",
        "omitted_reasons": "0",
        "omitted_references": "0",
      },
    })
  }

  /// Gives the stream the language German and that title in it, in the lists of every language and every title too,
  /// from which the primary language and the convenience title come (field guide section 3.2).
  fn titled(stream: &mut Value, title: &str) {
    let german = json!({ "as_written": "ger", "tag": "de", "code_system": "iso639_alpha3" });
    stream["language"] = german.clone();
    stream["languages"] = json!([{ "language": german, "roles": [] }]);
    stream["title"] = json!(title);
    stream["titles"] = json!([{ "text": title, "languages": [german], "countries": [] }]);
  }

  /// A stream of that index, kind and codec, with that header status and the information of its kind.
  fn stream(index: usize, kind: &str, codec: &str, header_status: &Value, information: Value) -> Value {
    let mut stream = json!({
      "index": index,
      "kind": kind,
      "codec": {
        "codec": codec,
        "candidates": [],
        "has_every_candidate": true,
        "raw_identifiers": [],
      },
      "header_status": header_status,
      "languages": [],
      "titles": [],
      "disposition": {},
    });
    stream[kind] = information;
    stream
  }

  #[test]
  fn a_complete_report_renders_every_value() {
    let parsed = json!({ "state": "parsed" });
    let video = json!({
      "display_size": { "width": 1920, "height": 1080 },
      "display_size_provenance": "declared",
      "frame_rate_mode": "constant",
    });
    let audio = json!({
      "sample_rate": { "numerator": 48000, "denominator": 1 },
      "sample_rate_provenance": "declared",
      "channel_count": 2,
      "channel_count_provenance": "declared",
    });
    let mut audio_stream = stream(1, "audio", "opus", &parsed, audio);
    titled(&mut audio_stream, "Commentary");
    let complete = report(
      json!({
        "media_type": "video/x-matroska",
        "duration": { "numerator": 21, "denominator": 2 },
        "duration_provenance": "declared",
      }),
      &json!([stream(0, "video", "h264", &parsed, video), audio_stream]),
      &completeness("complete", &json!([])),
    );
    let expected = "\
Matroska
media type: video/x-matroska
duration: 10.5 s
#0 video H.264, 1920x1080
#1 audio Opus [de] \"Commentary\", 48000 Hz, 2 channels
";
    assert_eq!(rendered(&complete), expected);
  }

  #[test]
  fn a_title_keeps_its_quotes_and_its_lines_escaped_on_one_line() {
    let parsed = json!({ "state": "parsed" });
    let audio = json!({
      "sample_rate": { "numerator": 48000, "denominator": 1 },
      "sample_rate_provenance": "declared",
    });
    let mut audio_stream = stream(0, "audio", "opus", &parsed, audio);
    titled(&mut audio_stream, "The \"director's\" cut\nPart 2\tend");
    let complete = report(json!({}), &json!([audio_stream]), &completeness("complete", &json!([])));
    let expected = "\
Matroska
duration: unknown
#0 audio Opus [de] \"The \\\"director's\\\" cut\\nPart 2\\tend\", 48000 Hz, channels unknown
";
    assert_eq!(rendered(&complete), expected);
  }

  #[test]
  fn an_incomplete_report_renders_each_absent_value() {
    let truncated = json!({ "kind": "truncated_input" });
    let cut = json!({ "state": "not_found_within_budget", "reason": truncated });
    let video = json!({ "frame_rate_mode": "unknown" });
    let reasons = json!([
      { "part": "headers", "stream": 0, "reason": truncated },
      { "part": "headers", "stream": 1, "reason": truncated },
    ]);
    let incomplete = report(
      json!({}),
      &json!([
        stream(0, "video", "h264", &cut, video),
        stream(1, "audio", "opus", &cut, json!({})),
      ]),
      &completeness("incomplete", &reasons),
    );
    let expected = "\
Matroska
duration: unknown
#0 video H.264, size unknown
    headers: not found within the budget (truncated input)
#1 audio Opus, sample rate unknown, channels unknown
    headers: not found within the budget (truncated input)
note: incomplete: headers stopped by truncated input
";
    assert_eq!(rendered(&incomplete), expected);
  }
}
