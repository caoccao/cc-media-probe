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

//! Scenario 6: export JSON, on its own and in the CLI's document form. Needs the `serde` feature.

use std::io::{self, Write};
use std::path::Path;

use cc_media_probe::document::{Document, DocumentEntry, EntryOutcome, Generator};
use cc_media_probe::source::PortablePath;
use cc_media_probe::{MediaReport, probe_file};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  // A report is a root record: it carries `schema_version`.
  match probe_file("movie.mkv") {
    Ok(report) => {
      let text = serde_json::to_string_pretty(&report)?;
      // Reading checks the version before anything else, so a report of another major version fails here.
      let again: MediaReport = serde_json::from_str(&text)?;
      assert_eq!(again, report);
    }
    Err(error) => eprintln!("movie.mkv: {error}"),
  }

  // Several inputs in the document form that the CLI writes, an error among them. The writer takes one entry at a
  // time and releases it before the next input is probed, so a scan of any length holds one report at a time. The
  // document writes each entry's position as its index.
  let entries = ["movie.mkv", "notes.txt"].into_iter().map(|path| {
    let input = PortablePath::from_path(Path::new(path));
    match probe_file(path) {
      Ok(report) => report_entry(input, report),
      Err(error) => DocumentEntry::from_error(input, error.to_record()),
    }
  });
  let generator = Generator::new("media-catalog", env!("CARGO_PKG_VERSION"));
  // Standard output carries the document and nothing else, so it stays valid JSON. An output error stops the scan,
  // and so does one in the final line break or the flush, which return their errors as the document does.
  let mut output = io::stdout().lock();
  Document::serialize_results(&generator, entries, &mut serde_json::Serializer::pretty(&mut output))?;
  writeln!(output)?;
  output.flush()?;

  // An earlier catalog, read back, gives each entry's input and what it holds. Each nested record has the
  // document's version. Only a missing catalog is skipped; one that cannot be read is an error.
  let text = match std::fs::read_to_string("catalog.json") {
    Ok(text) => text,
    Err(error) if error.kind() == io::ErrorKind::NotFound => {
      eprintln!("catalog.json: none yet");
      return Ok(());
    }
    Err(error) => return Err(error.into()),
  };
  let document: Document = serde_json::from_str(&text)?;
  for entry in document.results() {
    let input = entry.input().display();
    match (entry.outcome(), entry.report(), entry.detection(), entry.error()) {
      (EntryOutcome::Unrecognized(outcome), ..) => eprintln!("{input}: {}, from a newer version", outcome.as_str()),
      (_, Some(report), _, _) => eprintln!("{input}: {}, {}", report.format(), report.completeness()),
      (outcome, _, Some(detection), _) => eprintln!("{input}: {}, {}", outcome.as_str(), detection.outcome().as_str()),
      (_, _, _, Some(record)) => eprintln!("{input}: {}", record.message()),
      // Reading checks that an entry of a known outcome holds exactly one record, so nothing else remains.
      (_, None, None, None) => {}
    }
  }
  Ok(())
}

/// The entry of a report. This version's own probe always gives a report whose outcome it derives. Only a report
/// read from a newer version can be refused, when only a status of that version decides its outcome; it then takes
/// an outcome that the caller gives. `with_outcome` refuses `error`, which no report allows, and an outcome of a
/// newer version, which only the entry as read can pass on.
fn report_entry(input: PortablePath, report: MediaReport) -> DocumentEntry {
  DocumentEntry::from_report(input, report)
    .or_else(|refused| refused.with_outcome(EntryOutcome::Partial))
    .expect("every report allows the outcome partial")
}
