extern crate array2d;
extern crate block_grid;
extern crate fastrand;

use std::convert::{From, Into};
use std::marker::Sized;
use std::ops::{Index, IndexMut};

use array2d::Array2D;
use block_grid::{BlockDim, BlockGrid, BlockWidth, Coords};

fn blur_by_index<T>(rows: usize, cols: usize, img: &T, out: &mut T)
where
    T: Index<Coords> + IndexMut<Coords>,
    <T as Index<Coords>>::Output: Copy + Into<u8> + From<u8> + Sized,
{
    assert!(rows >= 3 && cols >= 3);
    // Copy perimeter
    for i in 0..rows {
        out[(i, 0)] = img[(i, 0)];
        out[(i, cols - 1)] = img[(i, cols - 1)];
    }
    for j in 0..cols {
        out[(0, j)] = img[(0, j)];
        out[(rows - 1, j)] = img[(rows - 1, j)];
    }
    // Iterate over each pixel
    for i in 1..(rows - 1) {
        for j in 1..(cols - 1) {
            // Set each pixel to average of 3x3 kernel
            let tot: u32 = [
                (i - 1, j - 1),
                (i - 1, j),
                (i - 1, j + 1),
                (i, j - 1),
                (i, j),
                (i, j + 1),
                (i + 1, j - 1),
                (i + 1, j),
                (i + 1, j + 1),
            ]
            .iter()
            .map(|&(ni, nj)| img[(ni, nj)].into() as u32)
            .sum();
            out[(i, j)] = ((tot / 9) as u8).into();
        }
    }
}

fn generic_test_blur_by_index<B: BlockDim>(rows: usize, cols: usize) {
    let mut in_bg = BlockGrid::<u8, B>::new(rows, cols).unwrap();
    let mut out_bg = in_bg.clone();

    let mut in_ar = Array2D::filled_with(0, rows, cols);
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
    generic_test_blur_by_index::<BlockWidth::U2>(100, 100);
    generic_test_blur_by_index::<BlockWidth::U8>(256, 1024);
    generic_test_blur_by_index::<BlockWidth::U64>(192, 320);
}
