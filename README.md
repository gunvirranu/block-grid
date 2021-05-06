<div class="title-block" style="text-align: center;" align="center">

# `block-grid`

#### A quick, cache-conscious, blocked 2D array

[![Crate][crate_badge]][crate]
[![Docs][docs_badge]][docs]
[![License][license_badge]][license]
[![CI][ci_badge]][ci]

</div>

`block-grid` gives you a fixed size, two-dimensional array, with a blocked memory representation. This has the sweet benefit of being much more cache-friendly if you're often accessing nearby coordinates.

## Features

- Can store any type
- Generic compile-time block sizes
- Indexing with `(row, col): (usize, usize)`
- Block level access with `Block` and `BlockMut`
- Constructors from row-major and column-major order arrays
- Iterators for in-memory and row-major order, and by block
- `no_std` and [`serde`][serde] support

## Example

```rust
use block_grid::{BlockGrid, CoordsIterator, U2};

fn main() {
    let data: Vec<_> = (0..(4 * 6)).collect();

    // Construct from row-major ordered data
    let grid = BlockGrid::<usize, U2>::from_row_major(4, 6, &data).unwrap();

    // The 2D grid looks like:
    // +-----------------------+
    // |  0  1 |  2  3 |  4  5 |
    // |  6  7 |  8  9 | 10 11 |
    // |-------+-------+-------|
    // | 12 13 | 14 15 | 16 17 |
    // | 18 19 | 20 21 | 22 23 |
    // +-----------------------+

    // Indexing
    assert_eq!(grid[(1, 3)], 9);

    // Access raw array
    let first_five = &grid.raw()[..5];
    assert_eq!(first_five, &[0, 1, 6, 7, 2]);

    // Iterate over blocks, and access the last
    let block = grid.block_iter().last().unwrap();
    assert_eq!(block[(0, 1)], 17);

    // Iterate in row-major order
    for (i, &x) in grid.row_major_iter().enumerate() {
        assert_eq!(x, i);
    }

    // Iterate in memory order, with coordinates
    for ((row, col), &x) in grid.each_iter().coords() {
        assert_eq!(row * 6 + col, x);
    }
}
```

## Why

TODO: Stuff about caches

## Trade-offs

- Non-resizable, and grid dimensions have to be a multiple of the block size.
- Currently, only square blocks, and power-of-two block sizes are supported.
- Computing the modified index takes just a bit more time.
- There are still cache misses when you cross tile boundaries.
- No support for strides or general subsets.

## Changelog

See [`CHANGELOG.md`](CHANGELOG.md).

## License

`block-grid` is licensed under the [MIT license](LICENSE).

## Alternatives

If your access patterns suit a typical row-major memory representation, check out [`array2d`][array2d], [`imgref`][imgref], [`grid`][grid], or [`toodee`][toodee]. The last two support dynamic resizing. For matrices and linear algebra, there's also [`nalgebra`][nalgebra].

<!-- Links -->
[serde]: https://crates.io/crates/serde "serde"
[array2d]: https://crates.io/crates/array2d "array2d"
[imgref]: https://crates.io/crates/imgref "imgref"
[grid]: https://crates.io/crates/grid "grid"
[toodee]: https://crates.io/crates/toodee "toodee"
[nalgebra]: https://nalgebra.org "nalgebra"

<!-- Badges -->
[crate]: https://crates.io/crates/block-grid "Crate"
[crate_badge]: https://img.shields.io/crates/v/block-grid?logo=rust "Crate"
[docs]: https://docs.rs/block-grid "Docs"
[docs_badge]: https://docs.rs/block-grid/badge.svg "Docs"
[ci]: https://github.com/gunvirranu/block-grid/actions "Github Actions"
[ci_badge]: https://github.com/gunvirranu/block-grid/workflows/CI/badge.svg?branch=master "Github Actions"
[license]: #license "License"
[license_badge]: https://img.shields.io/badge/license-MIT-blue.svg "License"
