// FIXME: Fix and remove eventally
#![allow(clippy::result_unit_err)]

use alloc::{vec, vec::Vec};
use core::marker::PhantomData;
use core::ops::{Index, IndexMut};

use crate::iters::{BlockIter, BlockIterMut, EachIter, EachIterMut, RowMajorIter, RowMajorIterMut};
use crate::{BlockDim, Coords};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct BlockGrid<T, B: BlockDim> {
    rows: usize,
    cols: usize,
    buf: Vec<T>,
    _phantom: PhantomData<B>,
}

#[derive(Clone, Copy, Debug)]
pub struct Block<'a, T, B: BlockDim> {
    arr: &'a [T],
    _phantom: PhantomData<B>,
}

#[derive(Debug)]
pub struct BlockMut<'a, T, B: BlockDim> {
    arr: &'a mut [T],
    _phantom: PhantomData<B>,
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

    #[inline]
    pub fn take_raw_vec(self) -> Vec<T> {
        self.buf
    }

    #[inline]
    pub fn rows(&self) -> usize {
        self.rows
    }

    #[inline]
    pub fn cols(&self) -> usize {
        self.cols
    }

    #[inline]
    pub fn size(&self) -> usize {
        self.rows() * self.cols()
    }

    #[inline]
    pub fn row_blocks(&self) -> usize {
        self.rows / B::WIDTH
    }

    #[inline]
    pub fn col_blocks(&self) -> usize {
        self.cols / B::WIDTH
    }

    #[inline]
    pub fn blocks(&self) -> usize {
        self.row_blocks() * self.col_blocks()
    }

    #[inline]
    pub fn contains(&self, (row, cols): Coords) -> bool {
        row < self.rows && cols < self.cols
    }

    #[inline]
    pub fn get(&self, coords: Coords) -> Option<&T> {
        if !self.contains(coords) {
            return None;
        }
        // SAFETY: `coords` is a valid index
        Some(unsafe { self.get_unchecked(coords) })
    }

    #[inline]
    pub fn get_mut(&mut self, coords: Coords) -> Option<&mut T> {
        if !self.contains(coords) {
            return None;
        }
        // SAFETY: `coords` is a valid index
        Some(unsafe { self.get_unchecked_mut(coords) })
    }

    // TODO: Document unsafety
    #[allow(clippy::missing_safety_doc)]
    #[inline]
    pub unsafe fn get_unchecked(&self, coords: Coords) -> &T {
        debug_assert!(self.contains(coords));
        let ind = self.calc_index(coords);
        self.buf.get_unchecked(ind)
    }

    // TODO: Document unsafety
    #[allow(clippy::missing_safety_doc)]
    #[inline]
    pub unsafe fn get_unchecked_mut(&mut self, coords: Coords) -> &mut T {
        debug_assert!(self.contains(coords));
        let ind = self.calc_index(coords);
        self.buf.get_unchecked_mut(ind)
    }

    #[inline]
    pub fn raw(&self) -> &[T] {
        &self.buf
    }

    #[inline]
    pub fn raw_mut(&mut self) -> &mut [T] {
        &mut self.buf
    }

    #[inline]
    pub fn each_iter(&self) -> EachIter<T, B> {
        EachIter::new(self)
    }

    #[inline]
    pub fn each_iter_mut(&mut self) -> EachIterMut<T, B> {
        EachIterMut::new(self)
    }

    #[inline]
    pub fn block_iter(&self) -> BlockIter<T, B> {
        BlockIter::new(self)
    }

    #[inline]
    pub fn block_iter_mut(&mut self) -> BlockIterMut<T, B> {
        BlockIterMut::new(self)
    }

    #[inline]
    pub fn row_major_iter(&self) -> RowMajorIter<T, B> {
        RowMajorIter::new(self)
    }

    #[inline]
    pub fn row_major_iter_mut(&mut self) -> RowMajorIterMut<T, B> {
        RowMajorIterMut::new(self)
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
        (row / B::WIDTH, col / B::WIDTH)
    }

    fn calc_offset(&self, (row, col): Coords) -> Coords {
        (row % B::WIDTH, col % B::WIDTH)
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
        Self::from_array_index_helper(rows, cols, elems, |row, col| cols * row + col)
    }

    pub fn from_col_major(rows: usize, cols: usize, elems: &[T]) -> Result<Self, ()> {
        Self::from_array_index_helper(rows, cols, elems, |row, col| rows * col + row)
    }

    fn from_array_index_helper(
        rows: usize,
        cols: usize,
        elems: &[T],
        calc_index: impl Fn(usize, usize) -> usize,
    ) -> Result<Self, ()> {
        if !Self::valid_size(rows, cols) || rows * cols != elems.len() {
            return Err(());
        }
        let mut grid = Self {
            rows,
            cols,
            buf: Vec::with_capacity(rows * cols),
            _phantom: PhantomData,
        };
        // Iterate in memory order by index and pull values from `elems`
        for bi in (0..grid.rows()).step_by(B::WIDTH) {
            for bj in (0..grid.cols()).step_by(B::WIDTH) {
                for si in 0..B::WIDTH {
                    for sj in 0..B::WIDTH {
                        let (row, col) = (bi + si, bj + sj);
                        let ind = calc_index(row, col);
                        // There's no 'simple' way to do this without `Clone`,
                        // because `elems` can't be easily drained out of order.
                        // TODO: Investigate a possible, but reallyy unsafe
                        //       method to memcpy elements out of `Vec`, and
                        //       then don't drop them when `Vec` is dropped.
                        grid.buf.push(elems[ind].clone());
                    }
                }
            }
        }
        debug_assert_eq!(grid.buf.len(), grid.size());
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

    #[inline]
    fn index(&self, coords: Coords) -> &Self::Output {
        self.get(coords).expect("Index out of bounds")
    }
}

