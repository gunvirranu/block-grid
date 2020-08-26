extern crate block_grid;

use std::ops::{Index, IndexMut};

pub fn blur_by_index<T>(rows: usize, cols: usize, img: &T, out: &mut T)
where
    T: Index<(usize, usize), Output = u8> + IndexMut<(usize, usize)>,
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
            .map(|&(ni, nj)| img[(ni, nj)] as u32)
            .sum();
            out[(i, j)] = (tot / 9) as u8;
        }
    }
}
