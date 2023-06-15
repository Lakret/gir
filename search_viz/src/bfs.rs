//! AoC 2016, D13, the BFS Example.
//!
//! Solution, helper functions, and such.
use std::collections::{HashMap, HashSet, VecDeque};

const ADJACENT_DELTA: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
  pub x: u32,
  pub y: u32,
}

impl Pos {
  pub fn adjacent(&self) -> impl Iterator<Item = Pos> + '_ {
    ADJACENT_DELTA.into_iter().filter_map(|(dx, dy)| {
      match (self.x.checked_add_signed(dx), self.y.checked_add_signed(dy)) {
        (Some(x), Some(y)) => Some(Pos { x, y }),
        _ => None,
      }
    })
  }

  pub fn is_open(&self, fav_number: u32) -> bool {
    let res = fav_number + self.x * self.x + 3 * self.x + 2 * self.x * self.y + self.y + self.y * self.y;
    // see https://stackoverflow.com/a/74163801/797544
    (0..32).map(|bit_num| (res >> bit_num) & 1).filter(|v| *v == 1).count() % 2 == 0
  }
}

/// Searches for `goal` starting at `start` using `fav_number` to check for the walls.
/// Returns `parents` - a map from a vertex to the vertex it was first reached from on a path from `start`.
/// If `goal` was found, it will be present as a key in `parents`.
pub fn bfs(fav_number: u32, start: Pos, goal: Pos) -> HashMap<Pos, Pos> {
  let mut parents = HashMap::new();
  let mut queue = VecDeque::from(vec![start]);
  let mut explored = HashSet::new();
  explored.insert(start);

  while let Some(curr) = queue.pop_front() {
    if curr == goal {
      return parents;
    }

    for next in curr.adjacent().filter(|next| next.is_open(fav_number)) {
      if !explored.contains(&next) {
        explored.insert(next);
        parents.insert(next, curr);
        queue.push_back(next);
      }
    }
  }

  parents
}

/// Reconstructs path from `start` to `goal` using `parents` returned from `bfs`.
/// Empty vector means that the `goal` wasn't reached.
/// The `goal` itself is not included into the returned vector.
pub fn reconstruct_path(parents: HashMap<Pos, Pos>, goal: Pos) -> Vec<Pos> {
  let mut path = vec![];
  let mut curr = goal;

  while let Some(&parent) = parents.get(&curr) {
    path.push(parent);
    curr = parent;
  }

  path.reverse();
  path
}

pub fn p1(fav_number: u32, start: Pos, goal: Pos) -> Option<usize> {
  let parents = bfs(fav_number, start, goal);
  let path = reconstruct_path(parents, goal);

  if path.len() > 0 {
    Some(path.len())
  } else {
    None
  }
}

/// This is a version of bfs where instead of searching for a goal we limit the depth of exploration
/// and return the number of unique vertices encountered.
pub fn bfs2(fav_number: u32, start: Pos, max_depth: usize) -> usize {
  // queue now also saves per-vertex path depth
  let mut queue = VecDeque::from(vec![(start, 0)]);
  let mut explored = HashSet::new();
  explored.insert(start);

  while let Some((curr, depth)) = queue.pop_front() {
    if depth >= max_depth {
      return explored.len();
    }

    for next in curr.adjacent().filter(|next| next.is_open(fav_number)) {
      if !explored.contains(&next) {
        explored.insert(next);
        queue.push_back((next, depth + 1));
      }
    }
  }

  explored.len()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn pos_adjacent_test() {
    assert_eq!(
      Pos { x: 0, y: 0 }.adjacent().collect::<Vec<_>>(),
      vec![Pos { x: 1, y: 0 }, Pos { x: 0, y: 1 },]
    );

    assert_eq!(
      Pos { x: 2, y: 2 }.adjacent().collect::<Vec<_>>(),
      vec![
        Pos { x: 1, y: 2 },
        Pos { x: 3, y: 2 },
        Pos { x: 2, y: 1 },
        Pos { x: 2, y: 3 },
      ]
    );
  }

  #[test]
  fn is_open_test() {
    assert!((Pos { x: 0, y: 0 }).is_open(10));
    assert!(!(Pos { x: 1, y: 0 }).is_open(10));
    assert!((Pos { x: 2, y: 0 }).is_open(10));
    assert!(!(Pos { x: 3, y: 0 }).is_open(10));
    assert!((Pos { x: 0, y: 1 }).is_open(10));
    assert!(!(Pos { x: 0, y: 2 }).is_open(10));
    assert!(!(Pos { x: 0, y: 3 }).is_open(10));
    assert!((Pos { x: 0, y: 4 }).is_open(10));
    assert!((Pos { x: 1, y: 1 }).is_open(10));
    assert!(!(Pos { x: 2, y: 1 }).is_open(10));
  }

  #[test]
  fn bfs_and_reconstruct_path_test() {
    let goal = Pos { x: 7, y: 4 };
    let parents = bfs(10, Pos { x: 1, y: 1 }, goal);

    let path = reconstruct_path(parents, goal);
    assert_eq!(
      path,
      vec![
        Pos { x: 1, y: 1 },
        Pos { x: 1, y: 2 },
        Pos { x: 2, y: 2 },
        Pos { x: 3, y: 2 },
        Pos { x: 3, y: 3 },
        Pos { x: 3, y: 4 },
        Pos { x: 4, y: 4 },
        Pos { x: 4, y: 5 },
        Pos { x: 5, y: 5 },
        Pos { x: 6, y: 5 },
        Pos { x: 7, y: 5 },
      ]
    );
    assert_eq!(path.len(), 11);
  }

  #[test]
  fn p1_test() {
    assert_eq!(p1(10, Pos { x: 1, y: 1 }, Pos { x: 7, y: 4 }), Some(11));
    assert_eq!(p1(1350, Pos { x: 1, y: 1 }, Pos { x: 31, y: 39 }), Some(92));
  }

  #[test]
  fn bfs2_test() {
    assert_eq!(bfs2(1350, Pos { x: 1, y: 1 }, 50), 124);
  }
}
