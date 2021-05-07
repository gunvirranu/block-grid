// FIXME: Fix and remove eventally
#![allow(clippy::result_unit_err)]

use alloc::{vec, vec::Vec};
use core::marker::PhantomData;
use core::ops::{Index, IndexMut};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::iters::{BlockIter, BlockIterMut, EachIter, EachIterMut, RowMajorIter, RowMajorIterMut};
use crate::{BlockDim, Coords};

/// A fixed-size 2D array with a blocked memory representation.
///
/// See [crate-level documentation][crate] for general usage info.
///
/// If your dimensions are not a multiple of the block size, you can use the helper function
/// [`BlockDim::round_up_to_valid`] to generate larger, valid dimensions.
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(bound(serialize = "T: Clone + Serialize")))]
#[cfg_attr(feature = "serde", serde(try_from = "serde_hack::ShadowBlockGrid<T>"))]
#[cfg_attr(feature = "serde", serde(into = "serde_hack::ShadowBlockGrid<T>"))]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct BlockGrid<T, B: BlockDim> {
    rows: usize,
    cols: usize,
    col_blocks: usize,
    buf: Vec<T>,
    _phantom: PhantomData<B>,
}

/// A view of a 2D block contiguous in memory.
///
/// Can be obtained via [`BlockIter`], which is created by calling [`BlockGrid::block_iter`].
#[derive(Clone, Copy, Debug)]
pub struct Block<'a, T, B: BlockDim> {
    block_coords: Coords,
    arr: &'a [T],
    _phantom: PhantomData<B>,
}

/// A mutable view of a 2D block contiguous in memory.
///
/// Can be obtained via [`BlockIterMut`], which is created by calling [`BlockGrid::block_iter_mut`].
#[derive(Debug)]
pub struct BlockMut<'a, T, B: BlockDim> {
    block_coords: Coords,
    arr: &'a mut [T],
    _phantom: PhantomData<B>,
}

impl<T, B: BlockDim> BlockGrid<T, B> {
    /// Constructs a `BlockGrid<T, B>` by consuming a [`Vec<T>`].
    ///
    /// The ordering of the memory is taken as is in the vector.
    ///
    /// # Errors
    ///
    /// If invalid dimensions, either because `rows` and `cols` do not divide evenly into the block
    /// size `B` or the length of `elems` does not match `rows * cols`.
    pub fn from_raw_vec(rows: usize, cols: usize, elems: Vec<T>) -> Result<Self, ()> {
        if !Self::valid_size(rows, cols) || rows * cols != elems.len() {
            return Err(());
        }
        Ok(Self {
            rows,
            cols,
            col_blocks: cols / B::WIDTH,
            buf: elems,
            _phantom: PhantomData,
        })
    }

    /// Converts a `BlockGrid<T, B>` to a [`Vec<T>`] in memory order.
    #[inline]
    pub fn take_raw_vec(self) -> Vec<T> {
        self.buf
    }

    /// Returns the nuumber of rows.
    #[inline]
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Returns the number of columns.
    #[inline]
    pub fn cols(&self) -> usize {
        self.cols
    }

    /// Returns the number of elements.
    #[inline]
    pub fn size(&self) -> usize {
        self.rows() * self.cols()
    }

    /// Returns the number of blocks in the vertical direction.
    #[inline]
    pub fn row_blocks(&self) -> usize {
        self.rows / B::WIDTH
    }

    /// Returns the number of blocks in the horizontal direction.
    #[inline]
    pub fn col_blocks(&self) -> usize {
        self.col_blocks
    }

    /// Returns the total number of blocks.
    #[inline]
    pub fn blocks(&self) -> usize {
        self.row_blocks() * self.col_blocks()
    }

    /// Returns `true` if the given coordinates are valid.
    #[inline]
    pub fn contains(&self, (row, col): Coords) -> bool {
        row < self.rows && col < self.cols
    }

    /// Returns a reference to the element at the given coordinates, or [`None`] if they are
    /// out-of-bounds.
    #[inline]
    pub fn get(&self, coords: Coords) -> Option<&T> {
        if !self.contains(coords) {
            return None;
        }
        // SAFETY: `coords` is a valid index
        Some(unsafe { self.get_unchecked(coords) })
    }

    /// Returns a mutable reference to the element at the given coordinates, or [`None`] if they
    /// are out-of-bounds.
    #[inline]
    pub fn get_mut(&mut self, coords: Coords) -> Option<&mut T> {
        if !self.contains(coords) {
            return None;
        }
        // SAFETY: `coords` is a valid index
        Some(unsafe { self.get_unchecked_mut(coords) })
    }

