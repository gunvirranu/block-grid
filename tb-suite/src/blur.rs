extern crate array2d;
extern crate block_grid;

use std::ops::{Index, IndexMut};

use array2d::Array2D;

/// New pixel is average of 3x3 kernel
fn get_new_pix<G>(img: &G, (i, j): (usize, usize)) -> u8
where
    G: Index<(usize, usize), Output = u8>,
{
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
    .map(|&(ni, nj)| img[(ni, nj)] as u32)
    .sum();
    (tot / 9) as u8
}

pub fn blur_by_index<G>(rows: usize, cols: usize, img: &G, out: &mut G)
where
    G: Index<(usize, usize), Output = u8> + IndexMut<(usize, usize)>,
{
    debug_assert!(rows >= 3 && cols >= 3);

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
            out[(i, j)] = get_new_pix(img, (i, j));
        }
    }
}

pub fn blur_array2d(img: &Array2D<u8>, out: &mut Array2D<u8>) {
    debug_assert_eq!(img.num_rows(), out.num_rows());
    debug_assert_eq!(img.num_columns(), out.num_columns());
    let (rows, cols) = (img.num_rows(), img.num_columns());
    debug_assert!(rows >= 3 && cols >= 3);

    // Iterate over each pixel
    for (i, row) in img.rows_iter().enumerate() {
        for (j, &x) in row.enumerate() {
            if i == 0 || j == 0 || i == rows - 1 || j == cols - 1 {
                // Copy perimeter
                out[(i, j)] = x;
            } else {
                out[(i, j)] = get_new_pix(img, (i, j));
            }
        }
    }
}
