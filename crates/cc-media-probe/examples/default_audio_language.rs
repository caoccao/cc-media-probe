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

//! Scenario 3: find the language of the default audio stream.

use cc_media_probe::{Error, MediaReport, Stream, StreamKind, probe_file};

fn main() -> Result<(), Error> {
  let report = probe_file("movie.mkv")?;
  let Some((stream, chosen)) = default_audio(&report) else {
    println!("no audio stream");
    return Ok(());
  };
  match stream.language() {
    Some(language) => match language.tag() {
      Some(tag) => println!("{chosen}: #{} in {tag}", stream.index()),
      // `und` has no BCP 47 tag; `as_written()` still gives what the file stored.
      None => println!("{chosen}: #{}, language {}", stream.index(), language.as_written()),
    },
    None => println!("{chosen}: #{}, language not stated", stream.index()),
  }
  Ok(())
}

/// The audio stream marked as the default, or else the first audio stream, with how it was chosen. A format that
/// states no default leaves `is_default()` absent, which is not the same as "not the default", and the first audio
/// stream can be one whose flag says it is not the default.
fn default_audio(report: &MediaReport) -> Option<(&Stream, &'static str)> {
  let audio = || {
    report
      .streams()
      .iter()
      .filter(|stream| matches!(stream.kind(), StreamKind::Audio))
  };
  audio()
    .find(|stream| stream.disposition().is_default() == Some(true))
    .map(|stream| (stream, "default audio"))
    .or_else(|| {
      audio()
        .next()
        .map(|stream| (stream, "first audio, none marked default"))
    })
}
