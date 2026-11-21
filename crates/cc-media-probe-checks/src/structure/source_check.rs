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

//! Every rule that applies to one source file.

use crate::findings::{Finding, Rule};
use crate::literals::check_literals;
use crate::names::check_names;
use crate::repository::SourceFile;
use crate::rules::{check_header, check_inclusion, check_line_limit, check_module_root};
use crate::syntax::ParsedSource;

/// Checks one file: the copyright header, R30 for module roots, R31, R32 and
/// R33.
#[must_use]
pub fn check_source(file: &SourceFile, source: &str) -> Vec<Finding> {
  let path = file.relative_path.as_str();
  let mut findings: Vec<Finding> = check_header(path, source).into_iter().collect();
  let parsed = match ParsedSource::parse(source) {
    Ok(parsed) => parsed,
    Err(error) => {
      findings.push(Finding::new(path, error.line, Rule::Syntax, error.message));
      return findings;
    }
  };
  if file.is_module_root() {
    findings.extend(check_module_root(path, &parsed, file.is_binary_root()));
  }
  findings.extend(check_line_limit(path, source, parsed.test_block));
  findings.extend(check_inclusion(path, &parsed));
  findings.extend(check_names(path, &parsed));
  findings.extend(check_literals(
    path,
    &parsed,
    file.is_exempt_from_constants(),
    file.is_module_root(),
  ));
  findings
}

#[cfg(test)]
mod tests {
  use super::check_source;
  use crate::findings::Rule;
  use crate::repository::classify;

  const HEADER: &str = "/*
* Copyright (c) 2026. caoccao.com Sam Cao
* All rights reserved.

* Licensed under the Apache License, Version 2.0 (the \"License\");
* you may not use this file except in compliance with the License.
* You may obtain a copy of the License at

* http://www.apache.org/licenses/LICENSE-2.0

* Unless required by applicable law or agreed to in writing, software
* distributed under the License is distributed on an \"AS IS\" BASIS,
* WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
* See the License for the specific language governing permissions and
* limitations under the License.
*/
";

  fn rules(path: &str, body: &str) -> Vec<Rule> {
    let file = classify(path, &[String::new()]);
    check_source(&file, &format!("{HEADER}\n{body}"))
      .into_iter()
      .map(|finding| finding.rule)
      .collect()
  }

  #[test]
  fn passes_a_clean_module_root_and_file() {
    assert_eq!(rules("src/lib.rs", "//! Docs.\n\npub mod segment;\n"), Vec::new());
    assert_eq!(
      rules(
        "src/segment.rs",
        "//! Segment.\n\npub fn parse(input: &[u8]) -> usize {\n    input.len()\n}\n"
      ),
      Vec::new()
    );
  }

  #[test]
  fn applies_every_rule() {
    assert_eq!(rules("src/mod.rs", "pub fn helper() {}\n"), vec![Rule::ModuleRoot]);
    assert_eq!(
      rules("src/reader.rs", "pub fn read(header_buf: u8) {}\n"),
      vec![Rule::Names]
    );
    assert_eq!(
      rules("src/reader.rs", "pub fn read() -> u8 { 7 }\n"),
      vec![Rule::Constants]
    );
    assert_eq!(
      rules("src/reader.rs", "include!(\"other.rs\");\n"),
      vec![Rule::Inclusion, Rule::Constants]
    );
    let long = "// line\n".repeat(500);
    assert_eq!(rules("src/long.rs", &long), vec![Rule::LineLimit]);
  }

  #[test]
  fn exempts_tests_from_literals_but_not_from_names() {
    assert_eq!(rules("tests/golden.rs", "fn sample() -> u8 { 7 }\n"), Vec::new());
    assert_eq!(rules("tests/golden.rs", "fn sample(x: u8) {}\n"), vec![Rule::Names]);
  }

  #[test]
  fn reports_missing_headers_and_syntax_errors() {
    let file = classify("src/reader.rs", &[String::new()]);
    let found: Vec<Rule> = check_source(&file, "fn broken( {\n")
      .into_iter()
      .map(|finding| finding.rule)
      .collect();
    assert_eq!(found, vec![Rule::Header, Rule::Syntax]);
  }
}
