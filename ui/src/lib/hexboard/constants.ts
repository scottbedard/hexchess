import { bilerp, flip, hexagon, pivot, reflect, sum } from './geometry'
import type { HexboardOptions, Vec } from './types'

/** svg viewbox, increasing this zooms out away from the board */
export const box = 23.6

export const defaultOptions: HexboardOptions = {
  colors: [
    'oklch(0.9015 0.0729 70.7)',
    'oklch(0.8366 0.1165 66.29)',
    'oklch(0.6806 0.1423 75.83)',
  ],
  highlightColor: 'oklch(90.5% 0.182 98.111 / 75%)', // yellow-300 / 75% opacity
  labelActiveColor: 'oklch(76.9% 0.188 70.08)', // amber-500
  labelColor: 'oklch(55.4% 0.046 257.417)', // slate-500
  labelInactiveColor: 'oklch(70.4% 0.04 256.788)', // slate-400
  labels: true,
  selectedColor: 'oklch(63.7% 0.237 25.331)', // red-500
  targetColor: 'oklch(63.7% 0.237 25.331)', // red-500
} as const

/** empty position */
export const emptyPosition = '1/3/5/7/9/11/11/11/11/11/11 w - 0 1'

/** initial position */
export const initialPosition = 'b/qbk/n1b1n/r5r/ppppppppp/11/5P5/4P1P4/3P1B1P3/2P2B2P2/1PRNQBKNRP1 w - 0 1'

/** label offset */
export const labelOffset = 1.7

