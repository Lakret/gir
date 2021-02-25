use std::collections::HashMap;

use derive_more::{From, Into};

#[derive(Debug, Eq, PartialEq, Clone, Copy, From, Into, std::hash::Hash)]
pub struct VId(usize);

#[derive(Debug, Eq, PartialEq, Clone, Copy, From, Into)]
pub struct EId(usize);

pub struct Graph<V, E> {
  vertices: Vec<V>,
  adjacency: HashMap<VId, Vec<(VId, E)>>,
}

impl<V, E> Graph<V, E> {
  pub fn new() -> Self {
    Graph {
      vertices: vec![],
      adjacency: HashMap::new(),
    }
  }

  pub fn push_vertex(self: &mut Self, vertex: V) -> VId {
    self.vertices.push(vertex);
    self.vertices.len().into()
  }
}

impl<V, E> Graph<V, E> {
  pub fn push_edge(self: &mut Self, from: VId, to: VId, edge: E) -> EId {
    let adjacent_to_from = self.adjacency.entry(from).or_default();
    adjacent_to_from.push((to, edge));
    adjacent_to_from.len().into()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn can_create_a_graph() {
    let mut g: Graph<String, String> = Graph::new();
    let a_id = g.push_vertex("A".to_string());
    let b_id = g.push_vertex("B".to_string());
    let c_id = g.push_vertex("C".to_string());

    g.push_edge(a_id, b_id, "A -> B".to_string());
    g.push_edge(b_id, c_id, "B -> C".to_string());
    g.push_edge(c_id, a_id, "C -> A".to_string());

    assert_eq!(g.vertices.len(), 3);
    assert_eq!(g.vertices, ["A".to_string(), "B".to_string(), "C".to_string()]);
    assert_eq!(g.adjacency.get(&a_id).unwrap(), &[(b_id, "A -> B".to_string())]);
    assert_eq!(g.adjacency.get(&b_id).unwrap(), &[(c_id, "B -> C".to_string())]);
    assert_eq!(g.adjacency.get(&c_id).unwrap(), &[(a_id, "C -> A".to_string())]);
  }
}
