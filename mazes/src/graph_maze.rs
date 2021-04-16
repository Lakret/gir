use rand::{thread_rng, Rng};
use std::{collections::HashSet, hash::Hash};

use crate::maze::{Cell, Maze, Wall};
use graphs::Graph;
use Wall::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Direction {
  ArrowLeft,
  ArrowRight,
  ArrowUp,
  ArrowDown,
}

use Direction::*;

type Passage = (Direction, u32);

impl Maze {
  /// Generates a Maze via minimum spanning tree algorithm on graphs.
  ///
  /// To make non-boring mazes sets each edge to a random weight.
  pub fn generate_maze_via_graph(width: u32, height: u32) -> Maze {
    let connected_graph = Maze::gen_connected_graph(width, height);

    let spanning_tree = connected_graph
      .minimum_spanning_tree(&(0, 0), &(|(_direction, weight)| *weight))
      .unwrap();

    Maze::mst_as_maze(&spanning_tree, width, height)
  }

  fn gen_connected_graph(width: u32, height: u32) -> Graph<Cell, Passage> {
    let mut g = Graph::new();
    // used to set each edge's weight to a random number
    // so that minimum spanning tree is different every time
    let mut rng = thread_rng();

    for row in 0..height {
      for col in 0..width {
        let to_cell = (row, col);
        g.push_vid(to_cell);

        // if there's a cell to the left, make edges `(row, col - 1) <-> (row, col)`.
        if col >= 1 {
          g.push_edge((row, col - 1), to_cell, (ArrowRight, rng.gen()));
          g.push_edge(to_cell, (row, col - 1), (ArrowLeft, rng.gen()));
        }

        // if there's a cell above, make an edge `(row - 1, col) ↕ (row, col)`.
        if row >= 1 {
          g.push_edge((row - 1, col), to_cell, (ArrowDown, rng.gen()));
          g.push_edge(to_cell, (row - 1, col), (ArrowUp, rng.gen()));
        }
      }
    }

    g
  }

  fn mst_as_maze(graph: &Graph<&Cell, &Passage, &()>, width: u32, height: u32) -> Maze {
    let mut maze = Maze::new(width, height);
    let all_walls = [Left, Right, Top, Bottom].iter().collect::<HashSet<_>>();

    for row in 0..height {
      for col in 0..width {
        let cell = (row, col);
        let mut walls = all_walls.clone();

        // remove walls blocking passages starting at this cell
        graph.map_adjacent(&cell, |&(_to_vid, (direction, _))| match direction {
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
