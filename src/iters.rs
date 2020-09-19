use core::iter::FusedIterator;
use core::marker::PhantomData;
use core::ptr::NonNull;
use core::slice::{ChunksExact, ChunksExactMut};

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
    fn current_coords(&self) -> Coords {
        (self.block_row, self.block_col)
    }
}

impl<'a, T, B: BlockDim> Iterator for BlockIter<'a, T, B> {
    type Item = Block<'a, T, B>;

    fn next(&mut self) -> Option<Self::Item> {
        self.block_col += 1;
        if self.block_col == self.col_blocks {
            self.block_row += 1;
            self.block_col = 0;
        }
        self.chunks.next().map(Block::new)
    }
}

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
    fn current_coords(&self) -> Coords {
        (self.block_row, self.block_col)
    }
}

impl<'a, T, B: BlockDim> Iterator for BlockIterMut<'a, T, B> {
    type Item = BlockMut<'a, T, B>;

    fn next(&mut self) -> Option<Self::Item> {
        self.block_col += 1;
        if self.block_col == self.col_blocks {
            self.block_row += 1;
            self.block_col = 0;
        }
        self.chunks.next().map(BlockMut::new)
    }
}

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
    fn current_coords(&self) -> Coords {
        (self.row, self.col)
    }
}

impl<'a, T, B: BlockDim> Iterator for RowMajorIter<'a, T, B> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let c = (self.row, self.col);
        self.col += 1;
        if self.col == self.grid.cols() {
            self.row += 1;
            self.col = 0;
        }
        self.grid.get(c)
    }
}

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
    fn current_coords(&self) -> Coords {
        (self.row, self.col)
    }
}

impl<'a, T, B: BlockDim> Iterator for RowMajorIterMut<'a, T, B> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let c = (self.row, self.col);
        self.col += 1;
        // SAFETY: `self.grid` is a valid pointer
        if self.col == unsafe { self.grid.as_ref().cols() } {
            self.row += 1;
            self.col = 0;
        }
        // SAFETY: `self.grid` is a valid mutable pointer
        unsafe { &mut *self.grid.as_ptr() }.get_mut(c)
    }
}

impl<I: CoordsIterator> Iterator for WithCoordsIter<I> {
    type Item = (Coords, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.iter.current_coords();
        self.iter.next().map(|x| (c, x))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    fn count(self) -> usize {
        self.iter.count()
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        if n > 1 {
            self.iter.nth(n - 1)?;
        }
        self.next()
    }
}

impl<I: CoordsIterator + ExactSizeIterator> ExactSizeIterator for WithCoordsIter<I> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<I: CoordsIterator + FusedIterator> FusedIterator for WithCoordsIter<I> {}
