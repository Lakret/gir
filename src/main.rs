use std::error::Error;
use svg::Document;

use gir::maze::{Maze, Wall::*};

fn draw(maze: &Maze) -> Document {
  let cell_side = 10;

  // TODO: draw SVG
  let (width, height) = (maze.width() * cell_side, maze.height() * cell_side);
  todo!()
}

// TODO: build a maze with randomized Prim algorithm
fn main() -> Result<(), Box<dyn Error>> {
  //     -----
  // | 1 | 4
  // | 2   3 |
  // ---------
  let mut maze = Maze::new(2, 2);
  maze
    .add_cell((0, 0), &[Left, Right])
    .add_cell((1, 0), &[Left, Bottom])
    .add_cell((1, 1), &[Bottom, Right])
    .add_cell((0, 1), &[Top, Left]);

  println!("Maze: {:?}", &maze);
  println!("Maze entrance: {:?}", maze[(0, 0)]);
  println!("Maze exit: {:?}", maze[(0, 1)]);
  Ok(())
}
