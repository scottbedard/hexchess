# Changelog

## 2.4.3

- Throw an explicit `IllegalMoveException` from PHP library

## 2.4.1

- Fixed interop of Rust / JavaScript libraries by serializing `San` positions to their index, not enum name. 

## 2.4.0

- Performance and inter-op improvements. The Rust library now only uses FEN index.

## 2.3.5

- Remove undocumented `random` constructor from `Bitboard` struct

## 2.3.4

- Fix regression with [`Eq`](https://doc.rust-lang.org/std/cmp/trait.Eq.html) and [`Hash`](https://doc.rust-lang.org/std/hash/trait.Hash.html) attributes

## 2.3.0

- Refactored Rust library to `u128` bitboards for state, improving performance ~11%
- Normalized tests and migrated from `json` to `yaml`
- Added developer CLI, run `node hexchess` for docs

## 2.2.1

- Various performance improvements to Rust library

## 2.2.0

- Add PHP library

## 2.1.2

- Refactor project structure and versioning

## 2.1.1

- Increase Rust performance of `moves_from` ~22x by finding unsafe moves from king's perspective, and simplifying the knight graph.

## 2.1.0

- Removed wasm bindings. Engines should import the crate directly
- Changed directory structure to separate crate and npm package
- Use pre-computed values for crate pawn and knight moves

## 2.0.1

- Add [`Eq`](https://doc.rust-lang.org/std/cmp/trait.Eq.html) and [`Hash`](https://doc.rust-lang.org/std/hash/trait.Hash.html) support to `Hexchess` instances.

## 2.0.0

Board state is now an array of values, and no longer as key-value pairs.

```js
// before
hexchess.board // { f11: null, e10: null, ... }

// after
hexchess.board // [null, null, ...]
```

Similarly, all `Position` values now represent the index of their position in a FEN.

```js
// before
San.parse('a1b2') // { from: 'a1', to: 'b2', promotion: null }

// after
San.parse('a1b2') // { from: 80, to: 70, promotion: null }
```

Additionally, the Rust and TypeScript libraries have been reunited. Their APIs and feature sets may diverge, but they will be versioned together to ensure compatibility.

## 2.0.0-beta.3

- Fix module compatibility

## 2.0.0-beta.2

- Export `initialPositions` and `positions` constants

## 2.0.0-beta.0

- Export wasm bindings API
- Add many essential methods to `Hexchess` class
  - [`apply`](https://github.com/scottbedard/hexchess?tab=readme-ov-file#apply)
  - [`currentMoves`](https://github.com/scottbedard/hexchess?tab=readme-ov-file#currentmoves)
  - [`movesFrom`](https://github.com/scottbedard/hexchess?tab=readme-ov-file#movesfrom)
  - [`movesFromUnsafe`](https://github.com/scottbedard/hexchess?tab=readme-ov-file#movesfromunsafe)
  - [`toString`](https://github.com/scottbedard/hexchess?tab=readme-ov-file#tostring)

## 2.0.0-alpha.3

- Begin implementing class API
- Rename `PositionName` type to `Position`

## 1.x

Version 1.x was ported as a pure TypeScript library, [and was maintained here &rarr;](https://github.com/scottbedard/hexchess.ts)

## 0.10.0

- Add `get-status` command to check for turn, stalemate, and checkmate
- Rename `apply-sequence` command to `apply`
- Rename `check-move` command to `test-move`
- Combine `all-targets` and `get-targets` to single command with optional position flag

## 0.9.0

- Add `check-move` command

## 0.8.0

- Add `all-targets` command

## 0.7.0

- Add `get-targets` command
- Remove `get` command

## 0.6.0

- Cargo library published to crates.io as [`hexchess`](https://crates.io/crates/hexchess)

## 0.5.0

- Add `applySequence` method
- Prevent `apply` from being called with a valid but illegal move
- To simplify type definitions, `findKing`, `getPieceColor` and `getPositionColor` will now return `undefined` instead of `null`

## 0.4.3

- Error when characters exist after the promotion notation

## 0.4.2

- Fixed notation parsing edge cases

## 0.4.1

- `parseNotation` and `parseHexchess` now return `undefined` when input is invalid, rather than erroring

## 0.4.0

- Add `findKing` method
- Add `isCheckmate` method
- Improved typescript definitions

## 0.3.2

- Fix self-check logic on opponent's turn

## 0.3.1

- Fix self-check logic when king is moved piece

## 0.3.0

- Add `isThreatened` method
- Prohibit self-checking moves
- Fix en passant capture by non-pawns
- Remove regex parser to cut wasm binary from 1.6MB &rarr; 180KB

## 0.2.5

- Fix en passant capture not removing enemy piece
