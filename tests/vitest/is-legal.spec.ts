import { describe, expect, test } from 'vitest'
import { Hexchess } from '../../js'
import { json } from './utils'

const data = json('move-legality.json')

describe('move legality', () => {
  for (const spec of data) {
    test(spec.description, () => {
      const hexchess = Hexchess.parse(spec.from)

      let result: boolean | null = null

      try {
        result = hexchess.isLegal(spec.san)
      } catch {
        result = false
      }

      expect(result).toBe(spec.result)
    })
  }
})
