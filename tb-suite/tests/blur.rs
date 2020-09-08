extern crate array2d;
extern crate block_grid;
extern crate fastrand;
extern crate tb_suite;

use array2d::Array2D;
use block_grid::{BlockDim, BlockGrid, BlockWidth::*};
use tb_suite::blur::*;

fn gen_test_index<B: BlockDim>(rows: usize, cols: usize) {
    let mut in_bg = BlockGrid::<u8, B>::new(rows, cols).unwrap();
    let mut out_bg = in_bg.clone();

    let mut in_ar = Array2D::filled_with(0u8, rows, cols);
    let mut out_ar = in_ar.clone();

    fastrand::seed(1234);
    for i in 0..rows {
        for j in 0..cols {
            let x = fastrand::u8(..);
            in_bg[(i, j)] = x;
            in_ar[(i, j)] = x;
        }
    }

    blur_by_index(rows, cols, &in_bg, &mut out_bg);
    blur_by_index(rows, cols, &in_ar, &mut out_ar);

    for i in 0..rows {
        for j in 0..cols {
            assert_eq!(out_bg[(i, j)], out_ar[(i, j)]);
        }
    }
}

#[test]
fn test_blur_by_index() {
    gen_test_index::<U2>(30, 30);
    gen_test_index::<U8>(16, 40);
    gen_test_index::<U32>(96, 64);
}
