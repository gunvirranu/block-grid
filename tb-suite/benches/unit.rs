extern crate block_grid;
extern crate criterion;

use block_grid::*;
use criterion::{black_box, criterion_group, criterion_main, Bencher, Criterion};

type T = u8;
type B = BlockWidth::U8;

const ROWS: usize = 128;
const COLS: usize = 256;

fn gen_data(len: usize) -> Vec<T> {
    (0..len).map(|x| x as T).collect()
}

fn constructors(c: &mut Criterion) {
    // NOTE: These will NOT fail if the constructor gives an `Err`, so make sure
    //       that they return a valid value. Perf. will show a huge increase.
    fn bench_from_row_major() -> impl Fn(&mut Bencher, &Coords) {
        |b, &(rows, cols)| {
            let data = gen_data(rows * cols);
            b.iter_with_large_drop(|| BlockGrid::<_, B>::from_row_major(rows, cols, &data))
        }
    }
    let mut g = c.benchmark_group("Constructors");
    g.bench_with_input("from_row_major", &(ROWS, COLS), bench_from_row_major());

    fn bench_from_col_major() -> impl Fn(&mut Bencher, &Coords) {
        |b, &(rows, cols)| {
            let data = gen_data(rows * cols);
            b.iter_with_large_drop(|| BlockGrid::<_, B>::from_col_major(rows, cols, &data))
        }
    }
    g.bench_with_input("from_col_major", &(ROWS, COLS), bench_from_col_major());
    g.finish();
}

fn indexing(c: &mut Criterion) {
    let data: Vec<_> = gen_data(ROWS * COLS);
    let grid = BlockGrid::<_, B>::from_raw_vec(ROWS, COLS, data).unwrap();

    let mut g = c.benchmark_group("Indexing");
    g.bench_function("index_row_major", |b| {
        b.iter(|| {
            for i in 0..grid.rows() {
                for j in 0..grid.cols() {
                    black_box(grid[(i, j)]);
                }
            }
        })
    });

    g.bench_function("index_mem_order", |b| {
        b.iter(|| {
            for bi in 0..grid.row_blocks() {
                for bj in 0..grid.col_blocks() {
                    for si in 0..B::WIDTH {
                        for sj in 0..B::WIDTH {
                            let (i, j) = (B::WIDTH * bi + si, B::WIDTH * bj + sj);
                            black_box(grid[(i, j)]);
                        }
                    }
                }
            }
        })
    });

    g.bench_function("get_unchecked_mem_order", |b| {
        b.iter(|| {
            for bi in 0..grid.row_blocks() {
                for bj in 0..grid.col_blocks() {
                    for si in 0..B::WIDTH {
                        for sj in 0..B::WIDTH {
                            let (i, j) = (B::WIDTH * bi + si, B::WIDTH * bj + sj);
                            black_box(unsafe { grid.get_unchecked((i, j)) });
                        }
                    }
                }
            }
        })
    });
    g.finish();
}

fn iterators(c: &mut Criterion) {
    let data: Vec<_> = gen_data(ROWS * COLS);
    let grid = BlockGrid::<_, B>::from_raw_vec(ROWS, COLS, data).unwrap();

    let mut g = c.benchmark_group("Iterators");
    g.bench_function("each_iter_no_coords", |b| {
        b.iter(|| {
            for (_, x) in grid.each_iter() {
                black_box(x);
            }
        })
    });

    g.bench_function("each_iter", |b| {
        b.iter(|| {
            for (c, x) in grid.each_iter() {
                black_box((c, x));
            }
        })
    });

    g.bench_function("block_iter", |b| {
        b.iter(|| {
            for block in grid.block_iter() {
                black_box(block);
            }
        })
    });

    g.bench_function("block_iter_index", |b| {
        b.iter(|| {
            for block in grid.block_iter() {
                for i in 0..B::WIDTH {
                    for j in 0..B::WIDTH {
                        black_box(block[(i, j)]);
                    }
                }
            }
        })
    });

    g.bench_function("block_iter_get_unchecked", |b| {
        b.iter(|| {
            for block in grid.block_iter() {
                for i in 0..B::WIDTH {
                    for j in 0..B::WIDTH {
                        black_box(unsafe { block.get_unchecked((i, j)) });
                    }
                }
            }
        })
    });

    g.bench_function("row_major_iter", |b| {
        b.iter(|| {
            for (c, x) in grid.row_major_iter() {
                black_box((c, x));
            }
        })
    });
    g.finish();
}

criterion_group!(bench, constructors, indexing, iterators);
criterion_main!(bench);
