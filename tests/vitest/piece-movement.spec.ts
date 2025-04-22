import { describe, expect, test } from 'vitest'
import { Hexchess } from '../../src'
import { index } from '../../src/utils'
import { json } from './utils'

describe('piece movement', () => {
  [
    json('knight-moves.json'),
    json('straight-line-moves.json'),
  ].forEach(suite => {
    suite.forEach(spec => {
      test(spec.description, () => {
        const hexchess = Hexchess.parse(spec.hexchess)
        const results = hexchess.movesFrom(index(spec.from)).map(String)

        expect(results, spec.description).toEqual(spec.result)
      })
    })
  })
})
