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

//! `Reason`, why something fell short (`api.md` section 5.5).

use super::{Feature, ReasonKind};
use crate::Limit;
use crate::form_record::form_record;

form_record! {
  /// Why something fell short. A struct, so that a later reason can carry more data without a breaking change.
  ///
  /// A reason of a known kind has exactly the members that its kind defines (`ReasonKind`); `detail()` is optional
  /// for every kind and kept only within the retained-data limit.
  ///
  /// ```
  /// use cc_media_probe::Limit;
  /// use cc_media_probe::completeness::{Reason, ReasonKind};
  ///
  /// fn describe(reason: &Reason) -> String {
  ///     match (reason.kind(), reason.limit(), reason.feature()) {
  ///         (ReasonKind::LimitExhausted, Some(limit), _) => format!("the limit {} was reached", limit.as_str()),
  ///         (ReasonKind::FeatureDisabled, _, Some(feature)) => format!("enable the feature {}", feature.as_str()),
  ///         (kind, _, _) => match reason.detail() {
  ///             Some(detail) => format!("{}: {detail}", kind.as_str()),
  ///             None => kind.as_str().to_owned(),
  ///         },
  ///     }
  /// }
  ///
  /// fn stopped_by_reads(reason: &Reason) -> bool {
  ///     reason.limit() == Some(&Limit::BytesRead)
  /// }
  /// ```
  #[derive(Clone, Debug, PartialEq, Eq, Hash)]
  pub struct Reason {
    kind: ReasonKind,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    limit: Option<Limit>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    feature: Option<Feature>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    detail: Option<String>,
  }
  check = Reason::check_form;
}

impl Reason {
  /// The kind of reason.
  #[must_use]
  pub fn kind(&self) -> &ReasonKind {
    &self.kind
  }

  /// The limit, for `LimitExhausted`, the limit exhausted, and for `CompanionLimited`, the first limit that refused
  /// the companion.
  #[must_use]
  pub fn limit(&self) -> Option<&Limit> {
    self.limit.as_ref()
  }

  /// The feature, for `FeatureDisabled`.
  #[must_use]
  pub fn feature(&self) -> Option<&Feature> {
    self.feature.as_ref()
  }

  /// Text for people, such as the reason that a resolver gave for declining, kept within the retained-data limit.
  #[must_use]
  pub fn detail(&self) -> Option<&str> {
    self.detail.as_deref()
  }

  /// The reason's cause for people: the limit's name, such as "per-stream header limit", where the reason names a
  /// limit, and the kind's text otherwise.
  pub(crate) fn cause(&self) -> &str {
    self.limit.as_ref().map_or_else(|| self.kind.text(), Limit::text)
  }

  /// Whether a reason of a known kind holds the members of its kind and no other (`json-output.md` section 7.2):
  /// the limit of `limit_exhausted` and `companion_limited`, the feature of `feature_disabled`, and neither for the
  /// other kinds, while a kind of a newer version may hold either.
  #[cfg(feature = "serde")]
  fn check_form(&self) -> Result<(), &'static str> {
    let (limit, feature) = (self.limit.is_some(), self.feature.is_some());
    let fits = match self.kind {
      ReasonKind::LimitExhausted | ReasonKind::CompanionLimited => limit && !feature,
      ReasonKind::FeatureDisabled => feature && !limit,
      ReasonKind::Unrecognized(_) => true,
      _ => !limit && !feature,
    };
    if fits {
      Ok(())
    } else {
      Err(super::constants::REASON_MESSAGE)
    }
  }

  /// Whether a reason that names a limit finds its counter among the limits that the record holds as exhausted
  /// (`json-output.md` section 7.2), which `exhausted` tells, so that a caller checking many reasons looks each
  /// limit up once (review of the third slice's first part).
  #[cfg(feature = "serde")]
  pub(crate) fn finds_its_counter(&self, exhausted: impl Fn(&Limit) -> bool) -> bool {
    match (&self.kind, &self.limit) {
      (ReasonKind::LimitExhausted | ReasonKind::CompanionLimited, Some(limit)) => exhausted(limit),
      _ => true,
    }
  }

  /// Whether the two state the same cause: the same kind, limit and feature, whatever their details, which a first
  /// reason drops (`api.md` section 5.3).
  #[cfg(feature = "serde")]
  pub(crate) fn same_apart_from_detail(&self, other: &Reason) -> bool {
    self.kind == other.kind && self.limit == other.limit && self.feature == other.feature
  }

  /// Whether the reason says that too few bytes were there to decide the format: the input's end, a named limit or
  /// a kind of a newer version (`api.md` section 5.1), as a detection's and an undetermined format's reason does.
  #[cfg(feature = "serde")]
  pub(crate) fn tells_too_few_bytes(&self) -> bool {
    matches!(
      self.kind,
      ReasonKind::TruncatedInput | ReasonKind::LimitExhausted | ReasonKind::Unrecognized(_)
    )
  }

  /// The limit that a reason of the kind `limit_exhausted` names as exhausted, and `None` for any other kind.
  #[cfg(feature = "serde")]
  pub(crate) fn exhausted_limit(&self) -> Option<&Limit> {
    match self.kind {
      ReasonKind::LimitExhausted => self.limit.as_ref(),
      _ => None,
    }
  }

  /// Whether the reason is a request's that a source entry keeps: `companion_not_found`, `companion_declined` or
  /// `companion_limited` (`api.md` section 6.15).
  #[cfg(feature = "serde")]
  pub(crate) fn is_a_requests(&self) -> bool {
    matches!(
      self.kind,
      ReasonKind::CompanionNotFound | ReasonKind::CompanionDeclined | ReasonKind::CompanionLimited
    )
  }
}

#[cfg(test)]
mod tests {
  use super::Reason;
  use crate::Limit;
  use crate::completeness::{Feature, ReasonKind};

  impl Reason {
    /// A reason of this kind, without a limit, a feature or a detail.
    pub(crate) fn sample(kind: ReasonKind) -> Reason {
      Reason {
        kind,
        limit: None,
        feature: None,
        detail: None,
      }
    }

    /// A reason of the kind `LimitExhausted` that names this limit.
    pub(crate) fn exhausted(limit: Limit) -> Reason {
      Reason {
        limit: Some(limit),
        ..Reason::sample(ReasonKind::LimitExhausted)
      }
    }

    /// A reason of the kind `FeatureDisabled` that names this feature.
    pub(crate) fn disabled(feature: Feature) -> Reason {
      Reason {
        feature: Some(feature),
        ..Reason::sample(ReasonKind::FeatureDisabled)
      }
    }

    /// The same reason with this detail.
    pub(crate) fn with_detail(self, detail: &str) -> Reason {
      Reason {
        detail: Some(detail.to_owned()),
        ..self
      }
    }
  }

  #[test]
  fn holds_the_members_of_its_kind() {
    let limit = Reason::exhausted(Limit::Streams);
    assert_eq!(limit.kind(), &ReasonKind::LimitExhausted);
    assert_eq!(limit.limit(), Some(&Limit::Streams));
    assert_eq!(limit.feature(), None);
    assert_eq!(limit.cause(), "stream limit");
    let feature = Reason::disabled(Feature::Zlib);
    assert_eq!(feature.feature(), Some(&Feature::Zlib));
    assert_eq!(feature.limit(), None);
    assert_eq!(feature.cause(), "a disabled feature");
    let declined = Reason::sample(ReasonKind::CompanionDeclined).with_detail("access denied");
    assert_eq!(declined.detail(), Some("access denied"));
    assert_eq!(Reason::sample(ReasonKind::TruncatedInput).detail(), None);
  }
}
