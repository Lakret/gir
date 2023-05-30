use std::{
  collections::{HashMap, HashSet, VecDeque},
  hash::Hash,
};

use crate::Graph;

/// Optional callbacks that will be executed at specific times during the search.
///
/// - `is_allowed_move(current, next)` - checks if a move from `current` vertex to `next` is allowed.
/// - `on_explore(parent, vertex)` will be called just before a `vertex` is added to the explored set
/// when it is reached via `parent`.
#[derive(Default)]
pub struct Opts<'a, VId> {
  is_allowed_move: Option<Box<dyn FnMut(&VId, &VId) -> bool + 'a>>,
  // 'a lifetime is needed to avoid requiring static lifetime accidentally;
  // FnMut since we need to call it multiple times, can allow mutation, but don't need ownership.
  on_explore: Option<Box<dyn FnMut(&VId, &VId) + 'a>>,
}

/// Searches the graph `g` using breadth-first search start at a vertex id `start`.
/// Stops when the `is_goal(vertex_id, depth)` function returns `true`.
/// If graph is exhausted and nothing is found, returns `false`, otherwise returns `true`.
///
/// Accepts an `opts` struct that may contain the following callbacks:
///
/// - `is_allowed_move(current, next)` - checks if a move from `current` vertex to `next` is allowed.
/// - `on_explore(parent, explored)` - called just before adding `explored` vertex id into the explored set.
/// `parent` is a vertex id from which we arrived to `explored`. Note, that for the `start` vertex id this
/// callback will not be called.
///
/// `depth` is calculated as a "level" of the graph we are exploring, counting from the `start`
/// vertex (`depth == 0`). A vertex B reached through the vertex A via this function will have
/// `depth_b = depth_a + 1`.
pub fn bfs<VId, E, V, GoalFn>(g: &Graph<VId, E, V>, start: &VId, is_goal: GoalFn, opts: &mut Opts<VId>) -> bool
where
  VId: Eq + Hash + Clone,
  V: Hash,
  GoalFn: Fn(&VId, usize) -> bool,
{
  let mut explored = HashSet::new();
  explored.insert(start);

  let mut queue = VecDeque::new();
  queue.push_back((start, 0));

  while let Some((curr, depth)) = queue.pop_front() {
    if is_goal(curr, depth) {
      return true;
    } else {
      for next in g.adjacent(curr) {
        let allowed = match &mut opts.is_allowed_move {
          None => true,
          Some(is_allowed_move) => is_allowed_move(curr, next),
        };

        if allowed && !explored.contains(next) {
          if let Some(on_explore) = &mut opts.on_explore {
            on_explore(curr, next);
          }
          explored.insert(next);

          queue.push_back((next, depth + 1));
        }
      }
    }
  }

  return false;
}

pub fn record_parents<'a, 'b, VId>(parents: &'b mut HashMap<VId, VId>, parent: &'a VId, explored: &'a VId)
where
  VId: Hash + Eq + Clone,
{
  parents.insert(explored.clone(), parent.clone());
}

pub fn path_from_parents<'a, VId>(parents: &'a HashMap<VId, VId>, vid: &'a VId) -> Vec<&'a VId>
where
  VId: Hash + Eq,
{
  let mut path = vec![];

  let mut curr = vid;
  while let Some(parent) = parents.get(curr) {
    path.push(parent);
    curr = parent;
  }

  path.reverse();
  path
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
          record_parents(&mut parents, parent, explored)
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

    assert_eq!(path_from_parents(&parents, &"L3_B"), vec![&"Root", &"L1_B", &"L2_C"]);

    // finds the start vertex
    assert!(bfs(&g, &"Root", |vid, _| *vid == "Root", &mut Opts::default()));

    // doesn't find a vertex that doesn't exist
    assert!(!bfs(&g, &"Root", |vid, _| *vid == "L3_C", &mut Opts::default()));

    // stops before the level 3 is reached (note, that we are using zero-based indexing).
    let mut all_explored = vec![];
    {
      let mut opts = Opts {
        on_explore: Some(Box::new(|_, explored| all_explored.push(*explored))),
        ..Opts::default()
      };

      assert!(bfs(&g, &"Root", |_, depth| depth == 2, &mut opts));
    }

    assert_eq!(all_explored.len(), 6);
    // start vertex id shouldn't be recorded, since it's assumed that it's explored before bfs runs.
    assert!(!all_explored.contains(&"Root"));
    assert!(all_explored.contains(&"L1_A"));
    assert!(all_explored.contains(&"L2_C"));
    assert!(!all_explored.contains(&"L3_A"));

    // only allow moves to the vertices ending with "_A" or "_B"
    let mut all_explored = vec![];
    {
      let mut opts = Opts {
        is_allowed_move: Some(Box::new(|_: &&str, next| next.ends_with("_A") || next.ends_with("_B"))),
        on_explore: Some(Box::new(|_, explored| all_explored.push(*explored))),
      };
      assert!(bfs(&g, &"Root", |vid, _| *vid == "L3_A", &mut opts));
    }

    assert_eq!(all_explored.len(), 5);
    assert!(all_explored.contains(&"L1_A"));
    assert!(all_explored.contains(&"L1_B"));
    assert!(all_explored.contains(&"L2_A"));
    assert!(all_explored.contains(&"L2_B"));
    assert!(all_explored.contains(&"L3_A"));
    assert!(!all_explored.contains(&"L1_C"));

    assert!(!bfs(
      &g,
      &"Root",
      |vid, _| *vid == "L3_C",
      &mut Opts {
        is_allowed_move: Some(Box::new(|_: &&str, next| next.ends_with("_A") || next.ends_with("_B"))),
        ..Opts::default()
      }
    ));
  }
}
