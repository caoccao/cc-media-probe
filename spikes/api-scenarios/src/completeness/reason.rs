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

//! A reason, `api.md` section 5.5, with a subset of its kinds. Since the fifteenth review a reason of a known kind
//! holds exactly the members of its kind: `limit_exhausted` names its limit, `feature_disabled` its feature and
//! `companion_limited` the limit that refused the companion, and the other kinds name neither. The stub has only the
//! first three kinds and no feature.

#[cfg(feature = "serde")]
use crate::constants::REASON_MESSAGE;
use crate::constants::{LIMIT_EXHAUSTED_NAME, RANGE_UNAVAILABLE_NAME, TRUNCATED_INPUT_NAME};
use crate::limits::Limit;
use crate::model::UnrecognizedValue;

/// Why something was not established. Read alone or inside a record, a reason of a known kind holds the members of
/// its kind (fifteenth review).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Reason {
  kind: ReasonKind,
  #[cfg_attr(
    feature = "serde",
    serde(
      default,
      deserialize_with = "crate::presence::present",
      skip_serializing_if = "Option::is_none"
    )
  )]
  limit: Option<Limit>,
  #[cfg_attr(
    feature = "serde",
    serde(
      default,
      deserialize_with = "crate::presence::present",
      skip_serializing_if = "Option::is_none"
    )
  )]
  detail: Option<String>,
}

#[cfg(feature = "serde")]
crate::neutral::checked_record_traits!(Reason from ReasonData);

/// A reason's members as read, before they are checked against its kind.
#[cfg(feature = "serde")]
#[derive(serde::Deserialize)]
struct ReasonData {
  kind: ReasonKind,
  #[serde(default, deserialize_with = "crate::presence::present")]
  limit: Option<Limit>,
  #[serde(default, deserialize_with = "crate::presence::present")]
  detail: Option<String>,
}

#[cfg(feature = "serde")]
impl TryFrom<ReasonData> for Reason {
  type Error = crate::document::InvalidDocument;

  /// A reason of a known kind with the members of its kind: `limit_exhausted` names its limit, and the other known
  /// kinds of the stub name none. A kind of a newer version holds whatever members it needs.
  fn try_from(data: ReasonData) -> Result<Reason, crate::document::InvalidDocument> {
    let fits = match data.kind {
      ReasonKind::LimitExhausted => data.limit.is_some(),
      ReasonKind::RangeUnavailable | ReasonKind::TruncatedInput => data.limit.is_none(),
      ReasonKind::Unrecognized(_) => true,
    };
    if !fits {
      return Err(crate::document::InvalidDocument::new(REASON_MESSAGE));
    }
    Ok(Reason {
      kind: data.kind,
      limit: data.limit,
      detail: data.detail,
    })
  }
}

impl Reason {
  /// The kind of reason.
  pub fn kind(&self) -> &ReasonKind {
    &self.kind
  }

  /// The limit, for an exhausted one.
  pub fn limit(&self) -> Option<&Limit> {
    self.limit.as_ref()
  }

  /// More detail, for people.
  pub fn detail(&self) -> Option<&str> {
    self.detail.as_deref()
  }

  /// Whether a reason that names an exhausted limit finds it among the exhausted ones (fourteenth review).
  #[cfg(feature = "serde")]
  pub(crate) fn finds_its_counter(&self, exhausted: &[Limit]) -> bool {
    match (&self.kind, &self.limit) {
      (ReasonKind::LimitExhausted, Some(limit)) => exhausted.contains(limit),
      _ => true,
    }
  }
}

/// The kinds of reason, plan section 8.3.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum ReasonKind {
  /// On a forward-only source, the bytes fell behind the read window before they were needed.
  RangeUnavailable,
  /// The input ended early.
  TruncatedInput,
  /// A limit was reached.
  LimitExhausted,
  /// A value that a newer version wrote.
  #[cfg_attr(feature = "serde", serde(with = "crate::model::unrecognized_text"))]
  Unrecognized(UnrecognizedValue<ReasonKind>),
}

impl ReasonKind {
  /// The serialized name, or the text of an unrecognized value.
  pub fn as_str(&self) -> &str {
    match self {
      ReasonKind::RangeUnavailable => RANGE_UNAVAILABLE_NAME,
      ReasonKind::TruncatedInput => TRUNCATED_INPUT_NAME,
      ReasonKind::LimitExhausted => LIMIT_EXHAUSTED_NAME,
      ReasonKind::Unrecognized(value) => value.as_str(),
    }
  }
}
