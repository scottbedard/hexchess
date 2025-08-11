import { describe, expect, test } from 'vitest'
import { Hexchess, type Position } from '../../src'
import { yaml } from '../utils'

interface Test {
  description: string
  expected: Position[]
  hexchess: string
  position: Position
}

const suite = yaml<Test[]>('get-position.yaml')

describe('get-position', () => {
  for (const spec of suite) {
    test(spec.description, () => {
      const hexchess = Hexchess.parse(spec.hexchess)
      const actual = hexchess.get(spec.position)

      if (actual) {
        expect(actual.toString()).toEqual(spec.expected)
      } else {
        expect(spec.expected).toBeNull()
      }
    })
  }
})
