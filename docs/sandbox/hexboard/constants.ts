import { positions } from '../../../js/src/constants'
import { flip, hexagon, pivot, reflect, sum } from './helpers'
import type { Vec } from './types'

/** svg viewbox, increasing this zooms out away from the board */
export const box = 23.6

/** label offset */
export const labelOffset = 1.7

/** side length of regular hexagon circumscribed around a unit circle */
export const sideLength = 2 / Math.sqrt(3)

// distance between the origin of neighboring hexagons
// one = direct neighbors, two = neighbor of neighbor, etc...
export const one = sideLength * (Math.sqrt(3) / 2) * 2 // <- inscribed radius * 2
export const two = one * 2
export const three = one * 3
export const four = one * 4
export const five = one * 5

// calculate the center for each position, using f6 as the origin
const f6: Vec<2> = [0, 0]

// the 6th rank is calculated as angled distances from F6
const a6 = pivot(f6, 150, five)
const b6 = pivot(f6, 150, four)
const c6 = pivot(f6, 150, three)
const d6 = pivot(f6, 150, two)
const e6 = pivot(f6, 150, one)
const g6 = pivot(f6, 30, one)
const h6 = pivot(f6, 30, two)
const i6 = pivot(f6, 30, three)
const k6 = pivot(f6, 30, four)
const l6 = pivot(f6, 30, five)

// other ranks are calculated by vertically translating the 6th rank
const a1 = sum(a6, [0, -five])
const b1 = sum(b6, [0, -five])
const c1 = sum(c6, [0, -five])
const d1 = sum(d6, [0, -five])
const e1 = sum(e6, [0, -five])
const f1 = sum(f6, [0, -five])
const g1 = sum(g6, [0, -five])
const h1 = sum(h6, [0, -five])
const i1 = sum(i6, [0, -five])
const k1 = sum(k6, [0, -five])
const l1 = sum(l6, [0, -five])

const a2 = sum(a6, [0, -four])
const b2 = sum(b6, [0, -four])
const c2 = sum(c6, [0, -four])
const d2 = sum(d6, [0, -four])
const e2 = sum(e6, [0, -four])
const f2 = sum(f6, [0, -four])
const g2 = sum(g6, [0, -four])
const h2 = sum(h6, [0, -four])
const i2 = sum(i6, [0, -four])
const k2 = sum(k6, [0, -four])
const l2 = sum(l6, [0, -four])

const a3 = sum(a6, [0, -three])
const b3 = sum(b6, [0, -three])
const c3 = sum(c6, [0, -three])
const d3 = sum(d6, [0, -three])
const e3 = sum(e6, [0, -three])
const f3 = sum(f6, [0, -three])
const g3 = sum(g6, [0, -three])
const h3 = sum(h6, [0, -three])
const i3 = sum(i6, [0, -three])
const k3 = sum(k6, [0, -three])
const l3 = sum(l6, [0, -three])

const a4 = sum(a6, [0, -two])
const b4 = sum(b6, [0, -two])
const c4 = sum(c6, [0, -two])
const d4 = sum(d6, [0, -two])
const e4 = sum(e6, [0, -two])
const f4 = sum(f6, [0, -two])
const g4 = sum(g6, [0, -two])
const h4 = sum(h6, [0, -two])
const i4 = sum(i6, [0, -two])
const k4 = sum(k6, [0, -two])
const l4 = sum(l6, [0, -two])

const a5 = sum(a6, [0, -one])
const b5 = sum(b6, [0, -one])
const c5 = sum(c6, [0, -one])
const d5 = sum(d6, [0, -one])
const e5 = sum(e6, [0, -one])
const f5 = sum(f6, [0, -one])
const g5 = sum(g6, [0, -one])
const h5 = sum(h6, [0, -one])
const i5 = sum(i6, [0, -one])
const k5 = sum(k6, [0, -one])
const l5 = sum(l6, [0, -one])

// 7th rank and higher start to have fewer files
const b7 = sum(b6, [0, one])
const c7 = sum(c6, [0, one])
const d7 = sum(d6, [0, one])
const e7 = sum(e6, [0, one])
const f7 = sum(f6, [0, one])
const g7 = sum(g6, [0, one])
const h7 = sum(h6, [0, one])
const i7 = sum(i6, [0, one])
const k7 = sum(k6, [0, one])

