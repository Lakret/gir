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

  #[test]
  fn ubahn_representation() {
    let mut ubahn: Graph<&str, &str, Vec<&str>> = Graph::new();
    let (u6_line, u8_line, u9_line) = ("U6", "U8", "U9");

    ubahn.push_vertex("Franz-Neumann-Platz", vec![u8_line]);
    ubahn.push_vertex("Osloer Straße", vec![u8_line, u9_line]);
    ubahn.push_vertex("Nauener Platz", vec![u9_line]);
    ubahn.push_vertex("Pankstraße", vec![u8_line]);
    ubahn.push_vertex("Leopoldplatz", vec![u6_line, u9_line]);
    ubahn.push_vertex("Gesundbrunnen", vec![u8_line]);
    ubahn.push_vertex("Wedding", vec![u6_line]);
    ubahn.push_vertex("Seestraße", vec![u6_line]);
    ubahn.push_vertex("Amrumer Straße", vec![u9_line]);

    ubahn.push_undirected_edge("Franz-Neumann-Platz", "Osloer Straße", u8_line);
    ubahn.push_undirected_edge("Osloer Straße", "Pankstraße", u8_line);
    ubahn.push_undirected_edge("Pankstraße", "Gesundbrunnen", u8_line);

    ubahn.push_undirected_edge("Osloer Straße", "Nauener Platz", u9_line);
    ubahn.push_undirected_edge("Nauener Platz", "Leopoldplatz", u9_line);
    ubahn.push_undirected_edge("Leopoldplatz", "Amrumer Straße", u9_line);

    ubahn.push_undirected_edge("Seestraße", "Leopoldplatz", u6_line);
    ubahn.push_undirected_edge("Leopoldplatz", "Wedding", u6_line);

    // dbg!(&ubahn);
  }

  #[derive(Debug, PartialEq, Eq, Hash)]
  enum Op {
    Plus,
    Minus,
    Multiply,
  }

  #[derive(Debug, PartialEq, Eq, Hash)]
  enum Node {
    Number(i64),
    Apply(Op),
    Var(String),
  }

  use Node::*;
  use Op::*;

  #[test]
  fn arithmetic_representation() {
    let mut expr: Graph<Node> = Graph::new();

    expr.push_vid(Var("x".to_string()));
    expr.push_vid(Var("y".to_string()));
  }
}
