import { describe, expect, test } from 'vitest'
import { Hexchess } from '../../src'
import { yaml } from '../utils'

interface Test {
  description: string
  error: boolean
  expected: string | null
  hexchess: string
  moves: string
}

const suite = yaml<Test[]>('apply-moves.yaml')

describe('apply-moves', () => {
  for (const spec of suite) {
    test(spec.description, () => {
      if (spec.error) {
        expect(() => Hexchess.parse(spec.hexchess).apply(spec.moves)).toThrow()
        return
      }

      const hexchess = Hexchess.parse(spec.hexchess).apply(spec.moves)

      if (spec.expected) {
        expect(hexchess.toString(), spec.description).toBe(spec.expected)
      }
    })
  }
})
