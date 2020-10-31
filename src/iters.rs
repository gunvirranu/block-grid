use core::iter::FusedIterator;
use core::marker::PhantomData;
use core::ptr::NonNull;
use core::slice::{ChunksExact, ChunksExactMut, Iter, IterMut};

use crate::{Block, BlockDim, BlockGrid, BlockMut, Coords};

pub trait CoordsIterator: Iterator {
    fn current_coords(&self) -> Coords;

    fn coords(self) -> WithCoordsIter<Self>
    where
        Self: Sized,
    {
        WithCoordsIter { iter: self }
    }
}

pub struct EachIter<'a, T, B: BlockDim> {
    row: usize,
    col: usize,
    cols: usize,
    iter: Iter<'a, T>,
    _phantom: PhantomData<B>,
}

pub struct EachIterMut<'a, T, B: BlockDim> {
    row: usize,
    col: usize,
    cols: usize,
    iter: IterMut<'a, T>,
    _phantom: PhantomData<B>,
}

pub struct BlockIter<'a, T, B: BlockDim> {
    block_row: usize,
    block_col: usize,
    col_blocks: usize,
    chunks: ChunksExact<'a, T>,
    _phantom: PhantomData<B>,
}

pub struct BlockIterMut<'a, T, B: BlockDim> {
    block_row: usize,
    block_col: usize,
    col_blocks: usize,
    chunks: ChunksExactMut<'a, T>,
    _phantom: PhantomData<B>,
}

pub struct RowMajorIter<'a, T, B: BlockDim> {
    row: usize,
    col: usize,
    grid: &'a BlockGrid<T, B>,
}

pub struct RowMajorIterMut<'a, T, B: BlockDim> {
    row: usize,
    col: usize,
    grid: NonNull<BlockGrid<T, B>>,
    _phantom: PhantomData<&'a mut BlockGrid<T, B>>,
}

pub struct WithCoordsIter<I> {
    iter: I,
}

impl<'a, T, B: BlockDim> EachIter<'a, T, B> {
    pub(crate) fn new(grid: &'a BlockGrid<T, B>) -> Self {
        Self {
            row: 0,
            col: 0,
            cols: grid.cols(),
            iter: grid.raw().iter(),
            _phantom: PhantomData,
        }
    }
}

impl<'a, T, B: BlockDim> CoordsIterator for EachIter<'a, T, B> {
    #[inline]
    fn current_coords(&self) -> Coords {
        (self.row, self.col)
    }
}

impl<'a, T, B: BlockDim> Iterator for EachIter<'a, T, B> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.col += 1;
        if self.col % B::WIDTH == 0 {
            self.row += 1;
            if self.row % B::WIDTH == 0 {
                if self.col == self.cols {
                    self.col = 0;
                } else {
                    self.row -= B::WIDTH;
                }
            } else {
                self.col -= B::WIDTH;
            }
        }
        self.iter.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    #[inline]
    fn count(self) -> usize {
        self.iter.count()
    }
}

impl<'a, T, B: BlockDim> ExactSizeIterator for EachIter<'a, T, B> {
    #[inline]
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<'a, T, B: BlockDim> FusedIterator for EachIter<'a, T, B> {}

impl<'a, T, B: BlockDim> EachIterMut<'a, T, B> {
    pub(crate) fn new(grid: &'a mut BlockGrid<T, B>) -> Self {
        Self {
            row: 0,
            col: 0,
            cols: grid.cols(),
            iter: grid.raw_mut().iter_mut(),
            _phantom: PhantomData,
        }
    }
}

impl<'a, T, B: BlockDim> CoordsIterator for EachIterMut<'a, T, B> {
    #[inline]
    fn current_coords(&self) -> Coords {
        (self.row, self.col)
    }
}

impl<'a, T, B: BlockDim> Iterator for EachIterMut<'a, T, B> {
    type Item = &'a mut T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.col += 1;
        if self.col % B::WIDTH == 0 {
            self.row += 1;
            if self.row % B::WIDTH == 0 {
                if self.col == self.cols {
                    self.col = 0;
                } else {
                    self.row -= B::WIDTH;
                }
            } else {
                self.col -= B::WIDTH;
            }
        }
        self.iter.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    #[inline]
    fn count(self) -> usize {
        self.iter.count()
    }
}

impl<'a, T, B: BlockDim> ExactSizeIterator for EachIterMut<'a, T, B> {
    #[inline]
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<'a, T, B: BlockDim> FusedIterator for EachIterMut<'a, T, B> {}

// TODO: See if I can use the anonymous lifetime `'_` everywhere here
impl<'a, T, B: BlockDim> BlockIter<'a, T, B> {
    pub(crate) fn new(grid: &'a BlockGrid<T, B>) -> Self {
        Self {
            block_row: 0,
            block_col: 0,
            col_blocks: grid.col_blocks(),
            chunks: grid.raw().chunks_exact(B::AREA),
            _phantom: PhantomData,
        }
    }
}

impl<'a, T, B: BlockDim> CoordsIterator for BlockIter<'a, T, B> {
    #[inline]
    fn current_coords(&self) -> Coords {
        (self.block_row, self.block_col)
    }
}

impl<'a, T, B: BlockDim> Iterator for BlockIter<'a, T, B> {
    type Item = Block<'a, T, B>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.block_col += 1;
        if self.block_col == self.col_blocks {
            self.block_row += 1;
            self.block_col = 0;
        }
        self.chunks.next().map(Block::new)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.chunks.size_hint()
    }

