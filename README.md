<div class="title-block" style="text-align: center;" align="center">

# `block-grid`

#### A quick, cache-conscious, blocked 2D array

[![CI][ci_badge]][ci]
[![License][license_badge]][license]

</div>

`block-grid` gives you a fixed size, two-dimensional array, with a blocked memory representation. This has the sweet benefit of being much more cache-friendly if you're often accessing nearby coordinates.

## Features

- Can store any type
- Generic compile-time block sizes
- Indexing with `(row, col): (usize, usize)`
- Block level access with `Block` and `BlockMut`
- Constructors from row-major and column-major order arrays
- Iterators for in-memory and row-major order, and by block
- `no_std` support

## Quickstart

TODO: Add code example

## Why

TODO: Stuff about caches

## Trade-offs

- Non-resizable, and grid dimensions have to be a multiple of the block size.
- Currently, only square blocks, and power-of-two block sizes are supported.
- Computing the modified index takes just a bit more time.
- There are still cache misses when you cross tile boundaries.
- No support for strides or general subsets.

## Changelog

TODO: Point to `CHANGELOG.md` when created

## License

`block-grid` is licensed under the [MIT license](LICENSE).

## Alternatives

If your access patterns suit a typical row-major memory representation, check out [`array2d`][array2d], [`imgref`][imgref], [`grid`][grid], or [`toodee`][toodee]. The last two support dynamic resizing. For matrices and linear algebra, there's also [`nalgebra`][nalgebra].

<!-- Links -->
[array2d]: https://crates.io/crates/array2d "array2d"
[imgref]: https://crates.io/crates/imgref "imgref"
[grid]: https://crates.io/crates/grid "grid"
[toodee]: https://crates.io/crates/toodee "toodee"
[nalgebra]: https://nalgebra.org "nalgebra"

<!-- Badges -->
[ci]: https://github.com/gunvirranu/block-grid/actions "Github Actions"
[ci_badge]: https://github.com/gunvirranu/block-grid/workflows/CI/badge.svg?branch=master "Github Actions"
[license]: #license "License"
[license_badge]: https://img.shields.io/badge/license-MIT-blue.svg "License"
