use crate::iters::BlockIter;
use crate::{BlockDim, Coords};

use std::marker::PhantomData;
use std::ops::{Index, IndexMut};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct BlockGrid<T, B: BlockDim> {
    rows: usize,
    cols: usize,
    pub(crate) buf: Vec<T>,
    _phantom: PhantomData<B>,
}

impl<T: Clone, B: BlockDim> BlockGrid<T, B> {
    pub fn filled(rows: usize, cols: usize, elem: T) -> Result<Self, ()> {
        if !Self::valid_size(rows, cols) {
            return Err(());
        }
        Ok(Self {
            rows,
            cols,
            buf: vec![elem; rows * cols],
            _phantom: PhantomData,
        })
    }
}

impl<T: Clone + Default, B: BlockDim> BlockGrid<T, B> {
    pub fn new(rows: usize, cols: usize) -> Result<Self, ()> {
        Self::filled(rows, cols, T::default())
    }
}

impl<T, B: BlockDim> BlockGrid<T, B> {
    pub fn from_raw_vec(rows: usize, cols: usize, elems: Vec<T>) -> Result<Self, ()> {
        if !Self::valid_size(rows, cols) || rows * cols != elems.len() {
            return Err(());
        }
        Ok(Self {
            rows,
            cols,
            buf: elems,
            _phantom: PhantomData,
        })
    }

    pub fn take_raw_vec(self) -> Vec<T> {
        self.buf
    }

    // TODO: Impl row-major constructor
    // TODO: Impl col-major constructor

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn size(&self) -> usize {
        self.rows() * self.cols()
    }

    pub fn row_blocks(&self) -> usize {
        self.rows >> B::SHIFT
    }

    pub fn col_blocks(&self) -> usize {
        self.cols >> B::SHIFT
    }

    pub fn blocks(&self) -> usize {
        self.row_blocks() * self.col_blocks()
    }

    pub fn contains(&self, (row, cols): Coords) -> bool {
        row < self.rows && cols < self.cols
    }

    pub fn get(&self, coords: Coords) -> Option<&T> {
        if !self.contains(coords) {
            return None;
        }
        self.buf.get(self.calc_index(coords))
    }

    pub fn get_mut(&mut self, coords: Coords) -> Option<&mut T> {
        if !self.contains(coords) {
            return None;
        }
        let ind = self.calc_index(coords);
        self.buf.get_mut(ind)
    }

    // TODO: Document unsafety
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn get_unchecked(&self, coords: Coords) -> &T {
        let ind = self.calc_index(coords);
        self.buf.get_unchecked(ind)
    }

    // TODO: Document unsafety
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn get_unchecked_mut(&mut self, coords: Coords) -> &mut T {
        let ind = self.calc_index(coords);
        self.buf.get_unchecked_mut(ind)
    }

    pub fn each_iter(&self) -> impl Iterator<Item = &T> + ExactSizeIterator {
        self.buf.iter()
    }

    pub fn each_iter_mut(&mut self) -> impl Iterator<Item = &mut T> + ExactSizeIterator {
        self.buf.iter_mut()
    }

    // TODO: More iterators

    pub fn block_iter(&self) -> BlockIter<T, B> {
        BlockIter {
            cur_block: 0,
            max_blocks: self.blocks(),
            grid: self,
        }
    }

    fn valid_size(rows: usize, cols: usize) -> bool {
        rows != 0 && cols != 0 && rows % B::WIDTH == 0 && cols % B::WIDTH == 0
    }

    fn calc_index(&self, coords: Coords) -> usize {
        // Get block
        let block_coords = self.calc_block(coords);
        let block_ind = self.calc_block_index(block_coords);
        // Offset within block
        let sub_coords = self.calc_offset(coords);
        let sub_ind = self.calc_sub_index(sub_coords);
        block_ind + sub_ind
    }

    fn calc_block_index(&self, (b_row, b_col): Coords) -> usize {
        B::AREA * (self.col_blocks() * b_row + b_col)
    }

    pub(crate) fn calc_sub_index(&self, (s_row, s_col): Coords) -> usize {
        B::WIDTH * s_row + s_col
    }

    fn calc_block(&self, (row, col): Coords) -> Coords {
        (row >> B::SHIFT, col >> B::SHIFT)
    }

    fn calc_offset(&self, (row, col): Coords) -> Coords {
        (row & B::MASK, col & B::MASK)
    }
}

impl<T, B: BlockDim> Index<Coords> for BlockGrid<T, B> {
    type Output = T;

    fn index(&self, coords: Coords) -> &Self::Output {
        // TODO: Benchmark against unchecked
        match self.get(coords) {
            Some(x) => x,
            None => panic!("Index out of bounds"),
        }
    }
}

