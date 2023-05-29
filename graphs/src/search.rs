use std::hash::Hash;

use crate::Graph;

/// Defines a condition for the search to be stopped, i.e. the "goal" or "target" we are searching for.
pub enum Goal<VId, V = ()> {
  ById(VId),
  ByValue(V),
  MaxDepth(usize),
}

/// Defines allowed moves, i.e. for a given `from` vertex, which adjacent vertices can be accessed
/// by the search procedure.
pub enum MoveFilter<VId, V = ()> {
  ByIds(fn(from: VId, to: VId) -> bool),
  ByValues(fn(from: V, to: V) -> bool),
}

/// Searches the graph `g` using breadth-first search.
pub fn bfs<VId, E, V>(g: Graph<VId, E, V>, start: VId, goal: Goal<VId, V>, move_filter: Option<MoveFilter<VId, V>>)
where
  VId: Eq + Hash,
  V: Hash,
{
  todo!()
}
