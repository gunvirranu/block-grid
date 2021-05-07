/// A type that represents compile time block dimensions.
pub trait BlockDim: Clone {
    /// Number of left shifts of 1 for value.
    const SHIFT: usize;

    /// Width and height of 2D block.
    const WIDTH: usize = 1 << Self::SHIFT;
    /// Number of elements in 2D block.
    const AREA: usize = Self::WIDTH * Self::WIDTH;
    /// Bitmask for value.
    const MASK: usize = Self::WIDTH - 1;

    /// Rounds up dimensions to next valid size. Returns `(rows, cols)`.
    ///
    /// # Example
    ///
    /// ```
    /// use block_grid::{BlockDim, U4};
    ///
    /// // (3, 10) are not valid dimensions for a block size of 4
    /// let new_valid = U4::round_up_to_valid(3, 10);
    /// // (4, 12) are the returned valid dimensions
    /// assert_eq!(new_valid, (4, 12));
    /// ```
    fn round_up_to_valid(rows: usize, cols: usize) -> (usize, usize) {
        let round_up = |i: usize| {
            let mut i = i.max(1);
            let rem = i % Self::WIDTH;
            if rem != 0 {
                i += Self::WIDTH - rem;
            }
            i
        };
        (round_up(rows), round_up(cols))
    }
}

macro_rules! make_block_width [
    ($($name: ident, $shift: literal);*) => {
        $(
            #[allow(missing_docs)]
            #[derive(Clone, Copy, Debug, Eq, PartialEq)]
            pub struct $name;

            impl BlockDim for $name {
                const SHIFT: usize = $shift;
            }
        )*
    }
];

make_block_width![
    U2,   1;
    U4,   2;
    U8,   3;
    U16,  4;
    U32,  5;
    U64,  6;
    U128, 7;
    U256, 8;
    U512, 9
];
