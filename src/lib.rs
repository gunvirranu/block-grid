mod block_width;

pub use block_width::{BlockDim, BlockWidth};

use std::marker::PhantomData;

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
