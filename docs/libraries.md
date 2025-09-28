---
# https://vitepress.dev/reference/default-theme-home-page
layout: doc
---
# General Overview

Game libraries are available in [Rust](https://crates.io/crates/hexchess), [PHP](https://packagist.org/packages/bedard/hexchess), and [TypeScript / JavaScript](https://www.npmjs.com/package/@bedard/hexchess). These libraries have similar APIs and abilities, but are designed for different purposes.

For game logic, use the PHP or TypeScript / JavaScript libraries. They use a [position-centric](https://www.chessprogramming.org/Board_Representation#Square_Centric) game state, and are not optimized for performance. In other words, the board is stored as a flat array of values, which provides better ergonomics for gameplay and rendering.

For engines, use the Rust crate. It uses [bitboards](https://www.chessprogramming.org/Bitboards), and is optimized for performance. Fast board queries can be performed using `u128` bitmasks.

```sh
# rust
cargo add hexchess

# typescript / javascript
npm install @bedard/hexchess

# php
composer require bedard/hexchess
```