import { describe, expect, test } from 'vitest'
import { Hexchess } from '../../src'
import { index } from '../../src/utils'
import { json } from './utils'

describe('piece movement', () => {
  [
    'piece-moves-king.json',
    'piece-moves-knight.json',
    'piece-moves-pawn.json',
    'piece-moves-straight-line.json',
  ].forEach(fixture => {
    const suite = json(fixture)

    suite.forEach(spec => {
      describe(fixture.replaceAll('-', ' ').slice(0, -5), () => {
        test(spec.description, () => {
          const hexchess = Hexchess.parse(spec.hexchess)
          const results = hexchess.movesFrom(index(spec.from)).map(String)

          expect(results, `${fixture} : ${spec.description}`).toEqual(spec.result)
        })
      })
    })
  })
})

