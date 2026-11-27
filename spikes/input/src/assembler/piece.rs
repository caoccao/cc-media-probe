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

//! `Piece`, one bounded delivery to an assembler.

/// One bounded delivery: a piece of one packet in packet mode, or a cut of the byte stream in stream mode, with the
/// three marks of plan section 7.4, "Packet mode".
#[derive(Clone, Copy, Debug)]
pub(crate) struct Piece<'bytes> {
  /// The bytes.
  pub(crate) bytes: &'bytes [u8],
  /// Whether it starts a packet, or in stream mode a delivery.
  pub(crate) starts_packet: bool,
  /// Whether it is the last piece of its packet. In stream mode the end of a delivery is not the end of the stream.
  pub(crate) ends_packet: bool,
  /// The packet's total length, where the container knows it.
  pub(crate) packet_length: Option<u64>,
}
