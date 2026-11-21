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

//! Members of the JSON that `cargo metadata` prints.

/// The member that lists the packages.
pub const PACKAGES_MEMBER: &str = "packages";
/// The member that names a package or target.
pub const NAME_MEMBER: &str = "name";
/// The member that lists a package's targets.
pub const TARGETS_MEMBER: &str = "targets";
/// The member that lists a target's kinds, or gives a dependency's kind.
pub const KIND_MEMBER: &str = "kind";
/// The member that lists the workspace members.
pub const WORKSPACE_MEMBERS_MEMBER: &str = "workspace_members";
