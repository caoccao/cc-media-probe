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

use cc_media_probe::document::{Document, DocumentEntry, Generator};
use cc_media_probe::source::PortablePath;
use cc_media_probe::{MediaReport, probe_file};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  // A report is a root record: it carries `schema_version`.
  let report = probe_file("movie.mkv")?;
  let text = serde_json::to_string_pretty(&report)?;
  // Reading checks the version before anything else, so a report of another major version fails here.
  let again: MediaReport = serde_json::from_str(&text)?;
  assert_eq!(again, report);

  // Several inputs in the document form that the CLI writes, an error among them.
  let mut document = Document::new(Generator::new("media-catalog", env!("CARGO_PKG_VERSION")));
  for (index, path) in ["movie.mkv", "notes.txt"].into_iter().enumerate() {
    let input = PortablePath::from_path(Path::new(path));
    document.push(match probe_file(path) {
      Ok(report) => DocumentEntry::report(index, input, report),
      Err(error) => DocumentEntry::error(index, input, error.to_record()),
    });
  }
  serde_json::to_writer_pretty(std::io::stdout().lock(), &document)?;
  Ok(())
}
