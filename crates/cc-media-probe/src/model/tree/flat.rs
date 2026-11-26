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

//! The flat form of a tree (`api.md` section 12; `json-output.md` section 5.9), as the maintainer chose at the sixth
//! review of `api.md`: one array of every node in depth-first order, each with its own members and with `parent`, the
//! position of its parent in the same array, and no `children`, so a tree adds no depth to the document. Reading
//! takes depth-first order only: each node's `parent` is the node before it or one of that node's ancestors, so a
//! node that returns to a subtree that has closed fails (ninth review). A node's members are read into it from the
//! input as they come, and `parent` is taken out where it stands, as the fourth review of the third slice's first
//! part asked, which read `{"titles": false, "extension": null}` as a chapter and got the later null, and an invalid
//! `titles` reported at the closing brace: the eighteenth review had a node's members kept as written until the end
//! of the node, which also held an ignored member of an extension whole. So a member fails where it stands, with its
//! place in the input, and a member that the node does not read passes by as it comes. A standalone chapter or tag is
//! the same array for its subtree, with exactly one root (seventh review).

use std::fmt;
use std::marker::PhantomData;

use serde::de::value::MapAccessDeserializer;
use serde::de::{DeserializeSeed, Error, IntoDeserializer, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::constants::{NODE_EXPECTING, PARENT_MEMBER, SUBTREE_MESSAGE, TREE_ORDER_MESSAGE};
use super::operations::{Branching, assemble, depth_first};
use crate::form::ReadAlone;

/// A node as written: the position of its parent, and its own members beside it.
#[derive(Serialize)]
struct NodeMembers<'tree, Node> {
  #[serde(skip_serializing_if = "Option::is_none")]
  parent: Option<usize>,
  #[serde(flatten)]
  node: &'tree Node,
}

/// A node as read: the position of its parent, and the node itself.
struct NodeForm<Node> {
  parent: Option<usize>,
  node: Node,
}

impl<'input, Node: ReadAlone<'input>> Deserialize<'input> for NodeForm<Node> {
  fn deserialize<Input: Deserializer<'input>>(deserializer: Input) -> Result<NodeForm<Node>, Input::Error> {
    deserializer.deserialize_map(NodeVisitor(PhantomData))
  }
}

/// Reads a node's map as it comes.
struct NodeVisitor<Node>(PhantomData<Node>);

impl<'input, Node: ReadAlone<'input>> Visitor<'input> for NodeVisitor<Node> {
  type Value = NodeForm<Node>;

  fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(NODE_EXPECTING)
  }

  /// The node's own members are read into it from the input as they come, and `parent` is taken out where it stands.
  fn visit_map<Members: MapAccess<'input>>(self, members: Members) -> Result<NodeForm<Node>, Members::Error> {
    let mut parent = None;
    let node = Node::read_members(MapAccessDeserializer::new(WithoutParent {
      members,
      parent: &mut parent,
    }))?;
    Ok(NodeForm { parent, node })
  }
}

/// A node's members without `parent`, which it takes out into its slot as it passes, so that the node reads the rest
/// as its own. The map that it wraps refuses a name given twice, `parent` included.
struct WithoutParent<'slot, Members> {
  members: Members,
  parent: &'slot mut Option<usize>,
}

impl<'input, Members: MapAccess<'input>> MapAccess<'input> for WithoutParent<'_, Members> {
  type Error = Members::Error;

  fn next_key_seed<Seed: DeserializeSeed<'input>>(
    &mut self,
    seed: Seed,
  ) -> Result<Option<Seed::Value>, Members::Error> {
    while let Some(name) = self.members.next_key::<String>()? {
      if name != PARENT_MEMBER {
        return seed
          .deserialize(IntoDeserializer::<Members::Error>::into_deserializer(name))
          .map(Some);
      }
      *self.parent = Some(self.members.next_value()?);
    }
    Ok(None)
  }

  fn next_value_seed<Seed: DeserializeSeed<'input>>(&mut self, seed: Seed) -> Result<Seed::Value, Members::Error> {
    self.members.next_value_seed(seed)
  }
}

/// Whether the parents describe depth-first order: each node's parent is the node before it or an ancestor of that
/// node, and a node without a parent closes every subtree.
fn is_depth_first<Node>(nodes: &[(Option<usize>, Node)]) -> bool {
  let mut open: Vec<usize> = Vec::new();
  for (position, (parent, _)) in nodes.iter().enumerate() {
    match parent {
      None => open.clear(),
      Some(parent) => {
        while open.last().is_some_and(|last| last != parent) {
          open.pop();
        }
        if open.is_empty() {
          return false;
        }
      }
    }
    open.push(position);
  }
  true
}

/// Writes the trees as the flat array of their nodes.
pub(crate) fn write_flat<Tree, Output>(roots: &[Tree], serializer: Output) -> Result<Output::Ok, Output::Error>
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

/// Reads the trees from the flat array of their nodes, in depth-first order only.
pub(crate) fn read_flat<'input, Tree, Input>(deserializer: Input) -> Result<Vec<Tree>, Input::Error>
where
  Tree: Branching,
  Tree::Node: ReadAlone<'input>,
  Input: Deserializer<'input>,
{
  let nodes: Vec<(Option<usize>, Tree::Node)> = Vec::<NodeForm<Tree::Node>>::deserialize(deserializer)?
    .into_iter()
    .map(|form| (form.parent, form.node))
    .collect();
  if is_depth_first(&nodes) {
    Ok(assemble(nodes))
  } else {
    Err(Input::Error::custom(TREE_ORDER_MESSAGE))
  }
}

/// Writes a standalone node as the flat array of its subtree, the node itself first.
pub(crate) fn write_subtree<Tree, Output>(tree: &Tree, serializer: Output) -> Result<Output::Ok, Output::Error>
where
  Tree: Branching,
  Tree::Node: Serialize,
  Output: Serializer,
{
  write_flat(std::slice::from_ref(tree), serializer)
}

/// Reads a standalone node from the flat array of its subtree, which has exactly one root.
pub(crate) fn read_subtree<'input, Tree, Input>(deserializer: Input) -> Result<Tree, Input::Error>
where
  Tree: Branching,
  Tree::Node: ReadAlone<'input>,
  Input: Deserializer<'input>,
{
  let mut roots = read_flat::<Tree, Input>(deserializer)?;
  match (roots.pop(), roots.is_empty()) {
    (Some(root), true) => Ok(root),
    _ => Err(Input::Error::custom(SUBTREE_MESSAGE)),
  }
}
