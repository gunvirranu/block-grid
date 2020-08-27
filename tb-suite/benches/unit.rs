extern crate block_grid;
extern crate criterion;

use block_grid::{BlockWidth::*, *};
use criterion::{black_box, criterion_group, criterion_main, Bencher, Criterion};

fn constructors(c: &mut Criterion) {
    const DIMS: Coords = (128, 256);

    fn gen_data_u8(len: usize) -> Vec<u8> {
        (0..len).map(|x| x as u8).collect()
    }

    fn bench_from_row_major<B: BlockDim>() -> impl Fn(&mut Bencher, &Coords) {
        |b, &(rows, cols)| {
            assert!(rows % B::WIDTH == 0 && cols % B::WIDTH == 0);
            let data = gen_data_u8(rows * cols);
            b.iter_with_large_drop(|| BlockGrid::<_, B>::from_row_major(rows, cols, &data))
        }
    }
    let mut g = c.benchmark_group("Constructors");
    g.bench_with_input("from_row_major<U2>", &DIMS, bench_from_row_major::<U2>());
    g.bench_with_input("from_row_major<U8>", &DIMS, bench_from_row_major::<U8>());

    fn bench_from_col_major<B: BlockDim>() -> impl Fn(&mut Bencher, &Coords) {
        |b, &(rows, cols)| {
            let data = gen_data_u8(rows * cols);
            b.iter_with_large_drop(|| BlockGrid::<_, B>::from_col_major(rows, cols, &data))
        }
    }
    g.bench_with_input("from_col_major<U2>", &DIMS, bench_from_col_major::<U2>());
    g.bench_with_input("from_col_major<U8>", &DIMS, bench_from_col_major::<U8>());
    g.finish();
}

fn indexing(c: &mut Criterion) {
    type B = U4;
    const DIMS: Coords = (128, 32);
    let data: Vec<_> = (0..(DIMS.0 * DIMS.1)).map(|x| x as u8).collect();
    let grid = BlockGrid::<_, B>::from_raw_vec(DIMS.0, DIMS.1, data).unwrap();

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
}

fn iterators(c: &mut Criterion) {
    type B = U4;
    const DIMS: Coords = (128, 64);
    let data: Vec<_> = (0..(DIMS.0 * DIMS.1)).map(|x| x as u32).collect();
    let grid = BlockGrid::<_, B>::from_raw_vec(DIMS.0, DIMS.1, data).unwrap();

    let mut g = c.benchmark_group("Iterators");
    g.bench_function("each_iter", |b| {
        b.iter(|| {
            for x in grid.each_iter() {
                black_box(x);
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
}

criterion_group!(bench, constructors, indexing, iterators);
criterion_main!(bench);
