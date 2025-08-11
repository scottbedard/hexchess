import { describe, expect, test } from 'vitest'
import { Hexchess, type Board, type Color } from '../../src'
import { yaml } from '../utils'

interface Test {
  description: string
  hexchess: {
    board: Board
    ep: number | null
    fullmove: number
    halfmove: number
    turn: string
  }
  expected: string[]
}

const suite = yaml<Test[]>('stringify-hexchess.yaml')

describe('stringify-hexchess', () => {
  for (const spec of suite) {
    test(spec.description, () => {
      const hexchess = new Hexchess()
      hexchess.board = spec.hexchess.board.map(piece => piece) as Board
      hexchess.ep = spec.hexchess.ep
      hexchess.fullmove = spec.hexchess.fullmove
      hexchess.halfmove = spec.hexchess.halfmove
      hexchess.turn = spec.hexchess.turn as Color

      expect(hexchess.toString()).toEqual(spec.expected)
    })
  }
})
