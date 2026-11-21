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

//! The third layer of D4: the outputs of every build script that ran in a
//! target directory, and the files each one left in its output directory.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::findings::{Finding, Rule};
use crate::inventory::constants::{
  INSTRUCTION_PREFIXES, LEGACY_OUTPUT_FILE, LINK_INSTRUCTION_FOUND, LINK_INSTRUCTIONS, LINK_LIBRARY_INSTRUCTION,
  MODIFIER_SEPARATOR, NATIVE_EXTENSIONS, NATIVE_OBJECT_FOUND, OUT_DIRECTORY, RUN_DIRECTORY, RUN_OUTPUT_FILE,
  VALUE_SEPARATOR,
};
use crate::repository::relative_path;

/// Inspects every build-script run below `target_directory`. A
/// `rustc-link-lib` instruction is allowed when its library is in
/// `allowed_libraries`; every other link instruction and every native object
/// is a finding. Returns the number of runs inspected and the findings.
///
/// # Errors
///
/// Returns an error when a directory or output file cannot be read.
pub fn check_build_outputs(target_directory: &Path, allowed_libraries: &[String]) -> io::Result<(usize, Vec<Finding>)> {
  let runs = build_script_runs(target_directory)?;
  let mut findings = Vec::new();
  for (output_file, out_directory) in &runs {
    let label = relative_path(target_directory, output_file);
    let text = fs::read_to_string(output_file)?;
    for (offset, line) in text.lines().enumerate() {
      if let Some(instruction) =
        link_instruction(line).filter(|instruction| !is_allowed(instruction, allowed_libraries))
      {
        findings.push(Finding::new(
          &label,
          offset.saturating_add(1),
          Rule::BuildOutput,
          format!("{LINK_INSTRUCTION_FOUND}: {instruction}"),
        ));
      }
    }
    for object in native_objects(out_directory)? {
      let object_label = relative_path(target_directory, &object);
      findings.push(Finding::new(&object_label, 0, Rule::BuildOutput, NATIVE_OBJECT_FOUND));
    }
  }
  findings.sort();
  Ok((runs.len(), findings))
}

/// The output file and output directory of every build-script run: in
/// Cargo 1.100 a unit directory with `run/stdout` and `out/`, in the older
/// layout a directory with `output` and `out/`.
fn build_script_runs(target_directory: &Path) -> io::Result<Vec<(PathBuf, PathBuf)>> {
  let mut runs = Vec::new();
  let mut pending = vec![target_directory.to_path_buf()];
  while let Some(directory) = pending.pop() {
    let run_output = directory.join(RUN_DIRECTORY).join(RUN_OUTPUT_FILE);
    let legacy_output = directory.join(LEGACY_OUTPUT_FILE);
    let out_directory = directory.join(OUT_DIRECTORY);
    if run_output.is_file() {
      runs.push((run_output, out_directory));
    } else if legacy_output.is_file() && out_directory.is_dir() {
      runs.push((legacy_output, out_directory));
    }
    for entry in fs::read_dir(&directory)? {
      let entry = entry?;
      if entry.file_type()?.is_dir() {
        pending.push(entry.path());
      }
    }
  }
  runs.sort();
  Ok(runs)
}

/// The instruction without its `cargo:` or `cargo::` prefix, if it links.
fn link_instruction(line: &str) -> Option<&str> {
  let instruction = INSTRUCTION_PREFIXES
    .iter()
    .find_map(|prefix| line.strip_prefix(prefix))?;
  let name = instruction.split(VALUE_SEPARATOR).next().unwrap_or_default();
  LINK_INSTRUCTIONS
    .iter()
    .any(|link| name.starts_with(link))
    .then_some(instruction)
}

/// Whether a `rustc-link-lib` instruction names an allowed library; the
/// link kind and modifiers, as in `dylib:+verbatim=name`, are ignored.
fn is_allowed(instruction: &str, allowed_libraries: &[String]) -> bool {
  let Some((name, value)) = instruction.split_once(VALUE_SEPARATOR) else {
    return false;
  };
  let library = value.rsplit(VALUE_SEPARATOR).next().unwrap_or(value);
  let library = library.rsplit(MODIFIER_SEPARATOR).next().unwrap_or(library);
  name == LINK_LIBRARY_INSTRUCTION && allowed_libraries.iter().any(|allowed| allowed == library)
}

