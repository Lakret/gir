use graphs::Graph;
use std::{collections::HashSet, hash::Hash};

use crate::maze::{Cell, Maze, Wall};
use Wall::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Passage {
  ArrowLeft,
  ArrowRight,
  ArrowUp,
  ArrowDown,
}

use Passage::*;

impl Maze {
  // FIXME: add randomness to make non-trivial mazes too
  pub fn generate_maze_via_graph(width: u32, height: u32) -> Maze {
    let connected_graph = Maze::gen_connected_graph(width, height);
    dbg!(&connected_graph);

    let spanning_tree = connected_graph.spanning_tree(&(0, 0));
    dbg!(&spanning_tree);

    Maze::as_maze(&spanning_tree, width, height)
  }

  fn gen_connected_graph(width: u32, height: u32) -> Graph<Cell, Passage> {
    let mut g = Graph::new();

    for row in 0..height {
      for col in 0..width {
        let to_cell = (row, col);
        g.push_vertex(to_cell, ());

        // if there's a cell to the left, make edges
        // (row, col - 1) <-> (row, col)
        if col >= 1 {
          g.push_edge((row, col - 1), to_cell, ArrowRight);
          // g.push_edge(to_vid, from_vid, ArrowLeft);
        }

        // if there's a cell above, make an edge
        // (row - 1, col) ↕ (row, col)
        if row >= 1 {
          g.push_edge((row - 1, col), to_cell, ArrowDown);
          // g.push_edge(to_vid, from_vid, ArrowUp);
        }
      }
    }

    g
  }

  fn as_maze(graph: &Graph<&Cell, &Passage, &()>, width: u32, height: u32) -> Maze {
    let mut maze = Maze::new(width, height);
    let all_walls = [Left, Right, Top, Bottom].iter().collect::<HashSet<_>>();

    for row in 0..height {
      for col in 0..width {
        let cell = (row, col);
        let mut walls = all_walls.clone();

        // remove walls blocking passages starting at this cell
        graph.map_adjacent(&cell, |&(_to_vid, edge)| match edge {
          ArrowLeft => walls.remove(&Left),
          ArrowRight => walls.remove(&Right),
          ArrowUp => walls.remove(&Top),
          ArrowDown => walls.remove(&Bottom),
        });

        // if a cell to the left exists, check if there's a passage
        if col >= 1 {
          if graph.get_edge(&(row, col - 1), &cell).is_some() {
            walls.remove(&Left);
          }
        }

        // if a cell above exists, check if there's a passage
        if row >= 1 {
          if graph.get_edge(&(row - 1, col), &cell).is_some() {
            walls.remove(&Top);
          }
        }

        maze.add_cell(cell, &walls.into_iter().cloned().collect::<Vec<_>>());
      }
    }

    maze
  }
}
