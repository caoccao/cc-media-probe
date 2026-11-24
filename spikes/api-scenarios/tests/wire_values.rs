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

//! The thirteenth review's cases of single values, `json-output.md` sections 4.1, 5.4, 6.2, 7.2 and 7.3: the
//! schema version in canonical decimal within 32 bits a part; integer identifiers in canonical decimal at their own
//! width; binary tag data in hexadecimal whose length is the size; the integer of a tag value within 64 bits; raw
//! codec identifiers in the spelling of their namespace, as the maintainer chose (byte codes in hexadecimal); and the
//! fixed-size record, in which each array names each limit or part once and the first-stopped pairs name exactly the
//! exhausted limits.

use cc_media_probe::completeness::LimitRecord;
use cc_media_probe::document::Document;
use cc_media_probe::model::{RawCodecIdentifier, TagValue};
use cc_media_probe::source::SourceIdentifier;
use serde_json::json;

fn document_of_version(version: &str) -> Result<Document, serde_json::Error> {
  serde_json::from_str(&format!(
    r#"{{"schema_version": "{version}", "generator": {{"name": "catalog", "version": "1"}}, "results": []}}"#
  ))
}

#[test]
fn a_schema_version_is_two_canonical_decimal_numbers_of_32_bits() {
  let read = document_of_version("1.4294967295").unwrap();
  assert_eq!(
    (read.schema_version().major(), read.schema_version().minor()),
    (1, u32::MAX)
  );
  for version in [
    "+1.+0",
    "01.0",
    "1.00",
    "1.4294967296",
    "1.-0",
    "1.0.0",
    "1",
    " 1.0",
    "1.0 ",
    "١.0",
  ] {
    let message = document_of_version(version).unwrap_err().to_string();
    assert!(
      message.starts_with("a schema version is two decimal numbers"),
      "{version}: {message}"
    );
  }
}

#[test]
fn an_integer_identifier_is_canonical_decimal_at_its_own_width() {
  // A 32-bit ISO track_ID kept in an error's observations, and a 64-bit source identifier.
  let track = |identifier: &str| {
    serde_json::from_value::<cc_media_probe::details::FormatDetails>(json!({
      "IsoBaseMedia": { "fragment_track_identifiers": [identifier], "tracks": [] }
    }))
  };
  assert!(track("4294967295").is_ok());
  for identifier in ["01", "+1", "4294967296", "-1", ""] {
    assert!(track(identifier).is_err(), "track_ID {identifier:?}");
  }
  let source = |identifier: &str| serde_json::from_value::<SourceIdentifier>(json!(identifier));
  assert_eq!(source("18446744073709551615").unwrap().value(), u64::MAX);
  for identifier in ["01", "+1", "18446744073709551616", "-1", ""] {
    assert!(source(identifier).is_err(), "source {identifier:?}");
  }
}

