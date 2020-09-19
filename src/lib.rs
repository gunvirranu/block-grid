#![no_std]

extern crate alloc;

#[cfg(test)]
#[macro_use]
extern crate std;

mod block_grid;
mod block_width;
mod iters;

#[cfg(test)]
mod tests;

pub use crate::block_grid::{Block, BlockGrid, BlockMut};
pub use crate::block_width::{BlockDim, BlockWidth};
pub use crate::iters::CoordsIterator;

pub type Coords = (usize, usize);
