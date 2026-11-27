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

//! `Walk`, the state of one probe besides its sources: the budget, the streams with their assemblers, the titles
//! with the record of their elements, the diagnostics, the companions with their requests and the source table. Its
//! methods are plain code, so the three walkers share them, and only the reads differ between them.

use std::mem;

use cc_media_probe::Limit;

use super::catalog::Catalog;
use super::constants::ELEMENT_WORK;
pub(crate) use super::constants::PRIMARY_SOURCE as PRIMARY;
use super::cursor::OpenList;
use super::{Cursor, Header};
use crate::assembler::{Assembler, Piece};
use crate::budget::{Budget, Reclaim};
use crate::codec::{Framing, Representation};
use crate::companion::companion_name;
use crate::options::Options;
use crate::report::{
  DetectionOutcome, Diagnostic, DiagnosticKind, HeaderStatus, Reason, SourceEntry, SourceOutcome, StreamReport, Title,
  ToyReport,
};
use crate::source::{Filled, Holdings};
use crate::toy::Element;
use crate::toy::constants::{CODE_LENGTH, ELEMENT_HEADER_LENGTH, MAXIMUM_TEXT_LENGTH};
use crate::utilities::widen;

/// The state of one probe besides its sources.
pub(crate) struct Walk<'options> {
  /// The options of the probe.
  pub(crate) options: &'options Options,
  /// The budget that every source and assembler charges.
  pub(crate) budget: Budget,
  /// The scratch buffer of one piece of a packet.
  pub(crate) piece: Vec<u8>,
  /// The scratch buffer of a short body.
  pub(crate) body: Vec<u8>,
  /// What detection concluded.
  pub(crate) detection: DetectionOutcome,
  lookup_enabled: bool,
  pub(super) streams: Vec<StreamState>,
  /// The stack of the lists that the cursor is inside, innermost last.
  pub(super) lists: Vec<OpenList>,
  titles: Vec<Title>,
  /// The title elements walked, by source and position, which a title reference must name (plan section 7.6).
  pub(super) title_elements: Vec<(u64, u64)>,
  diagnostics: Vec<Diagnostic>,
  pub(super) catalog: Catalog,
  requests_made: u64,
  sources_opened: u64,
  entries: Vec<SourceEntry>,
  stopped: Option<Reason>,
}

/// One declared stream.
pub(super) struct StreamState {
  pub(super) source: u64,
  pub(super) number: u8,
  pub(super) framing: Framing,
  pub(super) representation: Representation,
  pub(super) codec: [u8; CODE_LENGTH],
  pub(super) assembler: Option<Assembler>,
}

/// A companion that the primary input names, listed once per identifier when its element is found, and requested
/// after the primary input's walk (plan section 7.7).
#[derive(Clone, Debug)]
pub(crate) struct Pending {
  /// The offset of the element that names it.
  pub(crate) offset: u64,
  /// The validated identifier.
  pub(crate) identifier: String,
  /// The name built from it, with the fixed extension.
  pub(crate) name: String,
}

/// A request that the limits allowed, with the source identifier that it got just before the resolver's call.
#[derive(Clone, Debug)]
pub(crate) struct Request {
  /// The identifier assigned before anything is opened.
  pub(crate) source_identifier: u64,
  /// The validated identifier.
  pub(crate) identifier: String,
  /// The name built from it, with the fixed extension.
  pub(crate) name: String,
}

