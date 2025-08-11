import { describe, expect, test } from 'vitest'
import { Hexchess } from '../../src'
import { yaml } from '../utils'

interface Test {
  description: string
  hexchess: string
  expected: string[]
}

const suite = yaml<Test[]>('current-moves.yaml')

describe('current-moves', () => {
  for (const spec of suite) {
    test(spec.description, () => {
      const hexchess = Hexchess.parse(spec.hexchess)
      const moves = hexchess.currentMoves().map(san => san.toString())

      expect(moves).toEqual(spec.expected)
    })
  }
})
