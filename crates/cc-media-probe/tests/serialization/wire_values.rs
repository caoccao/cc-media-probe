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

//! Single values of the form (`json-output.md` sections 5.4 to 5.6, 6.2, 7.2 and 7.3): integer identifiers in
//! canonical decimal at their width; binary tag data in hexadecimal whose length is the size; raw codes and protection
//! schemes in the spelling of their namespace; twins on both sides of the exact range; every typed number within it;
//! rationals in the form that `Rational::new` gives; and the bytes, bits and times of the value records.

use cc_media_probe::completeness::LimitRecord;
use cc_media_probe::diagnostics::Diagnostic;
use cc_media_probe::model::{
  FloatingPointValue, Guid, ProtectionScheme, RawCodecIdentifier, Size, TagValue, TargetLevel, UtcTime,
};
use cc_media_probe::source::SourceIdentifier;
use cc_media_probe::{MediaReport, Rational, Seconds};
use serde_json::{Value, json};

use crate::fixtures::{record, report, usage, with};

#[test]
fn a_source_identifier_is_canonical_decimal_within_64_bits() {
  let source = |identifier: Value| serde_json::from_value::<SourceIdentifier>(identifier);
  assert_eq!(source(json!("18446744073709551615")).unwrap().value(), u64::MAX);
  assert_eq!(serde_json::to_value(source(json!("7")).unwrap()).unwrap(), json!("7"));
  for identifier in [
    json!("01"),
    json!("+1"),
    json!("18446744073709551616"),
    json!("-1"),
    json!(""),
    json!(1),
  ] {
    assert!(source(identifier.clone()).is_err(), "source {identifier}");
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
  assert!(serde_json::from_str::<TagValue>(r#"{"kind": "binary", "binary": {"size": 2}}"#).is_ok());
  assert!(serde_json::from_str::<TagValue>(r#"{"kind": "binary", "binary": {"size": 0, "data": ""}}"#).is_ok());
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
fn a_raw_codec_identifier_is_spelled_by_its_namespace() {
  let read = |namespace: &str, value: &str| {
    serde_json::from_value::<RawCodecIdentifier>(json!({ "namespace": namespace, "value": value }))
  };
  assert_eq!(read("iso_sample_entry", "61766331").unwrap().value(), "61766331");
  assert!(read("riff_four_character_code", "00000000").is_ok());
  assert!(read("quicktime_code", "6a706567").is_ok());
  assert!(read("matroska_codec_identifier", "V_MPEG4/ISO/AVC").is_ok());
  for code in ["avc1", "6176633", "617663310", "6176633G", "6176633A", "+1766331"] {
    assert!(read("iso_sample_entry", code).is_err(), "{code}");
  }
  assert!(read("wave_format_tag", "85").is_ok());
  assert!(read("wave_format_tag", "65535").is_ok());
  for tag in ["085", "+85", "65536", "0x55", "-1", ""] {
    assert!(read("wave_format_tag", tag).is_err(), "{tag}");
  }
  assert!(read("mpeg4_object_type", "64").is_ok());
  assert!(read("mpeg4_object_type", "256").is_err());
  let newer = json!({ "namespace": "future_namespace", "value": "any text" });
  assert_eq!(
    serde_json::from_value::<RawCodecIdentifier>(newer).unwrap().value(),
    "any text"
  );
}

#[test]
fn a_codec_identifier_holds_only_what_its_decoder_writes() {
  let read = |value: &str| {
    serde_json::from_value::<RawCodecIdentifier>(json!({ "namespace": "matroska_codec_identifier", "value": value }))
  };
  assert!(read("A_OPUS").is_ok());
  assert!(read("A_\u{fffd}PUS").is_ok());
  for value in ["A_\u{1f600}", "A_OPUS\u{0}", "A_OPUS\n", "A_\u{e9}"] {
    assert!(read(value).is_err(), "{value:?}");
  }
}

#[test]
fn an_extensible_wave_subformat_is_its_16_bytes_in_hexadecimal() {
  let read = |value: &str| {
    serde_json::from_value::<RawCodecIdentifier>(json!({ "namespace": "wave_subformat", "value": value }))
  };
  assert!(read("0100000000001000800000aa00389b71").is_ok());
  for value in [
    "0100000000001000800000aa00389b7",
    "0100000000001000800000AA00389B71",
    "1",
  ] {
    assert!(read(value).is_err(), "{value}");
  }
}

#[test]
fn a_protection_scheme_is_spelled_by_its_namespace() {
  let read = |namespace: &str, value: &str| {
    serde_json::from_value::<ProtectionScheme>(json!({ "namespace": namespace, "value": value }))
  };
  assert!(read("iso_scheme_type", "63656e63").is_ok());
  assert!(read("iso_scheme_type", "cenc").is_err());
  assert!(read("matroska_content_encryption_algorithm", "5").is_ok());
  assert!(read("matroska_content_encryption_algorithm", "05").is_err());
  assert!(read("future_namespace", "anything").is_ok());
}

#[test]
fn the_record_names_each_limit_or_part_once_in_each_array() {
  let read = |value: Value| serde_json::from_value::<LimitRecord>(value);
  let pair = |limit: &str, part: &str| json!({ "limit": limit, "part": part });
  let stopped = with(
    with(record(), "exhausted", json!(["bytes_read", "working_memory"])),
    "first_stopped",
    json!([pair("bytes_read", "discovery"), pair("working_memory", "discovery")]),
  );
  let reason = json!([{ "part": "discovery", "reason": { "kind": "limit_exhausted", "limit": "bytes_read" } }]);
  let read_record = read(with(stopped, "first_reasons", reason)).unwrap();
  assert!(read_record.exhausted(&cc_media_probe::Limit::WorkingMemory));
  let detection_reason = json!([{ "part": "detection", "reason": { "kind": "truncated_input" } }]);
  for (exhausted, first_stopped, first_reasons) in [
    (
      json!(["bytes_read", "bytes_read"]),
      json!([pair("bytes_read", "detection")]),
      detection_reason.clone(),
    ),
    (
      json!(["bytes_read"]),
      json!([pair("bytes_read", "detection"), pair("bytes_read", "discovery")]),
      detection_reason.clone(),
    ),
    (json!(["bytes_read"]), json!([]), json!([])),
    (
      json!([]),
      json!([pair("bytes_read", "detection")]),
      detection_reason.clone(),
    ),
    (
      json!(["bytes_read", "working_memory"]),
      json!([pair("working_memory", "detection"), pair("bytes_read", "detection")]),
      detection_reason,
    ),
    (
      json!([]),
      json!([]),
      json!([
        { "part": "tags", "reason": { "kind": "truncated_input" } },
        { "part": "tags", "reason": { "kind": "truncated_input" } },
      ]),
    ),
  ] {
    let broken = with(
      with(with(record(), "exhausted", exhausted), "first_stopped", first_stopped),
      "first_reasons",
      first_reasons,
    );
    assert!(read(broken.clone()).is_err(), "{broken}");
  }
}

#[test]
fn usage_counters_and_diagnostic_offsets_are_twins_on_both_sides_of_the_range() {
  let read = |usage: Value| serde_json::from_value::<MediaReport>(with(report("1.0"), "usage", usage));
  let large = read(with(
    crate::fixtures::without(usage(), "bytes_read"),
    "bytes_read_exact",
    json!("9007199254740993"),
  ))
  .unwrap();
  assert_eq!(large.usage().bytes_read(), 9_007_199_254_740_993);
  assert_eq!(
    serde_json::to_value(&large).unwrap()["usage"]["bytes_read_exact"],
    json!("9007199254740993")
  );
  assert!(read(with(usage(), "bytes_read", json!(9_007_199_254_740_991_u64))).is_ok());
  for malformed in [
    with(usage(), "bytes_read", json!(9_007_199_254_740_993_u64)),
    with(
      crate::fixtures::without(usage(), "bytes_read"),
      "bytes_read_exact",
      json!("9007199254740991"),
    ),
    with(usage(), "bytes_read_exact", json!("9007199254740993")),
    crate::fixtures::without(usage(), "work_units"),
  ] {
    assert!(read(malformed.clone()).is_err(), "{malformed}");
  }
  let diagnostic = |offset: (&str, Value)| {
    let mut diagnostic =
      json!({ "severity": "warning", "code": "matroska.void", "message": "m", "source_identifier": "0" });
    diagnostic[offset.0] = offset.1;
    serde_json::from_value::<Diagnostic>(diagnostic)
  };
  assert_eq!(
    diagnostic(("logical_offset_exact", json!("9007199254740993")))
      .unwrap()
      .logical_offset(),
    Some(9_007_199_254_740_993)
  );
  assert!(diagnostic(("offset", json!(9_007_199_254_740_993_u64))).is_err());
  assert!(diagnostic(("offset_exact", json!("12"))).is_err());
}

#[test]
fn every_typed_number_lies_within_the_exact_range() {
  let read = |member: &str, value: Value| serde_json::from_value::<MediaReport>(with(report("1.0"), member, value));
  assert!(read("size", json!(9_007_199_254_740_991_u64)).is_ok());
  assert!(read("size", json!(9_007_199_254_740_993_u64)).is_err());
  let level = |value: Value| serde_json::from_value::<TargetLevel>(json!({ "value": value }));
  assert!(level(json!(50)).is_ok());
  assert!(level(json!(9_007_199_254_740_993_u64)).is_err());
  let size = |width: Value| serde_json::from_value::<Size>(json!({ "width": width, "height": 1 }));
  assert!(size(json!(9_007_199_254_740_991_u64)).is_ok());
  assert!(size(json!(9_007_199_254_740_992_u64)).is_err());
}

#[test]
fn a_rational_reads_back_only_in_the_form_that_new_gives() {
  let read = |value: Value| serde_json::from_value::<Rational>(value);
  assert_eq!(
    read(json!({ "numerator": -3, "denominator": 2 })).ok(),
    Rational::new(-3, 2)
  );
  for malformed in [
    json!({ "numerator": 2, "denominator": 4 }),
    json!({ "numerator": 1, "denominator": 0 }),
    json!({ "numerator": 0, "denominator": 2 }),
    json!({ "numerator": 9_007_199_254_740_992_i64, "denominator": 1 }),
    json!({ "numerator": 1, "denominator": -1 }),
    json!([1, 2]),
    json!({ "numerator": 1 }),
  ] {
    assert!(read(malformed.clone()).is_err(), "{malformed}");
  }
  let seconds: Seconds = serde_json::from_value(json!({ "numerator": 21, "denominator": 2 })).unwrap();
  assert_eq!(seconds.to_string(), "10.5 s");
  assert_eq!(
    serde_json::to_value(seconds).unwrap(),
    json!({ "numerator": 21, "denominator": 2 })
  );
}

#[test]
fn bytes_bits_and_times_have_one_form() {
  let guid = "a1b2c3d4e5f60718293a4b5c6d7e8f90";
  let read_guid: Guid = serde_json::from_value(json!(guid)).unwrap();
  assert_eq!(serde_json::to_value(read_guid).unwrap(), json!(guid));
  for malformed in [json!("A1B2C3D4E5F60718293A4B5C6D7E8F90"), json!("a1b2"), json!(1)] {
    assert!(serde_json::from_value::<Guid>(malformed).is_err());
  }
  let double = json!({ "width": "double", "bits": "3ff0000000000000" });
  let value: FloatingPointValue = serde_json::from_value(double.clone()).unwrap();
  assert_eq!(value.to_f64(), 1.0);
  assert_eq!(serde_json::to_value(&value).unwrap(), double);
  let single = json!({ "width": "single", "bits": "3f800000" });
  assert_eq!(
    serde_json::from_value::<FloatingPointValue>(single.clone())
      .unwrap()
      .to_f64(),
    1.0
  );
  for malformed in [
    json!({ "width": "single", "bits": "3ff0000000000000" }),
    json!({ "width": "double", "bits": "3F80000000000000" }),
    json!({ "width": "double", "bits": 1 }),
  ] {
    assert!(
      serde_json::from_value::<FloatingPointValue>(malformed.clone()).is_err(),
      "{malformed}"
    );
  }
  let time: UtcTime = serde_json::from_value(json!("2026-11-23T08:30:00.25Z")).unwrap();
  assert_eq!(
    (time.seconds_since_unix_epoch(), time.nanoseconds()),
    (1_795_422_600, 250_000_000)
  );
  assert_eq!(serde_json::to_value(time).unwrap(), json!("2026-11-23T08:30:00.25Z"));
  assert!(serde_json::from_value::<UtcTime>(json!("2026-11-23T08:30:00.250Z")).is_err());
}

#[test]
fn a_nested_version_that_is_not_a_string_is_malformed_when_it_is_null() {
  let partial = |version: Value| {
    let mut partial = report("1.0");
    partial["schema_version"] = version;
    with(crate::fixtures::error_record("1.0"), "partial_report", partial)
  };
  let message = serde_json::from_value::<cc_media_probe::error::ErrorRecord>(partial(json!(17)))
    .unwrap_err()
    .to_string();
  assert!(message.starts_with("the version error"), "{message}");
  let message = serde_json::from_value::<cc_media_probe::error::ErrorRecord>(partial(Value::Null))
    .unwrap_err()
    .to_string();
  assert!(message.starts_with("a member holds null"), "{message}");
}