impl<'options> Walk<'options> {
  /// The state of a probe with nothing found yet.
  pub(crate) fn new(options: &'options Options, lookup_enabled: bool) -> Walk<'options> {
    Walk {
      options,
      budget: Budget::new(&options.limits),
      piece: Vec::new(),
      body: Vec::new(),
      detection: DetectionOutcome::Unknown,
      lookup_enabled,
      streams: Vec::new(),
      lists: Vec::new(),
      titles: Vec::new(),
      title_elements: Vec::new(),
      diagnostics: Vec::new(),
      catalog: Catalog::default(),
      requests_made: 0,
      sources_opened: 0,
      entries: Vec::new(),
      stopped: None,
    }
  }

  /// Reserves the scratch buffers of a piece and of a short body, each charged before it is reserved. They are
  /// required, so each may take back the optional blocks of `room`, the source's cache, before it names the limit.
  pub(crate) fn prepare(&mut self, room: &mut dyn Reclaim) -> Result<(), Limit> {
    if self.piece.is_empty() {
      let size = self.options.piece.get();
      let mut piece = self
        .budget
        .require(Some(&mut *room), |budget| budget.try_allocate(size))?;
      piece.resize(size, 0);
      self.piece = piece;
      let mut body = self
        .budget
        .require(Some(room), |budget| budget.try_allocate(MAXIMUM_TEXT_LENGTH))?;
      body.resize(MAXIMUM_TEXT_LENGTH, 0);
      self.body = body;
    }
    Ok(())
  }

  /// A cursor at `start`, the first element of a source, inside no list.
  pub(crate) fn begin(&mut self, start: u64) -> Cursor {
    self.lists.clear();
    Cursor::new(start)
  }

  /// The position of the next element header, after leaving every list that ends there and every list whose rest
  /// is too short for a header, which overruns its list: no header read reaches past its parent.
  pub(crate) fn next_header(&mut self, cursor: &mut Cursor, source: u64) -> u64 {
    loop {
      let position = cursor.position();
      while self.lists.last().is_some_and(|list| position >= list.end) {
        self.lists.pop();
      }
      match self.lists.last() {
        Some(list) if list.end.saturating_sub(position) < widen(ELEMENT_HEADER_LENGTH) => {
          self.diagnose(source, position, DiagnosticKind::ElementOverrunsParent);
          self.leave(cursor);
        }
        _ => return position,
      }
    }
  }

  /// Leaves the innermost list, whose rest is skipped.
  pub(super) fn leave(&mut self, cursor: &mut Cursor) {
    if let Some(list) = self.lists.pop() {
      cursor.leave(list);
    }
  }

  /// Records the first reason that stopped the traversal.
  pub(crate) fn stop(&mut self, reason: Reason) {
    if self.stopped.is_none() {
      self.stopped = Some(reason);
    }
  }

  /// Records a diagnostic within the diagnostics limit; beyond it, the diagnostic is dropped.
  pub(crate) fn diagnose(&mut self, source: u64, offset: u64, kind: DiagnosticKind) {
    if widen(self.diagnostics.len()) < self.budget.limits().diagnostics() {
      self.diagnostics.push(Diagnostic { source, offset, kind });
    }
  }

  /// What a read of an element's header at `position` gives: the element, the end of the traversal at a truncated
  /// element or a limit, or the end of the input there. An element costs a work unit once its header is read, so
  /// an allowance of exactly the elements present completes the traversal. A read that gives nothing ends the input
  /// cleanly only where `holdings` delivered the byte before `position`; otherwise the walker reads that byte to
  /// tell a clean end from a jump past it.
  pub(crate) fn element(
    &mut self,
    source: u64,
    position: u64,
    filled: &Filled,
    header: [u8; ELEMENT_HEADER_LENGTH],
    holdings: &dyn Holdings,
  ) -> Header {
    match filled {
      Filled::Complete => {
        if let Err(limit) = self.budget.charge_work(ELEMENT_WORK) {
          self.stop(Reason::Limit(limit));
          return Header::Ended;
        }
        let element = Element::parse(position, header);
        if element.is_none() {
          self.diagnose(source, position, DiagnosticKind::ElementOverrunsParent);
        }
        element.map_or(Header::Ended, Header::Element)
      }
      Filled::End(0) if holdings.delivered_before(position) => {
        self.input_ended(source);
        Header::Ended
      }
      Filled::End(0) => Header::Confirm(position.saturating_sub(1)),
      _ => {
        self.interrupted(source, position, filled);
        Header::Ended
      }
    }
  }

  /// Takes in the read of the byte before the position where the input gave nothing. Without it, the walker jumped
  /// past the end, so the element or list that it jumped over is truncated; a limit that kept the read from being
  /// made stops the traversal with it named, since the end was not established (plan section 7.5).
  pub(crate) fn confirm_end(&mut self, cursor: &Cursor, source: u64, filled: &Filled) {
    if *filled != Filled::Complete {
      self.interrupted(source, cursor.last_element(), filled);
    }
    self.input_ended(source);
  }

  /// The end of the input inside an open list cuts that list short.
  fn input_ended(&mut self, source: u64) {
    if let Some(list) = self.lists.last().map(|list| list.position) {
      self.diagnose(source, list, DiagnosticKind::TruncatedElement);
    }
  }

  /// Handles a read of an element's part that was not complete, and says whether the traversal goes on: past a
  /// range behind the window, as with a missing element (plan section 7.6), but not past the input's end or a
  /// limit.
  pub(crate) fn interrupted(&mut self, source: u64, position: u64, filled: &Filled) -> bool {
    match filled {
      Filled::Complete => true,
      Filled::End(_) => {
        self.diagnose(source, position, DiagnosticKind::TruncatedElement);
        false
      }
      Filled::Unavailable => {
        self.diagnose(source, position, DiagnosticKind::RangeUnavailable);
        true
      }
      Filled::Exhausted(_, limit) => {
        self.stop(Reason::Limit(limit.clone()));
        false
      }
    }
  }

  /// Whether the stream waits for its header: `None` for a stream that no declaration names.
  pub(crate) fn waiting(&self, source: u64, number: u8) -> Option<bool> {
    self
      .streams
      .iter()
      .find(|stream| stream.source == source && stream.number == number)
      .map(|stream| stream.assembler.as_ref().is_some_and(Assembler::is_waiting))
  }

  /// Offers a piece to the stream's assembler, and says whether more of the same packet is wanted.
  pub(crate) fn offer(&mut self, source: u64, number: u8, piece: &Piece<'_>, room: &mut dyn Reclaim) -> bool {
    let budget = &mut self.budget;
    self
      .streams
      .iter_mut()
      .find(|stream| stream.source == source && stream.number == number)
      .and_then(|stream| stream.assembler.as_mut())
      .is_some_and(|assembler| assembler.offer(budget, room, piece))
  }

  /// Ends every stream of `source` that still waits, for the reason that stopped the traversal, or else because the
  /// input ended.
  pub(crate) fn finish_streams(&mut self, source: u64) {
    let reason = self.stopped.clone().unwrap_or(Reason::TruncatedInput);
    let budget = &mut self.budget;
    for stream in self.streams.iter_mut().filter(|stream| stream.source == source) {
      if let Some(assembler) = stream.assembler.as_mut() {
        assembler.finish(budget, reason.clone());
      }
    }
  }

  /// Ends every stream of `source` that still waits, because the source failed: each keeps the fields read.
  pub(crate) fn fail_streams(&mut self, source: u64) {
    let budget = &mut self.budget;
    for stream in self.streams.iter_mut().filter(|stream| stream.source == source) {
      if let Some(assembler) = stream.assembler.as_mut() {
        assembler.fail(budget);
      }
    }
  }

  /// Ends every stream of `source` that still waits once its walk is over: failed when the walk ended with the
  /// source's error, and finished otherwise.
  pub(crate) fn end_streams<Value>(&mut self, source: u64, walked: &std::io::Result<Value>) {
    if walked.is_err() {
      self.fail_streams(source);
    } else {
      self.finish_streams(source);
    }
  }

  /// Keeps a title.
  pub(crate) fn title(&mut self, source: u64, text: &[u8], referenced: bool) {
    self.titles.push(Title {
      source,
      text: String::from_utf8_lossy(text).into_owned(),
      referenced,
    });
  }

  /// The count of companions that wait for their request.
  pub(crate) fn queued(&self) -> usize {
    self.catalog.queued()
  }

  /// The companion that waits at `index` of the queue, in the order named.
  pub(crate) fn pending(&self, index: usize) -> Option<Pending> {
    let (offset, identifier) = self.catalog.request(index)?;
    Some(Pending {
      offset,
      identifier: identifier.to_owned(),
      name: companion_name(identifier.as_bytes())?,
    })
  }

  /// The request for `pending`, with its source identifier, when the companion-request limit and then the
  /// sources-opened limit allow one more, checked just before the resolver would be called (plan section 7.7).
  /// Otherwise the companion is limited: no source identifier, no entry and no call.
  pub(crate) fn admit(&mut self, pending: Pending) -> Option<Request> {
    let limits = self.budget.limits();
    let refused = if self.requests_made >= limits.companion_requests() {
      Some(Limit::CompanionRequests)
    } else if self.sources_opened >= limits.sources_opened() {
      Some(Limit::SourcesOpened)
    } else {
      None
    };
    if let Some(limit) = refused {
      self.diagnose(PRIMARY, pending.offset, DiagnosticKind::CompanionLimited(limit));
      return None;
    }
    self.requests_made = self.requests_made.saturating_add(1);
    Some(Request {
      source_identifier: self.requests_made,
      identifier: pending.identifier,
      name: pending.name,
    })
  }

  /// Counts a source that the resolver opened, whether it is read or closed again as an alias.
  pub(crate) fn count_opened(&mut self) {
    self.sources_opened = self.sources_opened.saturating_add(1);
  }

  /// Adds the source-table entry of a request.
  pub(crate) fn record(&mut self, request: &Request, outcome: SourceOutcome, bytes_read: u64, alias_of: Option<u64>) {
    self.entries.push(SourceEntry {
      identifier: request.source_identifier,
      name: Some(request.name.clone()),
      outcome,
      bytes_read,
      alias_of,
    });
  }

  /// Whether lookup is on.
  pub(super) fn lookup_enabled(&self) -> bool {
    self.lookup_enabled
  }

  /// The report, once every source has been released, with the primary input's `outcome`: opened, or failed when
  /// the probe ends with its error.
  pub(crate) fn into_report(mut self, primary_delivered: u64, outcome: SourceOutcome) -> ToyReport {
    self.budget.free(mem::take(&mut self.piece));
    self.budget.free(mem::take(&mut self.body));
    self.budget.free(mem::take(&mut self.title_elements));
    self.budget.free(mem::take(&mut self.lists));
    self.catalog.release(&mut self.budget);
    let mut sources = vec![SourceEntry {
      identifier: PRIMARY,
      name: None,
      outcome,
      bytes_read: primary_delivered,
      alias_of: None,
    }];
    sources.append(&mut self.entries);
    let streams = self
      .streams
      .into_iter()
      .map(|stream| StreamReport {
        source: stream.source,
        number: stream.number,
        framing: stream.framing,
        representation: stream.representation,
        codec: stream.codec,
        header: stream
          .assembler
          .as_ref()
          .map_or(HeaderStatus::Unsupported, Assembler::status),
        packets_offered: stream.assembler.as_ref().map_or(0, Assembler::packets_offered),
      })
      .collect();
    ToyReport {
      detection: self.detection,
      streams,
      titles: self.titles,
      diagnostics: self.diagnostics,
      expected_companions: self.catalog.identifiers,
      sources,
      exhausted: self.budget.exhausted(),
      usage: self.budget.usage(),
    }
  }
}
