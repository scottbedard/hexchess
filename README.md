# [`hexchess`](https://hexchess.club)

[![Build](https://github.com/scottbedard/hexchess/actions/workflows/build.yml/badge.svg)](https://github.com/scottbedard/hexchess/actions/workflows/build.yml)
[![Coverage](https://codecov.io/gh/scottbedard/hexchess/graph/badge.svg?token=uHmFqhQDps)](https://codecov.io/gh/scottbedard/hexchess)
[![Crates.io](https://img.shields.io/crates/v/hexchess?logo=rust&logoColor=%23f74c00&label=cargo)](https://crates.io/crates/hexchess)
[![NPM](https://img.shields.io/npm/v/%40bedard%2Fhexchess?logo=javascript&logoColor=%23f7df1e)](https://www.npmjs.com/package/@bedard/hexchess)
[![Packagist](https://img.shields.io/packagist/v/bedard/hexchess?logo=php&color=%23777BB3)](https://packagist.org/packages/bedard/hexchess)
[![Bundlephobia](https://img.shields.io/bundlephobia/minzip/%40bedard%2Fhexchess?label=size)](https://bundlephobia.com/package/@bedard/hexchess)
[![License](https://img.shields.io/badge/license-MIT-blue)](https://github.com/scottbedard/hexchess/blob/main/LICENSE)

A cross-language library for [Gliński's hexagonal chess](https://en.wikipedia.org/wiki/Hexagonal_chess#Gli%C5%84ski's_hexagonal_chess), and the brain of [hexchess.club](https://hexchess.club).

<p align="center">
  <a href="https://hexchess.club">
    <img src="assets/hexchess.svg" width="500" />
  </a>
</p>

## Libraries

Game libraries are available in [Rust](https://crates.io/crates/hexchess), [PHP](https://packagist.org/packages/bedard/hexchess), and [Typescript / JavaScript](https://www.npmjs.com/package/@bedard/hexchess). While these libraries offer similar abilities, they serve different purposes and are designed differently.

For basic game logic, use the PHP or TypeScript / JavaScript libraries. These are not optimized for performance, and use a [position-centric](https://www.chessprogramming.org/Board_Representation#Square_Centric) game state. In other words, the board is stored as a flat array of values, with each value representing the occupying piece. This makes things like rendering and rules logic simpler.

For game engines, use the Rust crate. It's optimized for performance, and uses [bitboards](https://www.chessprogramming.org/Bitboards) to represent the game state. This allows for fast board querying using `u128` bitmasks.

## Versioning

Each library is tested against [a shared test suite](https://github.com/scottbedard/hexchess/tree/main/tests), and are versioned together with respect to these tests. Because of this, if a change is made in one library, it will cause the version numbers for all libraries to increment.

Put simply, all libraries are versioned together to indicate that they support the same tests and are compatible with one another.

## Local development

Depending on which library you're working on, you'll need to install a few things.

- [Rust](https://www.rust-lang.org/tools/install)
- [PHP](https://www.php.net/)
- [Node.js](https://nodejs.org/) and [pnpm](https://pnpm.io/installation)

First, clone the repository, and setup the CLI.

```
git clone git@github.com:scottbedard/hexchess.git

cd hexchess

pnpm install
```

Next run `node hexchess` to see the following commands available.

```
Usage: hexchess [options] [command]

Options:
  -h, --help             display help for command

Commands:
  build                  Build all projects
  build:js               Build NPM package
  build:rs               Build Rust crate
  lint:php               Run linting
  set-version <version>  Set the version of the project
  test                   Run all tests
  test:js [options]      Run JavaScript tests
  test:php [options]     Run PHP tests
  test:rs [options]      Run Rust tests
  versions [options]     Check the versions of the dependencies
  help [command]         display help for command
```

## License

[MIT](https://github.com/scottbedard/hexchess/blob/main/LICENSE)

Copyright (c) 2024-present, Scott Bedard
