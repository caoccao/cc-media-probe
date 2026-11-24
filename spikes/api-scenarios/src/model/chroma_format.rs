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

//! `ChromaFormat`, `api.md` section 6.5, with two of its unit variants: an enum that carries data and has variants
//! without a value. Serialized, a variant without a value is the map of its `kind` alone, as in
//! `{"kind": "monochrome"}`, and `Other` holds the member named after its kind (section 12).

#[cfg(feature = "serde")]
use serde::de::MapAccess;
#[cfg(feature = "serde")]
use serde::ser::SerializeMap;
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[cfg(feature = "serde")]
use super::constants::CHROMA_FORMAT_MEMBERS;
use super::constants::{CHROMA_420_KIND, MONOCHROME_KIND, OTHER_KIND};
use super::unrecognized_kind::UnrecognizedKind;
#[cfg(feature = "serde")]
use crate::constants::{CHROMA_FORMAT_MESSAGE, KIND_MEMBER};
#[cfg(feature = "serde")]
use crate::kept::{KeptMembers, KindRead, MemberValue, read_by_kind, write_kept};

/// The chroma format of a video stream.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ChromaFormat {
  /// Luma only.
  Monochrome,
  /// 4:2:0.
  Chroma420,
  /// A subsampling kept as written.
  Other(ChromaSubsampling),
  /// A kind that a newer version wrote, with its other members unread.
  Unrecognized(UnrecognizedKind<ChromaFormat>),
}

impl ChromaFormat {
  /// The serialized name of the kind, or the text of an unrecognized kind.
  pub fn as_str(&self) -> &str {
    match self {
      ChromaFormat::Monochrome => MONOCHROME_KIND,
      ChromaFormat::Chroma420 => CHROMA_420_KIND,
      ChromaFormat::Other(_) => OTHER_KIND,
      ChromaFormat::Unrecognized(kind) => kind.as_str(),
    }
  }
}

crate::form::strict_form! {
  #[form(remote = "ChromaSubsampling")]
  /// A chroma subsampling as written: the horizontal and vertical subsampling of the chroma planes.
  #[derive(Clone, Debug, PartialEq, Eq, Hash)]
  pub struct ChromaSubsampling {
    horizontal: u8,
    vertical: u8,
  }
}

impl ChromaSubsampling {
  /// The horizontal subsampling, as written.
  pub fn horizontal(&self) -> u8 {
    self.horizontal
  }

  /// The vertical subsampling, as written.
  pub fn vertical(&self) -> u8 {
    self.vertical
  }
}

/// Writes the kind, and for `Other` the member named after it; a kind of a newer version writes its kept members.
#[cfg(feature = "serde")]
impl Serialize for ChromaFormat {
  fn serialize<Output: Serializer>(&self, serializer: Output) -> Result<Output::Ok, Output::Error> {
    if let ChromaFormat::Unrecognized(unrecognized) = self {
      return write_kept(serializer, unrecognized.as_str(), unrecognized.members());
    }
    let mut map = serializer.serialize_map(None)?;
    map.serialize_entry(KIND_MEMBER, self.as_str())?;
    if let ChromaFormat::Other(subsampling) = self {
      map.serialize_entry(OTHER_KIND, subsampling)?;
    }
    map.end()
  }
}

/// Read strictly, alone as inside a read (eighteenth review).
#[cfg(feature = "serde")]
impl<'input> Deserialize<'input> for ChromaFormat {
  fn deserialize<Input: Deserializer<'input>>(deserializer: Input) -> Result<ChromaFormat, Input::Error> {
    crate::neutral::read_alone(deserializer)
  }
}

/// A variant without a value reads `kind` alone, and `Other` needs its member, read as it comes once the kind is
/// known (nineteenth review). Either way no member that another known kind reads may remain, and any other member is
/// passed by.
#[cfg(feature = "serde")]
impl<'input> crate::neutral::ReadAlone<'input> for ChromaFormat {
  fn read_members<Input: Deserializer<'input>>(deserializer: Input) -> Result<ChromaFormat, Input::Error> {
    read_by_kind(deserializer)
  }
}

/// What a kind of chroma format that this version knows gathers from its members.
#[cfg(feature = "serde")]
pub(crate) enum ChromaGathered {
  Monochrome,
  Chroma420,
  Other(Option<ChromaSubsampling>),
}

#[cfg(feature = "serde")]
impl KindRead for ChromaFormat {
  type Gathered = ChromaGathered;

  const MEMBERS: &'static [&'static str] = &CHROMA_FORMAT_MEMBERS;
  const MESSAGE: &'static str = CHROMA_FORMAT_MESSAGE;

  fn start(kind: &str) -> Option<ChromaGathered> {
    match kind {
      MONOCHROME_KIND => Some(ChromaGathered::Monochrome),
      CHROMA_420_KIND => Some(ChromaGathered::Chroma420),
      OTHER_KIND => Some(ChromaGathered::Other(None)),
      _ => None,
    }
  }

  fn reads(gathered: &ChromaGathered, name: &str) -> bool {
    matches!(gathered, ChromaGathered::Other(_)) && name == OTHER_KIND
  }

  fn gather<'input, Members: MapAccess<'input>>(
    gathered: &mut ChromaGathered,
    _name: &str,
    value: MemberValue<'_, Members>,
  ) -> Result<(), Members::Error> {
    if let ChromaGathered::Other(subsampling) = gathered {
      *subsampling = Some(value.read()?);
    }
    Ok(())
  }

  fn finish<Failure: serde::de::Error>(gathered: ChromaGathered) -> Result<ChromaFormat, Failure> {
    Ok(match gathered {
      ChromaGathered::Monochrome => ChromaFormat::Monochrome,
      ChromaGathered::Chroma420 => ChromaFormat::Chroma420,
      ChromaGathered::Other(subsampling) => {
        ChromaFormat::Other(subsampling.ok_or_else(|| Failure::custom(CHROMA_FORMAT_MESSAGE))?)
      }
    })
  }

  fn unrecognized(kind: String, members: KeptMembers) -> ChromaFormat {
    ChromaFormat::Unrecognized(UnrecognizedKind::new(kind, members))
  }
}
