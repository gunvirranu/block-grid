use core::marker::PhantomData;
use core::ptr::NonNull;

use crate::{Block, BlockDim, BlockGrid, BlockMut, Coords};
use core::slice::{ChunksExact, ChunksExactMut};

pub struct BlockIter<'a, T, B: BlockDim> {
    chunks: ChunksExact<'a, T>,
    _phantom: PhantomData<B>,
}

pub struct BlockIterMut<'a, T, B: BlockDim> {
    chunks: ChunksExactMut<'a, T>,
    _phantom: PhantomData<B>,
}

pub struct RowMajorIter<'a, T, B: BlockDim> {
    coords: Coords,
    grid: &'a BlockGrid<T, B>,
}

pub struct RowMajorIterMut<'a, T, B: BlockDim> {
    coords: Coords,
    grid: NonNull<BlockGrid<T, B>>,
    _phantom: PhantomData<&'a mut BlockGrid<T, B>>,
}

impl<'a, T, B: BlockDim> BlockIter<'a, T, B> {
    pub(crate) fn new(grid: &'a BlockGrid<T, B>) -> Self {
        Self {
            chunks: grid.raw().chunks_exact(B::AREA),
            _phantom: PhantomData,
        }
    }
}

impl<'a, T, B: BlockDim> Iterator for BlockIter<'a, T, B> {
    type Item = Block<'a, T, B>;

    fn next(&mut self) -> Option<Self::Item> {
        self.chunks.next().map(Block::new)
    }
}

impl<'a, T, B: BlockDim> BlockIterMut<'a, T, B> {
    pub(crate) fn new(grid: &'a mut BlockGrid<T, B>) -> Self {
        Self {
            chunks: grid.raw_mut().chunks_exact_mut(B::AREA),
            _phantom: PhantomData,
        }
    }
}

impl<'a, T, B: BlockDim> Iterator for BlockIterMut<'a, T, B> {
    type Item = BlockMut<'a, T, B>;

    fn next(&mut self) -> Option<Self::Item> {
        self.chunks.next().map(BlockMut::new)
    }
}

impl<'a, T, B: BlockDim> RowMajorIter<'a, T, B> {
    pub(crate) fn new(grid: &'a BlockGrid<T, B>) -> Self {
        Self {
            coords: (0, 0),
            grid,
        }
    }
}

impl<'a, T, B: BlockDim> Iterator for RowMajorIter<'a, T, B> {
    type Item = (Coords, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.coords;
        self.coords.1 += 1;
        if self.coords.1 >= self.grid.cols() {
            self.coords = (c.0 + 1, 0);
        }
        let elem = self.grid.get(c)?;
        Some((c, elem))
    }
}

impl<'a, T, B: BlockDim> RowMajorIterMut<'a, T, B> {
    pub(crate) fn new(grid: &'a mut BlockGrid<T, B>) -> Self {
        Self {
            coords: (0, 0),
            grid: grid.into(),
            _phantom: PhantomData,
        }
    }
}

impl<'a, T, B: BlockDim> Iterator for RowMajorIterMut<'a, T, B> {
    type Item = (Coords, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.coords;
        self.coords.1 += 1;
        // SAFETY: `self.grid` is a valid pointer
        if self.coords.1 >= unsafe { self.grid.as_ref().cols() } {
            self.coords = (c.0 + 1, 0);
        }
        // SAFETY: `self.grid` is a valid mutable pointer
        let elem = unsafe { &mut *self.grid.as_ptr() }.get_mut(c)?;
        Some((c, elem))
    }
}
