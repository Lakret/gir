use fnv::{FnvHashMap, FnvHashSet, FnvHasher};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

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

pub trait AbstractGraph<V, E> {
  type VId;

  fn new() -> Self;
  fn push_vertex(self: &mut Self, vertex: V) -> Self::VId;
  fn push_edge(self: &mut Self, from: Self::VId, to: Self::VId, edge: E);

  fn adjacent<'a>(self: &Self, vid: Self::VId) -> Vec<Self::VId>;

  fn map_adjacent<F, R>(self: &Self, vid: Self::VId, f: F) -> Vec<R>
  where
    F: Fn(&(Self::VId, E)) -> R;
}

//
// Indexed Graph
//

pub struct IGraph<V, E, VId> {
  vertices: FnvHashSet<V>,
  adjacency: FnvHashMap<VId, Vec<(VId, E)>>,
  indexer: fn(&V) -> VId,
}

fn hash_vertex<V>(vertex: &V) -> u64
where
  V: Hash,
{
  let mut state = FnvHasher::default();
  vertex.hash(&mut state);
  state.finish()
}

impl<V, E> IGraph<V, E, u64>
where
  V: Hash,
{
  pub fn new() -> IGraph<V, E, u64> {
    IGraph {
      vertices: FnvHashSet::default(),
      adjacency: FnvHashMap::default(),
      indexer: hash_vertex,
    }
  }
}

impl<V, E> AbstractGraph<V, E> for IGraph<V, E, u64>
where
  V: Eq + Hash,
{
  type VId = u64;

  fn new() -> Self {
    IGraph::new()
  }

  fn push_vertex(self: &mut IGraph<V, E, u64>, vertex: V) -> Self::VId {
    let vid = (self.indexer)(&vertex);
    self.vertices.insert(vertex);
    vid
  }

  fn push_edge(self: &mut Self, from: Self::VId, to: Self::VId, edge: E) {
    let adjacent_to_from = self.adjacency.entry(from).or_default();
    adjacent_to_from.push((to, edge));
  }

  fn adjacent<'a>(self: &Self, vid: Self::VId) -> Vec<Self::VId> {
    self.adjacency.get(&vid).unwrap().iter().map(|(vid, _e)| *vid).collect()
  }

  fn map_adjacent<F, R>(self: &Self, vid: Self::VId, f: F) -> Vec<R>
  where
    F: Fn(&(Self::VId, E)) -> R,
  {
    self
      .adjacency
      .get(&vid)
      .unwrap()
      .iter()
      .map(|vid_and_e| f(vid_and_e))
      .collect()
  }
}

//
// VecGraph
//

impl<V, E> AbstractGraph<V, E> for VecGraph<V, E> {
  type VId = usize;

  fn new() -> Self {
    VecGraph::new()
  }

  fn push_vertex(self: &mut VecGraph<V, E>, vertex: V) -> Self::VId {
    self.push_vertex(vertex)
  }

  fn push_edge(self: &mut VecGraph<V, E>, from: Self::VId, to: Self::VId, edge: E) {
    self.push_edge(from, to, edge);
  }

  fn adjacent(self: &VecGraph<V, E>, vid: Self::VId) -> Vec<Self::VId> {
    VecGraph::adjacent(self, &vid).map(|(v, _e)| *v).collect()
  }

  fn map_adjacent<F, R>(self: &VecGraph<V, E>, vid: Self::VId, f: F) -> Vec<R>
  where
    F: Fn(&(Self::VId, E)) -> R,
  {
    self
      .adjacency
      .get(&vid)
      .unwrap()
      .iter()
      .map(|v_and_e| f(v_and_e))
      .collect()
  }
}

// Note: currently this cannot support deletion of vertices,
// since it will shift their positions in the vector.
#[derive(Debug, Clone)]
pub struct VecGraph<V, E> {
  vertices: Vec<V>,
  adjacency: HashMap<usize, Vec<(usize, E)>>,
}

impl<V, E> VecGraph<V, E> {
  pub fn new() -> Self {
    VecGraph {
      vertices: vec![],
      adjacency: HashMap::new(),
    }
  }

  pub fn push_vertex(self: &mut Self, vertex: V) -> usize {
    let last_idx = self.vertices.len();
    self.vertices.push(vertex);
    last_idx
  }

  pub fn push_edge(self: &mut Self, from: usize, to: usize, edge: E) {
    let adjacent_to_from = self.adjacency.entry(from).or_default();
    adjacent_to_from.push((to, edge));
  }

  pub fn adjacent(self: &Self, vertex: &usize) -> impl Iterator<Item = &(usize, E)> {
    self.adjacency.get(vertex).unwrap().iter()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn can_create_a_graph() {
    let mut g: VecGraph<&str, String> = VecGraph::new();
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
      g.map_adjacent(a_id, |x| x.clone()),
      [(b_id, "A -> B".to_string()), (a_id, "A loop".to_string())]
    );
    assert_eq!(g.map_adjacent(b_id, |x| x.clone()), [(c_id, "B -> C".to_string())]);
    assert_eq!(g.map_adjacent(c_id, |x| x.clone()), [(a_id, "C -> A".to_string())]);
  }

  #[test]
  fn can_create_an_indexed_graph() {
    let mut g: IGraph<&str, String, u64> = IGraph::new();
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
      g.map_adjacent(a_id, |x| x.clone()),
      [(b_id, "A -> B".to_string()), (a_id, "A loop".to_string())]
    );
    assert_eq!(g.map_adjacent(b_id, |x| x.clone()), [(c_id, "B -> C".to_string())]);
    assert_eq!(g.map_adjacent(c_id, |x| x.clone()), [(a_id, "C -> A".to_string())]);
  }
}
