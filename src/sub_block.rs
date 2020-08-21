use crate::{BlockDim, BlockGrid, Coords};

use std::marker::PhantomData;
use std::ops::Index;

#[derive(Debug, PartialEq, Eq)]
pub struct SubBlock<'a, T, B: BlockDim> {
    pub(crate) b_ind: usize,
    pub(crate) grid: &'a BlockGrid<T, B>,
    pub(crate) _phantom: PhantomData<B>,
}

impl<'a, T, B: BlockDim> SubBlock<'a, T, B> {
    pub fn get(&self, coords: Coords) -> Option<&T> {
        // FIXME: Add out of bounds check
        self.grid.buf.get(self.calc_index(coords))
    }

    // TODO: Document unsafety
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn get_unchecked(&self, coords: Coords) -> &T {
        self.grid.buf.get_unchecked(self.calc_index(coords))
    }

    fn calc_index(&self, coords: Coords) -> usize {
        self.b_ind + self.grid.calc_sub_index(coords)
    }
}

impl<'a, T, B: BlockDim> Index<Coords> for SubBlock<'a, T, B> {
    type Output = T;

    fn index(&self, coords: Coords) -> &Self::Output {
        match self.get(coords) {
            Some(x) => x,
            None => panic!("Index out of bounds"),
        }
    }
}
