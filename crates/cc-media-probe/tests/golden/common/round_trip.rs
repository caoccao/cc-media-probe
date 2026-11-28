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

//! A record's text read back by the library's strict reader, the round trip of plan section 8.8 for every fixture:
//! each probe's result is written as text, which must read back as the record that it is and write again as the
//! same text, before the golden pass compares it or writes it. The pass converted each result to a JSON value
//! directly before the second review of plan Phase 3, task 4, found that a writer whose output the reader refused,
//! or read as another record, would then have been accepted, also when expectations were written.

use std::fmt::Debug;

use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;

/// The JSON value of `record`'s text, once that text reads back as `record` and is written again as itself, or why
/// it is not.
pub fn read_back<Record>(record: &Record) -> Result<Value, String>
where
  Record: Serialize + DeserializeOwned + PartialEq + Debug,
{
  let text = serde_json::to_string(record).map_err(|error| format!("{error}: {record:?}"))?;
  let read: Record =
    serde_json::from_str(&text).map_err(|error| format!("the strict reader refuses the text: {error}: {text}"))?;
  if read != *record {
    return Err(format!("the text reads back as another record: {read:?}: {text}"));
  }
  let again = serde_json::to_string(&read).map_err(|error| format!("{error}: {read:?}"))?;
  if again != text {
    return Err(format!(
      "the record read back is written as other text: {again}: {text}"
    ));
  }
  serde_json::from_str(&text).map_err(|error| format!("{error}: {text}"))
}
