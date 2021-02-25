use std::collections::HashMap;

use derive_more::{From, Into};

// position in Graph.vertices
#[derive(Debug, Eq, PartialEq, Clone, Copy, From, Into, std::hash::Hash)]
pub struct VId(usize);

// (from, position in Graph.adjacency's vector for from)
#[derive(Debug, Eq, PartialEq, Clone, Copy, From, Into)]
pub struct EId(VId, usize);

#[derive(Debug, Clone)]
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

  pub fn get_vertex(self: &Self, vertex_id: VId) -> &V {
    &self.vertices[vertex_id.0]
  }

  pub fn get_edge(self: &Self, edge_id: EId) -> &E {
    let EId(vertex_id, edge_pos) = edge_id;
    let (_, e) = &self.adjacency.get(&vertex_id).unwrap()[edge_pos];
    e
  }

  pub fn get_adjacent(self: &Self, vertex_id: VId) -> impl Iterator<Item = &(VId, E)> {
    self.adjacency.get(&vertex_id).unwrap().iter()
  }

  pub fn push_vertex(self: &mut Self, vertex: V) -> VId {
    let last_idx = self.vertices.len();
    self.vertices.push(vertex);
    last_idx.into()
  }

  pub fn push_edge(self: &mut Self, from: VId, to: VId, edge: E) -> EId {
    let adjacent_to_from = self.adjacency.entry(from).or_default();

    let last_idx = adjacent_to_from.len();
    adjacent_to_from.push((to, edge));
    EId(from, last_idx)
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

    let a_to_b_id = g.push_edge(a_id, b_id, "A -> B".to_string());
    let b_to_c_id = g.push_edge(b_id, c_id, "B -> C".to_string());
    let c_to_a_id = g.push_edge(c_id, a_id, "C -> A".to_string());
    let a_loop_id = g.push_edge(a_id, a_id, "A loop".to_string());

    assert_eq!(g.vertices.len(), 3);
    assert_eq!(g.vertices, ["A".to_string(), "B".to_string(), "C".to_string()]);
    assert_eq!(
      g.adjacency.get(&a_id).unwrap(),
      &[(b_id, "A -> B".to_string()), (a_id, "A loop".to_string())]
    );
    assert_eq!(g.adjacency.get(&b_id).unwrap(), &[(c_id, "B -> C".to_string())]);
    assert_eq!(g.adjacency.get(&c_id).unwrap(), &[(a_id, "C -> A".to_string())]);

    assert_eq!(g.get_vertex(a_id), "A");
    assert_eq!(g.get_vertex(b_id), "B");
    assert_eq!(g.get_vertex(c_id), "C");

    assert_eq!(g.get_edge(a_to_b_id), "A -> B");
    assert_eq!(g.get_edge(b_to_c_id), "B -> C");
    assert_eq!(g.get_edge(c_to_a_id), "C -> A");
    assert_eq!(g.get_edge(a_loop_id), "A loop");

    assert_eq!(
      g.get_adjacent(a_id).collect::<Vec<_>>(),
      [&(b_id, "A -> B".to_string()), &(a_id, "A loop".to_string())]
    );
    assert_eq!(
      g.get_adjacent(b_id).collect::<Vec<_>>(),
      [&(c_id, "B -> C".to_string())]
    );
    assert_eq!(
      g.get_adjacent(c_id).collect::<Vec<_>>(),
      [&(a_id, "C -> A".to_string())]
    );
  }
}
