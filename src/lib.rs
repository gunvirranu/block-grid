#![warn(missing_debug_implementations)]
#![warn(rust_2018_idioms)]
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
pub use crate::block_width::*;
pub use crate::iters::CoordsIterator;

pub type Coords = (usize, usize);
