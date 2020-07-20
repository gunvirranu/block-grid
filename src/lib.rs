mod block_width;

pub use block_width::{BlockDim, BlockWidth};

use std::marker::PhantomData;

type Coords = (usize, usize);

#[derive(Clone, PartialEq, Eq)]
pub struct BlockGrid<T, B: BlockDim> {
    rows: usize,
    cols: usize,
    buf: Vec<T>,
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

    fn calc_index(&self, coords: Coords) -> usize {
        // Get block
        let (b_row, b_col) = self.calc_block(coords);
        let b_ind = B::AREA * (self.row_blocks() * b_row + b_col);
        // Offset within block
        let (s_row, s_col) = self.calc_offset(coords);
        let s_ind = B::WIDTH * s_row + s_col;
        b_ind + s_ind
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