impl<T, B: BlockDim> IndexMut<Coords> for BlockGrid<T, B> {
    fn index_mut(&mut self, coords: Coords) -> &mut Self::Output {
        match self.get_mut(coords) {
            Some(x) => x,
            None => panic!("Index out of bounds"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BlockWidth;

    type T = usize;
    // TODO: Look into testing generically in a not-ugly way
    type B = BlockWidth::U2;
    type BGrid = BlockGrid<T, B>;

    const GOOD_SIZES: &[Coords] = &[(2, 2), (4, 6), (100, 100), (64, 256)];
    const BAD_SIZES: &[Coords] = &[(0, 0), (0, 1), (3, 5), (7, 13)];

    #[test]
    fn test_from_raw_vec() {
        for &(rows, cols) in GOOD_SIZES {
            let data: Vec<T> = (0..(rows * cols)).collect();
            let grid = BGrid::from_raw_vec(rows, cols, data.clone()).unwrap();
            assert_eq!(data.len(), grid.size());
            for (&x, &y) in grid.each_iter().zip(data.iter()) {
                assert_eq!(x, y);
            }
        }
    }

    #[test]
    fn test_from_raw_vec_invalid() {
        for &(rows, cols) in &[(2, 2), (4, 6), (2048, 8192)] {
            let data: Vec<T> = (0..(rows * cols)).collect();
            let grid = BGrid::from_raw_vec(rows + 1, cols + 1, data);
            assert!(grid.is_err());
        }

        for &(rows, cols) in BAD_SIZES {
            let data: Vec<T> = (0..(rows * cols)).collect();
            let grid = BGrid::from_raw_vec(rows, cols, data);
            assert!(grid.is_err());
        }
    }

    #[test]
    fn test_filled() {
        for &(rows, cols) in GOOD_SIZES {
            let grid = BGrid::filled(rows, cols, 7).unwrap();
            assert_eq!(grid.size(), rows * cols);
            for &x in grid.each_iter() {
                assert_eq!(x, 7);
            }
        }
    }

    #[test]
    fn test_filled_invalid() {
        for &(rows, cols) in BAD_SIZES {
            let grid = BGrid::filled(rows, cols, 7);
            assert!(grid.is_err());
        }
    }

    #[test]
    fn test_row_col_size() {
        for &(rows, cols) in GOOD_SIZES {
            let grid = BGrid::new(rows, cols).unwrap();
            assert_eq!(grid.rows(), rows);
            assert_eq!(grid.cols(), cols);
            assert_eq!(grid.size(), rows * cols);
        }
    }

    #[test]
    fn test_get_and_get_mut() {
        for &(rows, cols) in GOOD_SIZES {
            let mut grid = BGrid::filled(rows, cols, 7).unwrap();
            // Try invalid coordinates
            for &coords in &[(rows, 0), (0, cols), (rows, cols)] {
                assert!(grid.get(coords).is_none());
            }
            // Test each coordinate, and mutate
            for i in 0..rows {
                for j in 0..cols {
                    assert_eq!(*grid.get((i, j)).unwrap(), 7);
                    let x = grid.get_mut((i, j)).unwrap();
                    *x = cols * i + j;
                }
            }
            // Check again
            for i in 0..rows {
                for j in 0..cols {
                    assert_eq!(*grid.get((i, j)).unwrap(), cols * i + j);
                }
            }
        }
    }

    #[test]
    fn test_block_size() {
        for &(rows, cols) in GOOD_SIZES {
            let grid = BGrid::new(rows, cols).unwrap();
            assert_eq!(grid.row_blocks() * B::WIDTH, rows);
            assert_eq!(grid.col_blocks() * B::WIDTH, cols);
            assert_eq!(grid.blocks() * B::AREA, grid.size());
        }
    }

    // TODO: Consider moving this to `iters.rs`?
    #[test]
    fn test_block_iter() {
        for &(rows, cols) in GOOD_SIZES {
            let data: Vec<T> = (0..(rows * cols)).collect();
            let grid = BGrid::from_raw_vec(rows, cols, data).unwrap();
            assert_eq!(grid.block_iter().count(), grid.blocks());

            let (mut bi, mut bj): Coords = (0, 0);
            for block in grid.block_iter() {
                for si in 0..B::WIDTH {
                    for sj in 0..B::WIDTH {
                        assert_eq!(block[(si, sj)], grid[(bi + si, bj + sj)]);
                    }
                }
                bj += B::WIDTH;
                if bj == grid.cols() {
                    bj = 0;
                    bi += B::WIDTH;
                }
            }
        }
    }
}
