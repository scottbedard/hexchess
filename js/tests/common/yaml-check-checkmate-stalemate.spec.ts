import { describe, expect, test } from 'vitest'
import { Hexchess } from '../../src'
import { yaml } from '../utils'

interface Test {
  check: boolean
  checkmate: boolean
  description: string
  hexchess: string
  stalemate: boolean
}

const suite = yaml<Test[]>('check-checkmate-stalemate.yaml')

describe('check-checkmate-stalemate', () => {
  for (const spec of suite) {
    test(spec.description, () => {
      const hexchess = Hexchess.parse(spec.hexchess)

      expect(spec.check).toBe(hexchess.isCheck())
      expect(spec.checkmate).toBe(hexchess.isCheckmate())
      expect(spec.stalemate).toBe(hexchess.isStalemate())
    })
  }
})
