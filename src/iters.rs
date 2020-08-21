use std::marker::PhantomData;
use std::ptr::NonNull;

use crate::{BlockDim, BlockGrid, Coords, SubBlock};

pub struct BlockIter<'a, T, B: BlockDim> {
    pub(crate) cur_block: usize,
    pub(crate) max_blocks: usize,
    pub(crate) grid: &'a BlockGrid<T, B>,
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
        if self.cur_block >= self.max_blocks {
            return None;
        }
        let block = SubBlock {
            b_ind: B::AREA * self.cur_block,
            grid: self.grid,
            _phantom: PhantomData,
        };
        self.cur_block += 1;
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
