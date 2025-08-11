import { describe, expect, test } from 'vitest'
import { San, type PromotionPiece, type Position } from '../../src'
import { yaml } from '../utils'

interface Test {
  description: string
  expected: string
  san: {
    from: string
    promotion: string | null
    to: string
  }
}

const suite = yaml<Test[]>('stringify-san.yaml')

describe('stringify-san', () => {
  for (const spec of suite) {
    test(spec.description, () => {
      const san = new San({
        from: spec.san.from as Position,
        promotion: spec.san.promotion as PromotionPiece | null,
        to: spec.san.to as Position,
      })

      expect(san.toString()).toEqual(spec.expected)
    })
  }
})