    #[inline]
    fn count(self) -> usize {
        self.chunks.count()
    }
}

impl<'a, T, B: BlockDim> ExactSizeIterator for BlockIter<'a, T, B> {
    #[inline]
    fn len(&self) -> usize {
        self.chunks.len()
    }
}

impl<'a, T, B: BlockDim> FusedIterator for BlockIter<'a, T, B> {}

impl<'a, T, B: BlockDim> BlockIterMut<'a, T, B> {
    pub(crate) fn new(grid: &'a mut BlockGrid<T, B>) -> Self {
        Self {
            block_row: 0,
            block_col: 0,
            col_blocks: grid.col_blocks(),
            chunks: grid.raw_mut().chunks_exact_mut(B::AREA),
            _phantom: PhantomData,
        }
    }
}

impl<'a, T, B: BlockDim> CoordsIterator for BlockIterMut<'a, T, B> {
    #[inline]
    fn current_coords(&self) -> Coords {
        (self.block_row, self.block_col)
    }
}

impl<'a, T, B: BlockDim> Iterator for BlockIterMut<'a, T, B> {
    type Item = BlockMut<'a, T, B>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.block_col += 1;
        if self.block_col == self.col_blocks {
            self.block_row += 1;
            self.block_col = 0;
        }
        self.chunks.next().map(BlockMut::new)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.chunks.size_hint()
    }

    #[inline]
    fn count(self) -> usize {
        self.chunks.count()
    }
}

impl<'a, T, B: BlockDim> ExactSizeIterator for BlockIterMut<'a, T, B> {
    #[inline]
    fn len(&self) -> usize {
        self.chunks.len()
    }
}

impl<'a, T, B: BlockDim> FusedIterator for BlockIterMut<'a, T, B> {}

impl<'a, T, B: BlockDim> RowMajorIter<'a, T, B> {
    pub(crate) fn new(grid: &'a BlockGrid<T, B>) -> Self {
        Self {
            row: 0,
            col: 0,
            grid,
        }
    }
}

impl<'a, T, B: BlockDim> CoordsIterator for RowMajorIter<'a, T, B> {
    #[inline]
    fn current_coords(&self) -> Coords {
        (self.row, self.col)
    }
}

impl<'a, T, B: BlockDim> Iterator for RowMajorIter<'a, T, B> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.row >= self.grid.rows() {
            return None;
        }
        let c = (self.row, self.col);
        self.col += 1;
        if self.col == self.grid.cols() {
            self.row += 1;
            self.col = 0;
        }
        // TODO: Can make unchecked
        self.grid.get(c)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let idx = self.row * self.grid.cols() + self.col;
        let k = self.grid.size() - idx;
        (k, Some(k))
    }

    #[inline]
    fn count(self) -> usize {
        self.len()
    }
}

impl<'a, T, B: BlockDim> ExactSizeIterator for RowMajorIter<'a, T, B> {}

impl<'a, T, B: BlockDim> FusedIterator for RowMajorIter<'a, T, B> {}

impl<'a, T, B: BlockDim> RowMajorIterMut<'a, T, B> {
    pub(crate) fn new(grid: &'a mut BlockGrid<T, B>) -> Self {
        Self {
            row: 0,
            col: 0,
            grid: grid.into(),
            _phantom: PhantomData,
        }
    }
}

impl<'a, T, B: BlockDim> CoordsIterator for RowMajorIterMut<'a, T, B> {
    #[inline]
    fn current_coords(&self) -> Coords {
        (self.row, self.col)
    }
}

impl<'a, T, B: BlockDim> Iterator for RowMajorIterMut<'a, T, B> {
    type Item = &'a mut T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        // SAFETY: `self.grid` is a valid pointer
        let (rows, cols) = unsafe {
            let grid = self.grid.as_ref();
            (grid.rows(), grid.cols())
        };
        if self.row >= rows {
            return None;
        }
        let c = (self.row, self.col);
        self.col += 1;
        if self.col == cols {
            self.row += 1;
            self.col = 0;
        }
        // TODO: Can be unchecked
        // SAFETY: `self.grid` is a valid mutable pointer
        unsafe { &mut *self.grid.as_ptr() }.get_mut(c)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        // SAFETY: `self.grid` is a valid pointer
        let grid = unsafe { self.grid.as_ref() };
        let idx = self.row * grid.cols() + self.col;
        let k = grid.size() - idx;
        (k, Some(k))
    }

    #[inline]
    fn count(self) -> usize {
        self.len()
    }
}

impl<'a, T, B: BlockDim> ExactSizeIterator for RowMajorIterMut<'a, T, B> {}

impl<'a, T, B: BlockDim> FusedIterator for RowMajorIterMut<'a, T, B> {}

impl<I: CoordsIterator> Iterator for WithCoordsIter<I> {
    type Item = (Coords, I::Item);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let c = self.iter.current_coords();
        self.iter.next().map(|x| (c, x))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    #[inline]
    fn count(self) -> usize {
        self.iter.count()
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        if n >= 1 {
            self.iter.nth(n - 1)?;
        }
        self.next()
    }
}

impl<I: CoordsIterator + ExactSizeIterator> ExactSizeIterator for WithCoordsIter<I> {
    #[inline]
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<I: CoordsIterator + FusedIterator> FusedIterator for WithCoordsIter<I> {}
