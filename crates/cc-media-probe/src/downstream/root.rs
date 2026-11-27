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

//! The public enums and structs of the crate root, which a crate that depends on the library can neither match
//! exhaustively nor build with a struct literal.

#[cfg(test)]
use crate as cc_media_probe;

no_exhaustive_match! {
  in [cc_media_probe];
  Codec { H264, H265, H266, Av1, Vp8, Vp9, Mpeg1Video, Mpeg2Video, Aac, MpegAudio, Ac3, EnhancedAc3, Ac4, Dts, TrueHd,
    MeridianLosslessPacking, Flac, Opus, Vorbis, Pcm, WebVtt, Pgs, Unknown, Unrecognized(..) }
  Format { Matroska, WebM, Mp4, ThreeGpp, ThreeGpp2, F4v, QuickTime, Avi, IsoBaseMedia, Riff, Unrecognized(..) }
  Limit { BytesRead, WorkingMemory, HeaderBytesPerStream, RetainedData, RetainedPayloads, DecompressedOutput,
    WorkUnits, NestingDepth, Streams, Diagnostics, CompanionRequests, SourcesOpened, Unrecognized(..) }
}

no_struct_literal! {
  in [cc_media_probe];
  Limits,
  Prober,
}
