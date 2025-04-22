import { describe, expect, test } from 'vitest'
import { Hexchess, type Direction } from '../../src'
import { index, walk } from '../../src/utils'
import { json } from './utils'

describe('graph traversal', () => {
  const hexchess = new Hexchess()

  json('graph-traversal.json').forEach(spec => {
    const from = index(spec.from)

    for (let direction = 0; direction < 12; direction++) {
      const result = spec.results[direction]

      test(`${spec.from} -> ${direction}`, () => {
        expect(walk(hexchess, from, direction as Direction, 'w')).toEqual(result.map(index))
      })
    }
  })
})
