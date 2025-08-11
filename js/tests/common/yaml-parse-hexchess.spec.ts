import { describe, expect, test } from 'vitest'
import { Hexchess } from '../../src'
import { yaml } from '../utils'

interface Test {
  description: string
  error: boolean
  expected: null | string[]
  hexchess: string
}

const suite = yaml<Test[]>('parse-hexchess.yaml')

describe('parse hexchess', () => {
  for (const spec of suite) {
    test(spec.description, () => {
      if (spec.error) {
        expect(() => Hexchess.parse(spec.hexchess)).toThrow()
        return
      }

      const hexchess = Hexchess.parse(spec.hexchess)

      expect(hexchess.board).toEqual(spec.expected)
    })
  }
})
