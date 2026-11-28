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

//! The fixtures' layout, checked whole, and the routes that a fixture states (plan section 9.5): the whole
//! fixture's in its header and each region's in its `@seed`, with the representation and context of a codec
//! target. Plan Phase 3, task 19, gives each fixture's seeds by name.

use std::error::Error;
use std::fs;
use std::io::{self, ErrorKind};
use std::path::Path;

use cc_media_probe_testkit::fixture::{Expectation, Fixture, FixtureError, fixture_path, layout, layout_of, names};
use cc_media_probe_testkit::syntax::{SyntaxError, is_hyphenated_name};

use crate::scratch::scratch;

/// What a test compares of a route: its target, representation, context and outcome, and the length of its bytes.
type Summary<'route> = (&'route str, Option<&'route str>, &'route [u8], &'route str, usize);

#[test]
fn the_fixtures_folder_holds_the_two_samples_among_its_fixtures() {
  let names = names().unwrap();
  for sample in ["flac/decoding-example-1", "toy/setup-and-parameters"] {
    assert!(names.iter().any(|name| name == sample), "{sample}");
  }
  let layout = layout().unwrap();
  assert!(layout.expectations.contains(&Expectation {
    fixture: "toy/setup-and-parameters".to_owned(),
    case: Some("short-reads".to_owned()),
    part: "adapters".to_owned(),
  }));
}

#[test]
fn a_file_or_folder_that_the_layout_does_not_allow_is_an_error() {
  let folder = scratch("layout");
  let base = folder.path.join("fixtures");
  let refused = |relative: &str| {
    let _ = fs::remove_dir_all(&base);
    fs::create_dir_all(base.join("toy")).unwrap();
    fs::write(base.join("toy").join("sample.hex"), "").unwrap();
    let path = base.join(relative);
    if relative.ends_with('/') {
      fs::create_dir_all(&path).unwrap();
    } else {
      fs::create_dir_all(path.parent().unwrap()).unwrap();
      fs::write(&path, "").unwrap();
    }
    match layout_of(&base) {
      Err(FixtureError::Misplaced(found, _)) => found.strip_prefix(&base).unwrap().to_path_buf(),
      other => panic!("{relative}: {other:?}"),
    }
  };
  for (relative, reported) in [
    ("loose.hex", "loose.hex"),
    ("toy/deeper/", "toy/deeper"),
    ("Flac/sample.hex", "Flac"),
    ("toy/Other.hex", "toy/Other.hex"),
    ("toy/sample.txt", "toy/sample.txt"),
    (
      "toy/sample.split.header.expected.json",
      "toy/sample.split.header.expected.json",
    ),
    ("toy/sample.answers.json", "toy/sample.answers.json"),
  ] {
    assert_eq!(refused(relative), Path::new(reported), "{relative}");
  }
  assert_eq!(refused("toy/other.expected.json"), Path::new("toy/other.expected.json"));
  let _ = fs::remove_dir_all(&base);
  fs::create_dir_all(base.join("toy")).unwrap();
  for file in [
    "sample.hex",
    "sample.expected.json",
    "sample.split-header.adapters.json",
    "sample.no-memory.outcomes.json",
  ] {
    fs::write(base.join("toy").join(file), "").unwrap();
  }
  let layout = layout_of(&base).unwrap();
  assert_eq!(layout.fixtures, ["toy/sample"]);
  assert_eq!(layout.expectations.len(), 3);
}

#[cfg(unix)]
#[test]
fn a_symbolic_link_in_the_fixtures_folder_or_as_it_is_an_error() {
  use std::os::unix::fs::symlink;

  // The review of plan Phase 3, task 4: a linked format folder or expectation passed, and writing followed it.
  let folder = scratch("layout-linked");
  let outside = folder.path.join("outside");
  fs::create_dir_all(&outside).unwrap();
  fs::write(outside.join("sample.expected.json"), "").unwrap();
  let base = folder.path.join("fixtures");
  let linked = |make: &dyn Fn(&Path)| {
    let _ = fs::remove_dir_all(&base);
    fs::create_dir_all(base.join("toy")).unwrap();
    fs::write(base.join("toy").join("sample.hex"), "").unwrap();
    make(&base);
    match layout_of(&base) {
      Err(FixtureError::Misplaced(found, message)) => {
        assert!(message.contains("symbolic link"), "{message}");
        found.strip_prefix(&base).unwrap().to_path_buf()
      }
      other => panic!("{other:?}"),
    }
  };
  let format = linked(&|base| symlink(&outside, base.join("flac")).unwrap());
  assert_eq!(format, Path::new("flac"));
  let expectation = linked(&|base| {
    symlink(
      outside.join("sample.expected.json"),
      base.join("toy").join("sample.expected.json"),
    )
    .unwrap();
  });
  assert_eq!(expectation, Path::new("toy/sample.expected.json"));
  let linked_base = folder.path.join("linked");
  symlink(&base, &linked_base).unwrap();
  let error = layout_of(&linked_base).unwrap_err();
  assert!(matches!(error, FixtureError::Misplaced(found, _) if found == linked_base));
}

