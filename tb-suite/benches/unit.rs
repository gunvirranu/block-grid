extern crate block_grid;
extern crate criterion;

use block_grid::{BlockWidth::*, *};
use criterion::{criterion_group, criterion_main, Bencher, Criterion};

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

criterion_group!(bench, constructors);
criterion_main!(bench);
