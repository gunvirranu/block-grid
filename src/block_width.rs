pub trait BlockDim: Clone {
    const SHIFT: usize;

    const WIDTH: usize = 1 << Self::SHIFT;
    const AREA: usize = Self::WIDTH * Self::WIDTH;
    const MASK: usize = Self::WIDTH - 1;
}

macro_rules! make_block_width [
    ($($name: ident, $shift: literal);*) => {
        $(
            #[derive(Clone, Copy, Debug)]
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
