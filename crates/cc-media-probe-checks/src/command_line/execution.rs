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

//! Running one command.

use std::fs;
use std::io;
use std::path::Path;

use crate::command_line::arguments::Command;
use crate::command_line::constants::BUILD_SCRIPT_RUNS_WORD;
use crate::constants::paths::MANIFEST_FILE;
use crate::coverage::{check_coverage, parse_lcov};
use crate::findings::Finding;
use crate::inventory::constants::{
  BUILD_SCRIPT_PLATFORM, FULL_METADATA_ARGUMENTS, PUBLISHED_PACKAGES, REVIEWED_SCRIPTS_FILE, SYSTEM_LIBRARIES_FILE,
};
use crate::inventory::{
  ParsedList, check_build_outputs, check_dynamic_libraries, check_inventory, parse_allowed_libraries,
  parse_reviewed_scripts, published_graph,
};
use crate::repository::{repository_name, repository_path};
use crate::structure::check_repository;
use crate::targets::constants::EXPECTED_TARGETS_FILE;
use crate::targets::{check_exclusion, compare_targets, parse_expected_targets, run_cargo_metadata, workspace_targets};

/// What a command found: lines to print, and the findings.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Outcome {
  /// Lines that describe what was checked.
  pub summary: Vec<String>,
  /// The problems found.
  pub findings: Vec<Finding>,
}

/// Runs `command`; `Command::Help` yields an empty outcome.
///
/// # Errors
///
/// Returns an error when a file cannot be read or Cargo cannot be run.
pub fn execute(command: &Command) -> io::Result<Outcome> {
  match command {
    Command::Structure { root } => Ok(Outcome {
      summary: Vec::new(),
      findings: check_repository(root)?,
    }),
    Command::Coverage { report, root } => {
      let text = fs::read_to_string(report)?;
      let records = parse_lcov(&text).map_err(io::Error::other)?;
      let coverage = check_coverage(&records, root);
      Ok(Outcome {
        summary: coverage.summary,
        findings: coverage.findings,
      })
    }
    Command::Targets { root } => targets(root),
    Command::Inventory { root } => inventory(root),
    Command::BuildOutput { target_directory, root } => {
      let (allowed, mut findings) = allowed_libraries(root)?;
      let build_script_libraries: Vec<String> = allowed
        .into_iter()
        .filter(|(platform, _)| platform == BUILD_SCRIPT_PLATFORM)
        .map(|(_, library)| library)
        .collect();
      let (runs, output_findings) = check_build_outputs(target_directory, &build_script_libraries)?;
      findings.extend(output_findings);
      Ok(Outcome {
        summary: vec![format!("{runs} {BUILD_SCRIPT_RUNS_WORD}")],
        findings,
      })
    }
    Command::DynamicLibraries {
      platform,
      libraries,
      root,
    } => {
      let (allowed, mut findings) = allowed_libraries(root)?;
      findings.extend(check_dynamic_libraries(platform, libraries, &allowed));
      Ok(Outcome {
        summary: Vec::new(),
        findings,
      })
    }
    Command::Help => Ok(Outcome::default()),
  }
}

fn targets(root: &Path) -> io::Result<Outcome> {
  let list = repository_name(EXPECTED_TARGETS_FILE);
  let text = fs::read_to_string(repository_path(root, EXPECTED_TARGETS_FILE.iter().copied()))?;
  let (expected, mut findings) = parse_expected_targets(&list, &text);
  let metadata = run_cargo_metadata(&root.join(MANIFEST_FILE), None)?;
  let actual = workspace_targets(&metadata);
  findings.extend(compare_targets(&list, &actual, &expected));
  findings.extend(check_exclusion(root)?);
  Ok(Outcome {
    summary: actual
      .iter()
      .map(|entry| format!("{} {} {}", entry.package, entry.kind, entry.name))
      .collect(),
    findings,
  })
}

fn inventory(root: &Path) -> io::Result<Outcome> {
  let text = fs::read_to_string(repository_path(root, REVIEWED_SCRIPTS_FILE.iter().copied()))?;
  let (reviewed, mut findings) = parse_reviewed_scripts(&repository_name(REVIEWED_SCRIPTS_FILE), &text);
  let metadata = run_cargo_metadata(&root.join(MANIFEST_FILE), Some(FULL_METADATA_ARGUMENTS))?;
  let graph = published_graph(&metadata, PUBLISHED_PACKAGES);
  let (summary, inventory_findings) = check_inventory(&graph, &reviewed);
  findings.extend(inventory_findings);
  Ok(Outcome { summary, findings })
}

fn allowed_libraries(root: &Path) -> io::Result<ParsedList> {
  let text = fs::read_to_string(repository_path(root, SYSTEM_LIBRARIES_FILE.iter().copied()))?;
  Ok(parse_allowed_libraries(&repository_name(SYSTEM_LIBRARIES_FILE), &text))
}

#[cfg(test)]
mod tests {
  use super::execute;
  use crate::command_line::arguments::Command;
  use std::fs;
  use std::path::{Path, PathBuf};

  fn repository() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
  }

  #[test]
  fn checks_the_targets_and_the_inventory_of_this_repository() {
    let targets = execute(&Command::Targets { root: repository() }).unwrap();
    assert_eq!(targets.findings, Vec::new());
    assert!(
      targets
        .summary
        .iter()
        .any(|line| line == "cc-media-probe lib cc_media_probe")
    );
    let inventory = execute(&Command::Inventory { root: repository() }).unwrap();
    assert_eq!(inventory.findings, Vec::new());
    assert!(inventory.summary.iter().any(|line| line.starts_with("cc-media-probe ")));
  }

  #[test]
  fn checks_coverage_build_outputs_and_libraries() {
    let directory = std::env::temp_dir().join(format!("checks-execution-{}", std::process::id()));
    fs::create_dir_all(&directory).unwrap();
    let report = directory.join("lcov.info");
    fs::write(
      &report,
      "SF:/elsewhere/crates/cc-media-probe/src/lib.rs\nDA:1,0\nend_of_record\n",
    )
    .unwrap();
    let coverage = execute(&Command::Coverage {
      report,
      root: PathBuf::from("/elsewhere"),
    })
    .unwrap();
    assert_eq!(coverage.findings.len(), 1);
    let outputs = execute(&Command::BuildOutput {
      target_directory: directory.clone(),
      root: repository(),
    })
    .unwrap();
    assert_eq!(outputs.summary, vec!["0 build-script runs inspected"]);
    let libraries = execute(&Command::DynamicLibraries {
      platform: "macos".to_owned(),
      libraries: vec![
        "/usr/lib/libSystem.B.dylib".to_owned(),
        "/opt/libnative.dylib".to_owned(),
      ],
      root: repository(),
    })
    .unwrap();
    fs::remove_dir_all(&directory).unwrap();
    assert_eq!(libraries.findings.len(), 1);
    assert_eq!(execute(&Command::Help).unwrap().findings, Vec::new());
  }

  #[test]
  fn reports_unreadable_inputs() {
    assert!(
      execute(&Command::Coverage {
        report: PathBuf::from("/nonexistent/lcov.info"),
        root: repository()
      })
      .is_err()
    );
    assert!(
      execute(&Command::Structure {
        root: PathBuf::from("/nonexistent/root")
      })
      .is_err()
    );
    assert!(
      execute(&Command::Targets {
        root: PathBuf::from("/nonexistent/root")
      })
      .is_err()
    );
  }
}
