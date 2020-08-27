extern crate block_grid;
extern crate criterion;

use criterion::{criterion_group, criterion_main, Criterion};

// TODO: Add tests

fn empty(_c: &mut Criterion) {}

criterion_group!(bench, empty);
criterion_main!(bench);
