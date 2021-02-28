use criterion::{black_box, criterion_group, criterion_main, Criterion};

use graphs::{AbstractGraph, IGraph, VecGraph};

fn push_vertices_vec_graph(n: u64) -> VecGraph<u64, u64> {
  let mut g = VecGraph::new();
  for i in 0..n {
    let _vid = g.push_vertex(i);
  }
  g
}

fn push_vertices_igraph(n: u64) -> IGraph<u64, u64, u64> {
  let mut g = IGraph::new();
  for i in 0..n {
    let _vid = g.push_vertex(i);
  }
  g
}

fn push_vertices(c: &mut Criterion) {
  c.bench_function("igraph (push_vertices)", |b| {
    b.iter(|| push_vertices_igraph(black_box(1_000)))
  });

  c.bench_function("vec_graph (push_vertices)", |b| {
    b.iter(|| push_vertices_vec_graph(black_box(1_000)))
  });
}

criterion_group!(benches, push_vertices);
criterion_main!(benches);
