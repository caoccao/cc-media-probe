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

//! The test kit's readers (`docs/design/test-kit.md` section 5): pieces that cut the input at fixed bytes, whatever
//! the reads ask for, as the third review of the G3.2 proposal asked.

use std::io::{Cursor, Read, Seek, SeekFrom};

use kit::readers::{FaultingReader, Pieces};

#[test]
fn pieces_end_reads_at_fixed_bytes_whatever_sizes_the_reads_ask_for() {
  let pieces = Pieces::new(&[40, 8]);
  let rooms: Vec<Option<u64>> = [0, 8, 39, 40, 47, 48, 52, 63, 64]
    .iter()
    .map(|position| pieces.room(*position))
    .collect();
  assert_eq!(
    rooms,
    [
      Some(40),
      Some(32),
      Some(1),
      Some(8),
      Some(1),
      Some(8),
      Some(4),
      Some(1),
      Some(8)
    ]
  );
  assert_eq!(Pieces::default().room(5), None);
  let mut buffer = [0_u8; 64];
  assert_eq!(pieces.cut(8, &mut buffer).len(), 32);
  assert_eq!(pieces.cut(40, &mut buffer[..3]).len(), 3);
}

#[test]
fn a_faulting_reader_cuts_at_the_bytes_of_its_pieces_across_seeks() {
  let bytes: Vec<u8> = (0..=99).collect();
  let mut reader = FaultingReader::new(Cursor::new(bytes)).with_pieces(&[40, 8]);
  let mut buffer = [0_u8; 64];
  let mut delivered = Vec::new();
  for _ in 0..3 {
    delivered.push(reader.read(&mut buffer[..8]).unwrap());
  }
  delivered.push(reader.read(&mut buffer).unwrap());
  delivered.push(reader.read(&mut buffer).unwrap());
  assert_eq!(delivered, [8, 8, 8, 16, 8]);
  reader.seek(SeekFrom::Start(44)).unwrap();
  assert_eq!(reader.read(&mut buffer).unwrap(), 4);
  assert_eq!(buffer[0], 44);
}