const c8 = sum(c6, [0, two])
const d8 = sum(d6, [0, two])
const e8 = sum(e6, [0, two])
const f8 = sum(f6, [0, two])
const g8 = sum(g6, [0, two])
const h8 = sum(h6, [0, two])
const i8 = sum(i6, [0, two])

const d9 = sum(d6, [0, three])
const e9 = sum(e6, [0, three])
const f9 = sum(f6, [0, three])
const g9 = sum(g6, [0, three])
const h9 = sum(h6, [0, three])

const e10 = sum(e6, [0, four])
const f10 = sum(f6, [0, four])
const g10 = sum(g6, [0, four])

const f11 = sum(f6, [0, five])

/** board position data, sorted by fen order  */
export const board: {
  color: number
  origin: [Vec<2>, Vec<2>]
  path: [Vec<6, Vec<2>>, Vec<6, Vec<2>>]
}[] = [
  { color: 2, origin: [f11, reflect(f11)], path: [hexagon(f11), flip(hexagon(f11))] },
  { color: 1, origin: [e10, reflect(e10)], path: [hexagon(e10), flip(hexagon(e10))] },
  { color: 0, origin: [f10, reflect(f10)], path: [hexagon(f10), flip(hexagon(f10))] },
  { color: 1, origin: [g10, reflect(g10)], path: [hexagon(g10), flip(hexagon(g10))] },
  { color: 0, origin: [d9, reflect(d9)], path: [hexagon(d9), flip(hexagon(d9))] },
  { color: 2, origin: [e9, reflect(e9)], path: [hexagon(e9), flip(hexagon(e9))] },
  { color: 1, origin: [f9, reflect(f9)], path: [hexagon(f9), flip(hexagon(f9))] },
  { color: 2, origin: [g9, reflect(g9)], path: [hexagon(g9), flip(hexagon(g9))] },
  { color: 0, origin: [h9, reflect(h9)], path: [hexagon(h9), flip(hexagon(h9))] },
  { color: 2, origin: [c8, reflect(c8)], path: [hexagon(c8), flip(hexagon(c8))] },
  { color: 1, origin: [d8, reflect(d8)], path: [hexagon(d8), flip(hexagon(d8))] },
  { color: 0, origin: [e8, reflect(e8)], path: [hexagon(e8), flip(hexagon(e8))] },
  { color: 2, origin: [f8, reflect(f8)], path: [hexagon(f8), flip(hexagon(f8))] },
  { color: 0, origin: [g8, reflect(g8)], path: [hexagon(g8), flip(hexagon(g8))] },
  { color: 1, origin: [h8, reflect(h8)], path: [hexagon(h8), flip(hexagon(h8))] },
  { color: 2, origin: [i8, reflect(i8)], path: [hexagon(i8), flip(hexagon(i8))] },
  { color: 1, origin: [b7, reflect(b7)], path: [hexagon(b7), flip(hexagon(b7))] },
  { color: 0, origin: [c7, reflect(c7)], path: [hexagon(c7), flip(hexagon(c7))] },
  { color: 2, origin: [d7, reflect(d7)], path: [hexagon(d7), flip(hexagon(d7))] },
  { color: 1, origin: [e7, reflect(e7)], path: [hexagon(e7), flip(hexagon(e7))] },
  { color: 0, origin: [f7, reflect(f7)], path: [hexagon(f7), flip(hexagon(f7))] },
  { color: 1, origin: [g7, reflect(g7)], path: [hexagon(g7), flip(hexagon(g7))] },
  { color: 2, origin: [h7, reflect(h7)], path: [hexagon(h7), flip(hexagon(h7))] },
  { color: 0, origin: [i7, reflect(i7)], path: [hexagon(i7), flip(hexagon(i7))] },
  { color: 1, origin: [k7, reflect(k7)], path: [hexagon(k7), flip(hexagon(k7))] },
  { color: 0, origin: [a6, reflect(a6)], path: [hexagon(a6), flip(hexagon(a6))] },
  { color: 2, origin: [b6, reflect(b6)], path: [hexagon(b6), flip(hexagon(b6))] },
  { color: 1, origin: [c6, reflect(c6)], path: [hexagon(c6), flip(hexagon(c6))] },
  { color: 0, origin: [d6, reflect(d6)], path: [hexagon(d6), flip(hexagon(d6))] },
  { color: 2, origin: [e6, reflect(e6)], path: [hexagon(e6), flip(hexagon(e6))] },
  { color: 1, origin: [f6, reflect(f6)], path: [hexagon(f6), flip(hexagon(f6))] },
  { color: 2, origin: [g6, reflect(g6)], path: [hexagon(g6), flip(hexagon(g6))] },
  { color: 0, origin: [h6, reflect(h6)], path: [hexagon(h6), flip(hexagon(h6))] },
  { color: 1, origin: [i6, reflect(i6)], path: [hexagon(i6), flip(hexagon(i6))] },
  { color: 2, origin: [k6, reflect(k6)], path: [hexagon(k6), flip(hexagon(k6))] },
  { color: 0, origin: [l6, reflect(l6)], path: [hexagon(l6), flip(hexagon(l6))] },
  { color: 1, origin: [a5, reflect(a5)], path: [hexagon(a5), flip(hexagon(a5))] },
  { color: 0, origin: [b5, reflect(b5)], path: [hexagon(b5), flip(hexagon(b5))] },
  { color: 2, origin: [c5, reflect(c5)], path: [hexagon(c5), flip(hexagon(c5))] },
  { color: 1, origin: [d5, reflect(d5)], path: [hexagon(d5), flip(hexagon(d5))] },
  { color: 0, origin: [e5, reflect(e5)], path: [hexagon(e5), flip(hexagon(e5))] },
  { color: 2, origin: [f5, reflect(f5)], path: [hexagon(f5), flip(hexagon(f5))] },
  { color: 0, origin: [g5, reflect(g5)], path: [hexagon(g5), flip(hexagon(g5))] },
  { color: 1, origin: [h5, reflect(h5)], path: [hexagon(h5), flip(hexagon(h5))] },
  { color: 2, origin: [i5, reflect(i5)], path: [hexagon(i5), flip(hexagon(i5))] },
  { color: 0, origin: [k5, reflect(k5)], path: [hexagon(k5), flip(hexagon(k5))] },
  { color: 1, origin: [l5, reflect(l5)], path: [hexagon(l5), flip(hexagon(l5))] },
  { color: 2, origin: [a4, reflect(a4)], path: [hexagon(a4), flip(hexagon(a4))] },
  { color: 1, origin: [b4, reflect(b4)], path: [hexagon(b4), flip(hexagon(b4))] },
  { color: 0, origin: [c4, reflect(c4)], path: [hexagon(c4), flip(hexagon(c4))] },
  { color: 2, origin: [d4, reflect(d4)], path: [hexagon(d4), flip(hexagon(d4))] },
  { color: 1, origin: [e4, reflect(e4)], path: [hexagon(e4), flip(hexagon(e4))] },
  { color: 0, origin: [f4, reflect(f4)], path: [hexagon(f4), flip(hexagon(f4))] },
  { color: 1, origin: [g4, reflect(g4)], path: [hexagon(g4), flip(hexagon(g4))] },
  { color: 2, origin: [h4, reflect(h4)], path: [hexagon(h4), flip(hexagon(h4))] },
  { color: 0, origin: [i4, reflect(i4)], path: [hexagon(i4), flip(hexagon(i4))] },
  { color: 1, origin: [k4, reflect(k4)], path: [hexagon(k4), flip(hexagon(k4))] },
  { color: 2, origin: [l4, reflect(l4)], path: [hexagon(l4), flip(hexagon(l4))] },
  { color: 0, origin: [a3, reflect(a3)], path: [hexagon(a3), flip(hexagon(a3))] },
  { color: 2, origin: [b3, reflect(b3)], path: [hexagon(b3), flip(hexagon(b3))] },
  { color: 1, origin: [c3, reflect(c3)], path: [hexagon(c3), flip(hexagon(c3))] },
  { color: 0, origin: [d3, reflect(d3)], path: [hexagon(d3), flip(hexagon(d3))] },
  { color: 2, origin: [e3, reflect(e3)], path: [hexagon(e3), flip(hexagon(e3))] },
  { color: 1, origin: [f3, reflect(f3)], path: [hexagon(f3), flip(hexagon(f3))] },
  { color: 2, origin: [g3, reflect(g3)], path: [hexagon(g3), flip(hexagon(g3))] },
  { color: 0, origin: [h3, reflect(h3)], path: [hexagon(h3), flip(hexagon(h3))] },
  { color: 1, origin: [i3, reflect(i3)], path: [hexagon(i3), flip(hexagon(i3))] },
  { color: 2, origin: [k3, reflect(k3)], path: [hexagon(k3), flip(hexagon(k3))] },
  { color: 0, origin: [l3, reflect(l3)], path: [hexagon(l3), flip(hexagon(l3))] },
  { color: 1, origin: [a2, reflect(a2)], path: [hexagon(a2), flip(hexagon(a2))] },
  { color: 0, origin: [b2, reflect(b2)], path: [hexagon(b2), flip(hexagon(b2))] },
  { color: 2, origin: [c2, reflect(c2)], path: [hexagon(c2), flip(hexagon(c2))] },
  { color: 1, origin: [d2, reflect(d2)], path: [hexagon(d2), flip(hexagon(d2))] },
  { color: 0, origin: [e2, reflect(e2)], path: [hexagon(e2), flip(hexagon(e2))] },
  { color: 2, origin: [f2, reflect(f2)], path: [hexagon(f2), flip(hexagon(f2))] },
  { color: 0, origin: [g2, reflect(g2)], path: [hexagon(g2), flip(hexagon(g2))] },
  { color: 1, origin: [h2, reflect(h2)], path: [hexagon(h2), flip(hexagon(h2))] },
  { color: 2, origin: [i2, reflect(i2)], path: [hexagon(i2), flip(hexagon(i2))] },
  { color: 0, origin: [k2, reflect(k2)], path: [hexagon(k2), flip(hexagon(k2))] },
  { color: 1, origin: [l2, reflect(l2)], path: [hexagon(l2), flip(hexagon(l2))] },
  { color: 2, origin: [a1, reflect(a1)], path: [hexagon(a1), flip(hexagon(a1))] },
  { color: 1, origin: [b1, reflect(b1)], path: [hexagon(b1), flip(hexagon(b1))] },
  { color: 0, origin: [c1, reflect(c1)], path: [hexagon(c1), flip(hexagon(c1))] },
  { color: 2, origin: [d1, reflect(d1)], path: [hexagon(d1), flip(hexagon(d1))] },
  { color: 1, origin: [e1, reflect(e1)], path: [hexagon(e1), flip(hexagon(e1))] },
  { color: 0, origin: [f1, reflect(f1)], path: [hexagon(f1), flip(hexagon(f1))] },
  { color: 1, origin: [g1, reflect(g1)], path: [hexagon(g1), flip(hexagon(g1))] },
  { color: 2, origin: [h1, reflect(h1)], path: [hexagon(h1), flip(hexagon(h1))] },
  { color: 0, origin: [i1, reflect(i1)], path: [hexagon(i1), flip(hexagon(i1))] },
  { color: 1, origin: [k1, reflect(k1)], path: [hexagon(k1), flip(hexagon(k1))] },
  { color: 2, origin: [l1, reflect(l1)], path: [hexagon(l1), flip(hexagon(l1))] },
]

