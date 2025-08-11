import { describe, expect, test } from 'vitest'
import { Color, Hexchess, position } from '../../src'
import { yaml } from '../utils'

interface Test {
  description: string,
  color: string,
  expected: string[]
  hexchess: string,
}

const suite = yaml<Test[]>('get-color.yaml')

describe('get-color', () => {
  for (const spec of suite) {
    test(spec.description, () => {
      const hexchess = Hexchess.parse(spec.hexchess)
      const positions = hexchess.getColor(spec.color as Color).map(n => position(n))

      expect(positions).toEqual(spec.expected)
    })
  }
})
