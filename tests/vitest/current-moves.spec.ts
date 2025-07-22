import { describe, expect, test } from 'vitest'
import { Hexchess } from '../../js'
import { json } from './utils'

const data = json('current-moves.json')

describe('hexchess current moves', () => {
  for (const spec of data) {
    test(spec.description, () => {
      const hexchess = Hexchess.parse(spec.from)
      const moves = hexchess.currentMoves().map(san => san.toString())

      expect(moves).toEqual(spec.result)
    })
  }
})
