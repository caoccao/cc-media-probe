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

//! The enums of `Strict`: the variant's name passes as it came, and what the variant holds is read strictly.

use serde::de::{DeserializeSeed, EnumAccess, VariantAccess, Visitor};

use super::access::StrictSeed;
use super::visitor::StrictVisitor;

/// An enum whose variant is read strictly.
pub(super) struct StrictEnum<Access> {
  access: Access,
}

impl<Access> StrictEnum<Access> {
  pub(super) fn new(access: Access) -> StrictEnum<Access> {
    StrictEnum { access }
  }
}

impl<'input, Access: EnumAccess<'input>> EnumAccess<'input> for StrictEnum<Access> {
  type Error = Access::Error;
  type Variant = StrictVariant<Access::Variant>;

  fn variant_seed<Seed: DeserializeSeed<'input>>(
    self,
    seed: Seed,
  ) -> Result<(Seed::Value, StrictVariant<Access::Variant>), Access::Error> {
    let (value, variant) = self.access.variant_seed(seed)?;
    Ok((value, StrictVariant { variant }))
  }
}

/// What a variant holds, read strictly.
pub(super) struct StrictVariant<Variant> {
  variant: Variant,
}

impl<'input, Variant: VariantAccess<'input>> VariantAccess<'input> for StrictVariant<Variant> {
  type Error = Variant::Error;

  fn unit_variant(self) -> Result<(), Variant::Error> {
    self.variant.unit_variant()
  }

  fn newtype_variant_seed<Seed: DeserializeSeed<'input>>(self, seed: Seed) -> Result<Seed::Value, Variant::Error> {
    self.variant.newtype_variant_seed(StrictSeed::value(seed))
  }

  fn tuple_variant<Read: Visitor<'input>>(self, length: usize, visitor: Read) -> Result<Read::Value, Variant::Error> {
    self.variant.tuple_variant(length, StrictVisitor::new(visitor, None))
  }

  fn struct_variant<Read: Visitor<'input>>(
    self,
    fields: &'static [&'static str],
    visitor: Read,
  ) -> Result<Read::Value, Variant::Error> {
    self.variant.struct_variant(fields, StrictVisitor::new(visitor, None))
  }
}
