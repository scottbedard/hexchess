import { describe, expect, test } from 'vitest'
import { position, San } from '../../src'
import { json } from './utils'

const data = json('san-parse.json')

describe('hexchess san parse', () => {
  for (const spec of data) {
    test(spec.description, () => {
      let result: null | San = null
      let error = false

      try {
        result = San.from(spec.san)
      } catch {
        error = true
      }

      expect(error).toEqual(spec.error)

      if (spec.expect || result) {
        expect(position(result?.from ?? -1)).toEqual(spec.expect?.from)
        expect(result?.promotion).toEqual(spec.expect?.promotion)
        expect(position(result?.to ?? -1)).toEqual(spec.expect?.to)
      }
    })
  }
})