/** piece size */
export const pieceSize = 1.7

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
export const board: [
  color: number,
  origin: Vec<2>,
  reflectedOrigin: Vec<2>,
  path: Vec<6, Vec<2>>,
  reflectedPath: Vec<6, Vec<2>>
][] = [
  [2, f11, reflect(f11), hexagon(f11), flip(hexagon(f11))],
  [1, e10, reflect(e10), hexagon(e10), flip(hexagon(e10))],
  [0, f10, reflect(f10), hexagon(f10), flip(hexagon(f10))],
  [1, g10, reflect(g10), hexagon(g10), flip(hexagon(g10))],
  [0, d9, reflect(d9), hexagon(d9), flip(hexagon(d9))],
  [2, e9, reflect(e9), hexagon(e9), flip(hexagon(e9))],
  [1, f9, reflect(f9), hexagon(f9), flip(hexagon(f9))],
  [2, g9, reflect(g9), hexagon(g9), flip(hexagon(g9))],
  [0, h9, reflect(h9), hexagon(h9), flip(hexagon(h9))],
  [2, c8, reflect(c8), hexagon(c8), flip(hexagon(c8))],
  [1, d8, reflect(d8), hexagon(d8), flip(hexagon(d8))],
  [0, e8, reflect(e8), hexagon(e8), flip(hexagon(e8))],
  [2, f8, reflect(f8), hexagon(f8), flip(hexagon(f8))],
  [0, g8, reflect(g8), hexagon(g8), flip(hexagon(g8))],
  [1, h8, reflect(h8), hexagon(h8), flip(hexagon(h8))],
  [2, i8, reflect(i8), hexagon(i8), flip(hexagon(i8))],
  [1, b7, reflect(b7), hexagon(b7), flip(hexagon(b7))],
  [0, c7, reflect(c7), hexagon(c7), flip(hexagon(c7))],
  [2, d7, reflect(d7), hexagon(d7), flip(hexagon(d7))],
  [1, e7, reflect(e7), hexagon(e7), flip(hexagon(e7))],
  [0, f7, reflect(f7), hexagon(f7), flip(hexagon(f7))],
  [1, g7, reflect(g7), hexagon(g7), flip(hexagon(g7))],
  [2, h7, reflect(h7), hexagon(h7), flip(hexagon(h7))],
  [0, i7, reflect(i7), hexagon(i7), flip(hexagon(i7))],
  [1, k7, reflect(k7), hexagon(k7), flip(hexagon(k7))],
  [0, a6, reflect(a6), hexagon(a6), flip(hexagon(a6))],
  [2, b6, reflect(b6), hexagon(b6), flip(hexagon(b6))],
  [1, c6, reflect(c6), hexagon(c6), flip(hexagon(c6))],
  [0, d6, reflect(d6), hexagon(d6), flip(hexagon(d6))],
  [2, e6, reflect(e6), hexagon(e6), flip(hexagon(e6))],
  [1, f6, reflect(f6), hexagon(f6), flip(hexagon(f6))],
  [2, g6, reflect(g6), hexagon(g6), flip(hexagon(g6))],
  [0, h6, reflect(h6), hexagon(h6), flip(hexagon(h6))],
  [1, i6, reflect(i6), hexagon(i6), flip(hexagon(i6))],
  [2, k6, reflect(k6), hexagon(k6), flip(hexagon(k6))],
  [0, l6, reflect(l6), hexagon(l6), flip(hexagon(l6))],
  [1, a5, reflect(a5), hexagon(a5), flip(hexagon(a5))],
  [0, b5, reflect(b5), hexagon(b5), flip(hexagon(b5))],
  [2, c5, reflect(c5), hexagon(c5), flip(hexagon(c5))],
  [1, d5, reflect(d5), hexagon(d5), flip(hexagon(d5))],
  [0, e5, reflect(e5), hexagon(e5), flip(hexagon(e5))],
  [2, f5, reflect(f5), hexagon(f5), flip(hexagon(f5))],
  [0, g5, reflect(g5), hexagon(g5), flip(hexagon(g5))],
  [1, h5, reflect(h5), hexagon(h5), flip(hexagon(h5))],
  [2, i5, reflect(i5), hexagon(i5), flip(hexagon(i5))],
  [0, k5, reflect(k5), hexagon(k5), flip(hexagon(k5))],
  [1, l5, reflect(l5), hexagon(l5), flip(hexagon(l5))],
  [2, a4, reflect(a4), hexagon(a4), flip(hexagon(a4))],
  [1, b4, reflect(b4), hexagon(b4), flip(hexagon(b4))],
  [0, c4, reflect(c4), hexagon(c4), flip(hexagon(c4))],
  [2, d4, reflect(d4), hexagon(d4), flip(hexagon(d4))],
  [1, e4, reflect(e4), hexagon(e4), flip(hexagon(e4))],
  [0, f4, reflect(f4), hexagon(f4), flip(hexagon(f4))],
  [1, g4, reflect(g4), hexagon(g4), flip(hexagon(g4))],
  [2, h4, reflect(h4), hexagon(h4), flip(hexagon(h4))],
  [0, i4, reflect(i4), hexagon(i4), flip(hexagon(i4))],
  [1, k4, reflect(k4), hexagon(k4), flip(hexagon(k4))],
  [2, l4, reflect(l4), hexagon(l4), flip(hexagon(l4))],
  [0, a3, reflect(a3), hexagon(a3), flip(hexagon(a3))],
  [2, b3, reflect(b3), hexagon(b3), flip(hexagon(b3))],
  [1, c3, reflect(c3), hexagon(c3), flip(hexagon(c3))],
  [0, d3, reflect(d3), hexagon(d3), flip(hexagon(d3))],
  [2, e3, reflect(e3), hexagon(e3), flip(hexagon(e3))],
  [1, f3, reflect(f3), hexagon(f3), flip(hexagon(f3))],
  [2, g3, reflect(g3), hexagon(g3), flip(hexagon(g3))],
  [0, h3, reflect(h3), hexagon(h3), flip(hexagon(h3))],
  [1, i3, reflect(i3), hexagon(i3), flip(hexagon(i3))],
  [2, k3, reflect(k3), hexagon(k3), flip(hexagon(k3))],
  [0, l3, reflect(l3), hexagon(l3), flip(hexagon(l3))],
  [1, a2, reflect(a2), hexagon(a2), flip(hexagon(a2))],
  [0, b2, reflect(b2), hexagon(b2), flip(hexagon(b2))],
  [2, c2, reflect(c2), hexagon(c2), flip(hexagon(c2))],
  [1, d2, reflect(d2), hexagon(d2), flip(hexagon(d2))],
  [0, e2, reflect(e2), hexagon(e2), flip(hexagon(e2))],
  [2, f2, reflect(f2), hexagon(f2), flip(hexagon(f2))],
  [0, g2, reflect(g2), hexagon(g2), flip(hexagon(g2))],
  [1, h2, reflect(h2), hexagon(h2), flip(hexagon(h2))],
  [2, i2, reflect(i2), hexagon(i2), flip(hexagon(i2))],
  [0, k2, reflect(k2), hexagon(k2), flip(hexagon(k2))],
  [1, l2, reflect(l2), hexagon(l2), flip(hexagon(l2))],
  [2, a1, reflect(a1), hexagon(a1), flip(hexagon(a1))],
  [1, b1, reflect(b1), hexagon(b1), flip(hexagon(b1))],
  [0, c1, reflect(c1), hexagon(c1), flip(hexagon(c1))],
  [2, d1, reflect(d1), hexagon(d1), flip(hexagon(d1))],
  [1, e1, reflect(e1), hexagon(e1), flip(hexagon(e1))],
  [0, f1, reflect(f1), hexagon(f1), flip(hexagon(f1))],
  [1, g1, reflect(g1), hexagon(g1), flip(hexagon(g1))],
  [2, h1, reflect(h1), hexagon(h1), flip(hexagon(h1))],
  [0, i1, reflect(i1), hexagon(i1), flip(hexagon(i1))],
  [1, k1, reflect(k1), hexagon(k1), flip(hexagon(k1))],
  [2, l1, reflect(l1), hexagon(l1), flip(hexagon(l1))],
]

