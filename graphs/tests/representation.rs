#[cfg(test)]
mod tests {
  use graphs::Graph;
  use Direction::*;

  #[derive(Clone, Debug, PartialEq, Eq, Hash)]
  enum Direction {
    Right,
    Down,
  }

  #[test]
  fn letter_maze_representation() {
    let mut maze: Graph<&str, Direction> = Graph::new();
    maze.push_vid("A");
    maze.push_vid("B");
    maze.push_vid("C");
    maze.push_vid("E");

    maze.push_edge("A", "B", Right);
    maze.push_edge("B", "E", Down);

    // dbg!(maze);
  }
}
