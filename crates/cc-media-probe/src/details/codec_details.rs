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

//! `CodecDetails`, the codec header's typed details (`api.md` section 11.1).

use super::enumeration::details_enumeration;

details_enumeration! {
  /// The codec header's typed details, from `Stream::codec_details()`. Each parsed codec adds its variant in its
  /// phase, such as `H264(H264Details)`. Adding a variant to this non-exhaustive enum is a compatible change.
  ///
  /// The kind's stable name, such as `h264`, and the conflicts, the observed lower bounds and the values beyond the
  /// exact range are reached the same way for every codec, through the methods below, without matching on a
  /// variant.
  ///
  /// ```
  /// use cc_media_probe::Stream;
  ///
  /// fn kind(stream: &Stream) -> Option<&str> {
  ///     stream.codec_details().map(|details| details.as_str())
  /// }
  ///
  /// fn kept_values(stream: &Stream) -> usize {
  ///     stream.codec_details().map_or(0, |details| {
  ///         details.conflicts().len() + details.observed_lower_bounds().len() + details.exact_values().len()
  ///     })
  /// }
  /// ```
  CodecDetails {}
}

#[cfg(test)]
mod tests {
  use super::CodecDetails;
  use crate::details::UnrecognizedDetails;

  #[test]
  fn a_newer_kind_lists_no_values() {
    let details = CodecDetails::Unrecognized(UnrecognizedDetails::sample("h264"));
    assert_eq!(details.as_str(), "h264");
    assert_eq!(details.conflicts(), []);
    assert_eq!(details.observed_lower_bounds(), []);
    assert_eq!(details.exact_values(), []);
  }
}
