/**
 * Indexed Graphs.
 */
use fnv::{FnvHashMap, FnvHasher};
use std::hash::{Hash, Hasher};

use crate::AbstractGraph;

type VId = u64;

#[derive(Debug)]
pub struct IGraph<V, E> {
  vertices: FnvHashMap<VId, V>,
  adjacency: FnvHashMap<VId, Vec<(VId, E)>>,
}

impl<V, E> IGraph<V, E>
where
  V: Hash,
{
  pub fn get_vid(&self, vertex: &V) -> u64 {
    let mut state = FnvHasher::default();
    vertex.hash(&mut state);
    state.finish()
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

  pub fn iter_complete_edges(&self) -> impl Iterator<Item = (VId, &VId, &E)> {
    self.iter_edges().flat_map(|(from_vid, incident)| {
      let from_vid = *from_vid;
      incident.iter().map(move |(to_vid, e)| (from_vid, to_vid, e))
    })
  }
}

impl<V, E> IGraph<V, E>
where
  V: Hash + Eq + Clone,
  E: Hash + Eq + Clone,
{
  /// Finds spanning tree (no minimality guarantee) for `self`.
  /// Returns it as a graph of references to vertices & edges owned
  /// by the current graph.
  pub fn spanning_tree(&self) -> IGraph<&V, &E> {
    let mut tree = IGraph::new();
    let mut edges_to_consider: Vec<(VId, VId, &E)> = vec![];

    if let Some((vid, v)) = self.vertices.iter().next() {
      tree.push_vertex(v);
      self.extend_with_incident(&mut edges_to_consider, vid);
    }

    while let Some((from_vid, to_vid, edge)) = edges_to_consider.pop() {
      if !tree.contains(&to_vid) {
        if let Some(to) = self.get_vertex(to_vid) {
          tree.push_vertex(to);
          tree.push_edge(from_vid, to_vid, edge);

          self.extend_with_incident(&mut edges_to_consider, &to_vid);
        }
      }
    }

    tree
  }

  fn extend_with_incident<'a, 'b>(&'a self, edges_to_consider: &'b mut Vec<(VId, VId, &'a E)>, vid: &'b VId) {
    if let Some(incident_edges) = self.adjacency.get(vid) {
      edges_to_consider.extend(incident_edges.iter().map(|(to, e)| (*vid, *to, e)));
    }
  }
}

impl<V, E> AbstractGraph<V, E> for IGraph<V, E>
where
  V: Eq + Hash,
{
  type VId = u64;

  fn new() -> Self {
    IGraph::new()
  }

  fn push_vertex(self: &mut IGraph<V, E>, vertex: V) -> Self::VId {
    let vid = self.get_vid(&vertex);
    self.vertices.insert(vid, vertex);
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

  fn get_vertex(self: &Self, vid: Self::VId) -> Option<&V> {
    self.vertices.get(&vid)
  }
}

impl<V, E> IGraph<V, E>
where
  V: Hash,
{
  pub fn new() -> IGraph<V, E> {
    IGraph {
      vertices: FnvHashMap::default(),
      adjacency: FnvHashMap::default(),
    }
  }

  pub fn push_edge_direct(self: &mut Self, from: &V, to: &V, edge: E) {
    let from_vid = self.get_vid(from);
    let to_vid = self.get_vid(to);

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
    let mut g: IGraph<&str, String> = IGraph::new();
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

    assert_eq!(g.get_vertex(g.get_vid(&"A")), Some(&"A"));
    assert_eq!(g.get_vertex(g.get_vid(&"B")), Some(&"B"));
    assert_eq!(g.get_vertex(g.get_vid(&"Z")), None);
  }

  #[test]
  fn spanning_tree_works() {
    let mut g: IGraph<&str, u32> = IGraph::new();

    let a_id = g.push_vertex("A");
    let b_id = g.push_vertex("B");
    let c_id = g.push_vertex("C");
    let d_id = g.push_vertex("D");

    g.push_edge(a_id, b_id, 0);
    g.push_edge(b_id, c_id, 1);
    g.push_edge(c_id, d_id, 2);

    g.push_edge(d_id, a_id, 3);
    g.push_edge(d_id, c_id, 4);
    g.push_edge(c_id, b_id, 5);

    dbg!(&g);
    let tree = g.spanning_tree();
    dbg!(&tree);

    assert_eq!(tree.iter_vertices().collect::<Vec<_>>().len(), 4);

    let tree_edges = tree.iter_complete_edges().collect::<Vec<_>>();
    assert_eq!(tree_edges.len(), 3);

    let mut edges = tree_edges.into_iter().map(|(_, _, edge)| **edge).collect::<Vec<_>>();
    edges.sort();
    assert_eq!(&edges, &[0, 1, 2]);
  }
}
