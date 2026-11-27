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

//! The same toy file through every input gives the same report (plan Phase 3, task 1): bytes, a seekable reader,
//! a forward-only reader, a custom positional source, a file, a custom async source with and without pending
//! reads, and the seekable and forward-only readers of Tokio and of futures-io, with both features enabled. The
//! hand-written blocking probe gives the same report, usage included.

use std::io::Cursor;

use futures_lite::future::block_on;
use input::asynchronous::{self, futures, tokio as tokio_entry};
use input::blocking;
use input::codec::Framing;
use input::handwritten;
use input::report::{DetectionOutcome, HeaderStatus, ToyReport};

use crate::support::{PendingSource, RecordingSource, TemporaryFolder, parsed, semantic, small_options, typical_file};

/// The report of the typical file from bytes, which every other input must match.
fn expected() -> ToyReport {
  blocking::probe_bytes(&typical_file(), &small_options()).unwrap()
}

#[test]
fn the_typical_file_gives_its_streams_titles_and_no_diagnostic() {
  let report = expected();
  assert_eq!(report.detection, DetectionOutcome::Identified);
  let headers: Vec<_> = report.streams.iter().map(|stream| stream.header.clone()).collect();
  assert_eq!(
    headers,
    [
      parsed(1920, 1080, &[7, 8]),
      parsed(640, 480, &[1, 2, 3]),
      HeaderStatus::Unsupported
    ]
  );
  assert_eq!(report.streams[1].framing, Framing::Stream);
  let titles: Vec<_> = report
    .titles
    .iter()
    .map(|title| (title.text.as_str(), title.referenced))
    .collect();
  assert_eq!(titles, [("Concert", false), ("Encore", false), ("Concert", true)]);
  assert_eq!(report.diagnostics, []);
  assert_eq!(report.exhausted, [] as [cc_media_probe::Limit; 0]);
  assert_eq!(report.usage.working_memory_at_end, 0);
}

#[test]
fn every_blocking_input_gives_the_same_report() {
  let options = small_options();
  let expected = semantic(&expected());
  let file = typical_file();
  let reader = blocking::probe_reader(Cursor::new(file.clone()), &options).unwrap();
  assert_eq!(semantic(&reader), expected);
  let stream = blocking::probe_stream(Cursor::new(file.clone()), &options).unwrap();
  assert_eq!(semantic(&stream), expected);
  let mut source = RecordingSource::new(&file);
  let positional = blocking::probe_source(&mut source, &options).unwrap();
  assert_eq!(semantic(&positional), expected);
  let folder = TemporaryFolder::new().unwrap();
  let path = folder.path().join("typical.toy");
  std::fs::write(&path, &file).unwrap();
  let from_file = blocking::probe_file(&path, &options).unwrap();
  assert_eq!(semantic(&from_file), expected);
}

#[test]
fn a_forward_only_reader_reads_every_byte_once_and_a_seekable_one_reads_whole_blocks() {
  let options = small_options();
  let file = typical_file();
  let length = u64::try_from(file.len()).unwrap();
  let stream = blocking::probe_stream(Cursor::new(file.clone()), &options).unwrap();
  assert_eq!(stream.usage.bytes_read, length);
  assert_eq!(stream.sources[0].bytes_read, length);
  let reader = blocking::probe_reader(Cursor::new(file), &options).unwrap();
  assert_eq!(reader.usage.bytes_read, reader.sources[0].bytes_read);
  assert!(reader.usage.bytes_read >= length);
}

#[test]
fn every_async_input_gives_the_same_report() {
  let options = small_options();
  let expected = semantic(&expected());
  let file = typical_file();
  let mut ready = RecordingSourceAsync::new(&file);
  let custom = block_on(asynchronous::probe_source(&mut ready, &options)).unwrap();
  assert_eq!(semantic(&custom), expected);
  let mut pending = PendingSource::new(&file);
  let hesitant = block_on(asynchronous::probe_source(&mut pending, &options)).unwrap();
  assert_eq!(semantic(&hesitant), expected);
  let seekable = block_on(futures::probe_reader(
    futures_lite::io::Cursor::new(file.clone()),
    &options,
  ))
  .unwrap();
  assert_eq!(semantic(&seekable), expected);
  let forward = block_on(futures::probe_stream(
    futures_lite::io::Cursor::new(file.clone()),
    &options,
  ))
  .unwrap();
  assert_eq!(semantic(&forward), expected);
  let runtime = tokio::runtime::Builder::new_current_thread().build().unwrap();
  let seekable = runtime
    .block_on(tokio_entry::probe_reader(Cursor::new(file.clone()), &options))
    .unwrap();
  assert_eq!(semantic(&seekable), expected);
  let forward = runtime
    .block_on(tokio_entry::probe_stream(Cursor::new(file.clone()), &options))
    .unwrap();
  assert_eq!(semantic(&forward), expected);
}

#[test]
fn a_tokio_file_gives_the_same_report_through_both_paths() {
  let options = small_options();
  let expected = semantic(&expected());
  let folder = TemporaryFolder::new().unwrap();
  let path = folder.path().join("typical.toy");
  std::fs::write(&path, typical_file()).unwrap();
  let runtime = tokio::runtime::Builder::new_current_thread().build().unwrap();
  runtime.block_on(async {
    let file = tokio::fs::File::open(&path).await.unwrap();
    let seekable = tokio_entry::probe_reader(file, &options).await.unwrap();
    assert_eq!(semantic(&seekable), expected);
    let file = tokio::fs::File::open(&path).await.unwrap();
    let forward = tokio_entry::probe_stream(file, &options).await.unwrap();
    assert_eq!(semantic(&forward), expected);
  });
}

#[test]
fn the_hand_written_probe_gives_the_same_report_and_usage() {
  let options = small_options();
  let file = typical_file();
  assert_eq!(handwritten::probe_bytes(&file, &options).unwrap(), expected());
  let async_reader = blocking::probe_reader(Cursor::new(file.clone()), &options).unwrap();
  let hand_reader = handwritten::probe_reader(Cursor::new(file), &options).unwrap();
  assert_eq!(hand_reader, async_reader);
}

#[test]
fn the_default_options_give_the_same_report_on_every_blocking_input() {
  let file = typical_file();
  let options = input::Options::default();
  let expected = semantic(&blocking::probe_bytes(&file, &options).unwrap());
  let reader = blocking::probe_reader(Cursor::new(file.clone()), &options).unwrap();
  assert_eq!(semantic(&reader), expected);
  let stream = blocking::probe_stream(Cursor::new(file), &options).unwrap();
  assert_eq!(semantic(&stream), expected);
  assert_eq!(expected.titles.len(), 3);
}

/// An async positional source that is always ready, over bytes.
struct RecordingSourceAsync {
  data: Vec<u8>,
}

impl RecordingSourceAsync {
  fn new(data: &[u8]) -> RecordingSourceAsync {
    RecordingSourceAsync { data: data.to_vec() }
  }
}

impl cc_media_probe::asynchronous::AsyncPositionalSource for RecordingSourceAsync {
  fn poll_read_at(
    self: std::pin::Pin<&mut Self>,
    _context: &mut std::task::Context<'_>,
    position: u64,
    buffer: &mut [u8],
  ) -> std::task::Poll<std::io::Result<usize>> {
    std::task::Poll::Ready(Ok(crate::support::copy_from(&self.data, position, buffer)))
  }

  fn poll_length(
    self: std::pin::Pin<&mut Self>,
    _context: &mut std::task::Context<'_>,
  ) -> std::task::Poll<std::io::Result<Option<u64>>> {
    std::task::Poll::Ready(Ok(None))
  }
}
