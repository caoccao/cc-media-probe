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

//! Runs the placeholder binary, so the tool meets the coverage bar from its
//! first commit (R28).

use std::process::Command;

#[test]
fn the_binary_prints_its_name_and_version() {
  let output = Command::new(env!("CARGO_BIN_EXE_cc-media-probe")).output().unwrap();
  assert!(output.status.success());
  let expected = format!("cc-media-probe {}\n", env!("CARGO_PKG_VERSION"));
  assert_eq!(String::from_utf8(output.stdout).unwrap(), expected);
}