    /// Returns a reference to the element at the given coordinates, without bounds checking.
    ///
    /// # Safety
    ///
    /// Calling this method with out-of-bounds coordinates is *undefined-behaviour*.
    #[inline]
    pub unsafe fn get_unchecked(&self, coords: Coords) -> &T {
        debug_assert!(self.contains(coords));
        let ind = self.calc_index(coords);
        self.buf.get_unchecked(ind)
    }

    /// Returns a mutable reference to the element at the given coordinates, without bounds
    /// checking.
    ///
    /// # Safety
    ///
    /// Calling this method with out-of-bounds coordinates is *undefined-behaviour*.
    #[inline]
    pub unsafe fn get_unchecked_mut(&mut self, coords: Coords) -> &mut T {
        debug_assert!(self.contains(coords));
        let ind = self.calc_index(coords);
        self.buf.get_unchecked_mut(ind)
    }

    /// Returns all elements as a slice in memory order.
    #[inline]
    pub fn raw(&self) -> &[T] {
        &self.buf
    }

    /// Returns all elements as a mutable slice in memory order.
    #[inline]
    pub fn raw_mut(&mut self) -> &mut [T] {
        &mut self.buf
    }

    /// Returns an iterator over all the elements in memory order.
    ///
    /// If you wanna visit each element arbitrarily, this would be the best way. If you also need
    /// coordinates while iterating, follow up with a chained [`.coords()`][coords] call.
    ///
    /// [coords]: crate::CoordsIterator::coords()
    #[inline]
    pub fn each_iter(&self) -> EachIter<'_, T, B> {
        EachIter::new(self)
    }

    /// Returns a mutable iterator over all the elements in memory order.
    ///
    /// If you wanna mutably visit each element arbitrarily, this would be the best way. If you
    /// also need coordinates while iterating, follow up with a chained [`.coords()`][coords] call.
    ///
    /// [coords]: crate::CoordsIterator::coords()
    #[inline]
    pub fn each_iter_mut(&mut self) -> EachIterMut<'_, T, B> {
        EachIterMut::new(self)
    }

    /// Returns an iterator over all blocks in memory order, yielding [`Block`]s.
    ///
    /// If you need the block coordinates while iterating, follow up with a chained
    /// [`.coords()`][coords] call. In this case, note that the 2D coordinates yielded are of the
    /// actual entire block. If you instead need the coordinates of the first (top-left) element
    /// in the block, see [`Block::starts_at`].
    ///
    /// [coords]: crate::CoordsIterator::coords()
    #[inline]
    pub fn block_iter(&self) -> BlockIter<'_, T, B> {
        BlockIter::new(self)
    }

    /// Returns a mutable iterator over all blocks in memory order, yielding [`BlockMut`]s.
    ///
    /// If you need the block coordinates while iterating, follow up with a chained
    /// [`.coords()`][coords] call. In this case, note that the 2D coordinates yielded are of the
    /// actual entire block. If you instead need the coordinates of the first (top-left) element
    /// in the block, see [`BlockMut::starts_at`].
    ///
    /// [coords]: crate::CoordsIterator::coords()
    #[inline]
    pub fn block_iter_mut(&mut self) -> BlockIterMut<'_, T, B> {
        BlockIterMut::new(self)
    }

    /// Returns an iterator over all the elements in [row-major order][row_major].
    ///
    /// This ordering is what you're probably used to with usual 2D arrays. This method may be
    /// useful for converting between array types or general IO. If you also need the coordinates
    /// while iterating, follow up with a chained [`.coords()`][coords] call.
    ///
    /// [row_major]: https://en.wikipedia.org/wiki/Row-_and_column-major_order
    /// [coords]: crate::CoordsIterator::coords()
    #[inline]
    pub fn row_major_iter(&self) -> RowMajorIter<'_, T, B> {
        RowMajorIter::new(self)
    }

    /// Returns an mutable iterator over all the elements in [row-major order][row_major].
    ///
    /// If you also need the coordinates while iterating, follow up with a chained
    /// [`.coords()`][coords] call.
    ///
    /// [row_major]: https://en.wikipedia.org/wiki/Row-_and_column-major_order
    /// [coords]: crate::CoordsIterator::coords()
    #[inline]
    pub fn row_major_iter_mut(&mut self) -> RowMajorIterMut<'_, T, B> {
        RowMajorIterMut::new(self)
    }

    /// Returns `true` if `rows` and `cols` form a valid sized `BlockGrid<T, B>`.
    fn valid_size(rows: usize, cols: usize) -> bool {
        rows > 0 && cols > 0 && rows % B::WIDTH == 0 && cols % B::WIDTH == 0
    }

    /// Returns the 1D memory index calculated from 2D coordinates.
    fn calc_index(&self, (row, col): Coords) -> usize {
        // Get block
        let (b_row, b_col) = (row / B::WIDTH, col / B::WIDTH);
        let block_ind = B::AREA * (self.col_blocks() * b_row + b_col);
        // Offset within block
        let (s_row, s_col) = (row % B::WIDTH, col % B::WIDTH);
        let sub_ind = B::WIDTH * s_row + s_col;
        block_ind + sub_ind
    }
}

