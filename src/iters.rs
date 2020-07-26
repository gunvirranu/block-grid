use std::marker::PhantomData;

use crate::{BlockDim, BlockGrid, SubBlock};

pub struct BlockIter<'a, T, B: BlockDim> {
    pub(crate) cur_block: usize,
    pub(crate) max_blocks: usize,
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
