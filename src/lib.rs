mod block_grid;
mod block_width;
mod iters;
mod sub_block;
#[cfg(test)]
mod tests;

pub use crate::block_grid::BlockGrid;
pub use crate::block_width::{BlockDim, BlockWidth};
pub use crate::sub_block::SubBlock;

pub type Coords = (usize, usize);
