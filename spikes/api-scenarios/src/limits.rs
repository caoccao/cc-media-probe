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

//! `Limits` and `Limit`, `api.md` section 4.5, with three of the counters.

use crate::constants::{
  BYTES_READ_NAME, DEFAULT_BYTES_READ, DEFAULT_STREAMS, DEFAULT_WORKING_MEMORY, STREAMS_NAME, WORKING_MEMORY_NAME,
};
use crate::model::UnrecognizedValue;

/// The counters of the shared budget (plan section 7.5).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct Limits {
  bytes_read: u64,
  working_memory: u64,
  streams: u64,
}

impl Default for Limits {
  fn default() -> Limits {
    Limits {
      bytes_read: DEFAULT_BYTES_READ,
      working_memory: DEFAULT_WORKING_MEMORY,
      streams: DEFAULT_STREAMS,
    }
  }
}

impl Limits {
  /// The bytes a probe may read from every source together.
  pub fn bytes_read(&self) -> u64 {
    self.bytes_read
  }

  /// Sets the bytes a probe may read.
  pub fn with_bytes_read(mut self, value: u64) -> Limits {
    self.bytes_read = value;
    self
  }

  /// The working memory a probe may use.
  pub fn working_memory(&self) -> u64 {
    self.working_memory
  }

  /// Sets the working memory a probe may use.
  pub fn with_working_memory(mut self, value: u64) -> Limits {
    self.working_memory = value;
    self
  }

  /// The stream declarations a probe keeps.
  pub fn streams(&self) -> u64 {
    self.streams
  }

  /// Sets the stream declarations a probe keeps.
  pub fn with_streams(mut self, value: u64) -> Limits {
    self.streams = value;
    self
  }

  /// A counter by name; `None` for an unrecognized limit.
  pub fn get(&self, limit: &Limit) -> Option<u64> {
    match limit {
      Limit::BytesRead => Some(self.bytes_read),
      Limit::WorkingMemory => Some(self.working_memory),
      Limit::Streams => Some(self.streams),
      Limit::Unrecognized(_) => None,
    }
  }

  /// Sets a counter by name; an unrecognized limit leaves the limits unchanged.
  pub fn with(self, limit: &Limit, value: u64) -> Limits {
    match limit {
      Limit::BytesRead => self.with_bytes_read(value),
      Limit::WorkingMemory => self.with_working_memory(value),
      Limit::Streams => self.with_streams(value),
      Limit::Unrecognized(_) => self,
    }
  }
}

/// The name of a counter: in the limits, the usage summary, the fixed-size record and the reasons.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum Limit {
  /// Bytes read.
  BytesRead,
  /// Working memory.
  WorkingMemory,
  /// Stream declarations.
  Streams,
  /// A value that a newer version wrote.
  Unrecognized(UnrecognizedValue),
}

impl Limit {
  /// The serialized name, or the text of an unrecognized value.
  pub fn as_str(&self) -> &str {
    match self {
      Limit::BytesRead => BYTES_READ_NAME,
      Limit::WorkingMemory => WORKING_MEMORY_NAME,
      Limit::Streams => STREAMS_NAME,
      Limit::Unrecognized(value) => value.as_str(),
    }
  }
}
