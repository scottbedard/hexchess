import { describe, expect, test } from 'vitest'
import { Hexchess, position } from '../../pkg'
import { json } from './utils'

const data = json('get-color.json')

describe('hexchess get color', () => {
  for (const spec of data) {
    test(spec.description, () => {
      const hexchess = Hexchess.parse(spec.from)
      const moves = hexchess.getColor(spec.color).map(n => position(n))

      expect(moves).toEqual(spec.result)
    })
  }
})
