import { describe, expect, test } from 'vitest'
import { Hexchess } from '../../src'
import { yaml } from '../utils'

interface Test {
  description: string
  expected: boolean
  hexchess: string
  san: string
}

const suite = yaml<Test[]>('move-legality.yaml')

describe('move-legality', () => {
  for (const spec of suite) {
    test(spec.description, () => {
      const hexchess = Hexchess.parse(spec.hexchess)

      let result: boolean | null = null

      try {
        result = hexchess.isLegal(spec.san)
      } catch {
        result = false
      }

      expect(result).toBe(spec.expected)
    })
  }
})
