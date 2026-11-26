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

//! Records of the report model that contradict themselves (`api.md` sections 6.5, 6.6 and 6.8; field guide sections
//! 3.4 and 3.7), as the fourth review of the third slice's first part asked, which read an unordered edition with
//! playback values, an edition said to link no segment beside a chapter that links one, a two-channel stream with a
//! layout of one position, an unassigned position at the front left, and a common precision of 8 bits beside a luma
//! of 10.

use cc_media_probe::model::{AudioInformation, BitDepth, ChannelPosition, ChapterEdition};
use serde::de::DeserializeOwned;
use serde_json::{Value, json};

fn reads<Record: DeserializeOwned>(value: &Value) -> bool {
  serde_json::from_value::<Record>(value.clone()).is_ok()
}

fn seconds(value: u64) -> Value {
  json!({ "numerator": value, "denominator": 1 })
}

/// A Matroska edition of that order and link flag, whose second chapter, below the first, has those members.
fn edition(ordered: bool, linked: bool, child: &[(&str, Value)]) -> Value {
  let mut second = json!({ "parent": 0, "titles": [] });
  for (name, value) in child {
    second[*name] = value.clone();
  }
  json!({
    "origin": "matroska_edition",
    "ordered": ordered,
    "has_linked_chapters": linked,
    "chapters": [{ "titles": [] }, second],
  })
}

#[test]
fn an_unordered_edition_has_no_playback_values() {
  let position = [("playback_position", seconds(5))];
  assert!(reads::<ChapterEdition>(&edition(true, false, &position)));
  // The review's case: an unordered edition with a chapter's playback position and a derived playback duration.
  assert!(!reads::<ChapterEdition>(&edition(false, false, &position)));
  let mut timed = edition(false, false, &[]);
  assert!(reads::<ChapterEdition>(&timed));
  timed["playback_duration"] = seconds(10);
  timed["playback_duration_provenance"] = json!("derived");
  assert!(!reads::<ChapterEdition>(&timed));
  timed["ordered"] = json!(true);
  assert!(reads::<ChapterEdition>(&timed));
  // An edition whose order is unknown may have them.
  let mut unknown = edition(true, false, &position);
  unknown.as_object_mut().map(|members| members.remove("ordered"));
  assert!(reads::<ChapterEdition>(&unknown));
}

#[test]
fn an_edition_said_to_link_no_segment_has_no_chapter_that_links_one() {
  let link = [(
    "segment_reference",
    json!({ "segment": "0123456789abcdef0123456789abcdef" }),
  )];
  assert!(reads::<ChapterEdition>(&edition(true, true, &link)));
  // The review's case, with the link below the first chapter.
  assert!(!reads::<ChapterEdition>(&edition(true, false, &link)));
  // A link not read leaves the flag yes all the same.
  assert!(reads::<ChapterEdition>(&edition(true, true, &[])));
}

#[test]
fn a_layout_has_one_position_for_each_output_channel() {
  let position = |speaker: &str| json!({ "speaker": speaker, "unassigned": false });
  let audio = |count: u64, positions: Vec<Value>| {
    json!({
      "channel_count": count,
      "channel_count_provenance": "declared",
      "channel_layout": { "channels": positions },
      "channel_layout_provenance": "declared",
    })
  };
  let stereo = || vec![position("front_left"), position("front_right")];
  assert!(reads::<AudioInformation>(&audio(2, stereo())));
  // The review's case: a two-channel stream with a layout of one position.
  assert!(!reads::<AudioInformation>(&audio(2, vec![position("front_left")])));
  assert!(!reads::<AudioInformation>(&audio(1, stereo())));
  let unassigned = json!({ "unassigned": true });
  assert!(reads::<AudioInformation>(&audio(
    2,
    vec![position("front_left"), unassigned]
  )));
  let mut uncounted = audio(2, stereo());
  uncounted.as_object_mut().map(|members| {
    members.remove("channel_count");
    members.remove("channel_count_provenance")
  });
  assert!(reads::<AudioInformation>(&uncounted));
}

#[test]
fn an_unassigned_channel_names_no_position() {
  assert!(reads::<ChannelPosition>(
    &json!({ "unassigned": true, "ambisonic_component": 3 })
  ));
  // The review's case: an unassigned channel at the front left.
  assert!(!reads::<ChannelPosition>(
    &json!({ "unassigned": true, "speaker": "front_left" })
  ));
  assert!(!reads::<ChannelPosition>(
    &json!({ "unassigned": true, "azimuth": seconds(30) })
  ));
  assert!(!reads::<ChannelPosition>(
    &json!({ "unassigned": true, "elevation": seconds(30) })
  ));
  assert!(reads::<ChannelPosition>(
    &json!({ "unassigned": false, "speaker": "front_left" })
  ));
}

#[test]
fn every_component_shares_the_common_precision() {
  let depth = |common: u64, luma: u64| json!({ "common": common, "components": [{ "component": "luma", "bits": luma }, { "component": "chroma", "bits": luma }] });
  assert!(reads::<BitDepth>(&depth(10, 10)));
  // The review's case: a common precision of 8 beside a luma of 10 bits.
  assert!(!reads::<BitDepth>(&depth(8, 10)));
  assert!(reads::<BitDepth>(
    &json!({ "components": [{ "component": "luma", "bits": 8 }, { "component": "chroma", "bits": 9 }] })
  ));
}
