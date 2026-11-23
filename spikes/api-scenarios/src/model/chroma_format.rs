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
use serde::de::Error as _;
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
use crate::kept::{read_kind, refuse_other_kinds, write_kept};

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

/// A chroma subsampling as written: the horizontal and vertical subsampling of the chroma planes.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
  feature = "serde",
  derive(serde::Serialize, serde::Deserialize),
  serde(remote = "Self")
)]
pub struct ChromaSubsampling {
  horizontal: u8,
  vertical: u8,
}

#[cfg(feature = "serde")]
crate::neutral::record_traits!(ChromaSubsampling);

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

/// A variant without a value reads `kind` alone, and `Other` needs its member. Either way no member that another
/// known kind reads may remain, and any other member is ignored.
#[cfg(feature = "serde")]
impl<'input> Deserialize<'input> for ChromaFormat {
  fn deserialize<Input: Deserializer<'input>>(deserializer: Input) -> Result<ChromaFormat, Input::Error> {
    let (kind, mut members) = read_kind(deserializer, CHROMA_FORMAT_MESSAGE)?;
    let value = match kind.as_str() {
      MONOCHROME_KIND => ChromaFormat::Monochrome,
      CHROMA_420_KIND => ChromaFormat::Chroma420,
      OTHER_KIND => ChromaFormat::Other(
        members
          .take::<_, Input::Error>(OTHER_KIND)?
          .ok_or_else(|| Input::Error::custom(CHROMA_FORMAT_MESSAGE))?,
      ),
      _ => return Ok(ChromaFormat::Unrecognized(UnrecognizedKind::new(kind, members))),
    };
    refuse_other_kinds::<Input::Error>(&members, &CHROMA_FORMAT_MEMBERS, CHROMA_FORMAT_MESSAGE)?;
    Ok(value)
  }
}
