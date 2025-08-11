import { describe, expect, test } from 'vitest'
import { Hexchess, type Position } from '../../src'
import { index } from '../../src/utils'
import { yaml } from '../utils'

interface Test {
  description: string
  expected: Position[]
  hexchess: string
  position: Position
}

function testMoves(file: string) {
  const suite = yaml<Test[]>(file)

  describe(file.replace('.yaml', ''), () => {
    for (const spec of suite) {
      test(spec.description, () => {
        const hexchess = Hexchess.parse(spec.hexchess)
        const moves = hexchess.movesFrom(index(spec.position)).map(san => san.toString())

        expect(moves).toEqual(spec.expected)
      })
    }
  })
}

testMoves('moves-from.yaml')

testMoves('moves-king.yaml')

testMoves('moves-knight.yaml')

testMoves('moves-pawn.yaml')

testMoves('moves-straight-line.yaml')