impl<T: Clone, B: BlockDim> BlockGrid<T, B> {
    /// Constructs a `BlockGrid<T, B>` by filling with a single element.
    ///
    /// # Errors
    ///
    /// If  `rows` and `cols` do not divide evenly into the block size `B`.
    pub fn filled(rows: usize, cols: usize, elem: T) -> Result<Self, ()> {
        if !Self::valid_size(rows, cols) {
            return Err(());
        }
        Ok(Self {
            rows,
            cols,
            col_blocks: cols / B::WIDTH,
            buf: vec![elem; rows * cols],
            _phantom: PhantomData,
        })
    }

    /// Constructs a `BlockGrid<T, B>` from a slice in [row-major order][row_major].
    ///
    /// This method may be useful for converting from a typical 2D array.
    ///
    /// # Errors
    ///
    /// If invalid dimensions, either because `rows` and `cols` do not divide evenly into the block
    /// size `B` or the length of `elems` does not match `rows * cols`.
    ///
    /// [row_major]: https://en.wikipedia.org/wiki/Row-_and_column-major_order
    pub fn from_row_major(rows: usize, cols: usize, elems: &[T]) -> Result<Self, ()> {
        Self::from_array_index_helper(rows, cols, elems, |row, col| cols * row + col)
    }

    /// Constructs a `BlockGrid<T, B>` from a slice in [column-major order][col_major].
    ///
    /// 2D arrays are not usually stored like this, but occasionally they are.
    ///
    /// # Errors
    ///
    /// If invalid dimensions, either because `rows` and `cols` do not divide evenly into the block
    /// size `B` or the length of `elems` does not match `rows * cols`.
    ///
    /// [col_major]: https://en.wikipedia.org/wiki/Row-_and_column-major_order
    pub fn from_col_major(rows: usize, cols: usize, elems: &[T]) -> Result<Self, ()> {
        Self::from_array_index_helper(rows, cols, elems, |row, col| rows * col + row)
    }

    /// Helper method to convert from a differently ordered array to a `BlockGrid<T, B>`.
    fn from_array_index_helper(
        rows: usize,
        cols: usize,
        elems: &[T],
        calc_index: impl Fn(usize, usize) -> usize,
    ) -> Result<Self, ()> {
        if !Self::valid_size(rows, cols) || rows * cols != elems.len() {
            return Err(());
        }
        let mut grid = Self {
            rows,
            cols,
            col_blocks: cols / B::WIDTH,
            buf: Vec::with_capacity(rows * cols),
            _phantom: PhantomData,
        };
        // Iterate in memory order by index and pull values from `elems`
        for bi in (0..grid.rows()).step_by(B::WIDTH) {
            for bj in (0..grid.cols()).step_by(B::WIDTH) {
                for si in 0..B::WIDTH {
                    for sj in 0..B::WIDTH {
                        let (row, col) = (bi + si, bj + sj);
                        let ind = calc_index(row, col);
                        // There's no 'simple' way to do this without `Clone`,
                        // because `elems` can't be easily drained out of order.
                        grid.buf.push(elems[ind].clone());
                    }
                }
            }
        }
        debug_assert_eq!(grid.buf.len(), grid.size());
        Ok(grid)
    }
}

impl<T: Clone + Default, B: BlockDim> BlockGrid<T, B> {
    /// Constructs a `BlockGrid<T, B>` by filling with the default value of `T`.
    ///
    /// # Errors
    ///
    /// If  `rows` and `cols` do not divide evenly into the block size `B`.
    pub fn new(rows: usize, cols: usize) -> Result<Self, ()> {
        Self::filled(rows, cols, T::default())
    }
}

impl<T, B: BlockDim> Index<Coords> for BlockGrid<T, B> {
    type Output = T;

