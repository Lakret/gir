use std::error::Error;
use std::time::Instant;

use gir::draw::draw;
use gir::maze::Maze;

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

/// You can pass arguments like this:
/// `cargo run --release -- 128 72`
/// The first argument is width, the second is height.
fn main() -> Result<(), Box<dyn Error>> {
  let args = std::env::args()
    .skip(1)
    .map(|arg| dbg!(arg))
    .map(|arg| arg.parse::<u32>())
    .collect::<Result<Vec<_>, _>>()?;

  if args.len() >= 2 {
    let (width, height) = (args[0], args[1]);

    let t = Instant::now();
    let maze = Maze::generate(width, height);
    println!("Generated {}x{} maze in {:?}.", width, height, t.elapsed());

    let t = Instant::now();
    let document = draw(&maze);
    svg::save("image.svg", &document)?;
    println!("Saved to SVG in {:?}.", t.elapsed());

    Ok(())
  } else {
    Err(format!("Invalid args (expected width and height): {:?}", args).into())
  }
}
