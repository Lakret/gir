use graphs::Graph;

use crate::maze::{Cell, Maze, Wall};

impl Maze {
  pub fn as_graph(&self) -> Graph<Cell, Wall> {
    let mut g: Graph<Cell, Wall> = Graph::new();

    for row in 0..self.height {
      for col in 0..self.width {
        let cell = (row, col);
        g.push_vertex(cell);
      }
    }

    // for row in 0..self.height {
    //   for col in 0..self.width {
    //     g.push_edge(from, to, )
    //   }
    // }

    todo!()
  }
}