    #[inline]
    fn index(&self, coords: Coords) -> &Self::Output {
        self.get(coords).expect("Index out of bounds")
    }
}

impl<T, B: BlockDim> IndexMut<Coords> for BlockGrid<T, B> {
    #[inline]
    fn index_mut(&mut self, coords: Coords) -> &mut Self::Output {
        self.get_mut(coords).expect("Index out of bounds")
    }
}

impl<'a, T, B: BlockDim> Block<'a, T, B> {
    /// Constructs a `Block<'a, T, B>` from an array slice.
    ///
    /// # Safety
    ///
    /// `block_coords` *must* be valid and `arr` *must* be of length `B::AREA`.
    pub(crate) unsafe fn new(block_coords: Coords, arr: &'a [T]) -> Self {
        debug_assert_eq!(arr.len(), B::AREA);
        Self {
            block_coords,
            arr,
            _phantom: PhantomData,
        }
    }

    /// Returns the coordinates of the entire block.
    ///
    /// Block coordinates mean that the `(i, j)` refers to the `i`-th *row of blocks* and the
    /// `j`-th block in that row. If you need the coordinates of the first (top-left) element,
    /// use [`starts_at`] instead.
    ///
    /// [`starts_at`]: Self::starts_at
    #[inline]
    pub fn coords(&self) -> Coords {
        self.block_coords
    }

    /// Returns the coordinates of the first (top-left) element in the block.
    #[inline]
    pub fn starts_at(&self) -> Coords {
        let (b_row, b_col) = self.block_coords;
        (B::WIDTH * b_row, B::WIDTH * b_col)
    }

    /// Returns `true` if the given coordinates are valid.
    #[inline]
    pub fn contains(&self, (row, col): Coords) -> bool {
        row < B::WIDTH && col < B::WIDTH
    }

    /// Returns a reference to the element at the given coordinates, or [`None`] if they are
    /// out-of-bounds.
    #[inline]
    pub fn get(&self, coords: Coords) -> Option<&T> {
        if !self.contains(coords) {
            return None;
        }
        // SAFETY: `coords` is a valid index
        Some(unsafe { self.get_unchecked(coords) })
    }

    /// Returns a reference to the element at the given coordinates, without bounds checking.
    ///
    /// # Safety
    ///
    /// Calling this method with out-of-bounds coordinates is *undefined-behaviour*.
    #[inline]
    pub unsafe fn get_unchecked(&self, coords: Coords) -> &T {
        debug_assert!(self.contains(coords));
        self.arr.get_unchecked(self.calc_index(coords))
    }

    /// Returns all elements in block as a slice in memory order.
    #[inline]
    pub fn raw(&self) -> &[T] {
        self.arr
    }

    /// Returns the 1D memory index calculated from 2D coordinates.
    fn calc_index(&self, (row, col): Coords) -> usize {
        B::WIDTH * row + col
    }
}

impl<'a, T, B: BlockDim> Index<Coords> for Block<'a, T, B> {
    type Output = T;

    #[inline]
    fn index(&self, coords: Coords) -> &Self::Output {
        self.get(coords).expect("Index out of bounds")
    }
}

impl<'a, T, B: BlockDim> BlockMut<'a, T, B> {
    /// Constructs a `BlockMut<'a, T, B>` from an array slice.
    ///
    /// # Safety
    ///
    /// `block_coords` *must* be valid and `arr` *must* be of length `B::AREA`.
    pub(crate) unsafe fn new(block_coords: Coords, arr: &'a mut [T]) -> Self {
        debug_assert_eq!(arr.len(), B::AREA);
        Self {
            block_coords,
            arr,
            _phantom: PhantomData,
        }
    }

    /// Returns the coordinates of the entire block.
    ///
    /// Block coordinates mean that the `(i, j)` refers to the `i`-th *row of blocks* and the
    /// `j`-th block in that row. If you need the coordinates of the first (top-left) element,
    /// use [`starts_at`] instead.
    ///
    /// [`starts_at`]: Self::starts_at
    #[inline]
    pub fn coords(&self) -> Coords {
        self.block_coords
    }

    /// Returns of the coordinates of the first (top-left) element in the block.
    #[inline]
    pub fn starts_at(&self) -> Coords {
        let (b_row, b_col) = self.block_coords;
        (B::WIDTH * b_row, B::WIDTH * b_col)
    }

    /// Returns `true` if the given coordinates are valid.
    #[inline]
    pub fn contains(&self, (row, col): Coords) -> bool {
        row < B::WIDTH && col < B::WIDTH
    }

