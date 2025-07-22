import { describe, expect, test } from 'vitest'
import { Hexchess } from '../../js'
import { json } from './utils'

const data = json('hexchess-apply.json')

describe('hexchess parse', () => {
  for (const spec of data) {
    test(spec.description, () => {
      if (spec.error) {
        expect(() => Hexchess.parse(spec.from).apply(spec.sequence)).toThrow()
        return
      }

      const hexchess = Hexchess.parse(spec.from).apply(spec.sequence)

      if (spec.to) {
        expect(hexchess.toString(), spec.description).toBe(spec.to)
      }
    })
  }
})
