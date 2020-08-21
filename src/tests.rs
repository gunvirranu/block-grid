use crate::{BlockWidth::*, *};

type BG<T, B> = BlockGrid<T, B>;

const BAD_SIZES: &[Coords] = &[(0, 0), (1, 0), (0, 1), (3, 5), (7, 13)];

fn gen_from_raw_vec<B: BlockDim>() {
    let (rows, cols) = (2 * B::WIDTH, 3 * B::WIDTH);
    let data: Vec<_> = (0..(rows * cols)).collect();
    let grid = BG::<_, B>::from_raw_vec(rows, cols, data.clone()).unwrap();
    assert_eq!(grid.rows(), rows);
    assert_eq!(grid.cols(), cols);
    assert_eq!(grid.size(), data.len());
    for (&x, &y) in grid.each_iter().zip(data.iter()) {
        assert_eq!(x, y);
    }
}

fn gen_from_raw_vec_invalid<B: BlockDim>() {
    for &(rows, cols) in &[(2, 2), (3, 5), (4, 6)] {
        let data: Vec<_> = (0..(rows * cols)).collect();
        let grid = BG::<_, B>::from_raw_vec(rows + 1, cols + 1, data);
        assert!(grid.is_err());
    }
    for &(rows, cols) in BAD_SIZES {
        let data: Vec<_> = (0..(rows * cols)).collect();
        let grid = BG::<_, B>::from_raw_vec(rows, cols, data);
        assert!(grid.is_err());
    }
}

fn gen_filled<B: BlockDim>() {
    let (rows, cols) = (2 * B::WIDTH, 3 * B::WIDTH);
    let grid = BG::<_, B>::filled(rows, cols, 7).unwrap();
    assert_eq!(grid.size(), rows * cols);
    for &x in grid.each_iter() {
        assert_eq!(x, 7);
    }
}

fn gen_filled_invalid<B: BlockDim>() {
    for &(rows, cols) in BAD_SIZES {
        let grid = BG::<_, B>::filled(rows, cols, 7);
        assert!(grid.is_err());
    }
}

fn gen_get_and_get_mut<B: BlockDim>() {
    let (rows, cols) = (2 * B::WIDTH, 3 * B::WIDTH);
    let mut grid = BG::<_, B>::filled(rows, cols, 7).unwrap();
    // Try invalid coordinates
    for &coords in &[(rows, 0), (0, cols), (rows, cols)] {
        assert!(grid.get(coords).is_none());
    }
    // Test each coordinate, and mutate
    for i in 0..rows {
        for j in 0..cols {
            assert_eq!(*grid.get((i, j)).unwrap(), 7);
            let x = grid.get_mut((i, j)).unwrap();
            *x = cols * i + j;
        }
    }
    // Check again
    for i in 0..rows {
        for j in 0..cols {
            assert_eq!(*grid.get((i, j)).unwrap(), cols * i + j);
        }
    }
}

fn gen_block_size<B: BlockDim>() {
    for &(n, m) in &[(1, 1), (2, 3), (3, 1), (4, 4)] {
        let (rows, cols) = (n * B::WIDTH, m * B::WIDTH);
        let grid = BG::<usize, B>::new(rows, cols).unwrap();
        assert_eq!(grid.row_blocks(), n);
        assert_eq!(grid.col_blocks(), m);
        assert_eq!(grid.blocks() * B::AREA, grid.size());
    }
}

fn gen_block_iter<B: BlockDim>() {
    let (rows, cols) = (2 * B::WIDTH, 3 * B::WIDTH);
    let data: Vec<_> = (0..(rows * cols)).collect();
    let grid = BG::<_, B>::from_raw_vec(rows, cols, data).unwrap();
    assert_eq!(grid.block_iter().count(), grid.blocks());

    let (mut bi, mut bj): Coords = (0, 0);
    for block in grid.block_iter() {
        for si in 0..B::WIDTH {
            for sj in 0..B::WIDTH {
                assert_eq!(block[(si, sj)], grid[(bi + si, bj + sj)]);
            }
        }
        assert!(block.get((B::WIDTH, B::WIDTH - 1)).is_none());
        assert!(block.get((B::WIDTH - 1, B::WIDTH)).is_none());
        assert!(block.get((B::WIDTH, B::WIDTH)).is_none());

        bj += B::WIDTH;
        if bj == grid.cols() {
            bj = 0;
            bi += B::WIDTH;
        }
    }
}

fn gen_row_major_iter<B: BlockDim>() {
    let (rows, cols) = (2 * B::WIDTH, 3 * B::WIDTH);
    let data: Vec<_> = (0..(rows * cols)).collect();
    let grid = BG::<_, B>::from_raw_vec(rows, cols, data).unwrap();
    assert_eq!(grid.row_major_iter().count(), grid.size());

    let mut it = grid.row_major_iter();
    for i in 0..rows {
        for j in 0..cols {
            let (c, &e) = it.next().unwrap();
            assert_eq!(c, (i, j));
            assert_eq!(e, grid[(i, j)]);
        }
    }
    assert!(it.next().is_none());
}

fn gen_row_major_iter_mut<B: BlockDim>() {
    let (rows, cols) = (2 * B::WIDTH, 3 * B::WIDTH);
    let mut grid = BG::<_, B>::filled(rows, cols, 7).unwrap();
    assert_eq!(grid.row_major_iter_mut().count(), grid.size());
    // Mutate while iterating
    let mut it = grid.row_major_iter_mut();
    for i in 0..rows {
        for j in 0..cols {
            let (c, e) = it.next().unwrap();
            assert_eq!(c, (i, j));
            assert_eq!(*e, 7);
            *e = rows * i + j;
        }
    }
    assert!(it.next().is_none());
    // Check if mutated correctly
    for i in 0..rows {
        for j in 0..cols {
            assert_eq!(grid[(i, j)], rows * i + j);
        }
    }
}

macro_rules! test_for {
    ($f: ident; $($B: ty),+) => {
        $(
            eprintln!("Testing: {}<{}>", stringify!($f), stringify!($B));
            $f::<$B>();
        )+
    };
}

#[test]
fn test_from_raw_vec() {
    test_for!(gen_from_raw_vec; U2, U4, U8, U16, U32);
}

#[test]
fn test_from_raw_vec_invalid() {
    test_for!(gen_from_raw_vec_invalid; U2, U4, U8, U16, U32);
}

#[test]
fn test_filled() {
    test_for!(gen_filled; U2, U4, U8, U16, U32);
}

#[test]
fn test_filled_invalid() {
    test_for!(gen_filled_invalid; U2, U4, U8, U16, U32);
}

#[test]
fn test_get_and_get_mut() {
    test_for!(gen_get_and_get_mut; U2, U4, U8, U16, U32);
}

#[test]
fn test_block_size() {
    test_for!(gen_block_size; U2, U4, U8, U16, U32);
}

#[test]
#[ignore]
fn test_block_iter() {
    test_for!(gen_block_iter; U2, U4, U8, U16, U32);
}

#[test]
fn test_row_major_iter() {
    test_for!(gen_row_major_iter; U2, U4, U8, U16, U32);
}

#[test]
fn test_row_major_iter_mut() {
    test_for!(gen_row_major_iter_mut; U2, U4, U8, U16, U32);
}
