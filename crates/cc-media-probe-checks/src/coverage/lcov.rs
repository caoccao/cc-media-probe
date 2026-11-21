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

//! Reading the line records of an LCOV report.

use crate::coverage::constants::{
  END_OF_RECORD, FIELD_SEPARATOR, LINE_DATA_PREFIX, LINE_RECORD_OUTSIDE_FILE, MALFORMED_LINE_RECORD, SOURCE_FILE_PREFIX,
};
use crate::source_error::SourceError;

/// The line hits of one source file.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FileRecord {
  /// The path as the report gives it.
  pub path: String,
  /// Each executable line with its number of executions.
  pub lines: Vec<(usize, u64)>,
}

/// Reads every file record of an LCOV report; records other than files and
/// line data are ignored.
///
/// # Errors
///
/// Returns an error, with the report line, for a line record that cannot
/// be read or that lies outside a file record.
pub fn parse_lcov(report: &str) -> Result<Vec<FileRecord>, SourceError> {
  let mut records = Vec::new();
  let mut current: Option<FileRecord> = None;
  for (offset, line) in report.lines().enumerate() {
    let line_number = offset.saturating_add(1);
    if let Some(path) = line.strip_prefix(SOURCE_FILE_PREFIX) {
      records.extend(current.take());
      current = Some(FileRecord {
        path: path.to_owned(),
        lines: Vec::new(),
      });
    } else if let Some(data) = line.strip_prefix(LINE_DATA_PREFIX) {
      let Some(record) = current.as_mut() else {
        return Err(SourceError {
          line: line_number,
          message: LINE_RECORD_OUTSIDE_FILE,
        });
      };
      let entry = parse_line_data(data).ok_or(SourceError {
        line: line_number,
        message: MALFORMED_LINE_RECORD,
      })?;
      record.lines.push(entry);
    } else if line == END_OF_RECORD {
      records.extend(current.take());
    }
  }
  records.extend(current);
  Ok(records)
}

fn parse_line_data(data: &str) -> Option<(usize, u64)> {
  let mut fields = data.split(FIELD_SEPARATOR);
  let line = fields.next()?.parse().ok()?;
  let hits = fields.next()?.parse().ok()?;
  Some((line, hits))
}

#[cfg(test)]
mod tests {
  use super::{FileRecord, parse_lcov};

  #[test]
  fn reads_file_and_line_records() {
    let report = "TN:\nSF:/repository/crates/cc-media-probe/src/lib.rs\nFN:3,code\nDA:3,1\nDA:4,0,checksum\nLF:2\nLH:1\nend_of_record\nSF:/repository/other.rs\nDA:1,5\n";
    assert_eq!(
      parse_lcov(report).unwrap(),
      vec![
        FileRecord {
          path: "/repository/crates/cc-media-probe/src/lib.rs".to_owned(),
          lines: vec![(3, 1), (4, 0)],
        },
        FileRecord {
          path: "/repository/other.rs".to_owned(),
          lines: vec![(1, 5)],
        },
      ]
    );
  }

  #[test]
  fn rejects_malformed_line_records() {
    assert_eq!(parse_lcov("DA:1,1\n").unwrap_err().line, 1);
    assert_eq!(parse_lcov("SF:a.rs\nDA:x,1\n").unwrap_err().line, 2);
    assert!(parse_lcov("SF:a.rs\nDA:1\n").is_err());
  }
}
