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

//! The writing of an expectation file: whole, by a rename of a file of its own over the old one, and only when its
//! text changes, as the third review of plan Phase 3, task 4, asked.

use std::fs;

use crate::common::expectation::replace;
use crate::folder::case_folder;

#[test]
fn an_expectation_is_written_whole_by_a_rename_and_only_when_its_text_changes() {
  let folder = case_folder();
  let path = folder.path.join("written.expected.json");
  replace(&path, "1\n");
  assert_eq!(fs::read_to_string(&path).unwrap(), "1\n");
  let first = fs::metadata(&path).unwrap();
  replace(&path, "1\n");
  let unchanged = fs::metadata(&path).unwrap();
  assert_eq!(unchanged.modified().unwrap(), first.modified().unwrap());
  replace(&path, "2\n");
  assert_eq!(fs::read_to_string(&path).unwrap(), "2\n");
  #[cfg(unix)]
  {
    use std::os::unix::fs::MetadataExt;

    // The same file when the text was the same; another file in its place once it changed, where `fs::write`
    // truncated the file and wrote it again, so that a concurrent reader could meet it empty.
    assert_eq!(unchanged.ino(), first.ino());
    assert_ne!(fs::metadata(&path).unwrap().ino(), first.ino());
  }
}
