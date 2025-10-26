---
layout: doc
---

# Rust

Install with the following command.

```sh
cargo add hexchess
```

## Basic usage

The Hexchess class is a deserialized version of [Forsyth–Edwards Notation](https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation). It contains the board state, current turn, en passant, and move numbers. Since castling is not a part of hexagonal chess, that section is omitted. Board state consists of twelve `Bitboard` values, each stored as a `u128`.

```rs
pub struct Hexchess {
    pub bitboard_black_bishop: Bitboard,
    pub bitboard_black_king: Bitboard,
    pub bitboard_black_knight: Bitboard,
    pub bitboard_black_pawn: Bitboard,
    pub bitboard_black_queen: Bitboard,
    pub bitboard_black_rook: Bitboard,
    pub bitboard_white_bishop: Bitboard,
    pub bitboard_white_king: Bitboard,
    pub bitboard_white_knight: Bitboard,
    pub bitboard_white_pawn: Bitboard,
    pub bitboard_white_queen: Bitboard,
    pub bitboard_white_rook: Bitboard,
    pub ep: Option<Position>,
    pub fullmove: u16,
    pub halfmove: u8,
    pub turn: Color,
}
```

The following constants are also available on `hexchess::Constants`

- `EMPTY_POSITION`
  ```
  1/3/5/7/9/11/11/11/11/11/11 w - 0 1
  ```

- `INITIAL_POSITION`
  ```
  b/qbk/n1b1n/r5r/ppppppppp/11/5P5/4P1P4/3P1B1P3/2P2B2P2/1PRNQBKNRP1 w - 0 1
  ```

## Macros

The following macros are available to construct more readable bitmasks at compile time. To use them, install the following crate.

```sh
cargo add hexchess_macros
```

#### `bitmask!`

```rs
use hexchess_macros::bitmask;

bitmask!("x/3/5/7/9/11/11/11/11/11/11"); // u128 with first bit set
```

#### `bitmask_csv!`

```rs
use hexchess_macros::bitmask_csv;

bitmask_csv!("a1, a2"); // u128 with a1 and a2 bits set
```

## Available methods

Method documentation not available yet.
