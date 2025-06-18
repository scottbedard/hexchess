import { describe, expect, test } from 'vitest'
import { Hexchess } from '../../pkg'
import { json } from './utils'

const data = json('check-checkmate-stalemate.json')

describe('check checkmate stalemate', () => {
  for (const spec of data) {
    test(spec.description, () => {
      const hexchess = Hexchess.parse(spec.from)

      expect(spec.check).toBe(hexchess.isCheck())
      expect(spec.checkmate).toBe(hexchess.isCheckmate())
      expect(spec.stalemate).toBe(hexchess.isStalemate())
    })
  }
})
