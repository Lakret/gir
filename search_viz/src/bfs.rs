//! AoC 2016, D13, the BFS Example.
//!
//! Solution, helper functions, and such.

const ADJACENT_DELTA: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
}
