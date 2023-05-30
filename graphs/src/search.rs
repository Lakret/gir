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

#[derive(Default)]
pub struct Opts<'a, VId, V> {
  // TODO: make it similar to on_explore and use it
  move_filter: Option<MoveFilter<VId, V>>,
  // 'a lifetime is needed to avoid requiring static lifetime accidentally
  on_explore: Option<Box<dyn FnMut(&VId, &VId) + 'a>>,
}

/// Searches the graph `g` using breadth-first search.
pub fn bfs<VId, E, V, GoalFn>(g: &Graph<VId, E, V>, start: &VId, is_goal: GoalFn, opts: &mut Opts<VId, V>) -> bool
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
          if let Some(on_explore) = &mut opts.on_explore {
            on_explore(curr, next);
          }
          explored.insert(next);

          // TODO: parents[next] = curr
          queue.push_back(next);
        }
      }
    }
  }

  return false;
}

pub fn record_parents<'a, 'b, VId>(parent: &'a VId, explored: &'a VId, parents_map: &'b mut HashMap<VId, VId>)
where
  VId: Hash + Eq + Clone,
{
  parents_map.insert(explored.clone(), parent.clone());
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

    // finds a vertex in the last layer even in presence of cycles while recording parents
    let mut parents = HashMap::new();
    {
      let mut opts = Opts {
        on_explore: Some(Box::new(|parent, explored| {
          record_parents(parent, explored, &mut parents)
        })),
        ..Opts::default()
      };

      assert!(bfs(&g, &"Root", |vid, _| *vid == "L3_B", &mut opts));
    }
    assert_eq!(parents.len(), 8);
    assert_eq!(parents.get("Root"), None);
    assert_eq!(parents.get("L1_A"), Some(&"Root"));
    assert_eq!(parents.get("L1_C"), Some(&"Root"));
    assert_eq!(parents.get("L2_B"), Some(&"L1_A"));
    assert_eq!(parents.get("L2_C"), Some(&"L1_B"));
    assert_eq!(parents.get("L3_A"), Some(&"L2_B"));
    assert_eq!(parents.get("L3_B"), Some(&"L2_C"));

    // finds the start vertex
    assert!(bfs(&g, &"Root", |vid, _| *vid == "Root", &mut Opts::default()));

    // doesn't find a vertex that doesn't exist
    assert!(!bfs(&g, &"Root", |vid, _| *vid == "L3_C", &mut Opts::default()));
  }
}
