use graphs::{AbstractGraph, IGraph};

use crate::maze::{Cell, Maze, Wall};
use Wall::*;

impl Maze {
  pub fn as_graph(&self) -> IGraph<Cell, Wall> {
    let mut cells = vec![];
    for row in 0..self.height {
      for col in 0..self.width {
        cells.push((row, col));
      }
    }

    let mut g: IGraph<Cell, Wall> = IGraph::new();
    for cell in cells.iter().cloned() {
      g.push_vertex(cell);
    }

    let walls = [Left, Right, Bottom, Top];
    for &cell in cells.iter() {
      let from = g.get_vid(&cell);

      for &wall in &walls {
        if let Some(neighbour_cell) = self.neighbour(cell, wall) {
          let to = g.get_vid(&neighbour_cell);
          g.push_edge(from, to, wall);
        }
      }
    }

    g
  }
}