#[cfg(unix)]
#[test]
fn a_fixture_loaded_by_its_name_meets_the_rule_on_symbolic_links() {
  use std::os::unix::fs::symlink;

  // The second review of plan Phase 3, task 4: a direct load followed a link that the layout refuses.
  let folder = scratch("load-linked");
  let text = fs::read_to_string(fixture_path("toy/setup-and-parameters")).unwrap();
  let outside = folder.path.join("outside");
  fs::create_dir_all(&outside).unwrap();
  fs::write(outside.join("sample.hex"), &text).unwrap();
  let base = folder.path.join("fixtures");
  fs::create_dir_all(base.join("toy")).unwrap();
  fs::write(base.join("toy").join("sample.hex"), &text).unwrap();
  let loaded = Fixture::load_from(&base, "toy/sample").unwrap();
  assert_eq!(loaded, Fixture::from_text("toy/sample", &text).unwrap());
  symlink(outside.join("sample.hex"), base.join("toy").join("linked.hex")).unwrap();
  symlink(&outside, base.join("flac")).unwrap();
  let linked_base = folder.path.join("linked");
  symlink(&base, &linked_base).unwrap();
  for (place, name, link) in [
    (&base, "toy/linked", base.join("toy").join("linked.hex")),
    (&base, "flac/sample", base.join("flac")),
    (&linked_base, "toy/sample", linked_base.clone()),
  ] {
    match Fixture::load_from(place, name) {
      Err(FixtureError::Misplaced(found, message)) => {
        assert_eq!(found, link);
        assert!(message.contains("symbolic link"), "{message}");
      }
      other => panic!("{name}: {other:?}"),
    }
  }
  match Fixture::load_from(&base, "toy/absent") {
    Err(FixtureError::Unreadable(name, error)) => {
      assert_eq!((name.as_str(), error.kind()), ("toy/absent", ErrorKind::NotFound));
    }
    other => panic!("{other:?}"),
  }
}

#[test]
fn a_fixture_loaded_by_its_name_is_an_entry_of_its_folder_so_spelled_and_a_regular_file() {
  // The third review of plan Phase 3, task 4: on a file system that ignores case, `toy/sample` read `toy/Sample.hex`,
  // which the layout refuses, and a direct load opened an entry of any type.
  let folder = scratch("load-listed");
  let text = fs::read_to_string(fixture_path("toy/setup-and-parameters")).unwrap();
  let base = folder.path.join("fixtures");
  fs::create_dir_all(base.join("toy")).unwrap();
  fs::write(base.join("toy").join("Sample.hex"), &text).unwrap();
  fs::create_dir_all(base.join("Flac")).unwrap();
  fs::write(base.join("Flac").join("sample.hex"), &text).unwrap();
  for name in ["toy/sample", "flac/sample"] {
    match Fixture::load_from(&base, name) {
      Err(FixtureError::Unreadable(found, error)) => {
        assert_eq!((found.as_str(), error.kind()), (name, ErrorKind::NotFound));
        assert_eq!(
          error.to_string(),
          "no entry of its folder has this name, spelled exactly so"
        );
      }
      other => panic!("{name}: {other:?}"),
    }
  }
  fs::create_dir_all(base.join("toy").join("folder.hex")).unwrap();
  match Fixture::load_from(&base, "toy/folder") {
    Err(FixtureError::Misplaced(found, _)) => assert_eq!(found, base.join("toy").join("folder.hex")),
    other => panic!("{other:?}"),
  }
}

#[cfg(unix)]
#[test]
fn a_fifo_named_as_a_fixture_is_refused_before_it_is_opened() {
  use std::process::Command;
  use std::sync::mpsc;
  use std::thread;
  use std::time::Duration;

  // The third review of plan Phase 3, task 4: opening a FIFO blocked the load until a writer came.
  let folder = scratch("load-fifo");
  let base = folder.path.join("fixtures");
  fs::create_dir_all(base.join("toy")).unwrap();
  let fifo = base.join("toy").join("sample.hex");
  // The system's own tool makes the FIFO, which the standard library cannot.
  if !Command::new("mkfifo")
    .arg(&fifo)
    .status()
    .is_ok_and(|status| status.success())
  {
    return;
  }
  let (sender, receiver) = mpsc::channel();
  thread::spawn(move || sender.send(Fixture::load_from(&base, "toy/sample").map(|_| ())));
  match receiver.recv_timeout(Duration::from_secs(30)) {
    Ok(Err(FixtureError::Misplaced(found, _))) => assert_eq!(found, fifo),
    other => panic!("{other:?}"),
  }
}