/** board labels */
export const labels: [string, Vec<2>, Vec<2>][] = [
  ['11', bilerp(g10, f11, labelOffset), reflect(bilerp(g10, f11, labelOffset))],
  ['10', bilerp(f10, e10, labelOffset), reflect(bilerp(f10, e10, labelOffset))],
  ['9', bilerp(e9, d9, labelOffset), reflect(bilerp(e9, d9, labelOffset))],
  ['8', bilerp(d8, c8, labelOffset), reflect(bilerp(d8, c8, labelOffset))],
  ['7', bilerp(c7, b7, labelOffset), reflect(bilerp(c7, b7, labelOffset))],
  ['6', bilerp(b6, a6, labelOffset), reflect(bilerp(b6, a6, labelOffset))],
  ['5', bilerp(b5, a5, labelOffset), reflect(bilerp(b5, a5, labelOffset))],
  ['4', bilerp(b4, a4, labelOffset), reflect(bilerp(b4, a4, labelOffset))],
  ['3', bilerp(b3, a3, labelOffset), reflect(bilerp(b3, a3, labelOffset))],
  ['2', bilerp(b2, a2, labelOffset), reflect(bilerp(b2, a2, labelOffset))],
  ['1', bilerp(b1, a1, labelOffset), reflect(bilerp(b1, a1, labelOffset))],

  ['a', bilerp(a2, a1, labelOffset), reflect(bilerp(a2, a1, labelOffset))],
  ['b', bilerp(b2, b1, labelOffset), reflect(bilerp(b2, b1, labelOffset))],
  ['c', bilerp(c2, c1, labelOffset), reflect(bilerp(c2, c1, labelOffset))],
  ['d', bilerp(d2, d1, labelOffset), reflect(bilerp(d2, d1, labelOffset))],
  ['e', bilerp(e2, e1, labelOffset), reflect(bilerp(e2, e1, labelOffset))],
  ['f', bilerp(f2, f1, labelOffset), reflect(bilerp(f2, f1, labelOffset))],
  ['g', bilerp(g2, g1, labelOffset), reflect(bilerp(g2, g1, labelOffset))],
  ['h', bilerp(h2, h1, labelOffset), reflect(bilerp(h2, h1, labelOffset))],
  ['i', bilerp(i2, i1, labelOffset), reflect(bilerp(i2, i1, labelOffset))],
  ['k', bilerp(k2, k1, labelOffset), reflect(bilerp(k2, k1, labelOffset))],
  ['l', bilerp(l2, l1, labelOffset), reflect(bilerp(l2, l1, labelOffset))],

  ['1', bilerp(k1, l1, labelOffset), reflect(bilerp(k1, l1, labelOffset))],
  ['2', bilerp(k2, l2, labelOffset), reflect(bilerp(k2, l2, labelOffset))],
  ['3', bilerp(k3, l3, labelOffset), reflect(bilerp(k3, l3, labelOffset))],
  ['4', bilerp(k4, l4, labelOffset), reflect(bilerp(k4, l4, labelOffset))],
  ['5', bilerp(k5, l5, labelOffset), reflect(bilerp(k5, l5, labelOffset))],
  ['6', bilerp(k6, l6, labelOffset), reflect(bilerp(k6, l6, labelOffset))],
  ['7', bilerp(i7, k7, labelOffset), reflect(bilerp(i7, k7, labelOffset))],
  ['8', bilerp(h8, i8, labelOffset), reflect(bilerp(h8, i8, labelOffset))],
  ['9', bilerp(g9, h9, labelOffset), reflect(bilerp(g9, h9, labelOffset))],
  ['10', bilerp(f10, g10, labelOffset), reflect(bilerp(f10, g10, labelOffset))],
  ['11', bilerp(e10, f11, labelOffset), reflect(bilerp(e10, f11, labelOffset))],
]

