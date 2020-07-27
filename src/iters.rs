use std::marker::PhantomData;

use crate::{BlockDim, BlockGrid, Coords, SubBlock};

pub struct BlockIter<'a, T, B: BlockDim> {
    pub(crate) cur_block: usize,
    pub(crate) max_blocks: usize,
    pub(crate) grid: &'a BlockGrid<T, B>,
}

pub struct RowMajorIter<'a, T, B: BlockDim> {
    pub(crate) row: usize,
    pub(crate) col: usize,
    pub(crate) grid: &'a BlockGrid<T, B>,
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
        let (i, j) = (self.row, self.col);
        self.col += 1;
        if self.col >= self.grid.cols() {
            self.col = 0;
            self.row += 1;
        }
        let elem = self.grid.get((i, j))?;
        Some(((i, j), elem))
    }
}
