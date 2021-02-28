/**
 * Indexed Graphs.
 */
use fnv::{FnvHashMap, FnvHashSet, FnvHasher};
use std::hash::{Hash, Hasher};

use crate::AbstractGraph;

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
    self.push_edge_vid(from, to, edge);
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

  pub fn push_edge_direct(self: &mut Self, from: &V, to: &V, edge: E) {
    let from_vid = (self.indexer)(from);
    let to_vid = (self.indexer)(to);

    self.push_edge_vid(from_vid, to_vid, edge);
  }

  fn push_edge_vid(self: &mut Self, from: u64, to: u64, edge: E) {
    let adjacent_to_from = self.adjacency.entry(from).or_default();
    adjacent_to_from.push((to, edge));
  }
}

#[cfg(test)]
mod tests {
  use super::*;

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
