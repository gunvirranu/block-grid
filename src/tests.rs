use crate::*;

type T = usize;
// TODO: Look into testing generically in a not-ugly way
type B = BlockWidth::U2;
type BGrid = BlockGrid<T, B>;

const GOOD_SIZES: &[Coords] = &[(2, 2), (4, 6), (100, 100), (64, 256)];
const BAD_SIZES: &[Coords] = &[(0, 0), (0, 1), (3, 5), (7, 13)];

#[test]
fn test_from_raw_vec() {
    for &(rows, cols) in GOOD_SIZES {
        let data: Vec<T> = (0..(rows * cols)).collect();
        let grid = BGrid::from_raw_vec(rows, cols, data.clone()).unwrap();
        assert_eq!(data.len(), grid.size());
        for (&x, &y) in grid.each_iter().zip(data.iter()) {
            assert_eq!(x, y);
        }
    }
}

#[test]
fn test_from_raw_vec_invalid() {
    for &(rows, cols) in &[(2, 2), (4, 6), (2048, 8192)] {
        let data: Vec<T> = (0..(rows * cols)).collect();
        let grid = BGrid::from_raw_vec(rows + 1, cols + 1, data);
        assert!(grid.is_err());
    }

    for &(rows, cols) in BAD_SIZES {
        let data: Vec<T> = (0..(rows * cols)).collect();
        let grid = BGrid::from_raw_vec(rows, cols, data);
        assert!(grid.is_err());
    }
}

#[test]
fn test_filled() {
    for &(rows, cols) in GOOD_SIZES {
        let grid = BGrid::filled(rows, cols, 7).unwrap();
        assert_eq!(grid.size(), rows * cols);
        for &x in grid.each_iter() {
            assert_eq!(x, 7);
        }
    }
}

#[test]
fn test_filled_invalid() {
    for &(rows, cols) in BAD_SIZES {
        let grid = BGrid::filled(rows, cols, 7);
        assert!(grid.is_err());
    }
}

#[test]
fn test_row_col_size() {
    for &(rows, cols) in GOOD_SIZES {
        let grid = BGrid::new(rows, cols).unwrap();
        assert_eq!(grid.rows(), rows);
        assert_eq!(grid.cols(), cols);
        assert_eq!(grid.size(), rows * cols);
    }
}

#[test]
fn test_get_and_get_mut() {
    for &(rows, cols) in GOOD_SIZES {
        let mut grid = BGrid::filled(rows, cols, 7).unwrap();
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
}

#[test]
fn test_block_size() {
    for &(rows, cols) in GOOD_SIZES {
        let grid = BGrid::new(rows, cols).unwrap();
        assert_eq!(grid.row_blocks() * B::WIDTH, rows);
        assert_eq!(grid.col_blocks() * B::WIDTH, cols);
        assert_eq!(grid.blocks() * B::AREA, grid.size());
    }
}

#[test]
fn test_block_iter() {
    for &(rows, cols) in GOOD_SIZES {
        let data: Vec<T> = (0..(rows * cols)).collect();
        let grid = BGrid::from_raw_vec(rows, cols, data).unwrap();
        assert_eq!(grid.block_iter().count(), grid.blocks());

        let (mut bi, mut bj): Coords = (0, 0);
        for block in grid.block_iter() {
            for si in 0..B::WIDTH {
                for sj in 0..B::WIDTH {
                    assert_eq!(block[(si, sj)], grid[(bi + si, bj + sj)]);
                }
            }
            bj += B::WIDTH;
            if bj == grid.cols() {
                bj = 0;
                bi += B::WIDTH;
            }
        }
    }
}

#[test]
fn test_row_major_iter() {
    for &(rows, cols) in GOOD_SIZES {
        let data: Vec<T> = (0..(rows * cols)).collect();
        let grid = BGrid::from_raw_vec(rows, cols, data).unwrap();
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
}
