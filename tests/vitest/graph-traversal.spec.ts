import { describe, expect, test } from 'vitest'
import { Hexchess } from '../../src'
import { index, walk } from '../../src/utils'
import { json } from './utils'

describe('graph traversal', () => {
  const hexchess = new Hexchess()

  json('graph-traversal.json').forEach(spec => {
    test(`${spec.from} -> ${spec.direction}`, () => {
      expect(walk(hexchess, index(spec.from), spec.direction, 'w')).toEqual(spec.result.map(index))
    })
  })
})
