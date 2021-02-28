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

fn make_sequence_vec_graph(n: u64) -> VecGraph<u64, u64> {
  let mut g = VecGraph::new();

  let mut prev_vid = None;
  for i in 0..n {
    let vid = g.push_vertex(i);

    if let Some(prev_vid) = prev_vid {
      g.push_edge(prev_vid, vid, i);
    }

    prev_vid = Some(vid);
  }

  g
}

fn make_sequence_igraph(n: u64) -> IGraph<u64, u64, u64> {
  let mut g = IGraph::new();

  let mut prev_vid = None;
  for i in 0..n {
    let vid = g.push_vertex(i);

    if let Some(prev_vid) = prev_vid {
      g.push_edge(prev_vid, vid, i);
    }

    prev_vid = Some(vid);
  }

  g
}

fn make_complete_vec_graph(n: u64) -> VecGraph<u64, u64> {
  todo!()
}

fn make_complete_igraph(n: u64) -> IGraph<u64, u64, u64> {
  todo!()
}

fn push_vertices(c: &mut Criterion) {
  c.bench_function("igraph (push_vertices)", |b| {
    b.iter(|| push_vertices_igraph(black_box(1_000)))
  });

  c.bench_function("vec_graph (push_vertices)", |b| {
    b.iter(|| push_vertices_vec_graph(black_box(1_000)))
  });
}

fn make_sequence(c: &mut Criterion) {
  c.bench_function("igraph (make_sequence)", |b| {
    b.iter(|| make_sequence_igraph(black_box(1_000)))
  });

  c.bench_function("vec_graph (make_sequence)", |b| {
    b.iter(|| make_sequence_vec_graph(black_box(1_000)))
  });
}

criterion_group!(benches, make_sequence);
// criterion_group!(benches, push_vertices, make_sequence);
criterion_main!(benches);
