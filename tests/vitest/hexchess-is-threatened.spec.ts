import { describe, expect, test } from 'vitest'
import { Hexchess } from '../../src'
import { json } from './utils'

const data = json('hexchess-is-threatened.json')

describe('is threatened', () => {
  for (const spec of data) {
    test(spec.description, () => {
      const result = Hexchess.parse(spec.from).isThreatened(spec.position)

      expect(result).toEqual(spec.expect)
    })
  }
})
