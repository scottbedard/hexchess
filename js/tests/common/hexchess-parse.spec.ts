import { describe, expect, test } from 'vitest'
import { Hexchess } from '../../src'
import { json } from '../utils'

describe('hexchess parse', () => {
  for (const spec of json('hexchess-parse.json')) {
    test(spec.description, () => {
      if (spec.error) {
        expect(() => Hexchess.parse(spec.fen), spec.description).toThrow()
      } else {
        expect(Hexchess.parse(spec.fen), spec.description).toEqual(spec.result)
      }
    })
  }
})
