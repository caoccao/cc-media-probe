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

//! Values that JSON does not carry, given by the tests' own self-describing reader as ciborium gives them
//! (`json-output.md` section 2): a map with integer names, a tagged value, a unit variant, an integer beyond 64 bits,
//! a byte string and a non-finite number. Before a root record's version each is held, so the version error comes
//! first; once the version is known each fails as malformed input, also in a member that no type reads. A nested
//! record reads strictly before its own version, in either order of its holder's members, and a tag fails around a
//! value of a known type, a root record and a value read alone, which a typed request would have let ciborium drop.

use cc_media_probe::Detection;
use cc_media_probe::completeness::DetectionStatus;
use cc_media_probe::error::ErrorRecord;
use cc_media_probe::model::{Candidate, TagValue};
use serde::Deserialize;
use serde_json::json;

use crate::fixtures::{detection, error_record, report};
use crate::self_describing::{Tree, name};
use crate::version_first::{DUPLICATE, NESTED, OTHER_MAJOR};

const NOT_JSON: &str = "a value that JSON does not carry";

/// A detection of that version, with one more member before its others or after them.
fn detection_with(version: &str, member: Option<(Tree, Tree)>, first: bool) -> Tree {
  let tree = Tree::of(&detection(version));
  match member {
    Some((name, value)) => tree.with(name, value, first),
    None => tree,
  }
}

fn read(tree: Tree) -> Result<Detection, String> {
  Detection::deserialize(tree).map_err(|error| error.to_string())
}

/// The values of the reviews that JSON does not carry and that a reader gives whole. An enum that is no newtype, as
/// the unit variant that the second review of the third slice's first part read, is refused by its reader where it
/// is read as one (below).
fn not_json() -> Vec<Tree> {
  vec![
    Tree::Map(vec![(Tree::Integer(1), name("x"))]),
    Tree::Tagged(Box::new(Tree::Integer(0))),
    Tree::Wide(1 << 64),
    Tree::Bytes(vec![1, 2]),
    Tree::Number(f64::NAN),
    Tree::Number(f64::INFINITY),
  ]
}

#[test]
fn a_detection_of_this_format_reads() {
  assert!(read(detection_with("1.0", None, false)).is_ok());
}

#[test]
fn a_read_that_fails_inside_a_tag_fails_before_the_version_error() {
  // The third review of the third slice's first part read text that is not UTF-8 inside a tag, before a version
  // 2.0, as the version error, where the same text without the tag gave the reader's own error.
  let broken = || Tree::Broken("invalid UTF-8 in a text string".to_owned());
  for value in [broken(), Tree::Tagged(Box::new(broken()))] {
    let message = read(detection_with("2.0", Some((name("future"), value.clone())), true)).unwrap_err();
    assert!(message.starts_with("invalid UTF-8"), "{value:?}: {message}");
  }
  // The fourth review's case: Serde's `MapAccessDeserializer` gives `{"name"}` as an enum, and the reader fails on the
  // missing colon before it hands the content over, which the version error had hidden.
  let colonless = Tree::Colonless("name".to_owned());
  let message = read(detection_with("2.0", Some((name("future"), colonless)), true)).unwrap_err();
  assert!(message.starts_with("expected `:`"), "{message}");
}

#[test]
fn an_enum_shape_that_the_reader_refuses_fails_as_the_readers_error() {
  // As the maintainer chose at the fourth review of the third slice's first part, a reader's refusal of the newtype
  // that an enum is read as is the reader's failure, which comes before the version error. After the version, the
  // enum fails as a value that JSON does not carry before its content is read.
  let refused = "invalid type: unit variant, expected newtype variant";
  let unit = || Some((name("future"), Tree::UnitVariant("future".to_owned())));
  for (version, first, expected) in [
    ("2.0", true, refused),
    ("1.0", true, refused),
    ("2.0", false, OTHER_MAJOR),
    ("1.0", false, NOT_JSON),
  ] {
    let message = read(detection_with(version, unit(), first)).unwrap_err();
    assert!(message.starts_with(expected), "{version} {first}: {message}");
  }
}

#[test]
fn such_a_value_before_the_version_is_held_and_the_version_error_comes_first() {
  for value in not_json() {
    for deep in [false, true] {
      let value = if deep {
        Tree::Map(vec![(
          name("values"),
          Tree::Sequence(vec![Tree::Integer(1), value.clone()]),
        )])
      } else {
        value.clone()
      };
      let message = read(detection_with("2.0", Some((name("future"), value)), true)).unwrap_err();
      assert!(message.starts_with(OTHER_MAJOR), "{message}");
    }
  }
  let message = read(detection_with("2.0", Some((Tree::Integer(7), name("x"))), true)).unwrap_err();
  assert!(message.starts_with(OTHER_MAJOR), "{message}");
}

#[test]
fn such_a_value_fails_once_the_version_is_known_in_either_order() {
  for value in not_json() {
    for first in [true, false] {
      let message = read(detection_with("1.0", Some((name("future"), value.clone())), first)).unwrap_err();
      assert!(message.starts_with(NOT_JSON), "{value:?} {first}: {message}");
    }
  }
  for first in [true, false] {
    let message = read(detection_with("1.0", Some((Tree::Integer(7), name("x"))), first)).unwrap_err();
    assert!(message.starts_with(NOT_JSON), "{first}: {message}");
  }
}

