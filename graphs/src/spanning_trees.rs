use crate::Graph;
use std::cmp::{Ord, Ordering, PartialOrd, Reverse};
use std::collections::BinaryHeap;
use std::hash::Hash;

// TODO: min spanning tree
struct WeightedEdge<'a, VId, E, W: Ord> {
  edge: (&'a VId, &'a VId, &'a E),
  weight: W,
}

impl<'a, VId, E, W: Ord> Ord for WeightedEdge<'a, VId, E, W> {
  fn cmp(&self, other: &Self) -> Ordering {
    self.weight.cmp(&other.weight)
  }
}

impl<'a, VId, E, W: Ord> PartialOrd for WeightedEdge<'a, VId, E, W> {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.weight.cmp(&other.weight))
  }
}

impl<'a, VId, E, W: Ord> PartialEq for WeightedEdge<'a, VId, E, W> {
  fn eq(&self, other: &Self) -> bool {
    self.weight == other.weight
  }
}

impl<'a, VId, E, W: Ord> Eq for WeightedEdge<'a, VId, E, W> {}

impl<VId, E, V> Graph<VId, E, V>
where
  VId: Eq + Hash + Clone,
  V: Hash + Eq + Clone,
  E: Hash + Eq + Clone,
{
  /// Finds minimum spanning tree for `self`.
  ///
  /// Returns it as a graph of references to vertices & edges owned
  /// by the current graph.
  pub fn minimum_spanning_tree<'a, F, W>(
    &'a self,
    start_vid: &'a VId,
    get_edge_weight: &F,
  ) -> Graph<&'a VId, &'a E, &'a V>
  where
    F: Fn(&E) -> W,
    W: Ord,
  {
    let mut tree = Graph::new();

    // TODO: switch to BinaryHeap; need to figure out how to make it Ord & map to the edges
    let mut edges_to_consider2: BinaryHeap<WeightedEdge<VId, E, W>> = BinaryHeap::new();
    // TODO: kill
    // let mut edges_to_consider: Vec<(&VId, &VId, &E)> = vec![];

    // FIXME: error handling
    let v = self.get_vertex(start_vid).unwrap();
    tree.push_vertex(start_vid, v);
    self.extend_with_incident2(&mut edges_to_consider2, get_edge_weight, start_vid);
    // TODO: kill
    // self.extend_with_incident(&mut edges_to_consider, start_vid);

    while let Some(weighted_edge) = edges_to_consider2.pop() {
      let WeightedEdge {
        edge: (from_vid, to_vid, edge),
        ..
      } = weighted_edge;

      if !tree.has_vertex(&&to_vid) {
        if let Some(to) = self.get_vertex(to_vid) {
          tree.push_vertex(&to_vid, to);
          tree.push_edge(&from_vid, &to_vid, edge);

          self.extend_with_incident2(&mut edges_to_consider2, get_edge_weight, &to_vid);
        }
      }
    }

    tree
  }

  fn extend_with_incident2<'a, 'b, F, W>(
    &'a self,
    edges_to_consider: &'b mut BinaryHeap<WeightedEdge<'a, VId, E, W>>,
    get_edge_weight: F,
    vid: &'a VId,
  ) where
    F: Fn(&E) -> W,
    W: Ord,
  {
    if let Some(incident_edges) = self.incident_edges(vid) {
      for (to, e) in incident_edges.iter() {
        let weighted_edge = WeightedEdge {
          edge: (vid, to, e),
          weight: get_edge_weight(e),
        };

        edges_to_consider.push(weighted_edge);
      }
    }
  }

  /// Finds spanning tree (no minimality guarantee) for `self`.
  ///
  /// Returns it as a graph of references to vertices & edges owned
  /// by the current graph.
  pub fn spanning_tree<'a>(&'a self, start_vid: &'a VId) -> Graph<&'a VId, &'a E, &'a V> {
    let mut tree = Graph::new();
    let mut edges_to_consider: Vec<(&VId, &VId, &E)> = vec![];

    // FIXME: error handling
    let v = self.get_vertex(start_vid).unwrap();
    tree.push_vertex(start_vid, v);
    self.extend_with_incident(&mut edges_to_consider, start_vid);

    while let Some((from_vid, to_vid, edge)) = edges_to_consider.pop() {
      if !tree.has_vertex(&&to_vid) {
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
    let mut g: Graph<&str, u32> = Graph::new();

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
