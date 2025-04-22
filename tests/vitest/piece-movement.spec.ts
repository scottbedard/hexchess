import { describe, expect, test } from 'vitest'
import { Hexchess } from '../../src'
import { index } from '../../src/utils'
import { json } from './utils'

describe('piece movement', () => {
  json('straight-line-moves.json').forEach(spec => {
    test(spec.description, () => {
      const hexchess = Hexchess.parse(spec.hexchess)
      const results = hexchess.movesFrom(index(spec.from)).map(String)

      expect(results).toEqual(spec.result)
    })
  })
})
