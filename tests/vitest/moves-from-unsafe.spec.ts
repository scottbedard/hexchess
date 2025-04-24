import { describe, expect, test } from 'vitest'
import { Hexchess } from '../../src'
import { index } from '../../src/utils'
import { json } from './utils'

describe('piece movement', () => {
  [
    'moves-from-unsafe.json',
  ].forEach(fixture => {
    const suite = json(fixture)

    suite.forEach(spec => {
      describe(fixture.replaceAll('-', ' ').slice(0, -5), () => {
        test(spec.description, () => {
          const hexchess = Hexchess.parse(spec.from)
          const results = hexchess.movesFromUnsafe(index(spec.position)).map(String)

          expect(results, `${fixture} : ${spec.description}`).toEqual(spec.expect)
        })
      })
    })
  })
})

