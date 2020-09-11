use alloc::{vec, vec::Vec};
use core::marker::PhantomData;
use core::ops::{Index, IndexMut};

use crate::iters::{BlockIter, BlockIterMut, RowMajorIter, RowMajorIterMut};
use crate::{BlockDim, Coords};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct BlockGrid<T, B: BlockDim> {
    rows: usize,
    cols: usize,
    buf: Vec<T>,
    _phantom: PhantomData<B>,
}

// TODO: Figure out how `PartialEq`/`Eq` should work
#[derive(Debug)]
pub struct Block<'a, T, B: BlockDim> {
    pub(crate) block_coords: Coords,
    pub(crate) grid: &'a BlockGrid<T, B>,
}

#[derive(Debug)]
pub struct BlockMut<'a, T, B: BlockDim> {
    pub(crate) block_coords: Coords,
    pub(crate) grid: &'a mut BlockGrid<T, B>,
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
        debug_assert!(self.contains(coords));
        let ind = self.calc_index(coords);
        self.buf.get_unchecked(ind)
    }

    // TODO: Document unsafety
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn get_unchecked_mut(&mut self, coords: Coords) -> &mut T {
        debug_assert!(self.contains(coords), "Index out of bounds");
        let ind = self.calc_index(coords);
        self.buf.get_unchecked_mut(ind)
    }

    pub fn each_iter(&self) -> impl Iterator<Item = (Coords, &T)> + ExactSizeIterator {
        let col_blocks = self.col_blocks();
        self.buf
            .iter()
            .enumerate()
            // TODO: Bench against `EachIterCoords` adapter that holds state
            .map(move |(ind, x)| (Self::mem_index_to_coords(ind, col_blocks), x))
    }

    pub fn each_iter_mut(&mut self) -> impl Iterator<Item = (Coords, &mut T)> + ExactSizeIterator {
        let col_blocks = self.col_blocks();
        self.buf
            .iter_mut()
            .enumerate()
            .map(move |(ind, x)| (Self::mem_index_to_coords(ind, col_blocks), x))
    }

    pub fn block_iter(&self) -> BlockIter<T, B> {
        BlockIter {
            block_coords: (0, 0),
            grid: self,
        }
    }

    pub fn block_iter_mut(&mut self) -> BlockIterMut<T, B> {
        BlockIterMut {
            block_coords: (0, 0),
            grid: self.into(), // `self` is a valid reference
            _phantom: PhantomData,
        }
    }

    pub fn row_major_iter(&self) -> RowMajorIter<T, B> {
        RowMajorIter {
            coords: (0, 0),
            grid: self,
        }
    }

    pub fn row_major_iter_mut(&mut self) -> RowMajorIterMut<T, B> {
        RowMajorIterMut {
            coords: (0, 0),
            grid: self.into(), // `self` is a valid reference
            _phantom: PhantomData,
        }
    }

    fn valid_size(rows: usize, cols: usize) -> bool {
        rows > 0 && cols > 0 && rows % B::WIDTH == 0 && cols % B::WIDTH == 0
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

    fn calc_sub_index(&self, (s_row, s_col): Coords) -> usize {
        B::WIDTH * s_row + s_col
    }

    fn calc_block(&self, (row, col): Coords) -> Coords {
        (row >> B::SHIFT, col >> B::SHIFT)
    }

    fn calc_offset(&self, (row, col): Coords) -> Coords {
        (row & B::MASK, col & B::MASK)
    }

    // Have to take `col_blocks` so `self` isn't aliased
    fn mem_index_to_coords(ind: usize, col_blocks: usize) -> Coords {
        let block = ind / B::AREA;
        let intra_block = ind % B::AREA;
        let row = B::WIDTH * (block / col_blocks) + (intra_block / B::WIDTH);
        let col = B::WIDTH * (block % col_blocks) + (intra_block % B::WIDTH);
        (row, col)
    }
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

    pub fn from_row_major(rows: usize, cols: usize, elems: &[T]) -> Result<Self, ()> {
        if !Self::valid_size(rows, cols) || rows * cols != elems.len() {
            return Err(());
        }
        let mut grid = Self {
            rows,
            cols,
            buf: Vec::with_capacity(rows * cols),
            _phantom: PhantomData,
        };
        // Iterate in memory order by index and pull values from row-major.
        // Yeah, this is kinda eh...
        for bi in (0..grid.rows()).step_by(B::WIDTH) {
            for bj in (0..grid.cols()).step_by(B::WIDTH) {
                for si in 0..B::WIDTH {
                    for sj in 0..B::WIDTH {
                        let (row, col) = (bi + si, bj + sj);
                        let row_maj_ind = grid.cols() * row + col;
                        // There's no 'simple' way to do this without `Clone`,
                        // because the row-major `elems` can't be easily drained
                        // out of order.
                        // TODO: A possible, but reallyy unsafe method could be
                        //       to memcpy elements out of `Vec`, and then don't
                        //       drop them the `Vec` is dropped. Investigate.
                        grid.buf.push(elems[row_maj_ind].clone());
                    }
                }
            }
        }
        assert_eq!(grid.buf.len(), grid.rows() * grid.cols());
        Ok(grid)
    }

    pub fn from_col_major(rows: usize, cols: usize, elems: &[T]) -> Result<Self, ()> {
        if !Self::valid_size(rows, cols) || rows * cols != elems.len() {
            return Err(());
        }
        let mut grid = Self {
            rows,
            cols,
            buf: Vec::with_capacity(rows * cols),
            _phantom: PhantomData,
        };
        // Iterate in memory order by index and pull values from col-major
        // Yeah, this too is kinda eh...
        for bi in (0..grid.rows()).step_by(B::WIDTH) {
            for bj in (0..grid.cols()).step_by(B::WIDTH) {
                for si in 0..B::WIDTH {
                    for sj in 0..B::WIDTH {
                        let (row, col) = (bi + si, bj + sj);
                        let col_maj_ind = grid.rows() * col + row;
                        grid.buf.push(elems[col_maj_ind].clone());
                    }
                }
            }
        }
        assert_eq!(grid.buf.len(), grid.rows() * grid.cols());
        Ok(grid)
    }
}

