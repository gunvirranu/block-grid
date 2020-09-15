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

TODO: Add some cons

## Changelog

TODO: Point to `CHANGELOG.md` when created

## License

`block-grid` is licensed under the [MIT license](LICENSE).

## Alternatives

TODO: Link to alternative crates

<!-- Badges -->
[ci]: https://github.com/gunvirranu/block-grid/actions "Github Actions"
[ci_badge]: https://github.com/gunvirranu/block-grid/workflows/CI/badge.svg?branch=master "Github Actions"
[license]: #license "License"
[license_badge]: https://img.shields.io/badge/license-MIT-blue.svg "License"
