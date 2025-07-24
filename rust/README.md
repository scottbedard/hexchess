# [`hexchess`](https://crates.io/crates/hexchess)

[![Build](https://github.com/scottbedard/hexchess/actions/workflows/build.yml/badge.svg)](https://github.com/scottbedard/hexchess/actions/workflows/build.yml)
[![Coverage](https://codecov.io/gh/scottbedard/hexchess/graph/badge.svg?token=uHmFqhQDps)](https://codecov.io/gh/scottbedard/hexchess)
[![Crates.io](https://img.shields.io/crates/v/hexchess?logo=rust&logoColor=%23f74c00&label=cargo)](https://crates.io/crates/hexchess)
[![License](https://img.shields.io/badge/license-MIT-blue)](https://github.com/scottbedard/hexchess/blob/main/LICENSE)

A library for [Gliński's hexagonal chess](https://en.wikipedia.org/wiki/Hexagonal_chess#Gli%C5%84ski's_hexagonal_chess), and the brain of [hexchess.club](https://hexchess.club).

<p align="center">
  <a href="https://hexchess.club">
    <img src="assets/hexchess.svg" width="500" />
  </a>
</p>

## Installation

Run the following Cargo command in your project directory:

```
cargo add hexchess
```

Or add `hexchess` as a dependency to your `Cargo.toml` file.

## Basic usage

The `Hexchess` struct represents a deserialized version of [Forsyth–Edwards Notation](https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation). Since castling is not a part of hexchess, that section is omitted. The following is a generalized version of the struct. The board is stored as an array of `Option<Piece>` values, sorted by FEN index.

```ts
{
  board: [
    'b',  'q',  'b',  'k',  'n',  null, 'b',  null, 'n',  'r',
    null, null, null, null, null, 'r',  'p',  'p',  'p',  'p',
    'p',  'p',  'p',  'p',  'p',  null, null, null, null, null,
    null, null, null, null, null, null, null, null, null, null,
    null, 'P',  null, null, null, null, null, null, null, null,
    null, 'P',  null, 'P',  null, null, null, null, null, null,
    null, 'P',  null, 'B',  null, 'P',  null, null, null, null,
    null, 'P',  null, null, 'B',  null, null, 'P',  null, null,
    null, 'P',  'R',  'N',  'Q',  'B',  'K',  'N',  'R',  'P',
    null
  ],
  turn: 'w',
  ep: null,
  halfmove: 0,
  fullmove: 1
}
```

The following methods are available,

#### `apply`

Apply a whitespace separated sequence of moves.

```rs
let mut hexchess = Hexchess::init();
let _ = hexchess.apply("g4g5 e7e6 f5f6 e6f6");
    
hexchess.to_string() // b/qbk/n1b1n/r5r/ppp1ppppp/5p5/6P4/4P6/3P1B1P3/2P2B2P2/1PRNQBKNRP1 w - 0 3
```

#### `apply_move`

Apply a single move by `San`.

```rs
let san = San::from("g4g5").unwrap();
let mut hexchess = Hexchess::init();
let _ = hexchess.apply_move(&san);

hexchess.to_string() // b/qbk/n1b1n/r5r/ppppppppp/11/5PP4/4P6/3P1B1P3/2P2B2P2/1PRNQBKNRP1 b - 0 1
```

#### `apply_move_unsafe`

Apply a single by `San`, regardless of turn or legality.

```ts
let san = San::from("b1b6").unwrap();
let mut hexchess = Hexchess::init();
let _ = hexchess.apply_move_unsafe(&san)

hexchess.to_string() // b/qbk/n1b1n/r5r/ppppppppp/1P9/5P5/4P1P4/3P1B1P3/2P2B2P2/2RNQBKNRP1 b - 0 1
```

#### `current_moves`

Get all current legal moves.

```rs
let hexchess = Hexchess::init();

for san in hexchess.current_moves() {
    // ...
}
```

#### `find_king`

Find FEN index for king belonging to `Color`.

```rs
let hexchess = Hexchess::init();

hexchess.find_king(Color::Black); // Some(3)
hexchess.find_king(Color::White); // Some(86)
```

#### `get`

Get board value from position name.

```rs
let hexchess = Hexchess::init();

hexchess.get("e1") // Some(WhiteQueen)
```

#### `get_color`

Get all board indices occupied by `Color` pieces.

```rs
let hexchess = Hexchess::init();

hexchess.get_color(Black) // [0,  1,  2,  ...]
```

#### `is_check`

Test if the board is in check.

```rs
let hexchess = Hexchess::init();

hexchess.is_check() // false
```

#### `is_checkmate`

Test if the game is in checkmate.

```rs
let hexchess = Hexchess::init();

hexchess.is_checkmate() // false
```

#### `is_stalemate`

Test if the game is in stalemate.

```ts
let hexchess = Hexchess::init();

hexchess.is_stalemate() // false
```

#### `moves_from`

Get all legal moves from a position.

```rs
let hexchess = Hexchess::init();

hexchess.moves_from(h!("f5"))  // [San { from: 41, promotion: None, to: 30 }]
```

#### `moves_from_unsafe`

Get all moves from a position, including ones that result in self-check.

```rs
let hexchess = Hexchess::parse("1/3/5/7/4r4/5K5/11/11/11/11/11 w - 0 1").unwrap();

hexchess.moves_from_unsafe(h!("f6")) // [San, San, San, ...]
```

#### `to_string`

Serialize `Hexchess` to string.

```ts
let hexchess = Hexchess::init();

hexchess.to_string() // b/qbk/n1b1n/r5r/ppppppppp/11/5P5/4P1P4/3P1B1P3/2P2B2P2/1PRNQBKNRP1 w - 0 1
```

## License

[MIT](https://github.com/scottbedard/hexchess/blob/main/LICENSE)

Copyright (c) 2024-present, Scott Bedard
