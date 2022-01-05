extern crate array2d;
extern crate block_grid;
extern crate criterion;
extern crate fastrand;
extern crate tb_suite;

use array2d::Array2D;
use block_grid::{BlockGrid, U1, U8};
use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use tb_suite::blur::*;

type B = U8;

const ROWS: usize = 256;
const COLS: usize = 128;

fn bench_blur(c: &mut Criterion) {
    let mut in_bg = BlockGrid::<u8, B>::new(ROWS, COLS).unwrap();
    let out_bg = in_bg.clone();

    let mut in_bg_u1 = BlockGrid::<u8, U1>::new(ROWS, COLS).unwrap();
    let out_bg_u1 = in_bg_u1.clone();

    let mut in_ar = Array2D::filled_with(0u8, ROWS, COLS);
    let out_ar = in_ar.clone();

    // Generate input data
    fastrand::seed(1234);
    for i in 0..ROWS {
        for j in 0..COLS {
            let x = fastrand::u8(..);
            in_bg[(i, j)] = x;
            in_bg_u1[(i, j)] = x;
            in_ar[(i, j)] = x;
        }
    }

    let mut g = c.benchmark_group("Blur");
    g.bench_function("block_grid_index", |b| {
        b.iter_batched_ref(
            || out_bg.clone(),
            |out_grid| {
                blur_by_index(ROWS, COLS, &in_bg, out_grid);
            },
            BatchSize::SmallInput,
        );
    });

    g.bench_function("block_grid_idiomatic", |b| {
        b.iter_batched_ref(
            || out_bg.clone(),
            |out_grid| {
                blur_blockgrid(&in_bg, out_grid);
            },
            BatchSize::SmallInput,
        );
    });

    g.bench_function("block_grid_u1_index", |b| {
        b.iter_batched_ref(
            || out_bg_u1.clone(),
            |out_grid| {
                blur_by_index(ROWS, COLS, &in_bg_u1, out_grid);
            },
            BatchSize::SmallInput,
        );
    });

    g.bench_function("block_grid_u1_idiomatic", |b| {
        b.iter_batched_ref(
            || out_bg_u1.clone(),
            |out_grid| {
                blur_blockgrid(&in_bg_u1, out_grid);
            },
            BatchSize::SmallInput,
        );
    });

    g.bench_function("array2d_index", |b| {
        b.iter_batched_ref(
            || out_ar.clone(),
            |out_grid| {
                blur_by_index(ROWS, COLS, &in_ar, out_grid);
            },
            BatchSize::SmallInput,
        );
    });

    g.bench_function("array2d_idiomatic", |b| {
        b.iter_batched_ref(
            || out_ar.clone(),
            |out_grid| {
                blur_array2d(&in_ar, out_grid);
            },
            BatchSize::SmallInput,
        );
    });
    g.finish()
}

criterion_group!(benches, bench_blur);
criterion_main!(benches);
