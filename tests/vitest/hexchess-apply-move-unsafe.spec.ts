import { describe, expect, test } from 'vitest'
import { Hexchess } from '../../pkg'
import { json } from './utils'

const data = json('hexchess-apply-move-unsafe.json')

describe('hexchess apply move unsafe', () => {
  for (const spec of data) {
    test(spec.description, () => {
      if (spec.error) {
        expect(() => Hexchess.parse(spec.from).applyMoveUnsafe(spec.sequence)).toThrow()
        return
      }

      const hexchess = Hexchess.parse(spec.from).applyMoveUnsafe(spec.sequence)

      if (spec.to) {
        expect(hexchess.toString(), spec.description).toBe(spec.to)
      }
    })
  }
})
