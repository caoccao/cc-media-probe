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

//! The scope of one read of the form, as the thirteenth review asked. Serde's derive passes nothing from a record to
//! the records that it holds, so the read keeps two facts beside the call stack, on its own thread, for as long as it
//! lasts: that a read of the form is under way, whose deserializer is already strict, and the version of the root
//! record whose body is being decoded. A nested root record compares its version with that one before it decodes its
//! own body, whatever the order of its parent's members. A read is one call on one thread, and a guard restores the
//! scope before it when it ends, also on an error.

use std::cell::Cell;

use crate::document::SchemaVersion;

/// What holds the value being read.
#[derive(Clone, Copy)]
pub(super) enum Enclosing {
  /// The body of a root record of that version.
  Record(SchemaVersion),
  /// A value read alone, outside any root record.
  Value,
}

thread_local! {
  /// The scope of the read under way on this thread, if any.
  static SCOPE: Cell<Option<Enclosing>> = const { Cell::new(None) };
}

/// The scope of the read under way, or `None` outside a read of the form.
pub(super) fn current() -> Option<Enclosing> {
  SCOPE.with(Cell::get)
}

/// Enters a scope; the guard restores the scope before it when it is dropped.
pub(super) fn enter(enclosing: Enclosing) -> ScopeGuard {
  ScopeGuard {
    previous: SCOPE.with(|scope| scope.replace(Some(enclosing))),
  }
}

/// Restores the scope that was current when it was entered.
pub(super) struct ScopeGuard {
  previous: Option<Enclosing>,
}

impl Drop for ScopeGuard {
  fn drop(&mut self) {
    SCOPE.with(|scope| scope.set(self.previous));
  }
}
