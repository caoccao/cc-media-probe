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

//! Operations on a tree of chapters or tags that walk it with an explicit stack: dropping, cloning, comparing,
//! hashing and `Debug` (`api.md` section 3.3; plan section 7.6). A document can hold a tree of any depth, so no
//! input can exhaust the stack.

use std::fmt;
use std::hash::{Hash, Hasher};

/// A node of a tree: its own members, and the trees below it in stored order.
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

/// The trees of nodes whose parents come before them, built from the last node to the first, so that no node is
/// built before the nodes below it.
pub(super) fn assemble<Tree: Branching>(nodes: Vec<(Option<usize>, Tree::Node)>) -> Vec<Tree> {
  let mut branches: Vec<Vec<Tree>> = nodes.iter().map(|_| Vec::new()).collect();
  let mut roots = Vec::new();
  for (position, (parent, node)) in nodes.into_iter().enumerate().rev() {
    let mut own = branches.get_mut(position).map_or_else(Vec::new, std::mem::take);
    own.reverse();
    let tree = Tree::from_parts(node, own);
    match parent
      .filter(|parent| *parent < position)
      .and_then(|parent| branches.get_mut(parent))
    {
      Some(siblings) => siblings.push(tree),
      None => roots.push(tree),
    }
  }
  roots.reverse();
  roots
}

/// Takes the tree apart without recursion: each node that it drops has nothing below it any more.
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
  assemble::<Tree>(nodes)
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

/// Hashes the number of nodes, then the nodes in depth-first order, each with the position of its parent. The
/// count comes first, as a slice's length does, so the data of one tree is never the start of another's, such as
/// that of a leaf and of the same leaf with a branch below it, and a tree hashed beside other fields keeps them
/// apart.
pub(crate) fn hash<Tree: Branching, State: Hasher>(tree: &Tree, state: &mut State) {
  let order = depth_first(std::slice::from_ref(tree));
  order.len().hash(state);
  for (parent, node) in order {
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

#[cfg(test)]
mod tests {
  use std::collections::hash_map::DefaultHasher;
  use std::fmt;
  use std::hash::Hasher;

  use super::{Branching, copy, equal, format, hash, release};

  /// A tree of numbers, which drops, clones, compares, hashes and formats itself through the operations.
  struct Numbers {
    value: u32,
    children: Vec<Numbers>,
  }

  impl Branching for Numbers {
    type Node = u32;

    fn node(&self) -> &u32 {
      &self.value
    }

    fn branches(&self) -> &[Numbers] {
      &self.children
    }

    fn branches_mut(&mut self) -> &mut Vec<Numbers> {
      &mut self.children
    }

    fn from_parts(node: u32, branches: Vec<Numbers>) -> Numbers {
      Numbers {
        value: node,
        children: branches,
      }
    }
  }

  impl Drop for Numbers {
    fn drop(&mut self) {
      release(self);
    }
  }

  impl fmt::Debug for Numbers {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
      format(self, formatter)
    }
  }

  fn leaf(value: u32) -> Numbers {
    Numbers::from_parts(value, Vec::new())
  }

  fn sample() -> Numbers {
    Numbers::from_parts(1, vec![Numbers::from_parts(2, vec![leaf(3)]), leaf(4)])
  }

  fn hashed(tree: &Numbers) -> u64 {
    let mut state = DefaultHasher::new();
    hash(tree, &mut state);
    state.finish()
  }

  #[test]
  fn copies_compares_hashes_and_formats_in_depth_first_order() {
    let tree = sample();
    let copied = copy(&tree);
    assert!(equal(&tree, &copied));
    assert_eq!(hashed(&tree), hashed(&copied));
    assert_eq!(
      format!("{tree:?}"),
      "[(None, 1), (Some(0), 2), (Some(1), 3), (Some(0), 4)]"
    );
    let other = Numbers::from_parts(1, vec![leaf(2), leaf(4)]);
    assert!(!equal(&tree, &other));
    assert_ne!(hashed(&tree), hashed(&other));
    assert!(!equal(&leaf(1), &leaf(2)));
  }

  /// A hasher that keeps every byte that it is given, in order.
  #[derive(Default)]
  struct Recorder {
    bytes: Vec<u8>,
  }

  impl Hasher for Recorder {
    fn finish(&self) -> u64 {
      0
    }

    fn write(&mut self, bytes: &[u8]) {
      self.bytes.extend_from_slice(bytes);
    }
  }

  fn recorded(tree: &Numbers) -> Vec<u8> {
    let mut state = Recorder::default();
    hash(tree, &mut state);
    state.bytes
  }

  #[test]
  fn hashes_no_tree_as_the_start_of_another() {
    let alone = recorded(&leaf(1));
    let with_branch = recorded(&Numbers::from_parts(1, vec![leaf(2)]));
    assert!(!with_branch.starts_with(&alone));
    assert_ne!(hashed(&leaf(1)), hashed(&Numbers::from_parts(1, vec![leaf(2)])));
    assert_eq!(Recorder::default().finish(), 0);
  }

  #[test]
  fn handles_a_tree_deeper_than_any_stack() {
    let mut tree = leaf(0);
    for value in 1..200_000 {
      tree = Numbers::from_parts(value, vec![tree]);
    }
    let copied = copy(&tree);
    assert!(equal(&tree, &copied));
    assert_eq!(hashed(&tree), hashed(&copied));
    drop(copied);
    drop(tree);
  }
}
