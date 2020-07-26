mod block_grid;
mod block_width;
mod iters;
mod sub_block;

pub use block_grid::BlockGrid;
pub use block_width::{BlockDim, BlockWidth};
pub use sub_block::SubBlock;

pub type Coords = (usize, usize);
