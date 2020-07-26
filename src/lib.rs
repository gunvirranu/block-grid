mod block_width;

pub use block_width::{BlockDim, BlockWidth};

use std::marker::PhantomData;
use std::ops::{Index, IndexMut};

type Coords = (usize, usize);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BlockGrid<T, B: BlockDim> {
    rows: usize,
    cols: usize,
    buf: Vec<T>,
    _phantom: PhantomData<B>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct SubBlock<'a, T, B: BlockDim> {
    b_ind: usize,
    grid: &'a BlockGrid<T, B>,
    _phantom: PhantomData<B>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct SubBlockMut<'a, T, B: BlockDim> {
    b_ind: usize,
    grid: &'a mut BlockGrid<T, B>,
    _phantom: PhantomData<B>,
}

impl<T, B: BlockDim> BlockGrid<T, B> {
    pub fn from_raw_vec(rows: usize, cols: usize, elems: Vec<T>) -> Self {
        // TODO: Maybe better error-handling?
        assert_eq!(rows * cols, elems.len());
        Self {
            rows,
            cols,
            buf: elems,
            _phantom: PhantomData,
        }
    }

    // TODO: Impl row-major constructor
    // TODO: Impl col-major constructor

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn get(&self, coords: Coords) -> Option<&T> {
        self.buf.get(self.calc_index(coords))
    }

    pub fn get_mut(&mut self, coords: Coords) -> Option<&mut T> {
        let ind = self.calc_index(coords);
        self.buf.get_mut(ind)
    }

    // TODO: Document unsafety
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn get_unchecked(&self, coords: Coords) -> &T {
        let ind = self.calc_index(coords);
        self.buf.get_unchecked(ind)
    }

    // TODO: Document unsafety
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn get_unchecked_mut(&mut self, coords: Coords) -> &mut T {
        let ind = self.calc_index(coords);
        self.buf.get_unchecked_mut(ind)
    }

    pub fn block(&self, block_coords: Coords) -> SubBlock<T, B> {
        // TODO: Check valid `block_coords`
        let block_ind = self.calc_block_index(block_coords);
        SubBlock {
            b_ind: block_ind,
            grid: self,
            _phantom: PhantomData,
        }
    }

    pub fn block_mut(&mut self, block_coords: Coords) -> SubBlockMut<T, B> {
        // TODO: Check valid `block_coords`
        let block_ind = self.calc_block_index(block_coords);
        SubBlockMut {
            b_ind: block_ind,
            grid: self,
            _phantom: PhantomData,
        }
    }

    // TODO: Impl iterators

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
        B::AREA * (self.row_blocks() * b_row + b_col)
    }

    fn calc_sub_index(&self, (s_row, s_col): Coords) -> usize {
        B::WIDTH + s_row + s_col
    }

    fn row_blocks(&self) -> usize {
        self.rows >> B::SHIFT
    }

    fn calc_block(&self, (row, col): Coords) -> Coords {
        (row >> B::SHIFT, col >> B::SHIFT)
    }

    fn calc_offset(&self, (row, col): Coords) -> Coords {
        (row & B::MASK, col & B::MASK)
    }
}

impl<T: Clone, B: BlockDim> BlockGrid<T, B> {
    pub fn filled(rows: usize, cols: usize, elem: T) -> Self {
        // TODO: Check if `rows` and `cols` divide block-size
        Self {
            rows,
            cols,
            buf: vec![elem; rows * cols],
            _phantom: PhantomData,
        }
    }
}

impl<'a, T, B: BlockDim> SubBlock<'a, T, B> {
    pub fn get(&self, coords: Coords) -> Option<&T> {
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

impl<'a, T, B: BlockDim> SubBlockMut<'a, T, B> {
    pub fn get(&self, coords: Coords) -> Option<&T> {
        self.grid.buf.get(self.calc_index(coords))
    }

    pub fn get_mut(&mut self, coords: Coords) -> Option<&mut T> {
        let ind = self.calc_index(coords);
        self.grid.buf.get_mut(ind)
    }

    // TODO: Document unsafety
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn get_unchecked(&self, coords: Coords) -> &T {
        self.grid.buf.get_unchecked(self.calc_index(coords))
    }

    // TODO: Document unsafety
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn get_unchecked_mut(&mut self, coords: Coords) -> &mut T {
        let ind = self.calc_index(coords);
        self.grid.buf.get_unchecked_mut(ind)
    }

    fn calc_index(&self, coords: Coords) -> usize {
        self.b_ind + self.grid.calc_sub_index(coords)
    }
}

macro_rules! impl_index {
    () => {
        fn index(&self, coords: Coords) -> &Self::Output {
            // TODO: Benchmark against unchecked
            match self.get(coords) {
                Some(x) => x,
                None => panic!("Index out of bounds"),
            }
        }
    };
}

macro_rules! impl_index_mut {
    () => {
        fn index_mut(&mut self, coords: Coords) -> &mut Self::Output {
            match self.get_mut(coords) {
                Some(x) => x,
                None => panic!("Index out of bounds"),
            }
        }
    };
}

impl<T, B: BlockDim> Index<Coords> for BlockGrid<T, B> {
    type Output = T;
    impl_index!();
}

impl<T, B: BlockDim> IndexMut<Coords> for BlockGrid<T, B> {
    impl_index_mut!();
}

impl<'a, T, B: BlockDim> Index<Coords> for SubBlock<'a, T, B> {
    type Output = T;
    impl_index!();
}

impl<'a, T, B: BlockDim> Index<Coords> for SubBlockMut<'a, T, B> {
    type Output = T;
    impl_index!();
}

impl<'a, T, B: BlockDim> IndexMut<Coords> for SubBlockMut<'a, T, B> {
    impl_index_mut!();
}
