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

//! What the text format refuses (`docs/design/test-kit.md` section 2), each with the line where it is found.

use kit::syntax::{SyntaxErrorKind, parse};

/// A header with every required member, before the body.
const HEADER: &str = "description: a case\nsource: a section\ntarget: toy\noutcome: report\n---\n";

/// The error of a fixture with that body after `HEADER`, which takes five lines.
fn refused(body: &str) -> (usize, SyntaxErrorKind) {
  let error = parse(&format!("{HEADER}{body}")).unwrap_err();
  (error.line, error.kind)
}

#[test]
fn a_well_formed_fixture_assembles_its_bytes_and_regions() {
  let text = format!(
    "{HEADER}## A spec, section 1: a header\n00 01 # two fields\r\n\n# a remark\n@seed target=toy_codec \
     representation=one_byte_lengths outcome=parsed # a codec header\n\"ab\" * 2 # a field\n@end\nff # a field\n"
  );
  let parsed = parse(&text).unwrap();
  assert_eq!(parsed.body.bytes, [0, 1, b'a', b'b', b'a', b'b', 0xff]);
  assert_eq!((parsed.body.regions[0].start, parsed.body.regions[0].end), (2, 6));
  assert_eq!(parsed.body.regions[0].route.context, Vec::<u8>::new());
}

#[test]
fn a_line_of_bytes_needs_its_comment_and_a_heading_above_it() {
  assert_eq!(refused("## a section\n00\n"), (7, SyntaxErrorKind::BytesWithoutComment));
  assert_eq!(
    refused("## a section\n00 #   \n"),
    (7, SyntaxErrorKind::BytesWithoutComment)
  );
  assert_eq!(refused("00 # a field\n"), (6, SyntaxErrorKind::BytesBeforeHeading));
  assert_eq!(refused("##\n"), (6, SyntaxErrorKind::EmptyHeading));
  assert_eq!(refused("## a section\n# nothing\n"), (0, SyntaxErrorKind::NoBytes));
}

#[test]
fn a_region_is_opened_once_and_closed_after_its_bytes() {
  let seed = "@seed target=toy_codec representation=one_byte_lengths outcome=parsed\n";
  assert_eq!(
    refused(&format!("## a\n{seed}00 # f\n")),
    (7, SyntaxErrorKind::UnclosedRegion)
  );
  assert_eq!(
    refused(&format!("## a\n{seed}{seed}")),
    (8, SyntaxErrorKind::NestedRegion)
  );
  assert_eq!(
    refused(&format!("## a\n{seed}@end\n")),
    (8, SyntaxErrorKind::EmptyRegion)
  );
  assert_eq!(refused("## a\n@end\n"), (7, SyntaxErrorKind::NoOpenRegion));
  assert_eq!(
    refused("## a\n@skip\n"),
    (7, SyntaxErrorKind::UnknownDirective("skip".to_owned()))
  );
  assert_eq!(
    refused("## a\n@seed target=toy_codec\n"),
    (7, SyntaxErrorKind::MissingMember("outcome"))
  );
  assert_eq!(
    refused("## a\n@seed target=toy outcome=report context=00\n"),
    (7, SyntaxErrorKind::ContextWithoutRepresentation)
  );
  assert_eq!(
    refused("## a\n@seed target=Toy outcome=report\n"),
    (7, SyntaxErrorKind::MalformedName("Toy".to_owned()))
  );
  assert_eq!(
    refused("## a\n@seed size=3\n"),
    (7, SyntaxErrorKind::UnknownMember("size".to_owned()))
  );
}

#[test]
fn the_header_states_each_member_once_and_its_length_exactly() {
  let body = "## a\n00 # f\n";
  let error = |header: &str| {
    let error = parse(&format!("{header}---\n{body}")).unwrap_err();
    (error.line, error.kind)
  };
  assert_eq!(
    error("description: a\nsource: b\ntarget: toy\n"),
    (4, SyntaxErrorKind::MissingMember("outcome"))
  );
  assert_eq!(
    error("description: a\ntarget: toy\noutcome: report\n"),
    (4, SyntaxErrorKind::MissingMember("source"))
  );
  assert_eq!(
    error("description: a\ndescription: b\n"),
    (2, SyntaxErrorKind::RepeatedMember("description".to_owned()))
  );
  assert_eq!(error("description\n"), (1, SyntaxErrorKind::MalformedMember));
  assert_eq!(
    error("format: toy\n"),
    (1, SyntaxErrorKind::UnknownMember("format".to_owned()))
  );
  assert_eq!(error("length: 0x10\n"), (1, SyntaxErrorKind::MalformedLength));
  assert_eq!(
    error("description: a\nsource: b\ntarget: toy\noutcome: report\nlength: 2\n"),
    (
      6,
      SyntaxErrorKind::LengthMismatch {
        declared: 2,
        assembled: 1
      }
    )
  );
  assert_eq!(
    parse("description: a\n").unwrap_err().kind,
    SyntaxErrorKind::NoHeaderEnd
  );
}

#[test]
fn an_end_stands_alone_and_a_region_always_has_a_target() {
  let seed = "@seed target=toy_codec representation=one_byte_lengths outcome=parsed\n";
  assert_eq!(
    refused(&format!("## a\n{seed}00 # f\n@end stray words\n")),
    (9, SyntaxErrorKind::EndWithWords("stray".to_owned()))
  );
  assert_eq!(
    refused("## a\n@seed target=none outcome=parsed\n"),
    (7, SyntaxErrorKind::RegionWithoutTarget)
  );
}

