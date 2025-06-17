import {
  index,
  graph,
  Hexchess,
  type Piece
} from '../src/index'

// white
// const hexchess = Hexchess.parse('1/1P1/1PPP1/1PPPPP1/1PPPPPPP1/1PPPPPPPPP1/PPPPPPPPPPP/PPPPP1PPPPP/PPPP3PPPP/PPP5PPP/PP7PP w - 0 1')

// black
const hexchess = Hexchess.parse('1/3/5/7/ppppppppp/ppppppppppp/ppppppppppp/ppppppppppp/ppppppppppp/ppppppppppp/11 w - 0 1')

const is_promotion_position = (position: number, piece: Piece) => piece === 'p'
  ? [
    index('a1'),
    index('b1'),
    index('c1'),
    index('d1'),
    index('e1'),
    index('f1'),
    index('g1'),
    index('h1'),
    index('i1'),
    index('k1'),
    index('l1'),
  ].includes(position)
  : [
    index('a6'),
    index('b7'),
    index('c8'),
    index('d9'),
    index('e10'),
    index('f11'),
    index('g10'),
    index('h9'),
    index('i8'),
    index('k7'),
    index('l6'),
  ].includes(position)

const is_starting_position = (position: number, piece: Piece) => {
  return piece === 'p'
    ? [
      index('b7'),
      index('c7'),
      index('d7'),
      index('e7'),
      index('f7'),
      index('g7'),
      index('h7'),
      index('i7'),
      index('k7'),
    ].includes(position)
    : [
      index('b1'),
      index('c2'),
      index('d3'),
      index('e4'),
      index('f5'),
      index('g4'),
      index('h3'),
      index('i2'),
      index('k1'),
    ].includes(position)
}

const result: any[] = []

for (let i = 0; i < hexchess.board.length; i++) {
  const piece = hexchess.board[i]

  if (piece === 'P' || piece === 'p') {
    const forward_1 = graph[i][piece === 'P' ? 0 : 6]!

    const forward_2 = is_starting_position(i, piece)
      ? graph[forward_1][piece === 'P' ? 0 : 6] ?? 255
      : 255

    const capture_portside = graph[i][piece === 'P' ? 10 : 4] ?? 255

    const capture_starboard = graph[i][piece === 'P' ? 2 : 8] ?? 255

    result[i] = {
      forward_1,
      forward_2,
      capture_portside,
      capture_starboard,
      promote_forward: is_promotion_position(forward_1, piece),
      promote_portside: is_promotion_position(capture_portside, piece),
      promote_starboard: is_promotion_position(capture_starboard, piece),
    }
  } else {
    result[i] = null
  }
}

console.log(
  result
)
