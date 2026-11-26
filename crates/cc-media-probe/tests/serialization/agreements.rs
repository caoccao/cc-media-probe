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

//! The members of a record that the second review of the third slice's first part found to disagree (`api.md`
//! sections 6.3, 6.4 and 6.9): a declaration of the codec's private data and a clear configuration; the raw
//! identifiers of a codec and those of its candidates; and the location of a target of a newer kind in a tag set.

use cc_media_probe::model::{CodecInformation, Protection, TagSet};
use serde::de::DeserializeOwned;
use serde_json::{Value, json};

fn reads<Read: DeserializeOwned>(value: &Value) -> bool {
  serde_json::from_value::<Read>(value.clone()).is_ok()
}

/// A Matroska track's protection examined in full, with declarations of these scopes and that clarity.
fn protection(scopes: &[&str], fully_examined: bool, clear: Option<bool>) -> Value {
  let declarations: Vec<Value> = scopes
    .iter()
    .map(|scope| {
      json!({ "scheme": { "namespace": "matroska_content_encryption_algorithm", "value": "5" },
      "scope": scope })
    })
    .collect();
  let mut protection = json!({
    "declarations": declarations,
    "fully_examined": fully_examined,
    "has_sample_descriptions": false,
    "sample_descriptions": [],
  });
  if let Some(clear) = clear {
    protection["configuration_clear"] = json!(clear);
  }
  protection
}

#[test]
fn a_declaration_of_the_private_data_protects_the_configuration() {
  for (scopes, fully_examined, clear) in [
    (&["packets"][..], true, Some(true)),
    (&["codec_private_data"][..], true, Some(false)),
    (&["packets", "codec_private_data"][..], false, Some(false)),
    (&["unread"][..], false, None),
    (&["unread", "codec_private_data"][..], false, Some(false)),
    (&["future_scope"][..], true, Some(true)),
  ] {
    let value = protection(scopes, fully_examined, clear);
    assert!(reads::<Protection>(&value), "{value}");
  }
  for (scopes, fully_examined, clear) in [
    // The review's case: a declaration of the private data beside a clear configuration.
    (&["codec_private_data"][..], true, Some(true)),
    (&["codec_private_data"][..], false, None),
    (&["unread"][..], false, Some(true)),
  ] {
    let value = protection(scopes, fully_examined, clear);
    assert!(!reads::<Protection>(&value), "{value}");
  }
}

fn identifier(value: &str) -> Value {
  json!({ "namespace": "iso_sample_entry", "value": value })
}

fn information(codec: Option<&str>, candidates: Vec<Value>, identifiers: Vec<Value>) -> Value {
  let mut information = json!({ "has_every_candidate": true });
  information["candidates"] = Value::Array(candidates);
  information["raw_identifiers"] = Value::Array(identifiers);
  if let Some(codec) = codec {
    information["codec"] = json!(codec);
  }
  information
}

#[test]
fn the_raw_identifiers_of_a_codec_are_its_candidates_in_their_order() {
  let avc = |index: u64, code: &str| json!({ "sample_description_index": index, "codec": "h264", "raw_identifiers": [identifier(code)] });
  for value in [
    information(Some("h264"), vec![avc(1, "61766331")], vec![identifier("61766331")]),
    information(
      Some("h264"),
      vec![avc(1, "61766331"), avc(2, "61766333")],
      vec![identifier("61766331"), identifier("61766333")],
    ),
    information(Some("h264"), Vec::new(), vec![identifier("61766331")]),
    information(None, Vec::new(), Vec::new()),
  ] {
    assert!(reads::<CodecInformation>(&value), "{value}");
  }
  for value in [
    // The review's case: one H.264 candidate with the code of `avc1` beside no raw identifier.
    information(Some("h264"), vec![avc(1, "61766331")], Vec::new()),
    information(
      Some("h264"),
      vec![avc(1, "61766331"), avc(2, "61766333")],
      vec![identifier("61766333"), identifier("61766331")],
    ),
    information(None, Vec::new(), vec![identifier("61766331")]),
  ] {
    assert!(!reads::<CodecInformation>(&value), "{value}");
  }
}

#[test]
fn a_target_of_a_newer_kind_is_not_held_to_its_lists_kind() {
  let set = |kind: &str| {
    json!({
      "stream_targets": [{ "kind": kind, "location": { "kind": "edition", "index": 0, "chapter_path": [] } }],
      "edition_targets": [],
      "chapter_targets": [],
      "attachment_targets": [],
      "target_scope": "entities",
      "superseded": false,
      "tags": [],
    })
  };
  // The review's case: the location of a target of a newer kind, which only its version understands.
  assert!(reads::<TagSet>(&set("future_kind")));
  assert!(!reads::<TagSet>(&set("resolved")));
}
