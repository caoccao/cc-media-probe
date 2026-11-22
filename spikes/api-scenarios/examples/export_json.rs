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

  // Several inputs in the document form that the CLI writes, an error among them.
  let mut document = Document::new(Generator::new("media-catalog", env!("CARGO_PKG_VERSION")));
  for (index, path) in ["movie.mkv", "notes.txt"].into_iter().enumerate() {
    let input = PortablePath::from_path(Path::new(path));
    document.push(match probe_file(path) {
      Ok(report) => DocumentEntry::from_report(index, input, report),
      Err(error) => DocumentEntry::from_error(index, input, error.to_record()),
    });
  }
  let text = serde_json::to_string_pretty(&document)?;
  println!("{text}");

  // Read back, a document gives each entry's input and what it holds. Each nested record has the document's version.
  let document: Document = serde_json::from_str(&text)?;
  for entry in document.results() {
    let input = entry.input().display();
    match (entry.outcome(), entry.report(), entry.error()) {
      (EntryOutcome::Ok | EntryOutcome::Partial, Some(report), _) => {
        println!("{input}: {}, {}", report.format(), report.completeness())
      }
      (EntryOutcome::Error, _, Some(record)) => println!("{input}: {}", record.message()),
      (outcome, _, _) => println!("{input}: {}, from a newer version", outcome.as_str()),
    }
  }
  Ok(())
}
