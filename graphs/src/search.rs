use std::{
  collections::{HashMap, HashSet, VecDeque},
  hash::Hash,
};

use crate::Graph;

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

// TODO: use those
#[derive(Debug, Default)]
pub struct Opts<'a, VId, V = ()> {
  move_filter: Option<MoveFilter<VId, V>>,
  on_explore: Option<OnExplore<'a, VId>>,
}

/// Searches the graph `g` using breadth-first search.
pub fn bfs<VId, E, V, GoalFn>(g: &Graph<VId, E, V>, start: &VId, is_goal: GoalFn, opts: Opts<VId, V>) -> bool
where
  VId: Eq + Hash + Clone,
  V: Hash,
  GoalFn: Fn(&VId, usize) -> bool,
{
  let mut explored = HashSet::new();
  explored.insert(start);

  let mut queue = VecDeque::new();
  queue.push_back(start);

  // TODO:
  let mut depth = 0;

  while let Some(curr) = queue.pop_front() {
    if is_goal(curr, depth) {
      // TODO: return parents
      return true;
    } else {
      for next in g.adjacent(curr) {
        if !explored.contains(next) {
          explored.insert(next);
          // parents[next] = curr
          queue.push_back(next);
        }
      }
    }
  }

  return false;
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn bfs_test() {
    // A directed graph with 3 layers and a cycle:
    // Root -> L1_A, L1_B, L1_C
    // L1_A -> L2_A, L2_B
    // L1_B -> L2_C
    // L2_B -> L3_A, L1_A
    // L2_C -> L3_B
    let mut g = Graph::new();
    for vid in ["Root", "L1_A", "L1_B", "L1_C", "L2_A", "L2_B", "L2_C", "L3_A", "L3_B"] {
      g.push_vid(vid);
    }
    for (from, to) in [
      ("Root", "L1_A"),
      ("Root", "L1_B"),
      ("Root", "L1_C"),
      ("L1_A", "L2_A"),
      ("L1_A", "L2_B"),
      ("L1_B", "L2_C"),
      ("L2_B", "L3_A"),
      ("L2_B", "L1_A"),
      ("L2_C", "L3_B"),
    ] {
      g.push_edge(from, to, ());
    }

    // finds a vertex in the last layer even in presence of cycles
    assert!(bfs(&g, &"Root", |vid, _| *vid == "L3_B", Opts::default()));
    // doesn't find a vertex that doesn't exist
    assert!(!bfs(&g, &"Root", |vid, _| *vid == "L3_C", Opts::default()));
  }
}
