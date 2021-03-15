use crate::AbstractGraph;
/**
 * Vector indexed Graphs.
 */
use fnv::FnvHashMap;

// Note: currently this cannot support deletion of vertices,
// since it will shift their positions in the vector.
#[derive(Debug, Clone)]
pub struct VecGraph<V, E> {
  vertices: Vec<V>,
  adjacency: FnvHashMap<usize, Vec<(usize, E)>>,
}

impl<V, E> VecGraph<V, E> {
  pub fn new() -> Self {
    VecGraph {
      vertices: vec![],
      adjacency: FnvHashMap::default(),
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

  fn map_adjacent<F, R>(self: &VecGraph<V, E>, vid: Self::VId, mut f: F) -> Vec<R>
  where
    F: FnMut(&(Self::VId, E)) -> R,
  {
    let edges = self.adjacency.get(&vid);

    match edges {
      None => vec![],
      Some(edges) => edges.iter().map(|vid_and_e| f(vid_and_e)).collect(),
    }
  }

  fn get_vertex(self: &Self, vid: Self::VId) -> Option<&V> {
    Some(&self.vertices[vid])
  }

  fn get_edge(self: &Self, from_vid: Self::VId, to_vid: Self::VId) -> Option<&E> {
    self.adjacency.get(&from_vid).and_then(|edges| {
      edges
        .iter()
        .find(|(curr_to_vid, _edge)| *curr_to_vid == to_vid)
        .map(|(_, edge)| edge)
    })
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
}
