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

//! `ExactValue` and `ExactInput`, a value beyond the exact range (`api.md` section 11.3; R24).

use crate::model::EntityLocation;

/// A quantity beyond 2^53 - 1 in magnitude, which the normalized field leaves absent with a diagnostic, kept here
/// as decimal strings with the named inputs it was computed from (R24; plan section 8.8).
///
/// ```
/// use cc_media_probe::details::ExactValue;
///
/// fn describe(exact: &ExactValue) -> String {
///     let value = match exact.denominator() {
///         Some(denominator) => format!("{}/{denominator}", exact.numerator()),
///         None => exact.numerator().to_owned(),
///     };
///     let inputs: Vec<String> =
///         exact.inputs().iter().map(|input| format!("{} = {}", input.name(), input.value())).collect();
///     format!("{} = {value} from {} ({:?}, {:?})", exact.field(), inputs.join(", "), exact.owner(), exact.stream())
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ExactValue {
  field: String,
  owner: Option<EntityLocation>,
  stream: Option<usize>,
  numerator: String,
  denominator: Option<String>,
  inputs: Vec<ExactInput>,
}

impl ExactValue {
  /// The field's name in the field guide.
  #[must_use]
  pub fn field(&self) -> &str {
    &self.field
  }

  /// The entity whose field it is; absent for a field of the whole report.
  #[must_use]
  pub fn owner(&self) -> Option<&EntityLocation> {
    self.owner.as_ref()
  }

  /// The owner's index when the owner is a stream.
  #[must_use]
  pub fn stream(&self) -> Option<usize> {
    self.stream
  }

  /// The numerator in canonical decimal, with a minus sign for a negative value.
  #[must_use]
  pub fn numerator(&self) -> &str {
    &self.numerator
  }

  /// The denominator in canonical decimal, present exactly when the field is a rational, `1` for a whole one, and
  /// absent for an integer field, such as a count.
  #[must_use]
  pub fn denominator(&self) -> Option<&str> {
    self.denominator.as_deref()
  }

  /// The named inputs that the value was computed from.
  #[must_use]
  pub fn inputs(&self) -> &[ExactInput] {
    &self.inputs
  }
}

/// One named input of an exact value, such as `chapter_time_start`, with its value in canonical decimal.
/// `ExactValue` shows an example.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ExactInput {
  name: String,
  value: String,
}

impl ExactInput {
  /// The input's name.
  #[must_use]
  pub fn name(&self) -> &str {
    &self.name
  }

  /// The input's value in canonical decimal.
  #[must_use]
  pub fn value(&self) -> &str {
    &self.value
  }
}

#[cfg(test)]
mod tests {
  use super::{ExactInput, ExactValue};
  use crate::model::{EntityKind, EntityLocation};

  #[test]
  fn keeps_the_value_and_its_inputs() {
    let exact = ExactValue {
      field: "start".to_owned(),
      owner: Some(EntityLocation::sample(EntityKind::Chapter, 0, &[0])),
      stream: None,
      numerator: "3689348814741910323".to_owned(),
      denominator: Some("200000000".to_owned()),
      inputs: vec![ExactInput {
        name: "chapter_time_start".to_owned(),
        value: "18446744073709551615".to_owned(),
      }],
    };
    assert_eq!(exact.field(), "start");
    assert_eq!(exact.owner().map(EntityLocation::chapter_path), Some(&[0][..]));
    assert_eq!(exact.stream(), None);
    assert_eq!(exact.numerator(), "3689348814741910323");
    assert_eq!(exact.denominator(), Some("200000000"));
    assert_eq!(exact.inputs()[0].name(), "chapter_time_start");
    assert_eq!(exact.inputs()[0].value(), "18446744073709551615");
  }
}
