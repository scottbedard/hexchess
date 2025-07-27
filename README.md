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

## Development

Clone the repository, and setup CLI

```
git clone git@github.com:scottbedard/hexchess.git

cd hexchess

pnpm install
```

## Languages

The game library is available in multiple languages. These libraries run the same suite of tests to ensure compatibility, and for that reason are versioned together.

- [Javascript](https://www.npmjs.com/package/@bedard/hexchess) <sup>(and TypeScript)</sup>
- [Rust](https://crates.io/crates/hexchess)

If you are building a game engine, it's strongly recommended to use the Rust library for better performance.

## License

[MIT](https://github.com/scottbedard/hexchess/blob/main/LICENSE)

Copyright (c) 2024-present, Scott Bedard
