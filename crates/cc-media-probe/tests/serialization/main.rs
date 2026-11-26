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

//! The serialization tests of plan section 8.8 for the forms of the report model and its three root records that
//! stand alone (`json-output.md`), read from JSON documents and written back with `serde_json`.

#[cfg(test)]
mod activity_counters;
#[cfg(test)]
mod activity_usage;
#[cfg(test)]
mod agreements;
#[cfg(test)]
mod bytes_read;
#[cfg(test)]
mod candidate_order;
#[cfg(test)]
mod consistency;
#[cfg(test)]
mod data_enums;
#[cfg(test)]
mod detail_values;
#[cfg(test)]
mod detection_stops;
#[cfg(test)]
mod detections;
#[cfg(test)]
mod endings;
#[cfg(test)]
mod entity_rules;
#[cfg(test)]
mod fixtures;
#[cfg(test)]
mod identities;
#[cfg(test)]
mod limit_scopes;
#[cfg(test)]
mod model_checks;
#[cfg(test)]
mod model_records;
#[cfg(test)]
mod neutral_values;
#[cfg(test)]
mod omitted_counts;
#[cfg(test)]
mod own_checks;
#[cfg(test)]
mod paths;
#[cfg(test)]
mod private_numbers;
#[cfg(test)]
mod protection_records;
#[cfg(test)]
mod record_rules;
#[cfg(test)]
mod report_references;
#[cfg(test)]
mod requests;
#[cfg(test)]
mod self_describing;
#[cfg(test)]
mod shortfalls;
#[cfg(test)]
mod skipped_sections;
#[cfg(test)]
mod standalone;
#[cfg(test)]
mod stopping_limits;
#[cfg(test)]
mod stream_records;
#[cfg(test)]
mod trees;
#[cfg(test)]
mod version_first;
#[cfg(test)]
mod wire_values;
