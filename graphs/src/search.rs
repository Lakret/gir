use std::{
  collections::{HashMap, HashSet, VecDeque},
  hash::Hash,
};

use crate::Graph;

/// Defines a condition for the search to be stopped, i.e. the "goal" or "target" we are searching for.
#[derive(Debug)]
pub enum Goal<VId, V = ()> {
  ById(VId),
  ByValue(V),
  MaxDepth(usize),
}

/// Defines allowed moves, i.e. for a given `from` vertex, which adjacent vertices can be accessed
/// by the search procedure.
#[derive(Debug)]
pub enum MoveFilter<VId, V = ()> {
  ByIds(fn(from: VId, to: VId) -> bool),
  ByValues(fn(from: V, to: V) -> bool),
}

#[derive(Debug)]
pub enum OnExplore<'a, VId> {
  RecordParents(&'a mut HashMap<VId, VId>),
}

#[derive(Debug, Default)]
pub struct Opts<'a, VId, V = ()> {
  move_filter: Option<MoveFilter<VId, V>>,
  on_explore: Option<OnExplore<'a, VId>>,
}

/// Searches the graph `g` using breadth-first search.
pub fn bfs<VId, E, V>(g: Graph<VId, E, V>, start: VId, goal: Goal<VId, V>, opts: Opts<VId, V>)
where
  VId: Eq + Hash + Clone,
  V: Hash,
{
  let mut explored = HashSet::new();
  explored.insert(start.clone());

  let mut queue = VecDeque::new();
  queue.push_back(start.clone());

  todo!();
}
