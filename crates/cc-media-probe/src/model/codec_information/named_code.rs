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

//! `Profile`, `Level` and `Tier`: the same shape for every codec, a specification's name and the coded value
//! (`api.md` section 6.4).

use std::fmt;

use crate::form_record::form_record;

form_record! {
  /// The shape that `Profile`, `Level` and `Tier` share: the specification's name and the coded value as written,
  /// each optional. `Display` writes the name, or the code without one.
  #[derive(Clone, Debug, PartialEq, Eq, Hash)]
  struct NamedCode {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    name: Option<String>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    code: Option<String>,
  }
}

/// A profile, a level and a tier are each the record of their name and code (`json-output.md` section 5.6).
#[cfg(feature = "serde")]
macro_rules! named_code_form {
  ($($value:ident),* $(,)?) => {
    $(
      impl serde::Serialize for $value {
        fn serialize<Output: serde::Serializer>(&self, serializer: Output) -> Result<Output::Ok, Output::Error> {
          self.0.serialize(serializer)
        }
      }

      impl<'input> serde::Deserialize<'input> for $value {
        fn deserialize<Input: serde::Deserializer<'input>>(deserializer: Input) -> Result<$value, Input::Error> {
          NamedCode::deserialize(deserializer).map($value)
        }
      }
    )*
  };
}

#[cfg(feature = "serde")]
named_code_form!(Profile, Level, Tier);

impl fmt::Display for NamedCode {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(self.name.as_deref().or(self.code.as_deref()).unwrap_or_default())
  }
}

/// A codec's profile: the specification's name, such as "High", and the coded value as written, such as `100`.
/// The codec-specific values stay in the codec details. `Display` writes the name, or the code without one.
///
/// ```
/// use cc_media_probe::Stream;
///
/// fn profile_and_level(stream: &Stream) -> String {
///     let codec = stream.codec();
///     let profile = codec.profile().map(|profile| profile.to_string()).unwrap_or_default();
///     let level = codec.level().and_then(|level| level.name().or(level.code())).unwrap_or("");
///     let tier = codec.tier().map(|tier| tier.to_string()).unwrap_or_default();
///     format!("{profile} {level} {tier}")
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Profile(NamedCode);

/// A codec's level, such as "3.1" or "1b", with the coded value as written. `Display` writes the name, or the code
/// without one; `Profile` shows an example.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Level(NamedCode);

/// A codec's tier, such as "Main" or "High", with the coded value as written. `Display` writes the name, or the
/// code without one; `Profile` shows an example.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Tier(NamedCode);

impl Profile {
  /// The specification's name, such as "High".
  #[must_use]
  pub fn name(&self) -> Option<&str> {
    self.0.name.as_deref()
  }

  /// The coded value as written, such as `100`.
  #[must_use]
  pub fn code(&self) -> Option<&str> {
    self.0.code.as_deref()
  }
}

impl Level {
  /// The specification's name, such as "3.1".
  #[must_use]
  pub fn name(&self) -> Option<&str> {
    self.0.name.as_deref()
  }

  /// The coded value as written, such as `31`.
  #[must_use]
  pub fn code(&self) -> Option<&str> {
    self.0.code.as_deref()
  }
}

impl Tier {
  /// The specification's name, such as "Main".
  #[must_use]
  pub fn name(&self) -> Option<&str> {
    self.0.name.as_deref()
  }

  /// The coded value as written, such as `0`.
  #[must_use]
  pub fn code(&self) -> Option<&str> {
    self.0.code.as_deref()
  }
}

impl fmt::Display for Profile {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    self.0.fmt(formatter)
  }
}

impl fmt::Display for Level {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    self.0.fmt(formatter)
  }
}

impl fmt::Display for Tier {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    self.0.fmt(formatter)
  }
}

#[cfg(test)]
mod tests {
  use super::{Level, NamedCode, Profile, Tier};

  fn named_code(name: Option<&str>, code: Option<&str>) -> NamedCode {
    NamedCode {
      name: name.map(str::to_owned),
      code: code.map(str::to_owned),
    }
  }

  impl Profile {
    /// A profile with this name and code.
    pub(crate) fn sample(name: Option<&str>, code: Option<&str>) -> Profile {
      Profile(named_code(name, code))
    }
  }

  impl Level {
    /// A level with this name and code.
    pub(crate) fn sample(name: Option<&str>, code: Option<&str>) -> Level {
      Level(named_code(name, code))
    }
  }

  impl Tier {
    /// A tier with this name and code.
    pub(crate) fn sample(name: Option<&str>, code: Option<&str>) -> Tier {
      Tier(named_code(name, code))
    }
  }

  #[test]
  fn writes_the_name_or_else_the_code() {
    let profile = Profile::sample(Some("High"), Some("100"));
    assert_eq!((profile.name(), profile.code()), (Some("High"), Some("100")));
    assert_eq!(profile.to_string(), "High");
    let level = Level::sample(None, Some("31"));
    assert_eq!((level.name(), level.code()), (None, Some("31")));
    assert_eq!(level.to_string(), "31");
    let tier = Tier::sample(Some("Main"), Some("0"));
    assert_eq!((tier.name(), tier.code()), (Some("Main"), Some("0")));
    assert_eq!(tier.to_string(), "Main");
    assert_eq!(Tier::sample(None, None).to_string(), "");
  }
}
