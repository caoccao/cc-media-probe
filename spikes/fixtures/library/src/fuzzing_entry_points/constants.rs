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

//! The names of the fuzz targets, the layout of their input encodings and the bound of an input
//! (`docs/design/test-kit.md` section 6).

/// The container target of the toy format.
pub const TOY: &str = "toy";
/// The codec target of the toy codec.
pub const TOY_CODEC: &str = "toy_codec";
/// The detection target.
pub const DETECTION: &str = "detection";
/// The target of the full public probe.
pub const PROBE: &str = "probe";
/// The length of the toy codec's context: the framing, then the packet's known length in 32 bits, big-endian, which
/// is the bytes' own length in packet mode and 0 in stream mode.
pub const TOY_CODEC_CONTEXT_LENGTH: usize = 5;
/// The most bytes of an input, the fuzz run's `-max_len` of plan section 9.5, 1 MiB: libFuzzer truncates a corpus
/// file beyond it, so a longer seed would never reach its target whole.
pub const MAXIMUM_INPUT_LENGTH: usize = 1024 * 1024;
/// The outcome of an input that its target rejects before any parser.
pub const REJECTED: &str = "rejected";
/// The outcome of a probe that gave a report.
pub const REPORT: &str = "report";
