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

//! The definition of a type of the form whose members Serde's derive reads, as the nineteenth review asked: the type
//! derives `Serialize`, and a private mirror derives `Deserialize` (`crate::neutral`). The macro is defined in every
//! build, since it also defines the type; what it adds for Serde stands behind the `serde` feature.

/// A type of the form whose members Serde's derive reads, given as the attribute `#[form(remote = "Name")]` that
/// names it and its definition: the definition with `Serialize` derived, and, inside a block, its private mirror
/// `Form` with `Deserialize` derived through `serde(remote)`, `ReadAlone` reading through the mirror and
/// `Deserialize` through `read_alone`. The name is a string, since `serde(remote)` takes only a literal, and the
/// macro consumes the attribute. A `check` given in it, a function of the value that returns its message as an
/// error, runs on every value read, so it holds alone as inside a report (nineteenth review).
macro_rules! strict_form {
  (
    #[form(remote = $remote:literal $(, check = $check:path)?)]
    $(#[$attribute:meta])*
    $visibility:vis $keyword:ident $name:ident $(<$parameter:ident>)? { $($body:tt)* }
  ) => {
    #[cfg_attr(feature = "serde", derive(serde::Serialize))]
    $(#[$attribute])*
    $visibility $keyword $name $(<$parameter>)? { $($body)* }

    #[cfg(feature = "serde")]
    const _: () = {
      #[derive(serde::Deserialize)]
      #[serde(remote = $remote)]
      #[allow(dead_code)]
      $(#[$attribute])*
      $keyword Form $(<$parameter>)? { $($body)* }

      impl<'input $(, $parameter: serde::Deserialize<'input>)?> $crate::neutral::ReadAlone<'input>
        for $name $(<$parameter>)?
      {
        fn read_members<Input: serde::Deserializer<'input>>(deserializer: Input) -> Result<Self, Input::Error> {
          let value = Form::deserialize(deserializer)?;
          $($check(&value).map_err(<Input::Error as serde::de::Error>::custom)?;)?
          Ok(value)
        }
      }

      impl<'input $(, $parameter: serde::Deserialize<'input>)?> serde::Deserialize<'input> for $name $(<$parameter>)? {
        fn deserialize<Input: serde::Deserializer<'input>>(deserializer: Input) -> Result<Self, Input::Error> {
          $crate::neutral::read_alone(deserializer)
        }
      }
    };
  };
}

pub(crate) use strict_form;
