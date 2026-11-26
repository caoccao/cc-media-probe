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

//! `Estimate` and `EstimateMethod`, an estimated value and how it was made (`api.md` section 6.1; R25).

use super::constants::{
  CONSTANT_BIT_RATE_OVER_SIZE_NAME, DELIVERY_CLOCK_SPAN_NAME, ENDPOINT_TIMESTAMPS_NAME, INDEX_SIZE_TOTALS_NAME,
  LAST_UNIT_START_NAME,
};
use crate::form_record::form_record;
use crate::string_enumeration::string_enumeration;

form_record! {
  /// An estimated value and how it was made. Estimates come only from the `estimated_` accessors, never from the
  /// normalized ones (R25).
  ///
  /// ```
  /// use cc_media_probe::MediaReport;
  ///
  /// fn duration_or_estimate(report: &MediaReport) -> Option<String> {
  ///     match (report.duration(), report.estimated_duration()) {
  ///         (Some(duration), _) => Some(duration.to_string()),
  ///         (None, Some(estimate)) => Some(format!("about {} ({})", estimate.value(), estimate.method().as_str())),
  ///         (None, None) => None,
  ///     }
  /// }
  /// ```
  #[derive(Clone, Debug, PartialEq, Eq, Hash)]
  pub struct Estimate<Value> {
    value: Value,
    method: EstimateMethod,
  }
}

impl<Value: Copy> Estimate<Value> {
  /// The estimated value, `Seconds` for a duration and a `Rational` of bits per second for a bit rate.
  ///
  /// ```
  /// use cc_media_probe::Stream;
  ///
  /// fn estimated_bit_rate(stream: &Stream) -> Option<f64> {
  ///     Some(stream.estimated_bit_rate()?.value().to_f64())
  /// }
  /// ```
  #[must_use]
  pub fn value(&self) -> Value {
    self.value
  }
}

impl<Value> Estimate<Value> {
  /// How the value was estimated.
  ///
  /// ```
  /// use cc_media_probe::MediaReport;
  /// use cc_media_probe::model::EstimateMethod;
  ///
  /// fn from_the_size(report: &MediaReport) -> bool {
  ///     report
  ///         .estimated_duration()
  ///         .is_some_and(|estimate| matches!(estimate.method(), EstimateMethod::ConstantBitRateOverSize))
  /// }
  /// ```
  #[must_use]
  pub fn method(&self) -> &EstimateMethod {
    &self.method
  }
}

string_enumeration! {
  /// How an estimate was made (plan section 8.4).
  ///
  /// ```
  /// use cc_media_probe::model::EstimateMethod;
  ///
  /// assert_eq!(EstimateMethod::LastUnitStart.as_str(), "last_unit_start");
  /// ```
  EstimateMethod {
    /// A constant bit rate applied to the size of the data.
    ConstantBitRateOverSize => CONSTANT_BIT_RATE_OVER_SIZE_NAME,
    /// The span of a delivery clock, such as a transport stream's program clock.
    DeliveryClockSpan => DELIVERY_CLOCK_SPAN_NAME,
    /// The first and the last timestamps that were read.
    EndpointTimestamps => ENDPOINT_TIMESTAMPS_NAME,
    /// The start of the last unit that was read.
    LastUnitStart => LAST_UNIT_START_NAME,
    /// The size totals of an index.
    IndexSizeTotals => INDEX_SIZE_TOTALS_NAME,
  }
}

#[cfg(test)]
mod tests {
  use super::{Estimate, EstimateMethod};
  use crate::Seconds;
  use crate::model::UnrecognizedValue;

  impl<Value> Estimate<Value> {
    /// An estimate of this value by this method.
    pub(crate) fn sample(value: Value, method: EstimateMethod) -> Estimate<Value> {
      Estimate { value, method }
    }
  }

  #[test]
  fn keeps_the_value_and_the_method() {
    let estimate = Estimate::sample(Seconds::sample(5_400, 1), EstimateMethod::ConstantBitRateOverSize);
    assert_eq!(estimate.value(), Seconds::sample(5_400, 1));
    assert_eq!(estimate.method(), &EstimateMethod::ConstantBitRateOverSize);
  }

  #[test]
  fn names_each_method() {
    for (method, name) in [
      (EstimateMethod::ConstantBitRateOverSize, "constant_bit_rate_over_size"),
      (EstimateMethod::DeliveryClockSpan, "delivery_clock_span"),
      (EstimateMethod::EndpointTimestamps, "endpoint_timestamps"),
      (EstimateMethod::LastUnitStart, "last_unit_start"),
      (EstimateMethod::IndexSizeTotals, "index_size_totals"),
      (
        EstimateMethod::Unrecognized(UnrecognizedValue::sample("guess")),
        "guess",
      ),
    ] {
      assert_eq!(method.as_str(), name);
    }
  }
}
