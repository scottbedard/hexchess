import { describe, expect, test } from 'vitest'
import { Hexchess, type Direction, type Position } from '../../src'
import { index, position as toPosition, walk } from '../../src/utils'
import { yaml } from '../utils'

interface Test {
  expected: Position[][]
  position: Position
}

const suite = yaml<Test[]>('board-traversal.yaml')

describe('board-traversal', () => {
  const hexchess = new Hexchess()

  for (const spec of suite) {
    test(spec.position, () => {
      const position = index(spec.position)

      for (let direction = 0; direction < 12; direction++) {
        const expected = spec.expected[direction]
        const actual = walk(hexchess, position, direction as Direction, 'w').map(toPosition)

        expect(actual).toEqual(expected)
      }
    })
  }
})
