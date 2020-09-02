extern crate array2d;
extern crate block_grid;
extern crate fastrand;
extern crate tb_suite;
extern crate toodee;

use array2d::Array2D;
use block_grid::{BlockDim, BlockGrid, BlockWidth::*};
use tb_suite::blur::*;
use toodee::TooDee;

fn generic_test_blur<B: BlockDim>(rows: usize, cols: usize) {
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

#[test]
fn test_blur_by_index_u2() {
    generic_test_blur::<U2>(30, 30);
}

#[test]
fn test_blur_by_index_u4() {
    generic_test_blur::<U4>(4, 12);
}

#[test]
fn test_blur_by_index_u8() {
    generic_test_blur::<U8>(32, 128);
}

#[test]
fn test_blur_by_index_u16() {
    generic_test_blur::<U16>(16, 16);
}

#[test]
fn test_blur_by_index_u32() {
    generic_test_blur::<U32>(96, 64);
}
