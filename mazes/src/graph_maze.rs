use graphs::{AbstractGraph, IGraph};
use std::{collections::HashSet, hash::Hash};

use crate::maze::{Cell, Maze, Wall};
use Wall::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Passage {
  ArrowRight,
  ArrowDown,
}

use Passage::*;

impl Maze {
  // FIXME: add randomness to make non-trivial mazes too
  pub fn generate_maze_via_graph(width: u32, height: u32) -> Maze {
    let connected_graph = Maze::gen_connected_graph(width, height);
    dbg!(&connected_graph);

    let start_vid = connected_graph.get_vid(&(0, 0));
    let spanning_tree = connected_graph.spanning_tree(start_vid);
    dbg!(&spanning_tree);

    Maze::as_maze(&spanning_tree, width, height)
  }

  fn gen_connected_graph(width: u32, height: u32) -> IGraph<Cell, Passage> {
    let mut g = IGraph::new();

    for row in 0..height {
      for col in 0..width {
        let to_vid = g.push_vertex((row, col));

        // if there's a cell to the left, make an edge
        // (row, col - 1) -> (row, col)
        if col >= 1 {
          let from_vid = g.get_vid(&(row, col - 1));
          g.push_edge(from_vid, to_vid, ArrowRight);
        }

        // if there's a cell above, make an edge
        // (row - 1, col) ↓ (row, col)
        if row >= 1 {
          let from_vid = g.get_vid(&(row - 1, col));
          g.push_edge(from_vid, to_vid, ArrowDown)
        }
      }
    }

    g
  }

  fn as_maze(graph: &IGraph<&Cell, &Passage>, width: u32, height: u32) -> Maze {
    let mut maze = Maze::new(width, height);
    let all_walls = [Left, Right, Top, Bottom].iter().collect::<HashSet<_>>();

    for row in 0..height {
      for col in 0..width {
        let cell = (row, col);
        let mut walls = all_walls.clone();

        let vid = graph.get_vid(&&cell);
        // remove walls blocking passages starting at this cell
        graph.map_adjacent(vid, |&(_to_vid, edge)| match edge {
          ArrowRight => walls.remove(&Right),
          ArrowDown => walls.remove(&Bottom),
        });

        // if a cell to the left exists, check if there's a passage
        if col >= 1 {
          let left_vid = graph.get_vid(&&(row, col - 1));
          if graph.get_edge(left_vid, vid).is_some() {
            walls.remove(&Left);
          }
        }

        // if a cell above exists, check if there's a passage
        if row >= 1 {
          let above_vid = graph.get_vid(&&(row - 1, col));
          if graph.get_edge(above_vid, vid).is_some() {
            walls.remove(&Top);
          }
        }

        maze.add_cell(cell, &walls.into_iter().cloned().collect::<Vec<_>>());
      }
    }

    maze
  }
}
