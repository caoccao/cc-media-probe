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

//! Companion lookup's limits and identities (plan section 7.7; `api.md` section 13): the two allowances checked
//! before each call, so a missing companion leaves the open allowance to the next; a custom resolver's identity
//! tokens compared with earlier companions only, held under working memory and returned at the end; and the
//! built-in resolver declining a named pipe before it could wait in the open. Run on macOS only, as the maintainer
//! chose on 2026-11-27.

use std::io;
#[cfg(unix)]
use std::os::unix::fs::MetadataExt;
#[cfg(unix)]
use std::process::Command;
#[cfg(unix)]
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
#[cfg(unix)]
use std::thread;
#[cfg(unix)]
use std::time::Duration;

use cc_media_probe::source::PositionalSource;
use cc_media_probe::{Limit, Limits};
use input::blocking;
use input::builder::{ToyFile, setup_unit};
use input::codec::Framing;
use input::companion::{Answer, BuiltInResolver, CompanionLookup, CompanionRequest, CompanionResolver, Resolution};
use input::report::{DiagnosticKind, SourceOutcome, ToyReport};

use crate::support::{TemporaryFolder, copy_from, small_options};

/// A resolver over named files in memory, each with an identity token or none, which records every call.
struct TokenResolver {
  files: Vec<(String, Vec<u8>, Option<Vec<u8>>)>,
  calls: Mutex<Vec<String>>,
}

impl TokenResolver {
  fn new(files: &[(&str, Option<Vec<u8>>)]) -> TokenResolver {
    TokenResolver {
      files: files
        .iter()
        .map(|(name, token)| ((*name).to_owned(), companion(), token.clone()))
        .collect(),
      calls: Mutex::new(Vec::new()),
    }
  }
}

impl CompanionResolver for TokenResolver {
  fn resolve(&self, request: &CompanionRequest) -> Resolution {
    self.calls.lock().unwrap().push(request.name.clone());
    match self.files.iter().find(|(name, _, _)| *name == request.name) {
      Some((_, data, identity)) => Answer::Found {
        source: Box::new(Bytes(data.clone())),
        identity: identity.clone(),
      },
      None => Answer::NotFound,
    }
  }
}

/// A blocking source over bytes.
struct Bytes(Vec<u8>);

impl PositionalSource for Bytes {
  fn read_at(&mut self, position: u64, buffer: &mut [u8]) -> io::Result<usize> {
    Ok(copy_from(&self.0, position, buffer))
  }

  fn length(&mut self) -> io::Result<Option<u64>> {
    Ok(None)
  }
}

/// A companion file with one stream.
fn companion() -> Vec<u8> {
  ToyFile::new()
    .stream(1, Framing::Packet)
    .packet(1, &setup_unit(800, 600, 0, 0))
    .build()
}

/// A primary file that names the companions `identifiers`.
fn primary(identifiers: &[&str]) -> Vec<u8> {
  identifiers
    .iter()
    .fold(ToyFile::new(), |file, identifier| file.companion(identifier))
    .build()
}

/// The outcomes of the source table after the primary input, with each alias.
fn outcomes(report: &ToyReport) -> Vec<(SourceOutcome, Option<u64>)> {
  report.sources[1..]
    .iter()
    .map(|entry| (entry.outcome.clone(), entry.alias_of))
    .collect()
}

#[test]
fn a_missing_companion_leaves_the_open_allowance_to_the_next() {
  let resolver = Arc::new(TokenResolver::new(&[("movie.toyc", None), ("extra.toyc", None)]));
  let options = input::Options {
    limits: Limits::default().with_sources_opened(1),
    companions: CompanionLookup::Custom(resolver.clone()),
    ..small_options()
  };
  let file = primary(&["absent", "movie", "extra"]);
  let report = blocking::probe_bytes(&file, &options).unwrap();
  assert_eq!(
    outcomes(&report),
    [(SourceOutcome::NotFound, None), (SourceOutcome::Opened, None)]
  );
  assert_eq!(
    report.diagnostics.last().unwrap().kind,
    DiagnosticKind::CompanionLimited(Limit::SourcesOpened)
  );
  assert_eq!(*resolver.calls.lock().unwrap(), ["absent.toyc", "movie.toyc"]);
}

