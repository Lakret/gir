use crate::{AbstractGraph, IGraph};
use std::hash::Hash;

impl<VId, E, V> IGraph<VId, E, V>
where
  VId: Eq + Hash + Clone,
  V: Hash + Eq + Clone,
  E: Hash + Eq + Clone,
{
  // TODO: min spanning tree

  /// Finds spanning tree (no minimality guarantee) for `self`.
  /// Returns it as a graph of references to vertices & edges owned
  /// by the current graph.
  pub fn spanning_tree<'a>(&'a self, start_vid: &'a VId) -> IGraph<&'a VId, &'a E, &'a V> {
    let mut tree = IGraph::new();
    let mut edges_to_consider: Vec<(&VId, &VId, &E)> = vec![];

    // FIXME: error handling
    let v = self.get_vertex(start_vid).unwrap();
    tree.push_vertex(start_vid, v);
    self.extend_with_incident(&mut edges_to_consider, start_vid);

    while let Some((from_vid, to_vid, edge)) = edges_to_consider.pop() {
      if !tree.contains(&&to_vid) {
        if let Some(to) = self.get_vertex(to_vid) {
          tree.push_vertex(&to_vid, to);
          tree.push_edge(&from_vid, &to_vid, edge);

          self.extend_with_incident(&mut edges_to_consider, &to_vid);
        }
      }
    }

    tree
  }

  fn extend_with_incident<'a, 'b>(&'a self, edges_to_consider: &'b mut Vec<(&'a VId, &'a VId, &'a E)>, vid: &'a VId) {
    if let Some(incident_edges) = self.incident_edges(vid) {
      for (to, e) in incident_edges.iter() {
        edges_to_consider.push((vid, to, e))
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn spanning_tree_works() {
    let mut g: IGraph<&str, u32> = IGraph::new();

    g.push_vertex("A", ());
    g.push_vertex("B", ());
    g.push_vertex("C", ());
    g.push_vertex("D", ());

    g.push_edge("A", "B", 0);
    g.push_edge("B", "C", 1);
    g.push_edge("C", "D", 2);

    g.push_edge("D", "A", 3);
    g.push_edge("D", "C", 4);
    g.push_edge("C", "B", 5);

    dbg!(&g);
    let tree = g.spanning_tree(&"A");
    dbg!(&tree);

    assert_eq!(tree.iter_vertices().collect::<Vec<_>>().len(), 4);

    let tree_edges = tree.iter_complete_edges().collect::<Vec<_>>();
    assert_eq!(tree_edges.len(), 3);

    let mut edges = tree_edges.into_iter().map(|(_, _, edge)| **edge).collect::<Vec<_>>();
    edges.sort();
    assert_eq!(&edges, &[0, 1, 2]);
  }
}
