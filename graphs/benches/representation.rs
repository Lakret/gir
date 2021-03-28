#![allow(dead_code)]

extern crate criterion;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

use graphs::{AbstractGraph, IGraph};

fn push_vertices_igraph(n: u64) -> IGraph<(), u64> {
  let mut g = IGraph::new();
  for i in 0..n {
    let _vid = g.push_vertex(i, ());
  }

  g
}

fn make_sequence_igraph(n: u64) -> IGraph<(), u64> {
  let mut g = IGraph::new();

  let mut prev_vid = None;
  for i in 0..n {
    g.push_vertex(i, ());

    if let Some(prev_vid) = prev_vid {
      g.push_edge(prev_vid, i, i);
    }

    prev_vid = Some(i);
  }

  g
}

fn make_complete_igraph(n: u64) -> IGraph<(), u64> {
  let mut g = IGraph::new();

  for i in 0..n {
    g.push_vertex(i, ());
  }

  let mut i = 0;
  for v1 in 0..n {
    for v2 in 0..n {
      if v1 != v2 {
        g.push_edge(v1, v2, i);
        i += 1;
      }
    }
  }

  g
}

fn push_vertices(c: &mut Criterion) {
  c.bench_function("igraph (push_vertices)", |b| {
    b.iter(|| push_vertices_igraph(black_box(1_000)))
  });
}

fn make_sequence(c: &mut Criterion) {
  c.bench_function("igraph (make_sequence)", |b| {
    b.iter(|| make_sequence_igraph(black_box(1_000)))
  });
}

fn make_complete(c: &mut Criterion) {
  c.bench_function("igraph (make_complete)", |b| {
    b.iter(|| make_complete_igraph(black_box(1_000)))
  });
}

criterion_group! {
  name = benches;
  config = Criterion::default().sample_size(10);
  targets = push_vertices, make_sequence, make_complete
}

// criterion_group!(benches, push_vertices, make_sequence, make_complete);
criterion_main!(benches);
