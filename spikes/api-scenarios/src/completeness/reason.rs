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
//! `companion_limited` the limit that refused the companion, and the other kinds name neither. The stub has four of
//! the kinds; since the sixteenth review, `feature_disabled` with its feature among them, since a first kept reason
//! is compared with its record's first reason in its feature too, which the review found missing.

#[cfg(feature = "serde")]
use crate::constants::REASON_MESSAGE;
use crate::constants::{
  FEATURE_DISABLED_NAME, LIMIT_EXHAUSTED_NAME, RANGE_UNAVAILABLE_NAME, TRUNCATED_INPUT_NAME, ZLIB_NAME,
};
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
  feature: Option<Feature>,
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
crate::neutral::checked_strict_traits!(Reason from ReasonData);

/// A reason's members as read, before they are checked against its kind.
#[cfg(feature = "serde")]
#[derive(serde::Deserialize)]
struct ReasonData {
  kind: ReasonKind,
  #[serde(default, deserialize_with = "crate::presence::present")]
  limit: Option<Limit>,
  #[serde(default, deserialize_with = "crate::presence::present")]
  feature: Option<Feature>,
  #[serde(default, deserialize_with = "crate::presence::present")]
  detail: Option<String>,
}

#[cfg(feature = "serde")]
impl TryFrom<ReasonData> for Reason {
  type Error = crate::document::InvalidDocument;

  /// A reason of a known kind with the members of its kind: `limit_exhausted` names its limit and no feature,
  /// `feature_disabled` its feature and no limit, and the other known kinds of the stub name neither. A kind of a
  /// newer version holds whatever members it needs.
  fn try_from(data: ReasonData) -> Result<Reason, crate::document::InvalidDocument> {
    let (limit, feature) = (data.limit.is_some(), data.feature.is_some());
    let fits = match data.kind {
      ReasonKind::LimitExhausted => limit && !feature,
      ReasonKind::FeatureDisabled => feature && !limit,
      ReasonKind::RangeUnavailable | ReasonKind::TruncatedInput => !limit && !feature,
      ReasonKind::Unrecognized(_) => true,
    };
    if !fits {
      return Err(crate::document::InvalidDocument::new(REASON_MESSAGE));
    }
    Ok(Reason {
      kind: data.kind,
      limit: data.limit,
      feature: data.feature,
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

  /// The feature, for a disabled one.
  pub fn feature(&self) -> Option<&Feature> {
    self.feature.as_ref()
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

crate::form::strict_form! {
  #[form(remote = "ReasonKind")]
  /// The kinds of reason, plan section 8.3.
  #[derive(Clone, Debug, PartialEq, Eq, Hash)]
  #[non_exhaustive]
  pub enum ReasonKind {
    /// On a forward-only source, the bytes fell behind the read window before they were needed.
    RangeUnavailable,
    /// The input ended early.
    TruncatedInput,
    /// A limit was reached.
    LimitExhausted,
    /// A Cargo feature that the data needs is off.
    FeatureDisabled,
    /// A value that a newer version wrote.
    #[cfg_attr(feature = "serde", serde(with = "crate::model::unrecognized_text"))]
    Unrecognized(UnrecognizedValue<ReasonKind>),
  }
}

impl ReasonKind {
  /// The serialized name, or the text of an unrecognized value.
  pub fn as_str(&self) -> &str {
    match self {
      ReasonKind::RangeUnavailable => RANGE_UNAVAILABLE_NAME,
      ReasonKind::TruncatedInput => TRUNCATED_INPUT_NAME,
      ReasonKind::LimitExhausted => LIMIT_EXHAUSTED_NAME,
      ReasonKind::FeatureDisabled => FEATURE_DISABLED_NAME,
      ReasonKind::Unrecognized(value) => value.as_str(),
    }
  }
}

crate::form::strict_form! {
  #[form(remote = "Feature")]
  /// A Cargo feature that can make information unavailable, `api.md` section 5.5.
  #[derive(Clone, Debug, PartialEq, Eq, Hash)]
  #[non_exhaustive]
  pub enum Feature {
    /// Inflating compressed data.
    Zlib,
    /// A value that a newer version wrote.
    #[cfg_attr(feature = "serde", serde(with = "crate::model::unrecognized_text"))]
    Unrecognized(UnrecognizedValue<Feature>),
  }
}

impl Feature {
  /// The serialized name, or the text of an unrecognized value.
  pub fn as_str(&self) -> &str {
    match self {
      Feature::Zlib => ZLIB_NAME,
      Feature::Unrecognized(value) => value.as_str(),
    }
  }
}