#[test]
fn the_target_none_gives_no_seed_of_the_whole_fixture() {
  let body = "---\n## a\n@seed target=toy_codec representation=one_byte_lengths outcome=parsed\n00 # f\n@end\n";
  let parsed = parse(&format!("description: a\nsource: b\ntarget: none\n{body}")).unwrap();
  assert_eq!(parsed.header.route, None);
  assert_eq!(parsed.body.regions.len(), 1);
  let error = parse(&format!(
    "description: a\nsource: b\ntarget: none\noutcome: report\n{body}"
  ))
  .unwrap_err();
  assert_eq!((error.line, error.kind), (5, SyntaxErrorKind::RouteOfNoTarget));
  let error = parse("description: a\nsource: b\ntarget: none\n---\n## a\n00 # f\n").unwrap_err();
  assert_eq!((error.line, error.kind), (0, SyntaxErrorKind::NoTargetWithoutRegion));
}

#[test]
fn a_case_gives_its_name_and_its_conditions_once_each() {
  let body = "---\n## a\n00 # f\n";
  let header = |lines: &str| format!("description: a\nsource: b\ntarget: toy\noutcome: report\n{lines}{body}");
  let parsed = parse(&header(
    "# a remark\ncase: split-header pieces=40,8\ncase: two-files companion.movie.toyc=found:toy/companion\n",
  ))
  .unwrap();
  let cases: Vec<(&str, Vec<(&str, &str)>)> = parsed
    .header
    .cases
    .iter()
    .map(|case| {
      let conditions = case
        .conditions
        .iter()
        .map(|condition| (condition.name.as_str(), condition.value.as_str()))
        .collect();
      (case.name.as_str(), conditions)
    })
    .collect();
  assert_eq!(
    cases,
    [
      ("split-header", vec![("pieces", "40,8")]),
      ("two-files", vec![("companion.movie.toyc", "found:toy/companion")]),
    ]
  );
  let error = |lines: &str| {
    let error = parse(&header(lines)).unwrap_err();
    (error.line, error.kind)
  };
  assert_eq!(
    error("case: a pieces=1\ncase: a pieces=2\n"),
    (6, SyntaxErrorKind::RepeatedCase("a".to_owned()))
  );
  assert_eq!(
    error("case: split-header\n"),
    (5, SyntaxErrorKind::MalformedCase("split-header".to_owned()))
  );
  assert_eq!(
    error("case: Split pieces=1\n"),
    (5, SyntaxErrorKind::MalformedCase("Split pieces=1".to_owned()))
  );
  assert_eq!(
    error("case: a pieces\n"),
    (5, SyntaxErrorKind::MalformedCondition("pieces".to_owned()))
  );
  assert_eq!(
    error("case: a pieces=1 pieces=2\n"),
    (5, SyntaxErrorKind::RepeatedMember("pieces".to_owned()))
  );
}

#[test]
fn a_default_line_gives_the_default_cases_conditions_once() {
  let body = "---\n## a\n00 # f\n";
  let header = |lines: &str| format!("description: a\nsource: b\ntarget: toy\noutcome: report\n{lines}{body}");
  let parsed = parse(&header("default: outcomes=per_adapter\n")).unwrap();
  let conditions: Vec<(&str, &str)> = parsed
    .header
    .default_conditions
    .iter()
    .map(|condition| (condition.name.as_str(), condition.value.as_str()))
    .collect();
  assert_eq!(conditions, [("outcomes", "per_adapter")]);
  assert_eq!(parsed.header.cases, []);
  assert_eq!(parse(&header("")).unwrap().header.default_conditions, []);
  let error = |lines: &str| {
    let error = parse(&header(lines)).unwrap_err();
    (error.line, error.kind)
  };
  assert_eq!(
    error("default: outcomes=shared\ndefault: outcomes=per_adapter\n"),
    (6, SyntaxErrorKind::RepeatedMember("default".to_owned()))
  );
  assert_eq!(
    error("default: per-adapter outcomes=per_adapter\n"),
    (5, SyntaxErrorKind::MalformedCondition("per-adapter".to_owned()))
  );
  assert_eq!(
    error("default: outcomes=shared outcomes=per_adapter\n"),
    (5, SyntaxErrorKind::RepeatedMember("outcomes".to_owned()))
  );
}

#[test]
fn a_file_name_is_a_plain_name_of_its_own() {
  let body = "---\n## a\n00 # f\n";
  let header =
    |name: &str| format!("description: a\nsource: b\ntarget: toy\noutcome: report\nfile_name: {name}\n{body}");
  assert_eq!(
    parse(&header("Movie.IDX")).unwrap().header.file_name.as_deref(),
    Some("Movie.IDX")
  );
  for name in [".hidden", "icon.backup.idx", "lpt10", "nul-a.idx"] {
    assert!(parse(&header(name)).is_ok(), "{name}");
  }
  for name in [
    "a/b.idx",
    "..",
    ".",
    "movie idx",
    "\u{e9}.idx",
    "movie.",
    "NUL.idx",
    "nul",
    "Aux.tar.gz",
    "com1.idx",
    "LPT9",
    "con.",
    "con.backup.idx",
  ] {
    let error = parse(&header(name)).unwrap_err();
    assert_eq!(
      error.kind,
      SyntaxErrorKind::MalformedFileName(name.to_owned()),
      "{name}"
    );
  }
}

#[test]
fn a_message_names_the_line_and_the_value() {
  let error = parse(&format!("{HEADER}## a\nabc # f\n")).unwrap_err();
  assert_eq!(error.to_string(), "7: hex bytes are pairs of digits: abc");
}
