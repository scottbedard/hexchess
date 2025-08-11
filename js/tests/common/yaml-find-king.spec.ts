import { describe, expect, test } from 'vitest'
import { Color, Hexchess, position } from '../../src'
import { yaml } from '../utils'

interface Test {
  color: string
  description: string
  expected: null | string
  hexchess: string
}

const suite = yaml<Test[]>('find-king.yaml')

describe('find-king', () => {
  for (const spec of suite) {
    test(spec.description, () => {
      const hexchess = Hexchess.parse(spec.hexchess)
      const king = hexchess.findKing(spec.color as Color)

      if (typeof king === 'number') {
        expect(position(king)).toEqual(spec.expected)
      } else {
        expect(king).toBeNull()
      }
    })
  }
})