impl<T: Clone + Default, B: BlockDim> BlockGrid<T, B> {
    pub fn new(rows: usize, cols: usize) -> Result<Self, ()> {
        Self::filled(rows, cols, T::default())
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

impl<'a, T, B: BlockDim> Block<'a, T, B> {
    pub fn contains(&self, (row, col): Coords) -> bool {
        row < B::WIDTH && col < B::WIDTH
    }

    pub fn get(&self, coords: Coords) -> Option<&T> {
        if !self.contains(coords) {
            return None;
        }
        self.grid.get(self.calc_coords(coords))
    }

    // TODO: Document unsafety
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn get_unchecked(&self, coords: Coords) -> &T {
        self.grid.get_unchecked(self.calc_coords(coords))
    }

    fn calc_coords(&self, (row, col): Coords) -> Coords {
        let (block_row, block_col) = self.block_coords;
        ((block_row << B::SHIFT) + row, (block_col << B::SHIFT) + col)
    }
}

impl<'a, T, B: BlockDim> Index<Coords> for Block<'a, T, B> {
    type Output = T;

    fn index(&self, coords: Coords) -> &Self::Output {
        match self.get(coords) {
            Some(x) => x,
            None => panic!("Index out of bounds"),
        }
    }
}

impl<'a, T, B: BlockDim> BlockMut<'a, T, B> {
    pub fn contains(&self, (row, col): Coords) -> bool {
        row < B::WIDTH && col < B::WIDTH
    }

    pub fn get(&self, coords: Coords) -> Option<&T> {
        if !self.contains(coords) {
            return None;
        }
        self.grid.get(self.calc_coords(coords))
    }

    pub fn get_mut(&mut self, coords: Coords) -> Option<&mut T> {
        if !self.contains(coords) {
            return None;
        }
        self.grid.get_mut(self.calc_coords(coords))
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn get_unchecked(&self, coords: Coords) -> &T {
        self.grid.get_unchecked(self.calc_coords(coords))
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn get_unchecked_mut(&mut self, coords: Coords) -> &T {
        self.grid.get_unchecked_mut(self.calc_coords(coords))
    }

    fn calc_coords(&self, (row, col): Coords) -> Coords {
        let (block_row, block_col) = self.block_coords;
        ((block_row << B::SHIFT) + row, (block_col << B::SHIFT) + col)
    }
}

impl<'a, T, B: BlockDim> Index<Coords> for BlockMut<'a, T, B> {
    type Output = T;

    fn index(&self, coords: Coords) -> &Self::Output {
        match self.get(coords) {
            Some(x) => x,
            None => panic!("Index out of bounds"),
        }
    }
}

impl<'a, T, B: BlockDim> IndexMut<Coords> for BlockMut<'a, T, B> {
    fn index_mut(&mut self, coords: Coords) -> &mut Self::Output {
        match self.get_mut(coords) {
            Some(x) => x,
            None => panic!("Index out of bounds"),
        }
    }
}
