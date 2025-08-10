import { describe, expect, test } from 'vitest'
import { Hexchess, type Direction } from '../../src'
import { index, position as toPosition, walk } from '../../src/utils'
import { yaml } from '../utils'

const data = yaml('board-traversal.yaml')

describe('board traversal', () => {
  const hexchess = new Hexchess()

  for (const spec of data) {
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
