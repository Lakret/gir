mod abstract_graph;
pub use abstract_graph::AbstractGraph;

mod igraph;
pub use igraph::IGraph;

mod vec_graph;
pub use vec_graph::VecGraph;

// TODO:
// - try slotmap in IGraph? https://docs.rs/slotmap/1.0.2/slotmap/
// - use Fnv in VecGraph
