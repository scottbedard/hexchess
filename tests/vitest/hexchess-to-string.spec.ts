import { describe, expect, test } from 'vitest'
import { Hexchess } from '../../src'
import { json } from './utils'

const data = json('hexchess-to-string.json')

describe('hexchess to string', () => {
  for (const spec of data) {
    test(spec.description, () => {
      const hexchess = new Hexchess()
      hexchess.board = spec.from.board
      hexchess.ep = spec.from.ep
      hexchess.fullmove = spec.from.fullmove
      hexchess.halfmove = spec.from.halfmove
      hexchess.turn = spec.from.turn

      expect(hexchess.toString()).toEqual(spec.result)
    })
  }
})
