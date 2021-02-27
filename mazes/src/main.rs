use std::error::Error;
use std::time::Instant;

use mazes::draw::draw;
use mazes::graph_maze;
use mazes::maze::Maze;

/// You can pass arguments like this:
/// `cargo run --release -- 128 72`
/// The first argument is width, the second is height.
fn main() -> Result<(), Box<dyn Error>> {
  let args = std::env::args()
    .skip(1)
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
