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

//! The twelfth review's case, `api.md` section 11.5: the H.264 details keep the picture parameter sets read and the
//! links that the first picture's activation read, so a caller can see why a slice's reference resolved to nothing.

use cc_media_probe::details::h264::{H264Details, H264ParameterSetOrigin};
use serde_json::json;

#[test]
fn the_details_keep_the_picture_parameter_sets_and_the_links_that_were_read() {
  // An `avc1` record whose only sequence parameter set is 0, with a picture parameter set that names set 1, and a
  // second picture parameter set from a linked parameter-set track, cut short after its own identifier.
  let details: H264Details = serde_json::from_value(json!({
    "sequence_parameter_sets": [{
      "origin": "ConfigurationRecord",
      "profile_indication": 100,
      "level_indication": 31,
      "sequence_parameter_set_identifier": 0,
    }],
    "picture_parameter_sets": [
      {
        "origin": "ConfigurationRecord",
        "picture_parameter_set_identifier": 0,
        "sequence_parameter_set_identifier": 1,
      },
      { "origin": "ParameterSetTrack", "picture_parameter_set_identifier": 2 },
    ],
    "activation": { "picture_parameter_set_identifier": 0 },
    "conflicts": [],
  }))
  .unwrap();
  let set = &details.sequence_parameter_sets()[0];
  assert_eq!(set.sequence_parameter_set_identifier(), Some(0));
  assert_eq!(set.origin(), &H264ParameterSetOrigin::ConfigurationRecord);
  let pictures = details.picture_parameter_sets();
  assert_eq!(pictures[0].picture_parameter_set_identifier(), Some(0));
  assert_eq!(pictures[0].sequence_parameter_set_identifier(), Some(1));
  assert_eq!(pictures[1].origin(), &H264ParameterSetOrigin::ParameterSetTrack);
  assert_eq!(pictures[1].sequence_parameter_set_identifier(), None, "cut short");
  let activation = details.activation().unwrap();
  assert_eq!(activation.picture_parameter_set_identifier(), Some(0));
  assert_eq!(activation.buffering_period_sequence_parameter_set_identifier(), None);
  assert!(details.primary_sequence_parameter_set().is_none());
}
