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

//! `detect`, detection over the internal source.

use std::io;

use super::{Detected, judge_first_element, judge_header};
use crate::budget::Budget;
use crate::source::{InternalSource, fill};
use crate::toy::constants::{ELEMENT_HEADER_LENGTH, FILE_HEADER_LENGTH};

/// Reads the file header and the first element's header, and ends detection, so a forward-only input keeps what
/// was read for the walker.
pub(crate) async fn detect<Source: InternalSource>(source: &mut Source, budget: &mut Budget) -> io::Result<Detected> {
  let mut header = [0_u8; FILE_HEADER_LENGTH];
  let filled = fill!(source, budget, 0, &mut header)?;
  let detected = match judge_header(&filled, &header) {
    Ok(first_element) => {
      let mut element = [0_u8; ELEMENT_HEADER_LENGTH];
      let filled = fill!(source, budget, first_element, &mut element)?;
      judge_first_element(&filled, element, first_element)
    }
    Err(outcome) => Detected::Concluded(outcome),
  };
  source.end_detection();
  Ok(detected)
}