#[test]
fn a_character_name_is_the_name_of_its_text_in_either_order() {
  for first in [true, false] {
    let member = Some((Tree::Character('x'), Tree::Integer(1)));
    assert!(read(detection_with("1.0", member.clone(), first)).is_ok(), "{first}");
    let message = read(detection_with("2.0", member, first)).unwrap_err();
    assert!(message.starts_with(OTHER_MAJOR), "{first}: {message}");
    let twice = detection_with("1.0", Some((name("x"), Tree::Integer(2))), first).with(
      Tree::Character('x'),
      Tree::Integer(1),
      !first,
    );
    let message = read(twice).unwrap_err();
    assert!(message.starts_with(DUPLICATE), "{first}: {message}");
    // The review's case: a member that no type reads holds a map with a character name, before the version or after
    // it.
    let nested = Some((
      name("future"),
      Tree::Map(vec![(Tree::Character('x'), Tree::Integer(1))]),
    ));
    assert!(read(detection_with("1.0", nested, first)).is_ok(), "{first}");
  }
}

/// An error record whose partial report holds one more member before its version, in either order of the record.
fn nested(member: (Tree, Tree), version: &str, report_first: bool) -> Result<ErrorRecord, String> {
  let partial = Tree::of(&report(version)).with(name("schema_version"), name(version), false);
  let partial = partial.with(member.0, member.1, true);
  let record = Tree::of(&error_record("1.0")).with(name("partial_report"), partial, report_first);
  let record = if report_first {
    record.with(name("schema_version"), name("1.0"), false)
  } else {
    record
  };
  ErrorRecord::deserialize(record).map_err(|error| error.to_string())
}

#[test]
fn a_nested_record_refuses_such_a_value_before_its_own_version_in_either_order() {
  for value in not_json() {
    for report_first in [true, false] {
      let message = nested((name("future"), value.clone()), "2.0", report_first).unwrap_err();
      assert!(message.starts_with(NOT_JSON), "{value:?} {report_first}: {message}");
    }
  }
  for report_first in [true, false] {
    let message = nested((name("future"), Tree::Integer(1)), "1.1", report_first).unwrap_err();
    assert!(message.starts_with(NESTED), "{report_first}: {message}");
  }
}

#[test]
fn a_nested_record_reads_its_version_before_a_later_integer_key() {
  for report_first in [true, false] {
    let partial = Tree::Map(vec![
      (name("schema_version"), name("2.0")),
      (Tree::Integer(1), name("x")),
    ]);
    let record = Tree::of(&error_record("1.0")).with(name("partial_report"), partial, report_first);
    let message = ErrorRecord::deserialize(record).map(|_| ()).unwrap_err().to_string();
    assert!(message.starts_with(NESTED), "{report_first}: {message}");
  }
}

/// An identified detection whose confidence is that value, with its version first or last.
fn identified(confidence: Tree, version_first: bool) -> Tree {
  let detection = Tree::of(&crate::fixtures::without(detection("1.0"), "schema_version"));
  let detection = detection.with(name("confidence"), confidence, false);
  detection.with(name("schema_version"), name("1.0"), version_first)
}

#[test]
fn a_tag_around_a_value_of_a_known_type_fails_in_either_order() {
  let tagged = || Tree::Tagged(Box::new(Tree::Integer(60)));
  for version_first in [true, false] {
    let message = read(identified(tagged(), version_first)).unwrap_err();
    assert!(message.starts_with(NOT_JSON), "{version_first}: {message}");
    assert!(read(identified(Tree::Integer(60), version_first)).is_ok());
  }
  let candidate = Tree::Map(vec![(name("format"), name("mp4")), (name("confidence"), tagged())]);
  let message = Candidate::deserialize(candidate).map(|_| ()).unwrap_err().to_string();
  assert!(message.starts_with(NOT_JSON), "{message}");
  let candidate = Tree::Map(vec![
    (name("format"), name("mp4")),
    (name("confidence"), Tree::Integer(60)),
  ]);
  let message = Candidate::deserialize(Tree::Tagged(Box::new(candidate)))
    .map(|_| ())
    .unwrap_err()
    .to_string();
  assert!(message.starts_with(NOT_JSON), "{message}");
}

#[test]
fn a_tag_around_a_root_record_or_a_value_read_alone_fails() {
  let tagged = |tree: Tree| Tree::Tagged(Box::new(tree));
  for version_first in [true, false] {
    let message = read(tagged(identified(Tree::Integer(60), version_first))).unwrap_err();
    assert!(message.starts_with(NOT_JSON), "{version_first}: {message}");
  }
  let status = DetectionStatus::deserialize(tagged(name("complete")));
  assert!(status.map(|_| ()).unwrap_err().to_string().starts_with(NOT_JSON));
  assert!(DetectionStatus::deserialize(name("complete")).is_ok());
  let text = Tree::Map(vec![(name("kind"), name("text")), (name("text"), name("Title"))]);
  let value = TagValue::deserialize(tagged(text.clone()));
  assert!(value.map(|_| ()).unwrap_err().to_string().starts_with(NOT_JSON));
  assert!(TagValue::deserialize(text).is_ok());
}

#[test]
fn an_integer_within_64_bits_is_that_integer_whatever_width_the_reader_gives() {
  let rational = |numerator: Tree, denominator: Tree| {
    cc_media_probe::Rational::deserialize(Tree::Map(vec![
      (name("numerator"), numerator),
      (name("denominator"), denominator),
    ]))
  };
  assert_eq!(
    rational(Tree::Wide(1), Tree::Wide(2)).ok(),
    cc_media_probe::Rational::new(1, 2)
  );
  assert!(rational(Tree::Wide(1 << 64), Tree::Integer(2)).is_err());
  let catch_all = TagValue::deserialize(Tree::Map(vec![
    (name("kind"), name("future")),
    (name("ratio"), Tree::Wide(7)),
  ]));
  assert_eq!(
    serde_json::to_value(catch_all.unwrap()).unwrap(),
    json!({ "kind": "future", "ratio": 7 })
  );
}
