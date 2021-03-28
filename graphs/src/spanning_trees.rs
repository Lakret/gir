use crate::Graph;
use std::cmp::{Ord, Ordering, PartialOrd, Reverse};
use std::collections::BinaryHeap;
use std::hash::Hash;

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
  /// Finds minimum spanning tree (MST) for `self`, starting at vertex with `start_vid`,
  /// and using `get_edge_weight` to find the weight of the edges.
  ///
  /// Uses Prim's algorithm with a binary heap to store candidate weighted edges.
  ///
  /// Returns the MST as a graph of references to vertices & edges owned by `self`.
  pub fn minimum_spanning_tree<'a, 'b, F, W>(
    &'a self,
    start_vid: &'a VId,
    get_edge_weight: &'b F,
  ) -> Option<Graph<&'a VId, &'a E, &'a V>>
  where
    F: Fn(&E) -> W,
    W: Ord,
  {
    let mut tree = Graph::new();
    let mut edges_to_consider: BinaryHeap<WeightedEdge<VId, E, Reverse<W>>> = BinaryHeap::new();

    self.get_vertex(start_vid).map(|v| {
      tree.push_vertex(start_vid, v);
      self.extend_with_incident(&mut edges_to_consider, get_edge_weight, start_vid);

      while let Some(weighted_edge) = edges_to_consider.pop() {
        let WeightedEdge {
          edge: (from_vid, to_vid, edge),
          ..
        } = weighted_edge;

        if !tree.has_vertex(&&to_vid) {
          if let Some(to) = self.get_vertex(to_vid) {
            tree.push_vertex(&to_vid, to);
            tree.push_edge(&from_vid, &to_vid, edge);

            self.extend_with_incident(&mut edges_to_consider, get_edge_weight, &to_vid);
          }
        }
      }

      tree
    })
  }

  fn extend_with_incident<'a, 'b, F, W>(
    &'a self,
    edges_to_consider: &'b mut BinaryHeap<WeightedEdge<'a, VId, E, Reverse<W>>>,
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
          weight: Reverse(get_edge_weight(e)),
        };

        edges_to_consider.push(weighted_edge);
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

    g.push_edge("A", "B", 4);
    g.push_edge("B", "C", 3);
    g.push_edge("C", "D", 2);
    g.push_edge("D", "A", 5);

    g.push_edge("A", "C", 1);
    g.push_edge("C", "B", 3);

    let tree = g.minimum_spanning_tree(&"A", &(|w| *w)).unwrap();

    // all vertices should be in the minimum spanning tree
    assert_eq!(tree.iter_vertices().collect::<Vec<_>>().len(), 4);

    // the expected minimum spanning tree should be found
    let mut edges = tree
      .iter_complete_edges()
      .map(|(from, to, edge)| (**from, **to, **edge))
      .collect::<Vec<_>>();
    edges.sort();
    assert_eq!(&edges, &[("A", "C", 1), ("C", "B", 3), ("C", "D", 2)]);
  }
}
