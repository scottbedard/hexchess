import { describe, expect, test } from 'vitest'
import { Hexchess } from '../../src'
import { json } from './utils'

describe('hexchess parse', () => {
  for (const spec of json('hexchess-parse.json')) {
    test(spec.description, () => {
      const hexchess = Hexchess.parse(spec.fen)

      expect(hexchess).toEqual(spec.result)
    })
  }
})
