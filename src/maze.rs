use std::collections::{HashMap, HashSet};
use std::hash::Hash;

/// (row, col), 0-indexed
type Cell = (u32, u32);

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Wall {
  Top = 0,
  Right = 1,
  Bottom = 2,
  Left = 3,
}

#[derive(Debug, Clone)]
pub struct Maze {
  width: u32,
  height: u32,
  walls: HashMap<Cell, HashSet<Wall>>,
}

impl Maze {
  pub fn new(width: u32, height: u32) -> Maze {
    let walls = HashMap::new();
    Maze { width, height, walls }
  }

  pub fn add_cell(&mut self, cell: Cell, walls: &[Wall]) -> &mut Maze {
    self.walls.insert(cell, walls.into_iter().copied().collect());
    self
  }

  pub fn width(&self) -> u32 {
    self.width
  }

  pub fn height(&self) -> u32 {
    self.height
  }
}

impl std::ops::Index<Cell> for Maze {
  type Output = HashSet<Wall>;

  fn index(&self, index: Cell) -> &Self::Output {
    self
      .walls
      .get(&index)
      .expect(&format!("Cell at {:?} doesn't exist.", &index))
  }
}
