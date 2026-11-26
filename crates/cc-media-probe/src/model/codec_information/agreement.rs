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

//! The agreement of a stream's codec with its candidates (`api.md` section 6.4), which the review of the third
//! slice's first part found unchecked: H.264 candidates beside Opus, and candidates out of index order. The
//! candidates rise strictly in their sample description's index, each from 1. While the first sample's description
//! is unproven, the codec is the candidates' one codec when every candidate is listed and they name one, two
//! candidates of `unknown` only with the same raw identifiers, and absent otherwise. A codec that the fields describe
//! without candidates is absent while a description that they need is missing. Without a codec, no field that a
//! codec supplies is present: a profile, a level, a tier or a codec string.
//!
//! The raw identifiers agree with the candidates too, as the second review of that part asked, which read one H.264
//! candidate with the sample-entry code of `avc1` beside no raw identifier: while candidates are listed, the
//! identifiers are every candidate's, in candidate order; without candidates and without a codec, there are none,
//! since the input named no codec or the description that names it was not read or kept (field guide section 3.3).

use super::{CodecCandidate, CodecInformation};
use crate::Codec;

/// The codec that the candidates decide: their one codec when every candidate is listed and they name one.
fn candidates_codec(information: &CodecInformation) -> Option<&Codec> {
  let (first, others) = information.candidates().split_first()?;
  let one = others.iter().all(|candidate| {
    candidate.codec() == first.codec()
      && (!matches!(first.codec(), Codec::Unknown) || candidate.raw_identifiers() == first.raw_identifiers())
  });
  (information.has_every_candidate() && one).then(|| first.codec())
}

/// The checks of a codec's information against its candidates.
pub(super) fn check_codec(information: &CodecInformation) -> Result<(), &'static str> {
  let mut previous = 0;
  let rising = information.candidates().iter().all(|candidate: &CodecCandidate| {
    let rises = candidate.sample_description_index() > previous;
    previous = candidate.sample_description_index();
    rises
  });
  let decided = if information.candidates().is_empty() {
    information.has_every_candidate() || information.codec().is_none()
  } else {
    information.codec() == candidates_codec(information)
  };
  let supplied = information.codec().is_some()
    || (information.profile().is_none()
      && information.level().is_none()
      && information.tier().is_none()
      && information.codec_string().is_none());
  let identified = if information.candidates().is_empty() {
    information.codec().is_some() || information.raw_identifiers().is_empty()
  } else {
    information.raw_identifiers().iter().eq(
      information
        .candidates()
        .iter()
        .flat_map(CodecCandidate::raw_identifiers),
    )
  };
  if rising && decided && supplied && identified {
    Ok(())
  } else {
    Err(super::constants::CODEC_AGREEMENT_MESSAGE)
  }
}
