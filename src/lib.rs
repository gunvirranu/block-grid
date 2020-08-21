mod block_grid;
mod block_width;
mod iters;
#[cfg(test)]
mod tests;

pub use crate::block_grid::{BlockGrid, SubBlock, SubBlockMut};
pub use crate::block_width::{BlockDim, BlockWidth};

pub type Coords = (usize, usize);
