// Known approaches:
//
//  1) `Rc` (https://github.com/nrc/r4cppp/blob/master/graphs/README.md)
//      Cons: Need to be careful with cycles, Rc's are leaked into userspace
//  2) `Arena`, `&`, and `UnsafeCell` (same source)
//      Cons: `unsafe`, additional fun with mutability if it's desired
//  3) Vector indices as keys for vertices
//  (http://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices/)
//  also used in https://docs.rs/petgraph/0.5.1/petgraph/
//  Cons: doesn't allow deletion, need to pass those indexes to use the API, cannot easily recover them
//  4) Indexed graphs - use HashMap(s) to save vertices and possibly edges.
//  Cons: additional memory and slowdown due to hashing. Still have indices in the API, but they are recoverable.
//  Pros: easiest to implement. Supports deletion. Since indices can be restored, doesn't require much thinking about them.

pub trait AbstractGraph<V, E> {
  type VId;

  fn new() -> Self;
  fn push_vertex(self: &mut Self, vertex: V) -> Self::VId;
  fn push_edge(self: &mut Self, from: Self::VId, to: Self::VId, edge: E);

  fn get_vertex(self: &Self, vid: Self::VId) -> Option<&V>;

  fn adjacent<'a>(self: &Self, vid: Self::VId) -> Vec<Self::VId>;
  fn map_adjacent<F, R>(self: &Self, vid: Self::VId, f: F) -> Vec<R>
  where
    F: Fn(&(Self::VId, E)) -> R;
}
