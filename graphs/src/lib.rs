use derive_more::From;
use std::collections::HashMap;
use std::collections::HashSet;

// Known approaches:
//
//  1) `Rc` (https://github.com/nrc/r4cppp/blob/master/graphs/README.md)
//      Cons: Need to be careful with cycles, Rc's are leaked into userspace
//  2) `Arena`, `&`, and `UnsafeCell` (same source)
//      Cons: `unsafe`, additional fun with mutability if it's desired
//  3) Vector indices as keys for vertices
//  (http://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices/)
//  also used in https://docs.rs/petgraph/0.5.1/petgraph/
//  Cons: doesn't allow deletion, need to pass those indexes to use the API, cannot easily recover them
//  4) Indexed graphs - use HashMap(s) to save vertices and possibly edges.
//  Cons: additional memory and slowdown due to hashing. Still have indices in the API, but they are recoverable.
//  Pros: easiest to implement. Supports deletion. Since indices can be restored, doesn't require much thinking about them.

trait AbstractGraph<V, E> {
  type VId;

  fn new() -> Self;
  fn push_vertex(self: &mut Self, vertex: V) -> VId;
  fn push_edge(self: &mut Self, from: VId, to: VId, edge: E);

  fn adjacent<X: Iterator<Item = (VId, E)>>(self: &mut Self, vid: VId) -> X;
}

// position in Graph.vertices
#[derive(Debug, Eq, PartialEq, Clone, Copy, From, std::hash::Hash)]
pub struct VId(usize);

// (from, position in Graph.adjacency's vector for from)
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct EId(VId, usize);

// Note: currently this cannot support deletion of vertices,
// since it will shift their positions in the vector.
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

  pub fn get_adjacent(self: &Self, vertex: &VId) -> impl Iterator<Item = &(VId, E)> {
    self.adjacency.get(vertex).unwrap().iter()
  }

  pub fn push_vertex(self: &mut Self, vertex: V) -> VId {
    let last_idx = self.vertices.len();
    self.vertices.push(vertex);
    VId(last_idx)
  }

  pub fn push_edge(self: &mut Self, from: VId, to: VId, edge: E) {
    let adjacent_to_from = self.adjacency.entry(from).or_default();
    adjacent_to_from.push((to, edge));
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn can_create_a_graph() {
    let mut g: Graph<&str, String> = Graph::new();
    let a_id = g.push_vertex("A");
    let b_id = g.push_vertex("B");
    let c_id = g.push_vertex("C");

    g.push_edge(a_id, b_id, "A -> B".to_string());
    g.push_edge(b_id, c_id, "B -> C".to_string());
    g.push_edge(c_id, a_id, "C -> A".to_string());
    g.push_edge(a_id, a_id, "A loop".to_string());

    assert_eq!(g.vertices.len(), 3);
    assert_eq!(
      g.adjacency.get(&a_id).unwrap(),
      &[(b_id, "A -> B".to_string()), (a_id, "A loop".to_string())]
    );
    assert_eq!(g.adjacency.get(&b_id).unwrap(), &[(c_id, "B -> C".to_string())]);
    assert_eq!(g.adjacency.get(&c_id).unwrap(), &[(a_id, "C -> A".to_string())]);

    assert_eq!(
      g.get_adjacent(&a_id).collect::<Vec<_>>(),
      [&(b_id, "A -> B".to_string()), &(a_id, "A loop".to_string())]
    );
    assert_eq!(
      g.get_adjacent(&b_id).collect::<Vec<_>>(),
      [&(c_id, "B -> C".to_string())]
    );
    assert_eq!(
      g.get_adjacent(&c_id).collect::<Vec<_>>(),
      [&(a_id, "C -> A".to_string())]
    );
  }
}
