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

//! The paths of generated test files (plan section 9.1, `docs/guide/test-files.md` sections 1 and 6): below
//! `target/test-files/` at the repository's root, a missing file failing with a message that names the generator,
//! and the recipe file of each name. The lookups run below a folder of each test's own, so that the files that a
//! developer generated never change a result, as the second review of plan Phase 3, task 4, found.

use std::error::Error;
use std::fs;
use std::path::Path;

use cc_media_probe_testkit::generated::{GeneratedError, folder, path, path_in, recipe_path};

use crate::scratch::scratch;

#[test]
fn a_missing_generated_file_fails_with_a_message_that_names_the_generator() {
  let base = scratch("generated-missing");
  let name = "containers/matroska/no-recipe-describes-this.mkv";
  let error = path_in(&base.path, name).unwrap_err();
  let expected = base
    .path
    .join("containers")
    .join("matroska")
    .join("no-recipe-describes-this.mkv");
  assert!(
    matches!(&error, GeneratedError::Missing(found) if *found == expected),
    "{error}"
  );
  for message in [error.to_string(), format!("{error:?}")] {
    assert!(
      message.contains("run `node scripts/generate-test-files.ts`"),
      "{message}"
    );
  }
}

#[test]
fn only_an_absent_file_is_missing() {
  // The second review of plan Phase 3, task 4: a folder at the path, or metadata that cannot be read, was missing.
  let base = scratch("generated-kinds");
  let format = base.path.join("audio").join("opus");
  fs::create_dir_all(format.join("folder.opus")).unwrap();
  fs::write(format.join("stereo.opus"), b"x").unwrap();
  let error = path_in(&base.path, "audio/opus/folder.opus").unwrap_err();
  assert!(matches!(&error, GeneratedError::NotAFile(found) if *found == format.join("folder.opus")));
  assert!(error.to_string().contains("something other than a file"), "{error}");
  assert_eq!(
    path_in(&base.path, "audio/opus/stereo.opus").unwrap(),
    format.join("stereo.opus")
  );
  if cfg!(unix) {
    // A file where the generator writes a folder: the system says that it is no folder, and nothing is missing.
    fs::write(base.path.join("video"), b"x").unwrap();
    let error = path_in(&base.path, "video/h264/a.h264").unwrap_err();
    assert!(matches!(&error, GeneratedError::Unreadable(..)), "{error}");
    assert!(error.to_string().contains("metadata cannot be read"), "{error}");
    // The third review of plan Phase 3, task 4: the cause is the source, so the message no longer repeats it, and
    // `Debug`, which an unwrap writes, gives both.
    let cause = error.source().map(ToString::to_string).unwrap();
    assert!(!error.to_string().contains(&cause), "{error}");
    assert_eq!(format!("{error:?}"), format!("{error}: {cause}"));
  }
}

#[test]
fn generated_files_lie_below_the_repositorys_target_folder() {
  let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..");
  assert_eq!(folder(), root.join("target").join("test-files"));
  assert!(root.join("scripts").join("generate-test-files.ts").is_file());
}

#[test]
fn a_recipe_file_is_named_after_the_topic_and_format_of_its_files() {
  let recipe = recipe_path("video/h264/baseline-320x240.h264").unwrap();
  let expected = Path::new(env!("CARGO_MANIFEST_DIR"))
    .join("test-files")
    .join("video")
    .join("h264.json");
  assert_eq!(recipe, expected);
}

#[test]
fn a_name_that_is_not_topic_format_and_file_is_refused() {
  for name in [
    "matroska/h264-opus.mkv",
    "containers/matroska/extra/h264-opus.mkv",
    "pictures/png/a.png",
    "containers/../a.mkv",
    "containers/./a.mkv",
    "containers/matroska/",
    "containers\\matroska\\a.mkv",
    "containers/matroska/c:a.mkv",
    // Names that the generator refuses, which the review of plan Phase 3, task 4, found accepted.
    "video/H264/no-extension",
    "video/H264/a.h264",
    "video/h264-/a.h264",
    "video/h264--high/a.h264",
    "video/h264/no-extension",
    "video/h264/A.h264",
    "video/h264/a.H264",
    "video/h264/-a.h264",
    "video/h264/.hidden.h264",
    "video/h264/a..h264",
    "video/h264/a.h264.",
    "video/h264/a_b.h264",
  ] {
    assert!(
      matches!(path(name), Err(GeneratedError::MalformedName(found)) if found == name),
      "{name}"
    );
    assert!(recipe_path(name).is_err(), "{name}");
  }
  for name in [
    "audio/opus/stereo-48k.opus",
    "containers/matroska-2/a--b-.tar.gz",
    "tags-and-registries/id3v2/0.mp3",
  ] {
    assert!(recipe_path(name).is_ok(), "{name}");
  }
}
