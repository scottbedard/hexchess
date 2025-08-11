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

const suite = yaml<Test[]>('apply-moves-unsafe.yaml')

describe('apply-moves-unsafe', () => {
  for (const spec of suite) {
    test(spec.description, () => {
      const moves = spec.moves.split(' ')

      if (spec.error) {
        expect(() => {
          const hexchess = Hexchess.parse(spec.hexchess)
          moves.forEach(san => hexchess.applyMoveUnsafe(san))
        }).toThrow()
        return
      }

      const hexchess = Hexchess.parse(spec.hexchess)
      moves.forEach(san => hexchess.applyMoveUnsafe(san))

      if (spec.expected) {
        expect(hexchess.toString(), spec.description).toBe(spec.expected)
      }
    })
  }
})
