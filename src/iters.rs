use std::marker::PhantomData;
use std::ptr::NonNull;

use crate::{BlockDim, BlockGrid, Coords, SubBlock, SubBlockMut};

pub struct BlockIter<'a, T, B: BlockDim> {
    pub(crate) block_coords: Coords,
    pub(crate) grid: &'a BlockGrid<T, B>,
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

impl<'a, T, B: BlockDim> Iterator for BlockIter<'a, T, B> {
    type Item = SubBlock<'a, T, B>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.block_coords.0 >= self.grid.row_blocks() {
            return None;
        }
        let block = SubBlock {
            block_coords: self.block_coords,
            grid: self.grid,
        };
        self.block_coords.1 += 1;
        if self.block_coords.1 >= self.grid.col_blocks() {
            self.block_coords = (self.block_coords.0 + 1, 0);
        }
        Some(block)
    }
}

impl<'a, T, B: BlockDim> Iterator for BlockIterMut<'a, T, B> {
    type Item = SubBlockMut<'a, T, B>;

    fn next(&mut self) -> Option<Self::Item> {
        // SAFETY: `self.grid` is a valid pointer
        let (row_blocks, col_blocks) = unsafe {
            let grid_ref = self.grid.as_ref();
            (grid_ref.row_blocks(), grid_ref.col_blocks())
        };
        if self.block_coords.0 >= row_blocks {
            return None;
        }
        let block = SubBlockMut {
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
