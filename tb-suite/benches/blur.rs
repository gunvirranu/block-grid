extern crate array2d;
extern crate block_grid;
extern crate criterion;
extern crate fastrand;
extern crate tb_suite;
extern crate toodee;

use array2d::Array2D;
use block_grid::{BlockGrid, BlockWidth};
use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use tb_suite::blur::*;
use toodee::TooDee;

type B = BlockWidth::U8;

const ROWS: usize = 256;
const COLS: usize = 128;

fn bench_blur(c: &mut Criterion) {
    let mut in_bg = BlockGrid::<u8, B>::new(ROWS, COLS).unwrap();
    let out_bg = in_bg.clone();

    let mut in_ar = Array2D::filled_with(0u8, ROWS, COLS);
    let out_ar = in_ar.clone();

    let mut in_td = TooDee::<u8>::new(ROWS, COLS);
    let out_td = in_td.clone();

    // Generate input data
    fastrand::seed(1234);
    for i in 0..ROWS {
        for j in 0..COLS {
            let x = fastrand::u8(..);
            in_bg[(i, j)] = x;
            in_ar[(i, j)] = x;
            in_td[(i, j)] = x;
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

    g.bench_function("array2d_index", |b| {
        b.iter_batched_ref(
            || out_ar.clone(),
            |out_grid| {
                blur_by_index(ROWS, COLS, &in_ar, out_grid);
            },
            BatchSize::SmallInput,
        );
    });

    g.bench_function("toodee_index", |b| {
        b.iter_batched_ref(
            || out_td.clone(),
            |out_grid| {
                blur_by_index(ROWS, COLS, &in_td, out_grid);
            },
            BatchSize::SmallInput,
        );
    });
    g.finish()
}

criterion_group!(benches, bench_blur);
criterion_main!(benches);
