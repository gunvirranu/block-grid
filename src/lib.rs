//! A quick, cache-conscious, blocked 2D array.
//!
//! `block-grid` gives you a fixed-size 2D array with a blocked / tiled memory representation.
//! This has the sweet benefit of being much more cache-friendly if you're often accessing nearby
//! coordinates. It also offers a bunch of utility methods and block access.
//!
//! # Example
//!
//! The following example offers a tour of basic usage and some features.
//!
//! ```
//! use block_grid::{BlockGrid, CoordsIterator, U2};
//!
//! let data: Vec<_> = (0..(4 * 6)).collect();
//!
//! // Construct from row-major ordered data
//! let grid = BlockGrid::<usize, U2>::from_row_major(4, 6, &data)?;
//!
//! // The 2D grid looks like:
//! // +-----------------------+
//! // |  0  1 |  2  3 |  4  5 |
//! // |  6  7 |  8  9 | 10 11 |
//! // |-------+-------+-------|
//! // | 12 13 | 14 15 | 16 17 |
//! // | 18 19 | 20 21 | 22 23 |
//! // +-----------------------+
//!
//! // Indexing
//! assert_eq!(grid[(1, 3)], 9);
//!
//! // Access raw array
//! let first_five = &grid.raw()[..5];
//! assert_eq!(first_five, &[0, 1, 6, 7, 2]);
//!
//! // Iterate over blocks, and access the last
//! let block = grid.block_iter().last().unwrap();
//! assert_eq!(block[(0, 1)], 17);
//!
//! // Iterate in row-major order
//! for (i, &x) in grid.row_major_iter().enumerate() {
//!     assert_eq!(x, i);
//! }
//!
//! // Iterate in memory order, with coordinates
//! for ((row, col), &x) in grid.each_iter().coords() {
//!     assert_eq!(row * 6 + col, x);
//! }
//!
//! # Ok::<(), ()>(())
//! ```
//!
//! # Usage
//!
//! ## Types
//!
//! The primary type is [`BlockGrid<T, B>`], where `T` is the stored type and `B` is a generic
//! parameter that controls the block size (all the `U*` types below). A view of a 2D block,
//! which is stored as a contiguous piece of memory, is a [`Block`] or [`BlockMut`].
//!
//! ## Indexing
//!
//! Indexing is by a pair of 2D coordinates, [`Coords`], which is simply a tuple `(row, column`).
//! You can use `[(i, j)]` or one of the many functions. When indexing elements in a specific
//! [`Block`] or [`BlockMut`], the coordinates are relative, meaning it's the row and column
//! *within* that block.
//!
//! ### Element Coordinates vs. Block Coordinates
//!
//! Coordinates typically refer to the locations of specific elements, but they can *also* be used
//! to index entire blocks. When using [`BlockGrid::block_iter`] chained by a
//! [`.coords()`][coords] call, or the [`Block::coords`] method, the returned *block coordinates*
//! instead refer to the entire block. This means that `(i, j`) would refer to the `i`-th *row of
//! blocks* and then the `j`-th block in that row. If you want the coordinates of the first
//! (top-left) element in a block, use the [`Block::starts_at`] method instead.
//!
//! [coords]: CoordsIterator::coords
//!
//! ## Iterating
//!
//! There are multiple ways of iterating over a 2D array. If you simply want to visit each
//! element, use [`BlockGrid::each_iter`]. You can alternatively iterate in row-major order using
//! [`BlockGrid::row_major_iter`]. Instead of iterating over elements, you can also iterate over
//! entire blocks with [`BlockGrid::block_iter`]. For any of these, if you also need coordinates
//! while iterating, you can chain a [`.coords()`][coords] call. If you only need a 1D iteration
//! count, then there's always [`Iterator::enumerate`].
//!
//! [coords]: CoordsIterator::coords
//!
//! # Optional Features
//!
//! ## Serde
//!
//! To use the [`serde`][serde] framework, enable the optional `serde` [feature] in your
//! `Cargo.toml`. There is an important sublety to its usage. Because the block size is generic
//! and compile-time, you have to know `B` when deserializing. You *could* write it decide based
//! on the input data, but I think it would lead to a bunch of extra code-gen, so I've left it
//! out. It does, however, verify that `B` is the same value as the one originally used to
//! serialize. If you always know the `B` value, then this shouldn't matter at all.
//!
//! [serde]: https://crates.io/crates/serde
//! [feature]: https://doc.rust-lang.org/cargo/reference/features.html

#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(rust_2018_idioms)]
#![no_std]

extern crate alloc;

#[cfg(test)]
#[macro_use]
extern crate std;

mod block_grid;
mod block_width;
pub mod iters;

#[cfg(test)]
mod tests;

pub use crate::block_grid::*;
pub use crate::block_width::*;
pub use crate::iters::CoordsIterator;

/// Type alias for a 2-tuple of indices, representing 2D coordinates.
pub type Coords = (usize, usize);
