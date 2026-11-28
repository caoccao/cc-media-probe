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

//! The writers against the specifications' own vectors (`docs/design/test-kit.md` section 5): the Exp-Golomb bit
//! strings of ITU-T H.264, tables 9-2 and 9-3; the coding example of the Vorbis I specification, section 2.1.6,
//! least significant bit first; and the escape of ITU-T H.264, section 7.4.1, which the NAL unit syntax of section
//! 7.3.1 undoes.

use cc_media_probe_testkit::builder::{BitOrder, BitWriter, BuildError, ByteWriter, insert_emulation_prevention};

/// The bytes of a string of `0` and `1`, most significant bit first, padded with zeros to a whole byte.
fn bytes_of(bits: &str) -> Vec<u8> {
  let digits: Vec<u8> = bits
    .bytes()
    .filter(|digit| *digit != b' ')
    .map(|digit| u8::from(digit == b'1'))
    .collect();
  digits
    .chunks(8)
    .map(|chunk| (0..8).fold(0_u8, |byte, index| (byte << 1) | chunk.get(index).copied().unwrap_or(0)))
    .collect()
}

#[test]
fn a_byte_writer_writes_each_order_and_width() {
  let mut writer = ByteWriter::new();
  writer
    .byte(1)
    .bytes(&[2, 3])
    .text("ab")
    .four_character_code(*b"ftyp")
    .big_endian_u16(0x0102)
    .big_endian_u24(0x03_0405)
    .big_endian_u32(0x0607_0809)
    .big_endian_u64(0x0a0b_0c0d_0e0f_1011)
    .little_endian_u16(0x0102)
    .little_endian_u24(0x03_0405)
    .little_endian_u32(0x0607_0809)
    .little_endian_u64(0x0a0b_0c0d_0e0f_1011);
  let expected: Vec<u8> = [
    &[1, 2, 3, b'a', b'b'][..],
    b"ftyp",
    &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17],
    &[2, 1, 5, 4, 3, 9, 8, 7, 6, 17, 16, 15, 14, 13, 12, 11, 10],
  ]
  .concat();
  assert_eq!(writer.as_bytes(), expected);
  assert_eq!(writer.into_bytes().unwrap(), expected);
}

#[test]
fn a_twenty_four_bit_value_wider_than_its_field_is_the_writers_error() {
  for wide in [
    ByteWriter::new().big_endian_u24(0x0100_0000).clone(),
    ByteWriter::new().little_endian_u24(u32::MAX).clone(),
  ] {
    assert_eq!(wide.as_bytes(), &[] as &[u8]);
    assert!(matches!(wide.into_bytes(), Err(BuildError::TooWide { bits: 24, .. })));
  }
  let mut first = ByteWriter::new();
  first.big_endian_u24(0x0100_0000).little_endian_u24(0x0200_0000).byte(9);
  assert_eq!(first.as_bytes(), [9]);
  assert_eq!(
    first.into_bytes(),
    Err(BuildError::TooWide {
      value: 0x0100_0000,
      bits: 24
    })
  );
}

#[test]
fn unsigned_exponential_golomb_codes_are_the_bit_strings_of_table_9_2() {
  let table = [
    "1", "010", "011", "00100", "00101", "00110", "00111", "0001000", "0001001", "0001010",
  ];
  let mut writer = BitWriter::most_significant_first();
  for code in 0..10 {
    writer.unsigned_exponential_golomb(code);
  }
  assert_eq!(
    writer.align_with_zeros().clone().into_bytes().unwrap(),
    bytes_of(&table.concat())
  );
  let mut widest = BitWriter::most_significant_first();
  widest.unsigned_exponential_golomb(u64::MAX).align_with_zeros();
  let expected: Vec<u8> = [vec![0; 8], vec![0x80], vec![0; 8]].concat();
  assert_eq!(widest.into_bytes().unwrap(), expected);
}

#[test]
fn signed_exponential_golomb_codes_follow_the_assignment_of_table_9_3() {
  let table: [(i64, u64); 7] = [(0, 0), (1, 1), (-1, 2), (2, 3), (-2, 4), (3, 5), (-3, 6)];
  let largest = (i64::MAX, u64::MAX - 2);
  for (value, code) in table.into_iter().chain([largest]) {
    let mut signed = BitWriter::most_significant_first();
    signed.signed_exponential_golomb(value).align_with_zeros();
    let mut unsigned = BitWriter::most_significant_first();
    unsigned.unsigned_exponential_golomb(code).align_with_zeros();
    assert_eq!(signed.into_bytes(), unsigned.into_bytes(), "{value}");
  }
}

#[test]
fn least_significant_first_packs_the_vorbis_coding_example() {
  // Vorbis I, section 2.1.6: 12 in 4 bits, -1 in 3 bits, 17 in 7 bits and 6969 in 13 bits.
  let mut writer = BitWriter::least_significant_first();
  writer.bits(12, 4).bits(0b111, 3).bits(17, 7).bits(6969, 13);
  assert!(!writer.is_aligned());
  writer.align_with_zeros();
  assert_eq!(writer.into_bytes().unwrap(), [0xfc, 0x48, 0xce, 0x06]);
}

