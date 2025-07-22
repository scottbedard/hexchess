import { describe, expect, test } from 'vitest'
import { Hexchess, position } from '../../js'
import { json } from './utils'

const data = json('find-king.json')

describe('find king', () => {
  for (const spec of data) {
    test(spec.description, () => {
      const hexchess = Hexchess.parse(spec.from)
      const king = hexchess.findKing(spec.color)

      if (typeof king === 'number') {
        expect(position(king)).toEqual(spec.result)
      } else {
        expect(king).toBeNull()
      }
    })
  }
})
