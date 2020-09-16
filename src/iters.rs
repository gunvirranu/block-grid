use core::marker::PhantomData;
use core::ptr::NonNull;

use crate::{Block, BlockDim, BlockGrid, BlockMut, Coords};
use core::slice::ChunksExact;

pub struct BlockIter<'a, T, B: BlockDim> {
    chunks: ChunksExact<'a, T>,
    _phantom: PhantomData<B>,
}

pub struct BlockIterMut<'a, T, B: BlockDim> {
    pub(crate) block_coords: Coords,
    pub(crate) grid: NonNull<BlockGrid<T, B>>,
    pub(crate) _phantom: PhantomData<&'a mut BlockGrid<T, B>>,
}

pub struct RowMajorIter<'a, T, B: BlockDim> {
    pub(crate) coords: Coords,
    pub(crate) grid: &'a BlockGrid<T, B>,
}

pub struct RowMajorIterMut<'a, T, B: BlockDim> {
    pub(crate) coords: Coords,
    pub(crate) grid: NonNull<BlockGrid<T, B>>,
    pub(crate) _phantom: PhantomData<&'a mut BlockGrid<T, B>>,
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

impl<'a, T, B: BlockDim> Iterator for BlockIterMut<'a, T, B> {
    type Item = BlockMut<'a, T, B>;

    fn next(&mut self) -> Option<Self::Item> {
        // SAFETY: `self.grid` is a valid pointer
        let (row_blocks, col_blocks) = unsafe {
            let grid_ref = self.grid.as_ref();
            (grid_ref.row_blocks(), grid_ref.col_blocks())
        };
        if self.block_coords.0 >= row_blocks {
            return None;
        }
        let block = BlockMut {
            block_coords: self.block_coords,
            // SAFETY: `self.grid` is a valid mutable pointer
            grid: unsafe { &mut *self.grid.as_ptr() },
        };
        self.block_coords.1 += 1;
        if self.block_coords.1 >= col_blocks {
            self.block_coords = (self.block_coords.0 + 1, 0);
        }
        Some(block)
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
