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

//! A two-file toy input through a resolver, blocking and async (plan Phase 3, task 1), and the built-in resolver's
//! check-then-open sequence against links inside and outside the root. Run on macOS only, as the maintainer chose
//! on 2026-11-27; the permanent tests of Phase 14 run on all three platforms.

use std::future::Future;
use std::io;
use std::path::Path;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

use cc_media_probe::source::PositionalSource;
use cc_media_probe::{Limit, Limits};
use futures_lite::future::block_on;
use input::asynchronous;
use input::blocking;
use input::builder::{ToyFile, setup_unit};
use input::codec::Framing;
use input::companion::{
  Answer, AsyncCompanionResolver, AsyncResolution, CompanionLookup, CompanionRequest, CompanionResolver, Resolution,
};
use input::report::{DiagnosticKind, ProbeError, SourceOutcome, ToyReport};

use crate::support::{PendingSource, TemporaryFolder, copy_from, parsed, size, small_options};

/// A primary file that names the companions `identifiers` and has one stream of its own.
fn primary(identifiers: &[&str]) -> Vec<u8> {
  identifiers
    .iter()
    .fold(ToyFile::new(), |file, identifier| file.companion(identifier))
    .stream(1, Framing::Packet)
    .packet(1, &setup_unit(320, 240, 0, 0))
    .build()
}

/// A companion file with one stream.
fn companion() -> Vec<u8> {
  ToyFile::new()
    .stream(1, Framing::Packet)
    .packet(1, &setup_unit(800, 600, 0, 0))
    .build()
}

/// The options with the built-in resolver.
fn built_in() -> input::Options {
  input::Options {
    companions: CompanionLookup::BuiltIn,
    ..small_options()
  }
}

/// The outcomes of the source table after the primary input.
fn outcomes(report: &ToyReport) -> Vec<(Option<String>, SourceOutcome, Option<u64>)> {
  report.sources[1..]
    .iter()
    .map(|entry| (entry.name.clone(), entry.outcome.clone(), entry.alias_of))
    .collect()
}

/// Writes the primary file `movie.toy` naming `identifiers` into `folder`, and gives its path.
fn write_primary(folder: &Path, identifiers: &[&str]) -> std::path::PathBuf {
  let path = folder.join("movie.toy");
  std::fs::write(&path, primary(identifiers)).unwrap();
  path
}

#[test]
fn the_built_in_resolver_opens_a_companion_beside_the_probed_file() {
  let folder = TemporaryFolder::new().unwrap();
  let path = write_primary(folder.path(), &["movie"]);
  std::fs::write(folder.path().join("movie.toyc"), companion()).unwrap();
  let report = blocking::probe_file(&path, &built_in()).unwrap();
  assert_eq!(
    outcomes(&report),
    [(Some("movie.toyc".to_owned()), SourceOutcome::Opened, None)]
  );
  let streams: Vec<_> = report
    .streams
    .iter()
    .map(|stream| (stream.source, stream.header.clone()))
    .collect();
  assert_eq!(streams, [(0, parsed(320, 240, &[])), (1, parsed(800, 600, &[]))]);
  assert_eq!(
    report.sources[1].bytes_read,
    report.usage.bytes_read - report.sources[0].bytes_read
  );
  assert_eq!(report.usage.working_memory_at_end, 0);
}

#[cfg(unix)]
#[test]
fn a_link_inside_the_root_is_followed_and_one_that_leads_outside_is_declined() {
  let folder = TemporaryFolder::new().unwrap();
  let outside = TemporaryFolder::new().unwrap();
  let path = write_primary(folder.path(), &["inside", "outside"]);
  std::fs::create_dir(folder.path().join("layout")).unwrap();
  std::fs::write(folder.path().join("layout").join("real.toyc"), companion()).unwrap();
  std::os::unix::fs::symlink(
    folder.path().join("layout").join("real.toyc"),
    folder.path().join("inside.toyc"),
  )
  .unwrap();
  std::fs::write(outside.path().join("other.toyc"), companion()).unwrap();
  std::os::unix::fs::symlink(outside.path().join("other.toyc"), folder.path().join("outside.toyc")).unwrap();
  let report = blocking::probe_file(&path, &built_in()).unwrap();
  let declined = SourceOutcome::Declined("the companion resolves outside the folder of the probed file".to_owned());
  assert_eq!(
    outcomes(&report),
    [
      (Some("inside.toyc".to_owned()), SourceOutcome::Opened, None),
      (Some("outside.toyc".to_owned()), declined, None),
    ]
  );
  assert_eq!(report.streams.len(), 2);
}

