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
  }

  #[derive(Debug, PartialEq, Eq, Hash)]
  enum Op {
    Sub,
    Mul,
  }

  #[derive(Debug, PartialEq, Eq, Hash)]
  enum Expr {
    Num(i64),
    Var(String),
    Apply { op: Op, lhs: Box<Expr>, rhs: Box<Expr> },
  }

  use Expr::*;
  use Op::*;

  #[derive(Debug, PartialEq, Eq, Hash)]
  struct Binding {
    name: String,
    expr: Expr,
  }

  #[test]
  fn bindings_representation() {
    let mut bindings: Graph<String, (), Expr> = Graph::new();

    let x_expr = Apply {
      op: Mul,
      lhs: Box::new(Apply {
        op: Sub,
        lhs: Box::new(Num(2)),
        rhs: Box::new(Num(7)),
      }),
      rhs: Box::new(Var("y".to_string())),
    };

    let y_expr = Apply {
      op: Sub,
      lhs: Box::new(Apply {
        op: Mul,
        lhs: Box::new(Var("x".to_string())),
        rhs: Box::new(Num(3)),
      }),
      rhs: Box::new(Num(2)),
    };

    bindings.push_vertex("x".to_string(), x_expr);
    bindings.push_vertex("y".to_string(), y_expr);

    bindings.push_edge("x".to_string(), "y".to_string(), ());
    bindings.push_edge("y".to_string(), "x".to_string(), ());
  }
}
