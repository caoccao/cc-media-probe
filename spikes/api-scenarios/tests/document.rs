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

//! The read side of `api.md` section 12: a document read back gives its entries through their accessors.

use std::path::Path;

use cc_media_probe::document::{Document, DocumentEntry, EntryOutcome, Generator, SchemaVersion};
use cc_media_probe::probe_bytes;
use cc_media_probe::source::PortablePath;

#[test]
fn a_document_read_back_gives_its_entries() {
  let mut document = Document::new(Generator::new("catalog", "1.2.3"));
  for (index, bytes) in [&b""[..], &b"not media"[..]].into_iter().enumerate() {
    let record = probe_bytes(bytes).unwrap_err().to_record();
    let input = PortablePath::from_path(Path::new("input.bin"));
    document.push(DocumentEntry::from_error(index, input, record));
  }
  let text = serde_json::to_string(&document).unwrap();
  let read: Document = serde_json::from_str(&text).unwrap();
  assert_eq!(read, document);
  assert_eq!(read.schema_version(), SchemaVersion::CURRENT);
  assert_eq!(read.generator().name(), "catalog");
  assert_eq!(read.generator().version(), "1.2.3");
  assert_eq!(read.usage_error(), None);
  let mut kinds = Vec::new();
  for (index, entry) in read.results().iter().enumerate() {
    assert_eq!(entry.index(), index);
    assert_eq!(entry.input().display(), "input.bin");
    assert_eq!(entry.outcome(), &EntryOutcome::Error);
    assert!(entry.report().is_none());
    kinds.push(entry.error().unwrap().kind().as_str().to_owned());
  }
  assert_eq!(kinds, ["format_not_determined", "unsupported_format"]);
}
