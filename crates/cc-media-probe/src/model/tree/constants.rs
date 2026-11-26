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

//! The constants of the flat form of a tree (R33; `json-output.md` section 5.9).

/// The member of a node that holds the position of its parent.
pub const PARENT_MEMBER: &str = "parent";

/// The message of nodes that are not in depth-first order: a parent that does not come before its node, or a node
/// that returns to a subtree that has closed.
pub const TREE_ORDER_MESSAGE: &str = "the nodes of a tree are not in depth-first order";

/// The message of a standalone node's array with no root or with two.
pub const SUBTREE_MESSAGE: &str = "a standalone node is not the flat array of one subtree";

/// What a node's reader expects: the map of a node.
pub const NODE_EXPECTING: &str = "the map of a node of a tree";
