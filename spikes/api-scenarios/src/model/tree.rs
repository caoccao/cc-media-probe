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

//! The chapter and tag trees, `api.md` sections 6.8, 6.9 and 12. No operation on a tree recurses on its depth:
//! dropping, cloning, comparing, hashing and `Debug` walk it with an explicit stack (plan section 7.6). Serialized,
//! a tree is one flat array in depth-first order, each node with the position of its parent, so a document's
//! nesting depth is fixed by its schema whatever the depth of the tree.

use std::fmt;
use std::hash::{Hash, Hasher};

/// A node of a tree, with the members of its own and the trees below it.
pub(crate) trait Branching: Sized {
  /// The node's own members.
  type Node: Clone + PartialEq + Hash + fmt::Debug;

  /// The node's own members.
  fn node(&self) -> &Self::Node;

  /// The trees below the node, in stored order.
  fn branches(&self) -> &[Self];

  /// The trees below the node, to take them apart.
  fn branches_mut(&mut self) -> &mut Vec<Self>;

  /// A node with these members and these trees below it.
  fn from_parts(node: Self::Node, branches: Vec<Self>) -> Self;
}

/// Every node of the trees in depth-first order, each with the position of its parent in that order.
pub(crate) fn depth_first<Tree: Branching>(roots: &[Tree]) -> Vec<(Option<usize>, &Tree)> {
  let mut order = Vec::new();
  let mut pending: Vec<(Option<usize>, &Tree)> = roots.iter().rev().map(|tree| (None, tree)).collect();
  while let Some((parent, tree)) = pending.pop() {
    let position = order.len();
    order.push((parent, tree));
    pending.extend(tree.branches().iter().rev().map(|branch| (Some(position), branch)));
  }
  order
}

/// Whether each node's parent comes before it, as a flat form needs.
#[cfg(feature = "serde")]
pub(crate) fn is_ordered<Node>(nodes: &[(Option<usize>, Node)]) -> bool {
  nodes
    .iter()
    .enumerate()
    .all(|(position, (parent, _))| parent.is_none_or(|parent| parent < position))
}

/// The trees of nodes whose parents come before them, built from the last node to the first, so that no node is
/// built before the nodes below it.
pub(crate) fn assemble<Tree: Branching>(nodes: Vec<(Option<usize>, Tree::Node)>) -> Vec<Tree> {
  let mut branches: Vec<Vec<Tree>> = nodes.iter().map(|_| Vec::new()).collect();
  let mut roots = Vec::new();
  for (position, (parent, node)) in nodes.into_iter().enumerate().rev() {
    let mut own = branches.get_mut(position).map(std::mem::take).unwrap_or_default();
    own.reverse();
    let tree = Tree::from_parts(node, own);
    match parent.and_then(|parent| branches.get_mut(parent)) {
      Some(siblings) if parent.is_some_and(|parent| parent < position) => siblings.push(tree),
      _ => roots.push(tree),
    }
  }
  roots.reverse();
  roots
}

/// Takes the tree apart without recursion: each node it drops has nothing below it any more.
pub(crate) fn release<Tree: Branching>(tree: &mut Tree) {
  let mut pending = std::mem::take(tree.branches_mut());
  while let Some(mut branch) = pending.pop() {
    pending.append(branch.branches_mut());
  }
}

/// A copy of the tree, built from its nodes in depth-first order.
pub(crate) fn copy<Tree: Branching>(tree: &Tree) -> Tree {
  let nodes = depth_first(std::slice::from_ref(tree))
    .into_iter()
    .map(|(parent, node)| (parent, node.node().clone()))
    .collect();
  let mut roots = assemble::<Tree>(nodes);
  roots
    .pop()
    .unwrap_or_else(|| Tree::from_parts(tree.node().clone(), Vec::new()))
}

/// Whether two trees hold the same nodes in the same shape.
pub(crate) fn equal<Tree: Branching>(first: &Tree, second: &Tree) -> bool {
  let mut pending = vec![(first, second)];
  while let Some((left, right)) = pending.pop() {
    if left.node() != right.node() || left.branches().len() != right.branches().len() {
      return false;
    }
    pending.extend(left.branches().iter().zip(right.branches()));
  }
  true
}

/// Hashes the nodes in depth-first order, each with the position of its parent.
pub(crate) fn hash<Tree: Branching, State: Hasher>(tree: &Tree, state: &mut State) {
  for (parent, node) in depth_first(std::slice::from_ref(tree)) {
    parent.hash(state);
    node.node().hash(state);
  }
}

/// Formats the nodes in depth-first order, each with the position of its parent.
pub(crate) fn format<Tree: Branching>(tree: &Tree, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
  formatter
    .debug_list()
    .entries(
      depth_first(std::slice::from_ref(tree))
        .into_iter()
        .map(|(parent, node)| (parent, node.node())),
    )
    .finish()
}

/// The flat form of a list of trees, for `#[serde(with)]`.
#[cfg(feature = "serde")]
pub(crate) mod flat {
  use serde::de::Error as _;
  use serde::{Deserialize, Deserializer, Serialize, Serializer};

  use super::{Branching, assemble, depth_first, is_ordered};
  use crate::constants::TREE_ORDER_MESSAGE;

  /// One node as written: its own members, with the position of its parent beside them.
  #[derive(Serialize)]
  struct NodeMembers<'tree, Node> {
    #[serde(skip_serializing_if = "Option::is_none")]
    parent: Option<usize>,
    #[serde(flatten)]
    node: &'tree Node,
  }

  /// One node as read.
  #[derive(Deserialize)]
  struct NodeData<Node> {
    parent: Option<usize>,
    #[serde(flatten)]
    node: Node,
  }

  /// Writes every node of the trees in depth-first order.
  pub(crate) fn serialize<Tree, Output>(roots: &[Tree], serializer: Output) -> Result<Output::Ok, Output::Error>
  where
    Tree: Branching,
    Tree::Node: Serialize,
    Output: Serializer,
  {
    serializer.collect_seq(depth_first(roots).into_iter().map(|(parent, tree)| NodeMembers {
      parent,
      node: tree.node(),
    }))
  }

  /// Reads the nodes and builds the trees; a node whose parent does not come before it is malformed.
  pub(crate) fn deserialize<'input, Tree, Input>(deserializer: Input) -> Result<Vec<Tree>, Input::Error>
  where
    Tree: Branching,
    Tree::Node: Deserialize<'input>,
    Input: Deserializer<'input>,
  {
    let nodes: Vec<(Option<usize>, Tree::Node)> = Vec::<NodeData<Tree::Node>>::deserialize(deserializer)?
      .into_iter()
      .map(|data| (data.parent, data.node))
      .collect();
    if !is_ordered(&nodes) {
      return Err(Input::Error::custom(TREE_ORDER_MESSAGE));
    }
    Ok(assemble(nodes))
  }
}