#[test]
fn a_bit_writer_aligns_with_zeros_or_ones_in_its_order() {
  let mut most = BitWriter::new(BitOrder::MostSignificantFirst);
  most
    .bits(0b101, 3)
    .flag(true)
    .align_with_zeros()
    .bits(0, 2)
    .align_with_ones();
  assert_eq!(most.into_bytes().unwrap(), [0b1011_0000, 0b0011_1111]);
  let mut least = BitWriter::new(BitOrder::LeastSignificantFirst);
  least
    .bits(0b101, 3)
    .flag(true)
    .align_with_zeros()
    .bits(0, 2)
    .align_with_ones();
  assert_eq!(least.into_bytes().unwrap(), [0b0000_1101, 0b1111_1100]);
  let mut whole = BitWriter::most_significant_first();
  whole.bits(u64::MAX, 64).bits(0, 0).align_with_ones();
  assert_eq!(whole.into_bytes().unwrap(), [0xff; 8]);
}

#[test]
fn a_bit_writer_refuses_a_value_wider_than_its_field_and_an_unaligned_end() {
  let mut wide = BitWriter::most_significant_first();
  wide.bits(8, 3).bits(0, 65).align_with_zeros();
  assert_eq!(wide.into_bytes(), Err(BuildError::TooWide { value: 8, bits: 3 }));
  let mut many = BitWriter::most_significant_first();
  many.bits(0, 65);
  assert_eq!(many.into_bytes(), Err(BuildError::TooManyBits(65)));
  let mut unaligned = BitWriter::most_significant_first();
  unaligned.bits(1, 3);
  assert_eq!(unaligned.into_bytes(), Err(BuildError::UnalignedEnd(3)));
  assert_eq!(
    BuildError::UnalignedEnd(3).to_string(),
    "the bits end inside a byte, after this many bits of it: 3"
  );
}

#[test]
fn emulation_prevention_escapes_what_section_7_4_1_forbids() {
  let cases: [(&[u8], &[u8]); 10] = [
    (&[], &[]),
    (&[0, 0, 0], &[0, 0, 3, 0, 3]),
    (&[0, 0, 1], &[0, 0, 3, 1]),
    (&[0, 0, 2], &[0, 0, 3, 2]),
    (&[0, 0, 3], &[0, 0, 3, 3]),
    (&[0, 0, 4], &[0, 0, 4]),
    (&[0, 0, 0, 0, 0], &[0, 0, 3, 0, 0, 3, 0, 3]),
    (&[0x25, 0, 0, 1, 0x80], &[0x25, 0, 0, 3, 1, 0x80]),
    // A cabac_zero_word at the end is represented by the three bytes 0x000003 (section 7.4.2.10).
    (&[0x80, 0, 0], &[0x80, 0, 0, 3]),
    (&[0, 3, 0, 0, 0x7f], &[0, 3, 0, 0, 0x7f]),
  ];
  for (payload, escaped) in cases {
    assert_eq!(insert_emulation_prevention(payload), escaped, "{payload:02x?}");
  }
}

/// The payload of escaped bytes as the NAL unit syntax of ITU-T H.264, section 7.3.1, reads it: two bytes, then a
/// skipped `emulation_prevention_three_byte`, wherever the next 24 bits are 0x000003.
fn unescaped(escaped: &[u8]) -> Vec<u8> {
  let mut payload = Vec::new();
  let mut rest = escaped;
  while let Some((byte, after)) = rest.split_first() {
    if let [0, 0, 3, skipped @ ..] = rest {
      payload.extend_from_slice(&[0, 0]);
      rest = skipped;
    } else {
      payload.push(*byte);
      rest = after;
    }
  }
  payload
}

#[test]
fn every_short_payload_escapes_to_bytes_that_section_7_4_1_allows_and_the_syntax_reads_back() {
  let alphabet = [0_u8, 1, 2, 3, 4];
  let mut payloads: Vec<Vec<u8>> = vec![Vec::new()];
  for _ in 0..5 {
    let longer: Vec<Vec<u8>> = payloads
      .iter()
      .filter(|payload| payload.len() == payloads.last().unwrap().len())
      .flat_map(|payload| alphabet.iter().map(|byte| [payload.as_slice(), &[*byte]].concat()))
      .collect();
    payloads.extend(longer);
  }
  assert_eq!(payloads.len(), 1 + 5 + 25 + 125 + 625 + 3125);
  for payload in payloads {
    let escaped = insert_emulation_prevention(&payload);
    // A payload whose last byte is 0x00 takes a final 0x03, which the syntax reads back only after the pairs of
    // zero bytes of cabac_zero_words, the only payload that can end so (section 7.4.1.1).
    let trailing_zeros = payload.iter().rev().take_while(|byte| **byte == 0).count();
    if trailing_zeros % 2 == 0 {
      assert_eq!(unescaped(&escaped), payload, "{payload:02x?}");
    }
    assert_ne!(escaped.last(), Some(&0), "{payload:02x?}");
    for window in escaped.windows(3) {
      assert!(!matches!(window, [0, 0, 0..=2]), "{payload:02x?} → {escaped:02x?}");
    }
    for window in escaped.windows(4) {
      assert!(
        !matches!(window, [0, 0, 3, 4..=u8::MAX]),
        "{payload:02x?} → {escaped:02x?}"
      );
    }
  }
}
