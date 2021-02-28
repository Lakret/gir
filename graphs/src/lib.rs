mod abstract_graph;
pub use abstract_graph::AbstractGraph;

mod igraph;
pub use igraph::IGraph;

mod vec_graph;
pub use vec_graph::VecGraph;

// TODO:
// - use Fnv in VecGraph
// - IGraph - hashmap with V keys instead of VId?