#[test]
fn a_fixture_error_gives_its_cause_as_its_source_and_leaves_it_out_of_its_message() {
  // The third review of plan Phase 3, task 4: `source()` gave nothing, though three kinds keep their cause.
  let folder = scratch("load-source");
  let missing = folder.path.join("absent");
  for error in [
    Fixture::load_from(&missing, "toy/sample").unwrap_err(),
    layout_of(&missing).unwrap_err(),
  ] {
    assert!(
      matches!(&error, FixtureError::Unlisted(path, _) if *path == missing),
      "{error:?}"
    );
    let cause = error.source().and_then(|cause| cause.downcast_ref::<io::Error>());
    assert_eq!(cause.map(io::Error::kind), Some(ErrorKind::NotFound));
    assert_eq!(
      error.to_string(),
      format!("{}: the folder cannot be listed", missing.display())
    );
  }
  fs::create_dir_all(folder.path.join("toy")).unwrap();
  let error = Fixture::load_from(&folder.path, "toy/absent").unwrap_err();
  assert_eq!(error.to_string(), "toy/absent: the fixture cannot be read");
  assert_eq!(
    error.source().map(ToString::to_string).as_deref(),
    Some("no entry of its folder has this name, spelled exactly so")
  );
  let error = Fixture::from_text("toy/a", "description: a\n").unwrap_err();
  assert_eq!(error.to_string(), "toy/a: the text is not a fixture");
  assert!(
    error
      .source()
      .and_then(|cause| cause.downcast_ref::<SyntaxError>())
      .is_some()
  );
  assert!(Fixture::load("toy/A").unwrap_err().source().is_none());
}

#[cfg(unix)]
#[test]
fn a_file_name_that_is_not_unicode_is_an_error() {
  use std::ffi::OsStr;
  use std::os::unix::ffi::OsStrExt;

  let folder = scratch("layout-unicode");
  let base = folder.path.join("fixtures");
  fs::create_dir_all(base.join("toy")).unwrap();
  let name = OsStr::from_bytes(b"sample\xff.hex");
  if fs::write(base.join("toy").join(name), "").is_err() {
    return;
  }
  assert!(matches!(layout_of(&base), Err(FixtureError::Misplaced(..))));
}

#[test]
fn a_fixture_is_loaded_only_by_a_name_of_the_layout() {
  for name in [
    "toy",
    "toy/Sample",
    "toy/a/b",
    "../toy/setup-and-parameters",
    "toy/nul",
    "com1/sample",
  ] {
    assert!(
      matches!(Fixture::load(name), Err(FixtureError::MalformedName(_))),
      "{name}"
    );
  }
}

#[test]
fn every_expectation_file_of_every_case_has_a_name_that_a_file_system_can_hold() {
  // The review of plan Phase 3, task 4, parsed a case name of 240 bytes, whose files could not be written.
  let fixture = |stem: &str, case: &str| {
    let text =
      format!("description: a\nsource: b\ntarget: toy\noutcome: report\ncase: {case} pieces=1\n---\n## a\n00 # f\n");
    Fixture::from_text(&format!("toy/{stem}"), &text)
  };
  // `<stem>.<case>.adapters.json` of 1 + 1 + 240 + 14 bytes, one beyond the 255.
  let error = fixture("a", &"b".repeat(240)).unwrap_err();
  assert!(
    matches!(&error, FixtureError::LongExpectationName(name, Some(case)) if name == "toy/a" && case.len() == 240)
  );
  assert!(error.to_string().starts_with("toy/a: bbb"), "{error}");
  assert!(error.to_string().contains("at most 255 bytes"), "{error}");
  fixture("a", &"b".repeat(239)).unwrap();
  // The default case's files hold no case name, so a stem of 242 bytes is already too long.
  let error = fixture(&"a".repeat(242), "b").unwrap_err();
  assert!(
    matches!(&error, FixtureError::LongExpectationName(_, None)),
    "{error:?}"
  );
  assert!(error.to_string().contains(": the default case: "), "{error}");
  assert!(matches!(fixture("A", "b"), Err(FixtureError::MalformedName(_))));
}

#[test]
fn a_name_of_the_layout_is_no_device_name_that_windows_reserves() {
  assert!(is_hyphenated_name("nul-a"));
  assert!(is_hyphenated_name("com10"));
  for name in ["nul", "con", "prn", "aux", "com1", "lpt9"] {
    assert!(!is_hyphenated_name(name), "{name}");
  }
}

#[test]
fn a_container_fixture_routes_its_embedded_codec_header_as_a_region_of_its_own() {
  let fixture = Fixture::load("toy/setup-and-parameters").unwrap();
  let route = fixture.header.route.as_ref().unwrap();
  let whole: Summary<'_> = (
    route.target.as_str(),
    route.representation.as_deref(),
    route.context.as_slice(),
    route.outcome.as_str(),
    fixture.body.bytes.len(),
  );
  assert_eq!(whole, ("toy", None, &[][..], "report", 48));
  let [region] = fixture.body.regions.as_slice() else {
    panic!("one region: {:?}", fixture.body.regions);
  };
  let part: Summary<'_> = (
    region.route.target.as_str(),
    region.route.representation.as_deref(),
    region.route.context.as_slice(),
    region.route.outcome.as_str(),
    region.end - region.start,
  );
  assert_eq!(
    part,
    (
      "toy_codec",
      Some("one_byte_lengths"),
      &[0, 0, 0, 0, 12][..],
      "parsed",
      12
    )
  );
  assert_eq!((region.start, region.end), (36, 48));
  assert_eq!(fixture.file_name(), "setup-and-parameters");
}
