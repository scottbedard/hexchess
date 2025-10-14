---
layout: doc
---

# TypeScript / JavaScript Library

Install with the following command.

```sh
npm install @bedard/hexchess
```

## Basic usage

The `Hexchess` class is a deserialized version of [Forsyth–Edwards Notation](https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation). It contains the board state, current turn, en passant, and move numbers. Since castling is not a part of hexagonal chess, that section is omitted. The board data is stored as an array of `Piece | null`, sorted in FEN-order.

To create a game at the starting position, use `Hexchess.init`.

```ts
import { Hexchess } from '@bedard/hexchess'

const hexchess = Hexchess.init()
```

`Hexchess` instances have the following shape. The `board` represents an array of position values, sorted in FEN-order.

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

The following constants are also available.

- `emptyPosition`
  ```
  1/3/5/7/9/11/11/11/11/11/11 w - 0 1
  ```

- `initialPosition`
  ```
  b/qbk/n1b1n/r5r/ppppppppp/11/5P5/4P1P4/3P1B1P3/2P2B2P2/1PRNQBKNRP1 w - 0 1
  ```

## Available methods

#### `apply`

Apply a whitespace separated sequence of moves.

```ts
const hexchess = Hexchess.init()

hexchess.apply('g4g5 e7e6 f5f6 e6f6')

hexchess.toString() // 'b/qbk/n1b1n/r5r/ppp1ppppp/5p5/6P4/4P6/3P1B1P3/2P2B2P2/1PRNQBKNRP1 w - 0 3'
```

#### `applyMove`

Apply a single move from `string` or `San`.

```ts
const hexchess = Hexchess.init()

hexchess.applyMove(San.parse('g4g6'))

hexchess.toString() // 'b/qbk/n1b1n/r5r/ppppppppp/11/5PP4/4P6/3P1B1P3/2P2B2P2/1PRNQBKNRP1 b - 0 1'
```

#### `applyMoveUnsafe`

Apply a single move from `string` or `San`, regardless of turn or legality.

```ts
const hexchess = Hexchess.init()

hexchess.applyMoveUnsafe('b1b6')

hexchess.toString() // 'b/qbk/n1b1n/r5r/ppppppppp/1P9/5P5/4P1P4/3P1B1P3/2P2B2P2/2RNQBKNRP1 b - 0 1'
```

#### `clone`

Deeply clone a `Hexchess` instance.

```ts
const hexchess = Hexchess.init()

const clone = hexchess.clone()

hexchess === clone // false
```

#### `currentMoves`

Get all current legal moves.

```ts
const hexchess = new Hexchess('1/3/5/7/9/11/5P5/11/11/11/11 w - 0 1')

const moves = hexchess.currentMoves()

moves.map(String) // ['f5f6, 'f5f7', ...]
```

#### `findKing`

Find FEN index for king belonging to `Color`.

```ts
const hexchess = Hexchess.init()

hexchess.findKing('b') // 3
hexchess.findKing('w') // 86
```

#### `get`

Get board value from position name.

```ts
const hexchess = Hexchess.init()

hexchess.get('e1') // 'Q'
```

#### `getColor`

Get all board indices occupied by `Color` pieces.

```ts
const hexchess = Hexchess.init()

hexchess.getColor('b') // [0,  1,  2,  ...]
```

#### `isCheck`

Test if the board is in check.

```ts
const hexchess = Hexchess.init()

hexchess.isCheck() // false
```

#### `isCheckmate`

Test if the game is in checkmate.

```ts
const hexchess = Hexchess.init()

hexchess.isCheckmate() // false
```

#### `isLegal`

Test if a move is legal.

```ts
const hexchess = Hexchess.init()

hexchess.isLegal('b1b6') // false
```

#### `isStalemate`

Test if the game is in stalemate.

```ts
const hexchess = Hexchess.init()

hexchess.isStalemate() // false
```

#### `isThreatened`

Test if a position is threatened by unsafe captures. This does not include forward pawn moves.

```ts
const hexchess = Hexchess.init()

hexchess.isThreatened('f6') // false
```

#### `movesFrom`

Get all legal moves from a position.

```ts
const hexchess = Hexchess.init()

const moves = hexchess.movesFrom('f6')

moves.map(String) // ['f6f7']
```

#### `movesFromUnsafe`

Get all moves from a position, including ones that result in self-check.

```ts
const hexchess = Hexchess.parse('1/3/5/7/4r4/5K5/11/11/11/11/11 w - 0 1')

const moves = hexchess.movesUnsafe()

moves.map(String) // ['f6f7', 'f6g7' ...]
```

#### `toString`

Serialize a `Hexchess` instance to string.

```ts
const hexchess = Hexchess.init()

hexchess.toString() // 'b/qbk/n1b1n/r5r/ppppppppp/11/5P5/4P1P4/3P1B1P3/2P2B2P2/1PRNQBKNRP1 w - 0 1'
```