impl<T, B: BlockDim> IndexMut<Coords> for BlockGrid<T, B> {
    #[inline]
    fn index_mut(&mut self, coords: Coords) -> &mut Self::Output {
        self.get_mut(coords).expect("Index out of bounds")
    }
}

impl<'a, T, B: BlockDim> Block<'a, T, B> {
    // `arr` **must** be of length `B::AREA`
    pub(crate) unsafe fn new(arr: &'a [T]) -> Self {
        debug_assert_eq!(arr.len(), B::AREA);
        Self {
            arr,
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub fn contains(&self, (row, col): Coords) -> bool {
        row < B::WIDTH && col < B::WIDTH
    }

    #[inline]
    pub fn get(&self, coords: Coords) -> Option<&T> {
        if !self.contains(coords) {
            return None;
        }
        // SAFETY: `coords` is a valid index
        Some(unsafe { self.get_unchecked(coords) })
    }

    // TODO: Document unsafety
    #[allow(clippy::missing_safety_doc)]
    #[inline]
    pub unsafe fn get_unchecked(&self, coords: Coords) -> &T {
        debug_assert!(self.contains(coords));
        self.arr.get_unchecked(self.calc_index(coords))
    }

    fn calc_index(&self, (row, col): Coords) -> usize {
        B::WIDTH * row + col
    }
}

impl<'a, T, B: BlockDim> Index<Coords> for Block<'a, T, B> {
    type Output = T;

    #[inline]
    fn index(&self, coords: Coords) -> &Self::Output {
        self.get(coords).expect("Index out of bounds")
    }
}

impl<'a, T, B: BlockDim> BlockMut<'a, T, B> {
    // `arr` **must** be of length `B::AREA`
    pub(crate) unsafe fn new(arr: &'a mut [T]) -> Self {
        debug_assert_eq!(arr.len(), B::AREA);
        Self {
            arr,
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub fn contains(&self, (row, col): Coords) -> bool {
        row < B::WIDTH && col < B::WIDTH
    }

    #[inline]
    pub fn get(&self, coords: Coords) -> Option<&T> {
        if !self.contains(coords) {
            return None;
        }
        // SAFETY: `coords` is a valid index
        Some(unsafe { self.get_unchecked(coords) })
    }

    #[inline]
    pub fn get_mut(&mut self, coords: Coords) -> Option<&mut T> {
        if !self.contains(coords) {
            return None;
        }
        // SAFETY: `coords` is a valid index
        Some(unsafe { self.get_unchecked_mut(coords) })
    }

    #[allow(clippy::missing_safety_doc)]
    #[inline]
    pub unsafe fn get_unchecked(&self, coords: Coords) -> &T {
        debug_assert!(self.contains(coords));
        self.arr.get_unchecked(self.calc_index(coords))
    }

    #[allow(clippy::missing_safety_doc)]
    #[inline]
    pub unsafe fn get_unchecked_mut(&mut self, coords: Coords) -> &mut T {
        debug_assert!(self.contains(coords));
        self.arr.get_unchecked_mut(self.calc_index(coords))
    }

    fn calc_index(&self, (row, col): Coords) -> usize {
        B::WIDTH * row + col
    }
}

impl<'a, T, B: BlockDim> Index<Coords> for BlockMut<'a, T, B> {
    type Output = T;

    #[inline]
    fn index(&self, coords: Coords) -> &Self::Output {
        self.get(coords).expect("Index out of bounds")
    }
}

impl<'a, T, B: BlockDim> IndexMut<Coords> for BlockMut<'a, T, B> {
    #[inline]
    fn index_mut(&mut self, coords: Coords) -> &mut Self::Output {
        self.get_mut(coords).expect("Index out of bounds")
    }
}
