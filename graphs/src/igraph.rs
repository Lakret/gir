/**
 * Indexed Graphs.
 */
use fnv::FnvHashMap;
use std::{collections::binary_heap::Iter, hash::Hash};

use crate::AbstractGraph;

/// A hashmap-based Graph representation.
/// Owns vertex and edge data, and exposes explicit vertex id type parameter `VId`.
///
/// ## Identity-only graphs
///
/// It's often easier to represent simple graphs that don't have any specific
/// info associated with each vertex, except for that vertex's identity
/// as `IGraph<(), E, VId>`. This is similar to how `HashSet` is defined:
/// it also uses `HashMap<Key, Value = ()>` internally.
///
/// If your `VId` type is `Copy` or a reference, you can get useability similar to
/// the famous [Python graph representation via hashmaps](https://www.python.org/doc/essays/graphs/).
#[derive(Debug)]
pub struct IGraph<V, E, VId = u64> {
  vertices: FnvHashMap<VId, V>,
  adjacency: FnvHashMap<VId, Vec<(VId, E)>>,
}

impl<V, E, VId> IGraph<V, E, VId>
where
  V: Hash,
  VId: Eq + Hash,
{
  pub fn new() -> IGraph<V, E, VId> {
    IGraph {
      vertices: FnvHashMap::default(),
      adjacency: FnvHashMap::default(),
    }
  }

  pub fn contains(&self, vid: &VId) -> bool {
    self.vertices.contains_key(vid)
  }

  pub fn iter_vertices(&self) -> impl Iterator<Item = (&VId, &V)> {
    self.vertices.iter()
  }

  pub fn iter_edges(&self) -> impl Iterator<Item = (&VId, &Vec<(VId, E)>)> {
    self.adjacency.iter().map(|(from_vid, incident)| (from_vid, incident))
  }

  pub fn iter_complete_edges(&self) -> impl Iterator<Item = (&VId, &VId, &E)> {
    self
      .iter_edges()
      .flat_map(|(from_vid, incident)| incident.iter().map(move |(to_vid, e)| (from_vid, to_vid, e)))
  }

  pub fn incident_edges(self: &Self, vid: &VId) -> Option<&Vec<(VId, E)>> {
    self.adjacency.get(vid)
  }
}

impl<V, E, VId> AbstractGraph<V, E> for IGraph<V, E, VId>
where
  V: Eq + Hash,
  VId: Eq + Hash,
{
  type VId = VId;

  fn new() -> Self {
    IGraph::new()
  }

  fn has_vertex(self: &Self, vid: &Self::VId) -> bool {
    self.vertices.contains_key(vid)
  }

  fn get_vertex(self: &Self, vid: &Self::VId) -> Option<&V> {
    self.vertices.get(vid)
  }

  fn push_vertex(self: &mut IGraph<V, E, VId>, vid: VId, vertex: V) {
    self.vertices.insert(vid, vertex);
  }

  fn get_edge(self: &Self, from_vid: Self::VId, to_vid: Self::VId) -> Option<&E> {
    self.adjacency.get(&from_vid).and_then(|edges| {
      edges
        .iter()
        .find(|(curr_to_vid, _edge)| *curr_to_vid == to_vid)
        .map(|(_, edge)| edge)
    })
  }

  fn push_edge(self: &mut Self, from: VId, to: VId, edge: E) {
    let adjacent_to_from = self.adjacency.entry(from).or_default();
    adjacent_to_from.push((to, edge));
  }

  fn adjacent(self: &Self, vid: Self::VId) -> Vec<&Self::VId> {
    self.adjacency.get(&vid).unwrap().iter().map(|(vid, _e)| vid).collect()
  }

  fn map_adjacent<F, R>(self: &Self, vid: Self::VId, mut f: F) -> Vec<R>
  where
    F: FnMut(&(Self::VId, E)) -> R,
  {
    let edges = self.adjacency.get(&vid);

    match edges {
      None => vec![],
      Some(edges) => edges.iter().map(|vid_and_e| f(vid_and_e)).collect(),
    }
  }
}

// TODO:
// impl Iterator for IGraph<VID, V, E>

// TODO:
// type VId = u64;
// pub fn get_vid(&self, vertex: &V) -> u64 {
//   let mut state = FnvHasher::default();
//   vertex.hash(&mut state);
//   state.finish()
// }

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn can_create_an_indexed_graph() {
    let mut g: IGraph<(), String, &str> = IGraph::new();
    g.push_vertex("A", ());
    g.push_vertex("B", ());
    g.push_vertex("C", ());

    g.push_edge("A", "B", "A -> B".to_string());
    g.push_edge("B", "C", "B -> C".to_string());
    g.push_edge("C", "A", "C -> A".to_string());
    g.push_edge("A", "A", "A loop".to_string());

    assert_eq!(g.vertices.len(), 3);
    assert_eq!(
      g.adjacency.get("A").unwrap(),
      &[("B", "A -> B".to_string()), ("A", "A loop".to_string())]
    );
    assert_eq!(g.adjacency.get("B").unwrap(), &[("C", "B -> C".to_string())]);
    assert_eq!(g.adjacency.get("C").unwrap(), &[("A", "C -> A".to_string())]);

    assert_eq!(
      g.map_adjacent("A", |x| x.clone()),
      [("B", "A -> B".to_string()), ("A", "A loop".to_string())]
    );
    assert_eq!(g.map_adjacent("B", |x| x.clone()), [("C", "B -> C".to_string())]);
    assert_eq!(g.map_adjacent("C", |x| x.clone()), [("A", "C -> A".to_string())]);

    assert_eq!(g.get_vertex(&"A"), Some(&()));
    assert_eq!(g.get_vertex(&"B"), Some(&()));
    assert_eq!(g.get_vertex(&"Z"), None);
  }
}