#[test]
fn a_directory_a_missing_companion_and_an_input_without_a_folder_are_told_apart() {
  let folder = TemporaryFolder::new().unwrap();
  let path = write_primary(folder.path(), &["folder", "missing"]);
  std::fs::create_dir(folder.path().join("folder.toyc")).unwrap();
  let report = blocking::probe_file(&path, &built_in()).unwrap();
  assert_eq!(
    outcomes(&report),
    [
      (
        Some("folder.toyc".to_owned()),
        SourceOutcome::Declined("the companion is not a regular file".to_owned()),
        None
      ),
      (Some("missing.toyc".to_owned()), SourceOutcome::NotFound, None),
    ]
  );
  let from_bytes = blocking::probe_bytes(&primary(&["movie"]), &built_in()).unwrap();
  assert_eq!(
    outcomes(&from_bytes),
    [(
      Some("movie.toyc".to_owned()),
      SourceOutcome::Declined("the input has no folder to look in".to_owned()),
      None
    )]
  );
}

#[cfg(unix)]
#[test]
fn one_identifier_twice_makes_one_request_and_two_names_of_one_file_give_an_alias() {
  let folder = TemporaryFolder::new().unwrap();
  let path = write_primary(folder.path(), &["alpha", "alpha", "beta"]);
  std::fs::write(folder.path().join("alpha.toyc"), companion()).unwrap();
  std::os::unix::fs::symlink(folder.path().join("alpha.toyc"), folder.path().join("beta.toyc")).unwrap();
  let report = blocking::probe_file(&path, &built_in()).unwrap();
  assert_eq!(
    outcomes(&report),
    [
      (Some("alpha.toyc".to_owned()), SourceOutcome::Opened, None),
      (Some("beta.toyc".to_owned()), SourceOutcome::Opened, Some(1)),
    ]
  );
  assert_eq!(report.expected_companions, ["alpha", "beta"]);
  assert_eq!(report.streams.len(), 2);
}

#[cfg(unix)]
#[test]
fn a_companion_that_is_a_hard_link_to_the_probed_file_is_an_alias_of_source_zero() {
  let folder = TemporaryFolder::new().unwrap();
  let path = write_primary(folder.path(), &["movie"]);
  std::fs::hard_link(&path, folder.path().join("movie.toyc")).unwrap();
  let report = blocking::probe_file(&path, &built_in()).unwrap();
  assert_eq!(
    outcomes(&report),
    [(Some("movie.toyc".to_owned()), SourceOutcome::Opened, Some(0))]
  );
  assert_eq!(report.streams.len(), 1);
}

#[test]
fn lookup_off_makes_no_request_and_an_invalid_identifier_makes_none_either() {
  let off = blocking::probe_bytes(&primary(&["movie"]), &small_options()).unwrap();
  assert_eq!(off.sources.len(), 1);
  assert_eq!(off.expected_companions, ["movie"]);
  let invalid = blocking::probe_bytes(&primary(&["Movie: 2"]), &built_in()).unwrap();
  assert_eq!(invalid.sources.len(), 1);
  assert_eq!(invalid.expected_companions, [] as [String; 0]);
  assert_eq!(invalid.diagnostics[0].kind, DiagnosticKind::InvalidIdentifier);
}

/// A resolver over named files in memory, which records the names that it was asked for. Each found source is
/// a blocking one for the blocking trait and a pending one for the async trait.
struct MemoryResolver {
  files: Vec<(String, Vec<u8>)>,
  failing: bool,
  calls: Mutex<Vec<String>>,
}

impl MemoryResolver {
  fn new(files: &[(&str, Vec<u8>)]) -> MemoryResolver {
    MemoryResolver {
      files: files
        .iter()
        .map(|(name, data)| ((*name).to_owned(), data.clone()))
        .collect(),
      failing: false,
      calls: Mutex::new(Vec::new()),
    }
  }

  fn find(&self, request: &CompanionRequest) -> Option<Vec<u8>> {
    self.calls.lock().unwrap().push(request.name.clone());
    self
      .files
      .iter()
      .find(|(name, _)| *name == request.name)
      .map(|(_, data)| data.clone())
  }
}

impl CompanionResolver for MemoryResolver {
  fn resolve(&self, request: &CompanionRequest) -> Resolution {
    match self.find(request) {
      Some(data) if self.failing => Answer::Found {
        source: Box::new(FailingSource(data)),
        identity: None,
      },
      Some(data) => Answer::Found {
        source: Box::new(MemorySource(data)),
        identity: None,
      },
      None => Answer::NotFound,
    }
  }
}

