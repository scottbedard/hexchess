import { describe, expect, test } from 'vitest'
import { Hexchess } from '../../js'
import { index, walk } from '../../js/utils'
import { json } from './utils'

describe('board traversal', () => {
  json('board-traversal.json').forEach(spec => {
    test(spec.description, () => {
      const hexchess = Hexchess.parse(spec.hexchess)
      const path = walk(hexchess, index(spec.from), spec.direction, spec.color)

      expect(path).toEqual(spec.result.map(index))
    })
  })
})