/** perimeter of the board */
export const perimeter: Vec<93, Vec<2>> = [
  board[0 /* f11 */][3][5],
  board[0 /* f11 */][3][0],
  board[0 /* f11 */][3][1],
  board[3 /* g10 */][3][5],
  board[3 /* g10 */][3][0],
  board[3 /* g10 */][3][1],
  board[8 /* h9 */][3][5],
  board[8 /* h9 */][3][0],
  board[8 /* h9 */][3][1],
  board[15 /* i8 */][3][5],
  board[15 /* i8 */][3][0],
  board[15 /* i8 */][3][1],
  board[24 /* k7 */][3][5],
  board[24 /* k7 */][3][0],
  board[24 /* k7 */][3][1],
  board[35 /* l6 */][3][5],
  board[35 /* l6 */][3][0],
  board[35 /* l6 */][3][1],
  board[35 /* l6 */][3][2],
  board[46 /* l5 */][3][0],
  board[46 /* l5 */][3][1],
  board[46 /* l5 */][3][2],
  board[57 /* l4 */][3][0],
  board[57 /* l4 */][3][1],
  board[57 /* l4 */][3][2],
  board[68 /* l3 */][3][0],
  board[68 /* l3 */][3][1],
  board[68 /* l3 */][3][2],
  board[79 /* l2 */][3][0],
  board[79 /* l2 */][3][1],
  board[79 /* l2 */][3][2],
  board[90 /* l1 */][3][0],
  board[90 /* l1 */][3][1],
  board[90 /* l1 */][3][2],
  board[90 /* l1 */][3][3],
  board[89 /* k1 */][3][2],
  board[89 /* k1 */][3][3],
  board[89 /* k1 */][3][4],
  board[88 /* i1 */][3][1],
  board[88 /* i1 */][3][2],
  board[88 /* i1 */][3][3],
  board[87 /* h1 */][3][1],
  board[87 /* h1 */][3][2],
  board[87 /* h1 */][3][3],
  board[86 /* g1 */][3][1],
  board[86 /* g1 */][3][2],
  board[86 /* g1 */][3][3],
  board[85 /* f1 */][3][1],
  board[85 /* f1 */][3][2],
  board[85 /* f1 */][3][3],
  board[85 /* f1 */][3][4],
  board[84 /* e1 */][3][3],
  board[84 /* e1 */][3][4],
  board[83 /* d1 */][3][2],
  board[83 /* d1 */][3][3],
  board[83 /* d1 */][3][4],
  board[82 /* c1 */][3][2],
  board[82 /* c1 */][3][3],
  board[82 /* c1 */][3][4],
  board[81 /* b1 */][3][2],
  board[81 /* b1 */][3][3],
  board[81 /* b1 */][3][4],
  board[80 /* a1 */][3][2],
  board[80 /* a1 */][3][3],
  board[80 /* a1 */][3][4],
  board[80 /* a1 */][3][5],
  board[69 /* a2 */][3][3],
  board[69 /* a2 */][3][4],
  board[69 /* a2 */][3][5],
  board[58 /* a3 */][3][3],
  board[58 /* a3 */][3][4],
  board[58 /* a3 */][3][5],
  board[47 /* a4 */][3][3],
  board[47 /* a4 */][3][4],
  board[47 /* a4 */][3][5],
  board[36 /* a5 */][3][3],
  board[36 /* a5 */][3][4],
  board[36 /* a5 */][3][5],
  board[25 /* a6 */][3][3],
  board[25 /* a6 */][3][4],
  board[25 /* a6 */][3][5],
  board[16 /* b7 */][3][4],
  board[16 /* b7 */][3][5],
  board[16 /* b7 */][3][0],
  board[9 /* c8 */][3][4],
  board[9 /* c8 */][3][5],
  board[9 /* c8 */][3][0],
  board[4 /* d9 */][3][4],
  board[4 /* d9 */][3][5],
  board[4 /* d9 */][3][0],
  board[1 /* e10 */][3][4],
  board[1 /* e10 */][3][5],
  board[1 /* e10 */][3][0],
]
