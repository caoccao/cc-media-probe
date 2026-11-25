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

//! `InventoryClass`, how a format reaches its stream inventory (`api.md` section 5.2; plan section 8.3).

use super::constants::{DECLARED_NAME, OBSERVED_NAME, SINGLE_STREAM_NAME};
use crate::string_enumeration::string_enumeration;

string_enumeration! {
  /// How a format reaches its stream inventory: a declared structure that lists the streams, a single-stream file
  /// whose primary structure was validated, or the first stream-carrying unit of an observed format.
  ///
  /// ```
  /// use cc_media_probe::MediaReport;
  /// use cc_media_probe::model::InventoryClass;
  ///
  /// fn streams_are_declared(report: &MediaReport) -> bool {
  ///     matches!(report.inventory_class(), InventoryClass::Declared)
  /// }
  ///
  /// assert_eq!(InventoryClass::SingleStream.as_str(), "single_stream");
  /// ```
  InventoryClass {
    /// A structure that lists the streams, such as Matroska's `Tracks`.
    Declared => DECLARED_NAME,
    /// A single-stream file whose primary structure was validated.
    SingleStream => SINGLE_STREAM_NAME,
    /// A format whose streams appear as its units are read, such as a transport stream.
    Observed => OBSERVED_NAME,
  }
}

#[cfg(test)]
mod tests {
  use super::InventoryClass;
  use crate::model::UnrecognizedValue;

  #[test]
  fn names_each_class() {
    for (class, name) in [
      (InventoryClass::Declared, "declared"),
      (InventoryClass::SingleStream, "single_stream"),
      (InventoryClass::Observed, "observed"),
      (
        InventoryClass::Unrecognized(UnrecognizedValue::sample("indexed")),
        "indexed",
      ),
    ] {
      assert_eq!(class.as_str(), name);
    }
  }
}
