use std::error::Error;

use gir::maze::Maze;
use gir::maze::Wall::*;

use gir::draw::draw;

/* TODO:
- 1st video (tutorial, showcase, theory + SVG drawing, Rust):
  build a maze with randomized Prim algorithm & show with SVG.
- 2nd video (tutorial, slides, theory, Rust): introducing undirected / directed graph,
  spanning tree, graph representation, and writing generic Prim's algorithm
- 3nd video (tutorial, showcase, SVG/CSS animation, Rust):
  finding path in a Maze with Dijkstra's algorithm & Heaps / 3D and non-rectangular mazes
- video (tutorial, showcase / slides, Rust): Let's make a spreadsheet / Topological sorting
- video (live coding, ): Graph layout with Yew & WASM
- video (tutorial) Retro AI for playing a game: A*.
- video (tutorial) Max-flow
- video (tutorial, Julia): Not only HashMaps.
Representing graphs in Julia with SparseMatrix,
counting walks with matrix's power (matrix product animation),
and easy parallel distributed processing.
- video (live-coding, Elixir/Rust):
Distributing graphs with Elixir and Rustler. FIXME: what's cool problem that it solves?
*/
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