#[test]
fn binary_tag_data_is_hexadecimal_and_as_long_as_the_size() {
  let read: TagValue = serde_json::from_str(r#"{"kind": "binary", "binary": {"size": 1, "data": "ff"}}"#).unwrap();
  let TagValue::Binary(binary) = &read else {
    panic!("{read:?}")
  };
  assert_eq!(binary.data(), Some(&[0xff_u8][..]));
  assert_eq!(
    serde_json::to_value(&read).unwrap(),
    json!({ "kind": "binary", "binary": { "size": 1, "data": "ff" } })
  );
  // Without its data, only the size; with it, exactly that many bytes, two lowercase digits each.
  assert!(serde_json::from_str::<TagValue>(r#"{"kind": "binary", "binary": {"size": 2}}"#).is_ok());
  for binary in [
    json!({ "size": 2, "data": "ff" }),
    json!({ "size": 2, "data": [255] }),
    json!({ "size": 1, "data": "FF" }),
    json!({ "size": 1, "data": "f" }),
    json!({ "size": 1, "data": "+f" }),
    json!({ "size_exact": "9007199254740993", "data": "ff" }),
  ] {
    let value = json!({ "kind": "binary", "binary": binary });
    assert!(serde_json::from_value::<TagValue>(value).is_err(), "{binary}");
  }
}

#[test]
fn a_tag_integer_lies_within_64_bits_signed_or_unsigned() {
  let read = |exact: &str| serde_json::from_value::<TagValue>(json!({ "kind": "integer", "integer_exact": exact }));
  let TagValue::Integer(smallest) = read("-9223372036854775808").unwrap() else {
    panic!()
  };
  assert_eq!(smallest.to_i64(), Some(i64::MIN));
  let TagValue::Integer(largest) = read("18446744073709551615").unwrap() else {
    panic!()
  };
  assert_eq!(largest.to_u64(), Some(u64::MAX));
  for exact in ["-9223372036854775809", "-18446744073709551615", "18446744073709551616"] {
    assert!(read(exact).is_err(), "{exact}");
  }
}

#[test]
fn a_raw_codec_identifier_is_spelled_by_its_namespace() {
  let read = |namespace: &str, value: &str| {
    serde_json::from_value::<RawCodecIdentifier>(json!({ "namespace": namespace, "value": value }))
  };
  // A four-character code is its four bytes in lowercase hexadecimal, as stored; a text code is its text.
  assert_eq!(read("IsoSampleEntry", "61766331").unwrap().value(), "61766331");
  assert_eq!(read("IsoSampleEntry", "00000000").unwrap().value(), "00000000");
  assert!(read("MatroskaCodecIdentifier", "V_MPEG4/ISO/AVC").is_ok());
  for code in ["avc1", "6176633", "617663310", "6176633G", "6176633A", "+1766331"] {
    assert!(read("IsoSampleEntry", code).is_err(), "{code}");
  }
  // A coded number is canonical decimal within its width: a WAVE format tag has 16 bits.
  assert!(read("WaveFormatTag", "85").is_ok());
  assert!(read("WaveFormatTag", "65535").is_ok());
  for tag in ["085", "+85", "65536", "0x55", "-1", ""] {
    assert!(read("WaveFormatTag", tag).is_err(), "{tag}");
  }
  // A namespace of a newer version keeps its value as written.
  let newer = json!({ "namespace": { "Unrecognized": "future_namespace" }, "value": "any text" });
  assert_eq!(
    serde_json::from_value::<RawCodecIdentifier>(newer).unwrap().value(),
    "any text"
  );
}

#[test]
fn the_record_names_each_limit_or_part_once_in_each_array() {
  let read = |record: serde_json::Value| serde_json::from_value::<LimitRecord>(record);
  // Two limits that first stopped the same part, each named in both arrays.
  let record = read(json!({
    "exhausted": ["BytesRead", "WorkingMemory"],
    "first_stopped": [{ "limit": "BytesRead", "part": "Discovery" }, { "limit": "WorkingMemory", "part": "Discovery" }],
    "first_reasons": [{ "part": "Discovery", "reason": { "kind": "LimitExhausted", "limit": "BytesRead" } }],
    "omitted_reasons": "0",
    "omitted_references": "0",
  }))
  .unwrap();
  assert!(record.exhausted(&cc_media_probe::Limit::WorkingMemory));
  for broken in [
    // A limit twice in `exhausted`.
    json!({ "exhausted": ["BytesRead", "BytesRead"], "first_stopped": [{ "limit": "BytesRead", "part": "Detection" }], "first_reasons": [], "omitted_reasons": "0", "omitted_references": "0" }),
    // A limit twice in `first_stopped`.
    json!({ "exhausted": ["BytesRead"], "first_stopped": [
      { "limit": "BytesRead", "part": "Detection" }, { "limit": "BytesRead", "part": "Discovery" } ], "first_reasons": [], "omitted_reasons": "0", "omitted_references": "0" }),
    // An exhausted limit without the part that it stopped first, and a first-stopped limit that is not exhausted.
    json!({ "exhausted": ["BytesRead"], "first_stopped": [], "first_reasons": [], "omitted_reasons": "0", "omitted_references": "0" }),
    json!({ "exhausted": [], "first_stopped": [{ "limit": "BytesRead", "part": "Detection" }], "first_reasons": [], "omitted_reasons": "0", "omitted_references": "0" }),
    // The two arrays in different orders.
    json!({ "exhausted": ["BytesRead", "WorkingMemory"], "first_stopped": [
      { "limit": "WorkingMemory", "part": "Detection" }, { "limit": "BytesRead", "part": "Detection" } ], "first_reasons": [], "omitted_reasons": "0", "omitted_references": "0" }),
    // A part twice in `first_reasons`.
    json!({ "exhausted": [], "first_stopped": [], "first_reasons": [
      { "part": "Tags", "reason": { "kind": "TruncatedInput" } },
      { "part": "Tags", "reason": { "kind": "TruncatedInput" } } ], "omitted_reasons": "0", "omitted_references": "0" }),
  ] {
    assert!(read(broken.clone()).is_err(), "{broken}");
  }
}

#[test]
fn a_codec_identifier_holds_only_what_its_decoder_writes() {
  // The fourteenth review: a Matroska `CodecID` is printable ASCII once its terminating null octets are removed, and
  // its decoder writes U+FFFD for any other byte, so no other character reads.
  let read = |value: &str| {
    serde_json::from_value::<RawCodecIdentifier>(json!({ "namespace": "MatroskaCodecIdentifier", "value": value }))
  };
  assert!(read("A_OPUS").is_ok());
  assert!(read("A_\u{fffd}PUS").is_ok());
  for value in ["A_\u{1f600}", "A_OPUS\u{0}", "A_OPUS\n", "A_\u{e9}"] {
    assert!(read(value).is_err(), "{value:?}");
  }
}

#[test]
fn an_extensible_wave_subformat_is_its_16_bytes_in_hexadecimal() {
  // The fourteenth review, as the maintainer chose: `wave_subformat` keeps the GUID as stored.
  let read =
    |value: &str| serde_json::from_value::<RawCodecIdentifier>(json!({ "namespace": "WaveSubformat", "value": value }));
  assert!(read("0100000000001000800000aa00389b71").is_ok());
  for value in [
    "0100000000001000800000aa00389b7",
    "0100000000001000800000AA00389B71",
    "1",
  ] {
    assert!(read(value).is_err(), "{value}");
  }
}

fn report_with(member: &str, value: serde_json::Value) -> serde_json::Value {
  let mut report = json!({
    "schema_version": "1.0",
    "format": "Matroska",
    "streams": [],
    "chapter_editions": [],
    "tag_sets": [],
    "attachments": [],
    "diagnostics": [],
    "completeness": {
      "detection": "Complete", "discovery": "Complete", "headers": "Complete", "tags": "Complete",
      "chapters": "Complete", "attachments": "Complete", "reasons": [],
      "record": { "exhausted": [], "first_stopped": [], "first_reasons": [], "omitted_reasons": "0", "omitted_references": "0" },
    },
    "usage": { "bytes_read": 0, "peak_working_memory": 0 },
    "sources": [],
  });
  report[member] = value;
  report
}

#[test]
fn usage_counters_and_diagnostic_offsets_are_twins_on_both_sides_of_the_range() {
  // The fourteenth review's case: `bytes_read` of 2^53 + 1 read as a number, and its twin was refused.
  let read =
    |usage: serde_json::Value| serde_json::from_value::<cc_media_probe::MediaReport>(report_with("usage", usage));
  let large = read(json!({ "bytes_read_exact": "9007199254740993", "peak_working_memory": 0 })).unwrap();
  assert_eq!(large.usage().bytes_read(), 9_007_199_254_740_993);
  assert_eq!(
    serde_json::to_value(&large).unwrap()["usage"]["bytes_read_exact"],
    json!("9007199254740993")
  );
  assert!(read(json!({ "bytes_read": 9_007_199_254_740_991_u64, "peak_working_memory": 0 })).is_ok());
  for usage in [
    json!({ "bytes_read": 9_007_199_254_740_993_u64, "peak_working_memory": 0 }),
    json!({ "bytes_read_exact": "9007199254740991", "peak_working_memory": 0 }),
    json!({ "bytes_read": 0, "peak_working_memory_exact": "1" }),
    json!({ "peak_working_memory": 0 }),
  ] {
    assert!(read(usage.clone()).is_err(), "{usage}");
  }
  let diagnostic = |offset: (&str, serde_json::Value)| {
    let mut diagnostic =
      json!({ "severity": "Warning", "code": "matroska.void", "message": "m", "source_identifier": "0" });
    diagnostic[offset.0] = offset.1;
    serde_json::from_value::<cc_media_probe::diagnostics::Diagnostic>(diagnostic)
  };
  assert_eq!(
    diagnostic(("offset_exact", json!("9007199254740993")))
      .unwrap()
      .offset(),
    Some(9_007_199_254_740_993)
  );
  assert!(diagnostic(("offset", json!(9_007_199_254_740_993_u64))).is_err());
  assert!(diagnostic(("offset_exact", json!("12"))).is_err());
}

#[test]
fn a_normalized_quantity_lies_within_the_exact_range() {
  // The fourteenth review's case: a report's size of 2^53 + 1 read.
  let read = |member: &str, value: serde_json::Value| {
    serde_json::from_value::<cc_media_probe::MediaReport>(report_with(member, value))
  };
  assert!(read("size", json!(9_007_199_254_740_991_u64)).is_ok());
  assert!(read("size", json!(9_007_199_254_740_993_u64)).is_err());
  let level =
    |value: serde_json::Value| serde_json::from_value::<cc_media_probe::model::TargetLevel>(json!({ "value": value }));
  assert!(level(json!(50)).is_ok());
  assert!(level(json!(9_007_199_254_740_993_u64)).is_err());
}

#[test]
fn a_version_that_is_not_a_string_is_the_version_error_and_a_nested_null_is_malformed() {
  // The fourteenth review: `17` and `null` gave Serde's own type error, not the version error.
  for version in ["17", "null", "true", "[1, 0]", "{}"] {
    let text =
      format!(r#"{{"schema_version": {version}, "generator": {{"name": "g", "version": "1"}}, "results": []}}"#);
    let message = serde_json::from_str::<Document>(&text).unwrap_err().to_string();
    assert!(
      message.starts_with("a schema version is two decimal numbers"),
      "{version}: {message}"
    );
  }
  let nested = |version: &str| {
    let error = format!(r#"{{"schema_version": {version}, "kind": "format_not_determined", "message": "m"}}"#);
    let entry = format!(r#"{{"index": 0, "input": "a", "outcome": "error", "error": {error}}}"#);
    let text =
      format!(r#"{{"schema_version": "1.0", "generator": {{"name": "g", "version": "1"}}, "results": [{entry}]}}"#);
    serde_json::from_str::<Document>(&text).unwrap_err().to_string()
  };
  assert!(nested("17").starts_with("a schema version is two decimal numbers"));
  assert!(nested("null").starts_with("a member holds null"));
}
