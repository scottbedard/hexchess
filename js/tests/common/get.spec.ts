import { describe, expect, test } from 'vitest'
import { Hexchess } from '../../src'
import { json } from '../utils'

const data = json('get.json')

describe('get', () => {
  for (const spec of data) {
    test(spec.description, () => {
      const hexchess = Hexchess.parse(spec.from)
      const result = hexchess.get(spec.position)

      if (result) {
        expect(result.toString()).toEqual(spec.result)
      } else {
        expect(spec.result).toBeNull()
      }
    })
  }
})
