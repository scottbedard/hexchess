---
# https://vitepress.dev/reference/default-theme-home-page
layout: doc
---

# Library overview

Game libraries are available in [Rust](https://crates.io/crates/hexchess), [PHP](https://packagist.org/packages/bedard/hexchess), and [TypeScript / JavaScript](https://www.npmjs.com/package/@bedard/hexchess). These libraries have similar APIs and abilities, but are designed for different purposes.

For game logic, use the PHP or TypeScript / JavaScript libraries. They are not optimized for performance, but use a [position-centric game state](https://www.chessprogramming.org/Board_Representation#Square_Centric). Put simply, the board is stored as a flat array of values, which creates better ergonomics for rules and rendering

For engine development, the Rust crate is recommended. It uses [bitboard game state](https://www.chessprogramming.org/Bitboards), and is optimized for performance. Fast board queries can be performed using `u128` bitmasks.

# Versioning

Each library is tested against [a shared test suite](https://github.com/scottbedard/hexchess/tree/main/tests), and are versioned together with respect to those tests. Because of this, a change made in one library will cause all library versions numbers to increment.

Put simply, the libraries are versioned together to ensure they support the same tests and are compatible with one another.
