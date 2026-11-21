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

//! The coverage figures per crate and per area, and their findings.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use crate::constants::paths::PATH_SEPARATOR;
use crate::coverage::areas::{area_of, crate_of};
use crate::coverage::constants::{
  BELOW_THRESHOLD, CRATE_DIRECTORY_PREFIX, LIBRARY_CRATE, NO_EXECUTABLE_LINES, PUBLISHED_CRATES,
  SOURCE_DIRECTORY_SEGMENT, WINDOWS_SEPARATOR,
};
use crate::coverage::lcov::FileRecord;
use crate::coverage::tally::Tally;
use crate::findings::{Finding, Rule};
use crate::repository::repository_file;
use crate::syntax::ParsedSource;

/// The figures and findings of one coverage report.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CoverageReport {
  /// One line per published crate and per library area, such as
  /// `crates/cc-media-probe: 93.10% (27/29)`.
  pub summary: Vec<String>,
  /// Every crate or area at or below the bar.
  pub findings: Vec<Finding>,
}

/// Applies R28 to the file records of a report. Source files are read below
/// `root` to drop the lines of their embedded test blocks.
#[must_use]
pub fn check_coverage(records: &[FileRecord], root: &Path) -> CoverageReport {
  let mut crates: BTreeMap<String, Tally> = BTreeMap::new();
  let mut areas: BTreeMap<String, Tally> = BTreeMap::new();
  for record in records {
    let relative = repository_relative(&record.path, root);
    let Some(crate_name) = crate_of(&relative) else {
      continue;
    };
    let tally = file_tally(record, &repository_file(root, &relative));
    crates.entry(crate_name.to_owned()).or_default().merge(tally);
    if let Some(area) = area_of(&relative) {
      areas.entry(area.to_owned()).or_default().merge(tally);
    }
  }
  let mut report = CoverageReport::default();
  for name in PUBLISHED_CRATES {
    let label = format!("{CRATE_DIRECTORY_PREFIX}{name}");
    record_figure(&mut report, &label, crates.get(*name).copied().unwrap_or_default());
  }
  for (area, tally) in &areas {
    let label = format!(
      "{CRATE_DIRECTORY_PREFIX}{LIBRARY_CRATE}{PATH_SEPARATOR}{SOURCE_DIRECTORY_SEGMENT}{PATH_SEPARATOR}{area}"
    );
    record_figure(&mut report, &label, *tally);
  }
  report
}

fn record_figure(report: &mut CoverageReport, label: &str, tally: Tally) {
  let Tally { covered, total } = tally;
  if total == 0 {
    report.summary.push(format!("{label}: {NO_EXECUTABLE_LINES}"));
    return;
  }
  let percentage = tally.percentage();
  report
    .summary
    .push(format!("{label}: {percentage} ({covered}/{total})"));
  if !tally.passes() {
    report.findings.push(Finding::new(
      label,
      0,
      Rule::Coverage,
      format!("{BELOW_THRESHOLD}: {percentage} ({covered}/{total})"),
    ));
  }
}

/// The lines of a file before its embedded test block; every line when the
/// file cannot be read or parsed.
fn file_tally(record: &FileRecord, source_path: &Path) -> Tally {
  let test_start = fs::read_to_string(source_path)
    .ok()
    .and_then(|source| ParsedSource::parse(&source).ok())
    .and_then(|parsed| parsed.test_block)
    .map(|block| block.line);
  let mut tally = Tally::default();
  for (line, hits) in &record.lines {
    if test_start.is_none_or(|start| *line < start) {
      tally.add_line(*hits);
    }
  }
  tally
}

/// The report's path relative to `root`, with `/`.
fn repository_relative(path: &str, root: &Path) -> String {
  let normalized = path.replace(WINDOWS_SEPARATOR, &PATH_SEPARATOR.to_string());
  let mut roots = vec![root.to_path_buf()];
  roots.extend(std::path::absolute(root).ok());
  roots.extend(fs::canonicalize(root).ok());
  for candidate in roots {
    let root_text = candidate
      .to_string_lossy()
      .replace(WINDOWS_SEPARATOR, &PATH_SEPARATOR.to_string());
    let prefix = format!("{}{PATH_SEPARATOR}", root_text.trim_end_matches(PATH_SEPARATOR));
    let has_prefix = normalized
      .get(..prefix.len())
      .is_some_and(|start| start.eq_ignore_ascii_case(&prefix));
    if has_prefix {
      return normalized.get(prefix.len()..).unwrap_or_default().to_owned();
    }
  }
  normalized
}