#[cfg(unix)]
#[test]
fn a_custom_token_is_compared_with_earlier_companions_only() {
  let folder = TemporaryFolder::new().unwrap();
  let path = folder.path().join("movie.toy");
  std::fs::write(&path, primary(&["other", "again"])).unwrap();
  let metadata = std::fs::metadata(&path).unwrap();
  let token = [metadata.dev().to_be_bytes(), metadata.ino().to_be_bytes()].concat();
  let resolver = TokenResolver::new(&[("other.toyc", Some(token.clone())), ("again.toyc", Some(token))]);
  let options = input::Options {
    companions: CompanionLookup::Custom(Arc::new(resolver)),
    ..small_options()
  };
  let report = blocking::probe_file(&path, &options).unwrap();
  assert_eq!(
    outcomes(&report),
    [(SourceOutcome::Opened, None), (SourceOutcome::Opened, Some(1))]
  );
  assert!(report.sources[1].bytes_read > 0);
  assert_eq!(report.sources[2].bytes_read, 0);
}

#[test]
fn tokens_are_held_under_working_memory_and_returned_at_the_end() {
  let large = vec![7; 1 << 20];
  let resolver = TokenResolver::new(&[("movie.toyc", Some(vec![1; 4096])), ("large.toyc", Some(large))]);
  let options = input::Options {
    limits: Limits::default().with_working_memory(64 * 1024),
    companions: CompanionLookup::Custom(Arc::new(resolver)),
    ..small_options()
  };
  let report = blocking::probe_bytes(&primary(&["movie", "large"]), &options).unwrap();
  assert_eq!(
    outcomes(&report),
    [
      (SourceOutcome::Opened, None),
      (SourceOutcome::LimitReached(Limit::WorkingMemory), None)
    ]
  );
  assert_eq!(report.sources[2].bytes_read, 0);
  assert!(report.usage.peak_working_memory >= 4096);
  assert_eq!(report.usage.working_memory_at_end, 0);
}

#[cfg(unix)]
#[test]
fn a_named_pipe_is_declined_before_an_open_that_would_wait() {
  let folder = TemporaryFolder::new().unwrap();
  let path = folder.path().join("movie.toy");
  std::fs::write(&path, primary(&["pipe"])).unwrap();
  let status = Command::new("mkfifo")
    .arg(folder.path().join("pipe.toyc"))
    .status()
    .unwrap();
  assert!(status.success());
  let (sender, receiver) = mpsc::channel();
  thread::spawn(move || {
    let options = input::Options {
      companions: CompanionLookup::BuiltIn,
      ..small_options()
    };
    let outcome = blocking::probe_file(&path, &options).map(|report| report.sources[1].outcome.clone());
    let _ = sender.send(outcome.map_err(|error| format!("{error:?}")));
  });
  let outcome = receiver.recv_timeout(Duration::from_secs(10));
  assert!(
    matches!(&outcome, Ok(Ok(SourceOutcome::Declined(reason))) if !reason.is_empty()),
    "the probe waited or failed: {outcome:?}"
  );
}

#[test]
fn built_in_lookup_charges_no_identity_when_no_companion_is_named() {
  let folder = TemporaryFolder::new().unwrap();
  let path = folder.path().join("movie.toy");
  std::fs::write(&path, ToyFile::new().title("x").build()).unwrap();
  let needed = blocking::probe_file(&path, &small_options())
    .unwrap()
    .usage
    .peak_working_memory;
  let options = input::Options {
    limits: Limits::default().with_working_memory(needed),
    companions: CompanionLookup::BuiltIn,
    ..small_options()
  };
  let report = blocking::probe_file(&path, &options).unwrap();
  assert_eq!(report.titles.len(), 1);
  assert_eq!(report.exhausted, []);
  assert_eq!(report.usage.peak_working_memory, needed);
  assert_eq!(report.usage.working_memory_at_end, 0);
}

#[test]
fn a_folder_that_cannot_be_resolved_fails_with_its_cause() {
  let folder = TemporaryFolder::new().unwrap();
  let missing = folder.path().join("gone").join("movie.toy");
  let resolver = BuiltInResolver::new(Some(&missing));
  let request = CompanionRequest {
    source_identifier: 1,
    identifier: "extra".to_owned(),
    name: "extra.toyc".to_owned(),
    primary_path: Some(missing.clone()),
  };
  let Answer::Failed(error) = resolver.resolve(&request) else {
    panic!("the resolver did not fail");
  };
  assert_eq!(error.kind(), io::ErrorKind::NotFound);
  let without = BuiltInResolver::new(None);
  assert!(matches!(without.resolve(&request), Answer::Declined(_)));
}
