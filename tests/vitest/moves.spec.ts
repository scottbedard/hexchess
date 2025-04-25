import { describe, expect, test } from 'vitest'
import { Hexchess } from '../../src'
import { index } from '../../src/utils'
import { json } from './utils'

describe('piece movement', () => {
  [
    'moves-from.json',
    'moves-king.json',
    'moves-knight.json',
    'moves-pawn.json',
    'moves-straight-line.json',
  ].forEach(fixture => {
    const suite = json(fixture)

    suite.forEach(spec => {
      describe(fixture.replaceAll('-', ' ').slice(0, -5), () => {
        test(spec.description, () => {
          const hexchess = Hexchess.parse(spec.from)
          const results = hexchess.movesFrom(index(spec.position)).map(String)

          expect(results, `${fixture} : ${spec.description}`).toEqual(spec.expect)
        })
      })
    })
  })
})

