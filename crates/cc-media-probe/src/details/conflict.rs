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

//! `Conflict` and `ConflictingValue`, the raw values of a conflict between sources (`api.md` section 11.3).

use crate::form_record::form_record;
use crate::model::EntityLocation;

form_record! {
  /// The raw values of a conflict between sources of one field: the accessor returns the value of the first source
  /// that establishes one, and the details keep every source's value. The diagnostic of the conflict names the same
  /// field.
  ///
  /// ```
  /// use cc_media_probe::details::Conflict;
  ///
  /// fn describe(conflict: &Conflict) -> String {
  ///     let values: Vec<String> = conflict
  ///         .values()
  ///         .iter()
  ///         .map(|value| format!("{} says {}", value.source(), value.value()))
  ///         .collect();
  ///     let owner = conflict.owner().map(|owner| owner.kind().as_str()).unwrap_or("the file");
  ///     format!("{} of {owner} (stream {:?}): {}", conflict.field(), conflict.stream(), values.join("; "))
  /// }
  /// ```
  #[derive(Clone, Debug, PartialEq, Eq, Hash)]
  pub struct Conflict {
    field: String,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    owner: Option<EntityLocation>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    stream: Option<usize>,
    values: Vec<ConflictingValue>,
  }
  check = Conflict::check_form;
}

impl Conflict {
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

  /// Each source's value.
  #[must_use]
  pub fn values(&self) -> &[ConflictingValue] {
    &self.values
  }

  /// Whether the entry's stream is its owner's index exactly when the owner is a stream (`json-output.md` section 8).
  #[cfg(feature = "serde")]
  fn check_form(&self) -> Result<(), &'static str> {
    super::owner::check_owner(self.owner.as_ref(), self.stream)
  }
}

form_record! {
  /// One source's value in a conflict, as text. `Conflict` shows an example.
  #[derive(Clone, Debug, PartialEq, Eq, Hash)]
  pub struct ConflictingValue {
    source: String,
    value: String,
  }
}

impl ConflictingValue {
  /// The source's name, such as the element or the header field that stated the value.
  #[must_use]
  pub fn source(&self) -> &str {
    &self.source
  }

  /// The value as text.
  #[must_use]
  pub fn value(&self) -> &str {
    &self.value
  }
}

#[cfg(test)]
mod tests {
  use super::{Conflict, ConflictingValue};
  use crate::model::{EntityKind, EntityLocation};

  #[test]
  fn keeps_every_source_value() {
    let conflict = Conflict {
      field: "display_size".to_owned(),
      owner: Some(EntityLocation::sample(EntityKind::Stream, 0, &[])),
      stream: Some(0),
      values: vec![
        ConflictingValue {
          source: "DisplayWidth".to_owned(),
          value: "1920".to_owned(),
        },
        ConflictingValue {
          source: "sequence_parameter_set".to_owned(),
          value: "1440".to_owned(),
        },
      ],
    };
    assert_eq!(conflict.field(), "display_size");
    assert_eq!(conflict.owner().map(EntityLocation::kind), Some(&EntityKind::Stream));
    assert_eq!(conflict.stream(), Some(0));
    assert_eq!(conflict.values()[1].source(), "sequence_parameter_set");
    assert_eq!(conflict.values()[1].value(), "1440");
  }
}