/** perimeter of the board, used to create a constant background for light/dark mode */
export const perimeter: Vec<93, Vec<2>> = [
  board[positions.indexOf('f11')]!.path[0][5],
  board[positions.indexOf('f11')]!.path[0][0],
  board[positions.indexOf('f11')]!.path[0][1],
  board[positions.indexOf('g10')]!.path[0][5],
  board[positions.indexOf('g10')]!.path[0][0],
  board[positions.indexOf('g10')]!.path[0][1],
  board[positions.indexOf('h9')]!.path[0][5],
  board[positions.indexOf('h9')]!.path[0][0],
  board[positions.indexOf('h9')]!.path[0][1],
  board[positions.indexOf('i8')]!.path[0][5],
  board[positions.indexOf('i8')]!.path[0][0],
  board[positions.indexOf('i8')]!.path[0][1],
  board[positions.indexOf('k7')]!.path[0][5],
  board[positions.indexOf('k7')]!.path[0][0],
  board[positions.indexOf('k7')]!.path[0][1],
  board[positions.indexOf('l6')]!.path[0][5],
  board[positions.indexOf('l6')]!.path[0][0],
  board[positions.indexOf('l6')]!.path[0][1],
  board[positions.indexOf('l6')]!.path[0][2],
  board[positions.indexOf('l5')]!.path[0][0],
  board[positions.indexOf('l5')]!.path[0][1],
  board[positions.indexOf('l5')]!.path[0][2],
  board[positions.indexOf('l4')]!.path[0][0],
  board[positions.indexOf('l4')]!.path[0][1],
  board[positions.indexOf('l4')]!.path[0][2],
  board[positions.indexOf('l3')]!.path[0][0],
  board[positions.indexOf('l3')]!.path[0][1],
  board[positions.indexOf('l3')]!.path[0][2],
  board[positions.indexOf('l2')]!.path[0][0],
  board[positions.indexOf('l2')]!.path[0][1],
  board[positions.indexOf('l2')]!.path[0][2],
  board[positions.indexOf('l1')]!.path[0][0],
  board[positions.indexOf('l1')]!.path[0][1],
  board[positions.indexOf('l1')]!.path[0][2],
  board[positions.indexOf('l1')]!.path[0][3],
  board[positions.indexOf('k1')]!.path[0][2],
  board[positions.indexOf('k1')]!.path[0][3],
  board[positions.indexOf('k1')]!.path[0][4],
  board[positions.indexOf('i1')]!.path[0][1],
  board[positions.indexOf('i1')]!.path[0][2],
  board[positions.indexOf('i1')]!.path[0][3],
  board[positions.indexOf('h1')]!.path[0][1],
  board[positions.indexOf('h1')]!.path[0][2],
  board[positions.indexOf('h1')]!.path[0][3],
  board[positions.indexOf('g1')]!.path[0][1],
  board[positions.indexOf('g1')]!.path[0][2],
  board[positions.indexOf('g1')]!.path[0][3],
  board[positions.indexOf('f1')]!.path[0][1],
  board[positions.indexOf('f1')]!.path[0][2],
  board[positions.indexOf('f1')]!.path[0][3],
  board[positions.indexOf('f1')]!.path[0][4],
  board[positions.indexOf('e1')]!.path[0][3],
  board[positions.indexOf('e1')]!.path[0][4],
  board[positions.indexOf('d1')]!.path[0][2],
  board[positions.indexOf('d1')]!.path[0][3],
  board[positions.indexOf('d1')]!.path[0][4],
  board[positions.indexOf('c1')]!.path[0][2],
  board[positions.indexOf('c1')]!.path[0][3],
  board[positions.indexOf('c1')]!.path[0][4],
  board[positions.indexOf('b1')]!.path[0][2],
  board[positions.indexOf('b1')]!.path[0][3],
  board[positions.indexOf('b1')]!.path[0][4],
  board[positions.indexOf('a1')]!.path[0][2],
  board[positions.indexOf('a1')]!.path[0][3],
  board[positions.indexOf('a1')]!.path[0][4],
  board[positions.indexOf('a1')]!.path[0][5],
  board[positions.indexOf('a2')]!.path[0][3],
  board[positions.indexOf('a2')]!.path[0][4],
  board[positions.indexOf('a2')]!.path[0][5],
  board[positions.indexOf('a3')]!.path[0][3],
  board[positions.indexOf('a3')]!.path[0][4],
  board[positions.indexOf('a3')]!.path[0][5],
  board[positions.indexOf('a4')]!.path[0][3],
  board[positions.indexOf('a4')]!.path[0][4],
  board[positions.indexOf('a4')]!.path[0][5],
  board[positions.indexOf('a5')]!.path[0][3],
  board[positions.indexOf('a5')]!.path[0][4],
  board[positions.indexOf('a5')]!.path[0][5],
  board[positions.indexOf('a6')]!.path[0][3],
  board[positions.indexOf('a6')]!.path[0][4],
  board[positions.indexOf('a6')]!.path[0][5],
  board[positions.indexOf('b7')]!.path[0][4],
  board[positions.indexOf('b7')]!.path[0][5],
  board[positions.indexOf('b7')]!.path[0][0],
  board[positions.indexOf('c8')]!.path[0][4],
  board[positions.indexOf('c8')]!.path[0][5],
  board[positions.indexOf('c8')]!.path[0][0],
  board[positions.indexOf('d9')]!.path[0][4],
  board[positions.indexOf('d9')]!.path[0][5],
  board[positions.indexOf('d9')]!.path[0][0],
  board[positions.indexOf('e10')]!.path[0][4],
  board[positions.indexOf('e10')]!.path[0][5],
  board[positions.indexOf('e10')]!.path[0][0],
]
