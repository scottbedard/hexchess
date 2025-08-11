import { describe, expect, test } from 'vitest'
import { San, position } from '../../src'
import { yaml } from '../utils'

interface Test {
  description: string
  san: string
  error: boolean
  expected: null | {
    from: string
    promotion: string | null
    to: string
  }
}

const suite = yaml<Test[]>('parse-san.yaml')

describe('parse-san', () => {
  for (const spec of suite) {
    test(spec.description, () => {
      let result: null | San = null
      let error = false

      try {
        result = San.from(spec.san)
      } catch {
        error = true
      }

      expect(error).toEqual(spec.error)

      if (spec.expected || result) {
        expect(position(result?.from ?? -1)).toEqual(spec.expected?.from)
        expect(result?.promotion).toEqual(spec.expected?.promotion)
        expect(position(result?.to ?? -1)).toEqual(spec.expected?.to)
      }
    })
  }
})