impl AsyncCompanionResolver for MemoryResolver {
  fn resolve<'request>(
    &'request self,
    request: &'request CompanionRequest,
  ) -> Pin<Box<dyn Future<Output = AsyncResolution> + Send + 'request>> {
    Box::pin(async move {
      let answer: AsyncResolution = match self.find(request) {
        Some(data) => Answer::Found {
          source: Box::pin(PendingSource::new(&data)),
          identity: None,
        },
        None => Answer::NotFound,
      };
      answer
    })
  }
}

/// A blocking source over bytes.
struct MemorySource(Vec<u8>);

impl PositionalSource for MemorySource {
  fn read_at(&mut self, position: u64, buffer: &mut [u8]) -> io::Result<usize> {
    Ok(copy_from(&self.0, position, buffer))
  }

  fn length(&mut self) -> io::Result<Option<u64>> {
    Ok(u64::try_from(self.0.len()).ok())
  }
}

/// A blocking source that fails on its first read.
struct FailingSource(Vec<u8>);

impl PositionalSource for FailingSource {
  fn read_at(&mut self, _position: u64, _buffer: &mut [u8]) -> io::Result<usize> {
    Err(io::Error::other("the companion's disk failed"))
  }

  fn length(&mut self) -> io::Result<Option<u64>> {
    Ok(u64::try_from(self.0.len()).ok())
  }
}

#[test]
fn a_custom_resolver_serves_bytes_and_an_async_one_serves_an_async_source() {
  let resolver = Arc::new(MemoryResolver::new(&[("movie.toyc", companion())]));
  let options = input::Options {
    companions: CompanionLookup::Custom(resolver.clone()),
    async_companions: Some(resolver.clone()),
    ..small_options()
  };
  let blocking_report = blocking::probe_bytes(&primary(&["movie", "absent"]), &options).unwrap();
  let async_source = PendingSource::new(&primary(&["movie", "absent"]));
  let async_report = block_on(asynchronous::probe_source(async_source, &options)).unwrap();
  for report in [&blocking_report, &async_report] {
    assert_eq!(
      outcomes(report),
      [
        (Some("movie.toyc".to_owned()), SourceOutcome::Opened, None),
        (Some("absent.toyc".to_owned()), SourceOutcome::NotFound, None),
      ]
    );
    assert_eq!(report.streams[1].header, parsed(800, 600, &[]));
  }
  assert_eq!(resolver.calls.lock().unwrap().len(), 4);
}

#[test]
fn the_request_limit_refuses_a_further_companion_before_any_request() {
  let resolver = Arc::new(MemoryResolver::new(&[]));
  let options = input::Options {
    limits: Limits::default().with_companion_requests(1),
    companions: CompanionLookup::Custom(resolver.clone()),
    ..small_options()
  };
  let report = blocking::probe_bytes(&primary(&["first", "second"]), &options).unwrap();
  assert_eq!(
    outcomes(&report),
    [(Some("first.toyc".to_owned()), SourceOutcome::NotFound, None)]
  );
  assert_eq!(
    report.diagnostics[0].kind,
    DiagnosticKind::CompanionLimited(Limit::CompanionRequests)
  );
  assert_eq!(*resolver.calls.lock().unwrap(), ["first.toyc"]);
}

#[test]
fn a_failing_companion_ends_the_probe_with_its_identifier_and_the_partial_report() {
  let mut resolver = MemoryResolver::new(&[("movie.toyc", companion())]);
  resolver.failing = true;
  let options = input::Options {
    companions: CompanionLookup::Custom(Arc::new(resolver)),
    ..small_options()
  };
  let Err(ProbeError::SourceFailed { source, cause, partial }) = blocking::probe_bytes(&primary(&["movie"]), &options)
  else {
    panic!("the probe did not fail");
  };
  assert_eq!(source, 1);
  assert_eq!(cause.to_string(), "the companion's disk failed");
  assert_eq!(
    outcomes(&partial),
    [(Some("movie.toyc".to_owned()), SourceOutcome::Failed, None)]
  );
  assert_eq!(partial.streams[0].header, parsed(320, 240, &[]));
}

#[test]
fn the_allowance_is_shared_by_the_primary_input_and_its_companion() {
  let resolver = Arc::new(MemoryResolver::new(&[("movie.toyc", companion())]));
  let file = primary(&["movie"]);
  let limit = u64::try_from(file.len()).unwrap() + 10;
  let options = input::Options {
    limits: Limits::default().with_bytes_read(limit),
    companions: CompanionLookup::Custom(resolver),
    block_size: size(16),
    ..small_options()
  };
  let report = blocking::probe_bytes(&file, &options).unwrap();
  assert_eq!(report.usage.bytes_read, limit);
  assert!(report.sources[1].bytes_read > 0);
  assert_eq!(report.sources[0].bytes_read + report.sources[1].bytes_read, limit);
  assert!(report.exhausted.contains(&Limit::BytesRead));
}