/// Native objects and libraries anywhere below an output directory.
fn native_objects(out_directory: &Path) -> io::Result<Vec<PathBuf>> {
  let mut objects = Vec::new();
  if !out_directory.is_dir() {
    return Ok(objects);
  }
  let mut pending = vec![out_directory.to_path_buf()];
  while let Some(directory) = pending.pop() {
    for entry in fs::read_dir(&directory)? {
      let entry = entry?;
      let path = entry.path();
      if entry.file_type()?.is_dir() {
        pending.push(path);
      } else if path
        .extension()
        .is_some_and(|extension| NATIVE_EXTENSIONS.iter().any(|native| extension == *native))
      {
        objects.push(path);
      }
    }
  }
  Ok(objects)
}

#[cfg(test)]
mod tests {
  use super::{check_build_outputs, is_allowed, link_instruction};
  use std::fs;

  #[test]
  fn recognizes_link_instructions() {
    assert_eq!(
      link_instruction("cargo::rustc-link-lib=static=native"),
      Some("rustc-link-lib=static=native")
    );
    assert_eq!(
      link_instruction("cargo:rustc-link-search=native=/opt/lib"),
      Some("rustc-link-search=native=/opt/lib")
    );
    assert_eq!(
      link_instruction("cargo::rustc-link-arg-bins=-lfoo"),
      Some("rustc-link-arg-bins=-lfoo")
    );
    assert_eq!(link_instruction("cargo::rustc-cfg=feature_flag"), None);
    assert_eq!(link_instruction("warning: text"), None);
  }

  #[test]
  fn allows_listed_libraries_only() {
    let allowed = vec!["System".to_owned()];
    assert!(is_allowed("rustc-link-lib=System", &allowed));
    assert!(is_allowed("rustc-link-lib=dylib:+verbatim=System", &allowed));
    assert!(!is_allowed("rustc-link-lib=native", &allowed));
    assert!(!is_allowed("rustc-link-search=System", &allowed));
    assert!(!is_allowed("rustc-link-lib", &allowed));
  }

  #[test]
  fn inspects_both_layouts() {
    let target = std::env::temp_dir().join(format!("checks-build-output-{}", std::process::id()));
    let _ = fs::remove_dir_all(&target);
    let modern = target.join("debug/build/parser/0123/");
    fs::create_dir_all(modern.join("run")).unwrap();
    fs::create_dir_all(modern.join("out/nested")).unwrap();
    fs::write(
      modern.join("run/stdout"),
      "cargo::rustc-cfg=flag\ncargo::rustc-link-lib=static=native\n",
    )
    .unwrap();
    fs::write(modern.join("out/nested/native.o"), "").unwrap();
    fs::write(modern.join("out/generated.rs"), "").unwrap();
    let legacy = target.join("debug/build/old-4567");
    fs::create_dir_all(legacy.join("out")).unwrap();
    fs::write(legacy.join("output"), "cargo:rustc-link-lib=System\n").unwrap();
    let library = target.join("debug/build/library/89ab/out");
    fs::create_dir_all(&library).unwrap();
    fs::write(library.join("library.rcgu.o"), "").unwrap();
    let (runs, findings) = check_build_outputs(&target, &["System".to_owned()]).unwrap();
    fs::remove_dir_all(&target).unwrap();
    assert_eq!(runs, 2);
    let labels: Vec<String> = findings
      .iter()
      .map(|finding| format!("{}:{}", finding.path, finding.line))
      .collect();
    assert_eq!(
      labels,
      vec![
        "debug/build/parser/0123/out/nested/native.o:0",
        "debug/build/parser/0123/run/stdout:2"
      ]
    );
  }

  #[test]
  fn reports_a_missing_target_directory() {
    assert!(check_build_outputs(std::path::Path::new("/nonexistent/target"), &[]).is_err());
  }
}
