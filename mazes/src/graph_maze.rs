use graphs::{AbstractGraph, IGraph};

use crate::maze::{Cell, Maze, Wall};
use Wall::*;

#[derive(Clone, std::hash::Hash, PartialEq, Eq)]
enum Passage {
  ArrowLeft,
  ArrowDown,
}

impl Maze {
  pub fn generate_maze_via_graph(width: u32, height: u32) -> Maze {
    let connected_graph = Maze::gen_connected_graph(width, height);
    let spanning_tree = connected_graph.spanning_tree();
    Maze::as_maze(&spanning_tree, width, height)
  }

  fn gen_connected_graph(width: u32, height: u32) -> IGraph<Cell, Passage> {
    let mut g = IGraph::new();
    // TODO:
    g
  }

  fn as_maze(graph: &IGraph<&Cell, &Passage>, width: u32, height: u32) -> Maze {
    let maze = Maze::new(width, height);
    // TODO:
    maze
  }

  // TODO: remove the old impl

  pub fn generate_with_graph(width: u32, height: u32) -> Maze {
    let maze = Maze::new(width, height);
    let graph_maze = maze.as_graph();

    let mut new_maze = maze.clone();
    // FIXME: need to iterate through each vertex instead, it seems
    for (from, _to, wall) in graph_maze.spanning_tree().iter_complete_edges() {
      let cell = *graph_maze.get_vertex(from).unwrap();
      new_maze.add_cell(cell, &[wall.opposite()]);
    }

    dbg!(&new_maze);
    dbg!(&graph_maze.spanning_tree());
    let mut spanning_tree_vertices = graph_maze
      .spanning_tree()
      .iter_vertices()
      .map(|(_vid, (row, col))| format!("({}, {})", row, col))
      .collect::<Vec<_>>();
    spanning_tree_vertices.sort();
    dbg!(&spanning_tree_vertices);
    new_maze
  }

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

    // TODO: should we only do it in one direction, i.e. from
    // the "smaller" to the "larger" cell coords?
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