#[cfg(test)]
mod tests {
  use super::{check_coverage, repository_relative};
  use crate::coverage::lcov::FileRecord;
  use std::fs;
  use std::path::Path;

  fn record(path: &str, covered: usize, total: usize) -> FileRecord {
    FileRecord {
      path: path.to_owned(),
      lines: (1..=total).map(|line| (line, u64::from(line <= covered))).collect(),
    }
  }

  #[test]
  fn fails_at_exactly_ninety_percent_for_a_crate_and_an_area() {
    let root = Path::new("/nonexistent");
    let records = vec![
      record("/nonexistent/crates/cc-media-probe/src/containers/matroska.rs", 9, 10),
      record("/nonexistent/crates/cc-media-probe-cli/src/main.rs", 901, 1000),
      record("/nonexistent/crates/cc-media-probe-testkit/src/lib.rs", 0, 10),
      record("/elsewhere/file.rs", 0, 10),
    ];
    let report = check_coverage(&records, root);
    let failing: Vec<&str> = report.findings.iter().map(|finding| finding.path.as_str()).collect();
    assert_eq!(
      failing,
      vec!["crates/cc-media-probe", "crates/cc-media-probe/src/containers"]
    );
    assert_eq!(
      report.summary,
      vec![
        "crates/cc-media-probe: 90.00% (9/10)",
        "crates/cc-media-probe-cli: 90.10% (901/1000)",
        "crates/cc-media-probe/src/containers: 90.00% (9/10)",
      ]
    );
  }

  #[test]
  fn passes_just_above_the_bar() {
    let records = vec![record(
      "/nonexistent/crates/cc-media-probe/src/tags/id3v2.rs",
      901,
      1000,
    )];
    let report = check_coverage(&records, Path::new("/nonexistent"));
    assert_eq!(report.findings, Vec::new());
    assert_eq!(report.summary[1], "crates/cc-media-probe-cli: no executable lines");
  }

  #[test]
  fn drops_the_lines_of_embedded_test_blocks() {
    let root = std::env::temp_dir().join(format!("checks-coverage-{}", std::process::id()));
    let directory = root.join("crates/cc-media-probe/src");
    fs::create_dir_all(&directory).unwrap();
    fs::write(
      directory.join("lib.rs"),
      "fn code() {}\nfn more() {}\n#[cfg(test)]\nmod tests {\n    fn sample() {}\n}\n",
    )
    .unwrap();
    let path = directory.join("lib.rs").to_string_lossy().into_owned();
    let records = vec![FileRecord {
      path,
      lines: vec![(1, 1), (2, 0), (5, 1)],
    }];
    let report = check_coverage(&records, &root);
    fs::remove_dir_all(&root).unwrap();
    assert_eq!(report.summary[0], "crates/cc-media-probe: 50.00% (1/2)");
  }

  #[test]
  fn makes_report_paths_relative() {
    assert_eq!(
      repository_relative("C:\\Work\\Repo\\crates\\a\\src\\lib.rs", Path::new("c:\\work\\repo")),
      "crates/a/src/lib.rs"
    );
    assert_eq!(
      repository_relative("/repo/crates/a.rs", Path::new("/repo/")),
      "crates/a.rs"
    );
    assert_eq!(repository_relative("/other/a.rs", Path::new("/repo")), "/other/a.rs");
    let current = std::env::current_dir().unwrap();
    let inside = current.join("crates/a/src/lib.rs").to_string_lossy().into_owned();
    assert_eq!(repository_relative(&inside, Path::new(".")), "crates/a/src/lib.rs");
    assert_eq!(repository_relative("/other/a.rs", Path::new("/repo")), "/other/a.rs");
  }
}
