use std::collections::HashSet;
use std::error::Error;

use gir::maze::Wall::*;
use gir::maze::{Cell, Maze, Wall};

use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::Document;

const CELL_SIDE: u32 = 10;
const STROKE_WIDTH: u32 = 2;

fn make_line(from: (u32, u32), relative_to: (u32, u32)) -> Path {
  let data = Data::new().move_to(from).line_by(relative_to);

  Path::new()
    .set("fill", "none")
    .set("stroke", "black")
    .set("storke-width", STROKE_WIDTH)
    .set("stroke-linejoin", "square")
    .set("stroke-linecap", "square")
    .set("d", data)
}

fn add_cell_paths(paths: &mut Vec<Path>, maze: &Maze, (row, col): Cell, walls: &HashSet<Wall>) {
  let left_corner = (col * CELL_SIDE, row * CELL_SIDE);
  let (left_corner_x, left_corner_y) = left_corner;

  for &wall in walls {
    match wall {
      Top => {
        let path = make_line(left_corner, (CELL_SIDE, 0));
        paths.push(path)
      }
      Left => {
        let path = make_line(left_corner, (0, CELL_SIDE));
        paths.push(path)
      }
      // only draw right and bottom for right and bottom edges, to avoid double lines
      Right => {
        if col == maze.width() - 1 {
          let path = make_line((left_corner_x + CELL_SIDE, left_corner_y), (0, CELL_SIDE));
          paths.push(path);
        }
      }
      Bottom => {
        if row == maze.height() - 1 {
          let path = make_line((left_corner_x, left_corner_y + CELL_SIDE), (CELL_SIDE, 0));
          paths.push(path);
        }
      }
    };
  }
}

fn draw(maze: &Maze) -> Document {
  let mut paths = vec![];

  for row in 0..maze.height() {
    for col in 0..maze.width() {
      let cell = (row, col);
      add_cell_paths(&mut paths, &maze, cell, &maze[cell]);
    }
  }

  let (width, height) = (maze.width() * CELL_SIDE, maze.height() * CELL_SIDE);
  let document = Document::new().set("viewBox", (0, 0, width, height));
  paths.into_iter().fold(document, |document, path| document.add(path))
}

// TODO: build a maze with randomized Prim algorithm
fn main() -> Result<(), Box<dyn Error>> {
  let mut maze = Maze::new(3, 3);
  maze
    .add_cell((0, 0), &[Left, Right])
    .add_cell((1, 0), &[Left, Bottom])
    .add_cell((1, 1), &[Bottom, Right])
    .add_cell((0, 1), &[Top, Left])
    .add_cell((0, 2), &[Top, Right])
    .add_cell((1, 2), &[Left, Right])
    .add_cell((2, 2), &[Bottom, Right])
    .add_cell((2, 1), &[Top, Bottom])
    .add_cell((2, 0), &[Top, Bottom]);

  println!("Maze: {:?}", &maze);
  println!("Maze entrance: {:?}", maze[(0, 0)]);
  println!("Maze exit: {:?}", maze[(0, 1)]);

  let document = draw(&maze);
  svg::save("image.svg", &document).map_err(|e| e.into())
}
