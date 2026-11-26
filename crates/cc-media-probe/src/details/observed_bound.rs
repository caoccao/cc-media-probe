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

//! `ObservedBound` and `ObservedValue`, a lower bound of a total that was not established (`api.md` section 11.3).

use super::constants::{COUNT_KIND, TIME_KIND};
use crate::Seconds;
use crate::data_enumeration::data_enumeration;
use crate::form_record::form_record;
use crate::model::EntityLocation;

form_record! {
  /// A lower bound of a total that the probe observed while the total itself was not established for the whole. A
  /// partial count is never a total (plan section 8.3).
  ///
  /// ```
  /// use cc_media_probe::details::{ObservedBound, ObservedValue};
  ///
  /// fn describe(bound: &ObservedBound) -> String {
  ///     let value = match bound.value() {
  ///         ObservedValue::Count(count) => count.to_string(),
  ///         ObservedValue::Time(time) => time.to_string(),
  ///         other => other.as_str().to_owned(),
  ///     };
  ///     format!("{} is at least {value} (owner {:?}, stream {:?})", bound.field(), bound.owner(), bound.stream())
  /// }
  /// ```
  #[derive(Clone, Debug, PartialEq, Eq, Hash)]
  pub struct ObservedBound {
    field: String,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    owner: Option<EntityLocation>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    stream: Option<usize>,
    value: ObservedValue,
  }
  check = ObservedBound::check_form;
}

impl ObservedBound {
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

  /// The value observed.
  #[must_use]
  pub fn value(&self) -> &ObservedValue {
    &self.value
  }

  /// Whether the entry's stream is its owner's index exactly when the owner is a stream (`json-output.md` section 8).
  #[cfg(feature = "serde")]
  fn check_form(&self) -> Result<(), &'static str> {
    super::owner::check_owner(self.owner.as_ref(), self.stream)
  }
}

data_enumeration! {
  /// A value that the probe observed: a count or a time. An enum that carries data: serialized, it is a map of its
  /// `kind` and the member named after it (`json-output.md` section 6.2). `ObservedBound` shows an example.
  ObservedValue {
    /// A count.
    Count(u64) => COUNT_KIND,
    /// A time.
    Time(Seconds) => TIME_KIND,
  }
}

#[cfg(test)]
mod tests {
  use super::{ObservedBound, ObservedValue};
  use crate::Seconds;
  use crate::model::UnrecognizedKind;

  #[test]
  fn keeps_the_field_and_its_bound() {
    let bound = ObservedBound {
      field: "frame_count".to_owned(),
      owner: None,
      stream: Some(1),
      value: ObservedValue::Count(1_200),
    };
    assert_eq!(bound.field(), "frame_count");
    assert_eq!(bound.owner(), None);
    assert_eq!(bound.stream(), Some(1));
    assert_eq!(bound.value(), &ObservedValue::Count(1_200));
    assert_eq!(bound.value().as_str(), "count");
    assert_eq!(ObservedValue::Time(Seconds::sample(90, 1)).as_str(), "time");
    assert_eq!(
      ObservedValue::Unrecognized(UnrecognizedKind::sample("bytes")).as_str(),
      "bytes"
    );
  }
}
