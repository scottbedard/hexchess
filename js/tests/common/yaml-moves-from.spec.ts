import { describe, expect, test } from 'vitest'
import { Hexchess, type Position } from '../../src'
import { index, position as toPosition } from '../../src/utils'
import { yaml } from '../utils'

interface Test {
  description: string
  direction: number
  expected: Position[]
  hexchess: string
  position: Position
}

const suite = yaml<Test[]>('moves-from.yaml')

describe('board-traversal', () => {
  for (const spec of suite) {
    test(spec.description, () => {
      const hexchess = Hexchess.parse(spec.hexchess)
      const moves = hexchess.movesFrom(index(spec.position)).map(san => toPosition(san.to))

      expect(moves).toEqual(spec.expected)
    })
  }
})
