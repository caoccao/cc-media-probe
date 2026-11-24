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

//! The answer of `Strict` to a request for the scope: a sequence of two, the version of the root record whose body
//! it reads, as text that is empty without one, and the strict deserializer itself, from which the record then reads
//! as it would have from the request.

use serde::Deserializer;
use serde::de::{DeserializeSeed, IntoDeserializer, SeqAccess};

use super::deserializer::{Holder, Strict};

/// The two elements of the answer, each taken once.
pub(super) struct ScopeAccess<'name, Inner> {
  holder: Option<Holder>,
  deserializer: Option<Strict<'name, Inner>>,
}

impl<'name, Inner> ScopeAccess<'name, Inner> {
  pub(super) fn new(holder: Holder, deserializer: Strict<'name, Inner>) -> ScopeAccess<'name, Inner> {
    ScopeAccess {
      holder: Some(holder),
      deserializer: Some(deserializer),
    }
  }
}

impl<'input, Inner: Deserializer<'input>> SeqAccess<'input> for ScopeAccess<'_, Inner> {
  type Error = Inner::Error;

  fn next_element_seed<Seed: DeserializeSeed<'input>>(
    &mut self,
    seed: Seed,
  ) -> Result<Option<Seed::Value>, Inner::Error> {
    if let Some(holder) = self.holder.take() {
      let text = holder.map(String::from).unwrap_or_default();
      return seed
        .deserialize(IntoDeserializer::<Inner::Error>::into_deserializer(text))
        .map(Some);
    }
    match self.deserializer.take() {
      Some(deserializer) => seed.deserialize(deserializer).map(Some),
      None => Ok(None),
    }
  }
}
