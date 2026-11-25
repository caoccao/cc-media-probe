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

//! What an audio stream says about its samples (`api.md` section 6.6).

mod ambisonics;
mod channel_layout;
mod constants;
mod immersive;
mod information;
mod sample_format;

pub use ambisonics::{AmbisonicChannelOrdering, AmbisonicNormalization, Ambisonics};
pub use channel_layout::{ChannelLayout, ChannelPosition, Speaker};
pub use immersive::{ImmersiveAudio, ImmersiveKind};
pub use information::AudioInformation;
pub use sample_format::{ByteOrder, SampleEncoding, SampleFormat};