    /// Returns a reference to the element at the given coordinates, or [`None`] if they are
    /// out-of-bounds.
    #[inline]
    pub fn get(&self, coords: Coords) -> Option<&T> {
        if !self.contains(coords) {
            return None;
        }
        // SAFETY: `coords` is a valid index
        Some(unsafe { self.get_unchecked(coords) })
    }

    /// Returns a mutable reference to the element at the given coordinates, or [`None`] if they
    /// are out-of-bounds.
    #[inline]
    pub fn get_mut(&mut self, coords: Coords) -> Option<&mut T> {
        if !self.contains(coords) {
            return None;
        }
        // SAFETY: `coords` is a valid index
        Some(unsafe { self.get_unchecked_mut(coords) })
    }

    /// Returns a reference to the element at the given coordinates, without bounds checking.
    ///
    /// # Safety
    ///
    /// Calling this method with out-of-bounds coordinates is *undefined-behaviour*.
    #[inline]
    pub unsafe fn get_unchecked(&self, coords: Coords) -> &T {
        debug_assert!(self.contains(coords));
        self.arr.get_unchecked(self.calc_index(coords))
    }

    /// Returns a mutable reference to the element at the given coordinates, without bounds
    /// checking.
    ///
    /// # Safety
    ///
    /// Calling this method with out-of-bounds coordinates is *undefined-behaviour*.
    #[inline]
    pub unsafe fn get_unchecked_mut(&mut self, coords: Coords) -> &mut T {
        debug_assert!(self.contains(coords));
        self.arr.get_unchecked_mut(self.calc_index(coords))
    }

    /// Returns all elements in block as a slice in memory order.
    #[inline]
    pub fn raw(&self) -> &[T] {
        self.arr
    }

    /// Returns all elements in block as a mutable slice in memory order.
    #[inline]
    pub fn raw_mut(&mut self) -> &mut [T] {
        self.arr
    }

    /// Returns the 1D memory index calculated from 2D coordinates.
    fn calc_index(&self, (row, col): Coords) -> usize {
        B::WIDTH * row + col
    }
}

impl<'a, T, B: BlockDim> Index<Coords> for BlockMut<'a, T, B> {
    type Output = T;

    #[inline]
    fn index(&self, coords: Coords) -> &Self::Output {
        self.get(coords).expect("Coordinates out of bounds")
    }
}

impl<'a, T, B: BlockDim> IndexMut<Coords> for BlockMut<'a, T, B> {
    #[inline]
    fn index_mut(&mut self, coords: Coords) -> &mut Self::Output {
        self.get_mut(coords).expect("Coordinates out of bounds")
    }
}

#[cfg(feature = "serde")]
mod serde_hack {
    use super::*;
    use core::convert::{From, TryFrom};
    use core::fmt;

    /// Error if invalid dimensions are passed in or deserialized.
    ///
    /// Currently, only used for `serde` deserialization, but in the future, this should be used
    /// for the [`BlockGrid<T, B>`] constructors as well.
    #[derive(Debug)]
    pub(super) struct InvalidSizeError;

    impl fmt::Display for InvalidSizeError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Dimensions are invalid")
        }
    }

    /// A "trick" to avoid writing (de)serialization code with validation.
    ///
    /// See PR for details.
    #[derive(Deserialize, Serialize)]
    pub(super) struct ShadowBlockGrid<T> {
        rows: usize,
        cols: usize,
        #[serde(rename = "b")]
        bwidth: usize,
        buf: Vec<T>,
    }

    // Serialization
    impl<T, B: BlockDim> From<BlockGrid<T, B>> for ShadowBlockGrid<T> {
        fn from(bgrid: BlockGrid<T, B>) -> Self {
            // Assumes `bgrid` is in valid state
            Self {
                rows: bgrid.rows(),
                cols: bgrid.cols(),
                bwidth: B::WIDTH,
                buf: bgrid.take_raw_vec(),
            }
        }
    }

    // Deserialization
    impl<T, B: BlockDim> TryFrom<ShadowBlockGrid<T>> for BlockGrid<T, B> {
        type Error = InvalidSizeError;

        fn try_from(shadow: ShadowBlockGrid<T>) -> Result<Self, Self::Error> {
            let ShadowBlockGrid {
                rows,
                cols,
                bwidth,
                buf,
            } = shadow;
            // Check that deserialized data is a valid state
            if bwidth != B::WIDTH {
                return Err(InvalidSizeError);
            }
            Self::from_raw_vec(rows, cols, buf).map_err(|_| InvalidSizeError)
        }
    }
}
