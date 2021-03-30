use fnv::FnvHashMap;
use std::hash::Hash;

/// A `HashMap`-based explicitly indexed Graph representation.
///
/// Owns vertex and edge data, and exposes explicit vertex id type parameter `VId`.
///
/// ## Type Parameters Order & Identity-Only Graphs
///
/// It's often easier to represent simple graphs that don't have any specific
/// info associated with each vertex, except for that vertex's identity
/// as `IGraph<VId, E, V = ()>`. This is similar to how `HashSet` is defined:
/// it also uses `HashMap<Key, Value = ()>` internally.
///
/// This is why `IGraph<VId, E, V>` type params are in this order.
/// Graphs that only represent association between identities are the most common;
/// sometimes we want to add info about the associations themselves;
/// and the most complex case if when we also have info associated with vertices.
///
/// If your `VId` type is `Copy` or a reference, you can get useability similar to
/// the famous [Python graph representation via dicts](https://www.python.org/doc/essays/graphs/).
///
/// ## Alternative Approaches
///
/// There are several known approaches for modelling graphs in Rust.
///
/// - Explicitly indexed graphs (this implementation) - uses `HashMap`s to save vertices and possibly edges.
///
///   **Pros:** Simplest APIs for graphs where `VId`s can fully represent vertices. Supports deletion.<br/>
///   **Cons:** additional memory and slowdown due to hashing.
/// - [`Rc`](https://github.com/nrc/r4cppp/blob/master/graphs/README.md#rcrefcellnode)-based.
///
///   **Pros:** Easy mutability, flexible since `Rc`s to vertices can be used outside of the graph.<br/>
///   **Cons:** `Rc`'s need to be handled by the users, ugly API. Need to be careful with cycles,
///   Slower than explicit references, but faster than indexed graphs.
/// - [`Arena`, `&`, and `UnsafeCell`](https://github.com/nrc/r4cppp/blob/master/graphs/README.md#node-and-unsafecell).
///
///   **Pros:** can support nice API. Efficient.<br/>
///   **Cons:** `unsafe`, complex to support mutability, needs an arena allocator,
///   which should be managed separately too.
/// - [Vector indices as `VId`](http://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices/)
///   also used in [petgraph](https://docs.rs/petgraph/0.5.1/petgraph/).
///
///   **Pros:** simpler version of indexed graphs.<br/>
///   **Cons:** doesn't allow deletion, need to pass those indexes to use the API,
///   essentially a more limited version of indexed graphs.
#[derive(Debug)]
pub struct Graph<VId, E = (), V = ()> {
  pub(crate) vertices: FnvHashMap<VId, V>,
  pub(crate) adjacency: FnvHashMap<VId, Vec<(VId, E)>>,
}

impl<VId, E, V> Graph<VId, E, V>
where
  VId: Eq + Hash,
  V: Hash,
{
  pub fn new() -> Graph<VId, E, V> {
    Graph {
      vertices: FnvHashMap::default(),
      adjacency: FnvHashMap::default(),
    }
  }

  pub fn push_vertex(self: &mut Graph<VId, E, V>, vid: VId, vertex: V) {
    self.vertices.insert(vid, vertex);
  }

  pub fn push_edge(self: &mut Self, from: VId, to: VId, edge: E) {
    let adjacent_to_from = self.adjacency.entry(from).or_default();
    adjacent_to_from.push((to, edge));
  }

  pub fn has_vertex(&self, vid: &VId) -> bool {
    self.vertices.contains_key(vid)
  }

  pub fn get_vertex(self: &Self, vid: &VId) -> Option<&V> {
    self.vertices.get(vid)
  }

  pub fn iter_vertices(&self) -> impl Iterator<Item = (&VId, &V)> {
    self.vertices.iter()
  }

  pub fn get_edge(self: &Self, from_vid: VId, to_vid: VId) -> Option<&E> {
    self.adjacency.get(&from_vid).and_then(|edges| {
      edges
        .iter()
        .find(|(curr_to_vid, _edge)| *curr_to_vid == to_vid)
        .map(|(_, edge)| edge)
    })
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

  pub fn adjacent(self: &Self, vid: VId) -> Vec<&VId> {
    self.adjacency.get(&vid).unwrap().iter().map(|(vid, _e)| vid).collect()
  }

  pub fn map_adjacent<F, R>(self: &Self, vid: VId, mut f: F) -> Vec<R>
  where
    F: FnMut(&(VId, E)) -> R,
  {
    let edges = self.adjacency.get(&vid);

    match edges {
      None => vec![],
      Some(edges) => edges.iter().map(|vid_and_e| f(vid_and_e)).collect(),
    }
  }
}

impl<VId, E> Graph<VId, E, ()>
where
  VId: Eq + Hash,
{
  pub fn push_vid(self: &mut Self, vid: VId) {
    self.vertices.insert(vid, ());
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn can_create_an_indexed_graph() {
    let mut g: Graph<&str, String> = Graph::new();
    g.push_vid("A");
    g.push_vid("B");
    // let's verify that this also works
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
