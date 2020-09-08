extern crate array2d;
extern crate block_grid;
extern crate fastrand;
extern crate tb_suite;
extern crate toodee;

use array2d::Array2D;
use block_grid::{BlockDim, BlockGrid, BlockWidth::*};
use tb_suite::blur::*;
use toodee::TooDee;

fn gen_test_index<B: BlockDim>(rows: usize, cols: usize) {
    let mut in_bg = BlockGrid::<u8, B>::new(rows, cols).unwrap();
    let mut out_bg = in_bg.clone();

    let mut in_ar = Array2D::filled_with(0u8, rows, cols);
    let mut out_ar = in_ar.clone();

    let mut in_td = TooDee::<u8>::new(rows, cols);
    let mut out_td = in_td.clone();

    fastrand::seed(1234);
    for i in 0..rows {
        for j in 0..cols {
            let x = fastrand::u8(..);
            in_bg[(i, j)] = x;
            in_ar[(i, j)] = x;
            in_td[(i, j)] = x;
        }
    }

    blur_by_index(rows, cols, &in_bg, &mut out_bg);
    blur_by_index(rows, cols, &in_ar, &mut out_ar);
    blur_by_index(rows, cols, &in_td, &mut out_td);

    for i in 0..rows {
        for j in 0..cols {
            let x = out_bg[(i, j)];
            assert_eq!(out_ar[(i, j)], x);
            assert_eq!(out_td[(i, j)], x);
        }
    }
}

fn gen_test_idiomatic<B: BlockDim>(rows: usize, cols: usize) {
    let mut in_bg = BlockGrid::<u8, B>::new(rows, cols).unwrap();
    let mut out_index = in_bg.clone();

    let mut in_td = TooDee::<u8>::new(cols, rows);
    let mut out_td = in_td.clone();

    fastrand::seed(1234);
    for i in 0..rows {
        for j in 0..cols {
            let x = fastrand::u8(..);
            in_bg[(i, j)] = x;
            in_td[(j, i)] = x;
        }
    }

    blur_by_index(rows, cols, &in_bg, &mut out_index);
    blur_toodee(&in_td, &mut out_td);

    for i in 0..rows {
        for j in 0..cols {
            let x = out_index[(i, j)];
            assert_eq!(out_td[(j, i)], x);
        }
    }
}

#[test]
fn test_blur_by_index() {
    gen_test_index::<U2>(30, 30);
    gen_test_index::<U8>(16, 40);
    gen_test_index::<U32>(96, 64);
}

#[test]
fn test_blur_idiomatic() {
    gen_test_idiomatic::<U2>(30, 30);
    gen_test_idiomatic::<U8>(16, 40);
    gen_test_idiomatic::<U32>(96, 64);
}
