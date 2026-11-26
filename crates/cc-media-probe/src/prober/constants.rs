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

//! The serialized names of the limits and their names for people (`json-output.md` section 6.1).

/// The serialized name of `Limit::BytesRead`.
pub const BYTES_READ_NAME: &str = "bytes_read";
/// The serialized name of `Limit::WorkingMemory`.
pub const WORKING_MEMORY_NAME: &str = "working_memory";
/// The serialized name of `Limit::HeaderBytesPerStream`.
pub const HEADER_BYTES_PER_STREAM_NAME: &str = "header_bytes_per_stream";
/// The serialized name of `Limit::RetainedData`.
pub const RETAINED_DATA_NAME: &str = "retained_data";
/// The serialized name of `Limit::RetainedPayloads`.
pub const RETAINED_PAYLOADS_NAME: &str = "retained_payloads";
/// The serialized name of `Limit::DecompressedOutput`.
pub const DECOMPRESSED_OUTPUT_NAME: &str = "decompressed_output";
/// The serialized name of `Limit::WorkUnits`.
pub const WORK_UNITS_NAME: &str = "work_units";
/// The serialized name of `Limit::NestingDepth`.
pub const NESTING_DEPTH_NAME: &str = "nesting_depth";
/// The serialized name of `Limit::Streams`.
pub const STREAMS_NAME: &str = "streams";
/// The serialized name of `Limit::Diagnostics`.
pub const DIAGNOSTICS_NAME: &str = "diagnostics";
/// The serialized name of `Limit::CompanionRequests`.
pub const COMPANION_REQUESTS_NAME: &str = "companion_requests";
/// The serialized name of `Limit::SourcesOpened`.
pub const SOURCES_OPENED_NAME: &str = "sources_opened";

/// The name for people of `Limit::BytesRead`.
pub const BYTES_READ_TEXT: &str = "read limit";
/// The name for people of `Limit::WorkingMemory`.
pub const WORKING_MEMORY_TEXT: &str = "working-memory limit";
/// The name for people of `Limit::HeaderBytesPerStream`.
pub const HEADER_BYTES_PER_STREAM_TEXT: &str = "per-stream header limit";
/// The name for people of `Limit::RetainedData`.
pub const RETAINED_DATA_TEXT: &str = "retained-data limit";
/// The name for people of `Limit::RetainedPayloads`.
pub const RETAINED_PAYLOADS_TEXT: &str = "retained-payload limit";
/// The name for people of `Limit::DecompressedOutput`.
pub const DECOMPRESSED_OUTPUT_TEXT: &str = "decompression limit";
/// The name for people of `Limit::WorkUnits`.
pub const WORK_UNITS_TEXT: &str = "work limit";
/// The name for people of `Limit::NestingDepth`.
pub const NESTING_DEPTH_TEXT: &str = "nesting-depth limit";
/// The name for people of `Limit::Streams`.
pub const STREAMS_TEXT: &str = "stream limit";
/// The name for people of `Limit::Diagnostics`.
pub const DIAGNOSTICS_TEXT: &str = "diagnostics limit";
/// The name for people of `Limit::CompanionRequests`.
pub const COMPANION_REQUESTS_TEXT: &str = "companion-request limit";
/// The name for people of `Limit::SourcesOpened`.
pub const SOURCES_OPENED_TEXT: &str = "source limit";

/// One mebibyte, the unit of the proposed byte limits.
const MEBIBYTE: u64 = 1_048_576;

/// The proposed default of `Limit::BytesRead`, 64 MiB (plan section 7.5; confirmed at G3.3).
pub const DEFAULT_BYTES_READ: u64 = 64 * MEBIBYTE;
/// The proposed default of `Limit::WorkingMemory`, 32 MiB (plan section 7.5; confirmed at G3.3).
pub const DEFAULT_WORKING_MEMORY: u64 = 32 * MEBIBYTE;
/// The proposed default of `Limit::HeaderBytesPerStream`, 1 MiB (plan section 7.5; confirmed at G3.3).
pub const DEFAULT_HEADER_BYTES_PER_STREAM: u64 = MEBIBYTE;
/// The proposed default of `Limit::RetainedData`, 16 MiB (plan section 7.5; confirmed at G3.3).
pub const DEFAULT_RETAINED_DATA: u64 = 16 * MEBIBYTE;
/// The proposed default of `Limit::RetainedPayloads`, 64 MiB (plan section 7.5; confirmed at G3.3).
pub const DEFAULT_RETAINED_PAYLOADS: u64 = 64 * MEBIBYTE;
/// The proposed default of `Limit::DecompressedOutput`, 16 MiB (plan section 7.5; confirmed at G3.3).
pub const DEFAULT_DECOMPRESSED_OUTPUT: u64 = 16 * MEBIBYTE;
/// The proposed default of `Limit::WorkUnits`, 2,000,000 units (plan section 7.5; confirmed at G3.3).
pub const DEFAULT_WORK_UNITS: u64 = 2_000_000;
/// The proposed default of `Limit::NestingDepth`, 64 levels (plan section 7.5; confirmed at G3.3).
pub const DEFAULT_NESTING_DEPTH: u64 = 64;
/// The proposed default of `Limit::Streams`, 4,096 declarations (plan section 7.5; confirmed at G3.3).
pub const DEFAULT_STREAMS: u64 = 4_096;
/// The proposed default of `Limit::Diagnostics`, 1,000 diagnostics (plan section 7.5; confirmed at G3.3).
pub const DEFAULT_DIAGNOSTICS: u64 = 1_000;
/// The proposed default of `Limit::CompanionRequests`, 256 requests (plan section 7.5; confirmed at G3.3).
pub const DEFAULT_COMPANION_REQUESTS: u64 = 256;
/// The proposed default of `Limit::SourcesOpened`, 64 sources (plan section 7.5; confirmed at G3.3).
pub const DEFAULT_SOURCES_OPENED: u64 = 64;
