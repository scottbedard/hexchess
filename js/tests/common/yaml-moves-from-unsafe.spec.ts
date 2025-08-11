import { describe, expect, test } from 'vitest'
import { Hexchess, type Position } from '../../src'
import { index } from '../../src/utils'
import { yaml } from '../utils'

interface Test {
  description: string
  expected: string[]
  hexchess: string
  position: string
}

const suite = yaml<Test[]>('moves-from-unsafe.yaml')

describe('moves-from-unsafe', () => {
  for (const spec of suite) {
    test(spec.description, () => {
      const hexchess = Hexchess.parse(spec.hexchess)
      const results = hexchess.movesFromUnsafe(index(spec.position as Position)).map(String)

      expect(results, `${spec.description}`).toEqual(spec.expected)
    })
  }
})
