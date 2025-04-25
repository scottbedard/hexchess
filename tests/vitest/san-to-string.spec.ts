import { describe, expect, test } from 'vitest'
import { San } from '../../src'
import { json } from './utils'

const data = json('san-to-string.json')

describe('san to string', () => {
  for (const spec of data) {
    test(spec.description, () => {
      const san = new San({
        from: spec.san.from,
        promotion: spec.san.promotion,
        to: spec.san.to,
      })

      expect(san.toString()).toEqual(spec.expect)
    })
  }
})
