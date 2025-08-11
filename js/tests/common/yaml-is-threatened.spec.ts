import { describe, expect, test } from 'vitest'
import { Hexchess, type Position } from '../../src'
import { yaml } from '../utils'

interface Test {
  description: string
  hexchess: string
  position: string
  expected: boolean
}

const suite = yaml<Test[]>('is-threatened.yaml')

describe('is-threatened', () => {
  for (const spec of suite) {
    test(spec.description, () => {
      const result = Hexchess.parse(spec.hexchess).isThreatened(spec.position as Position)

      expect(result).toEqual(spec.expected)
    })
  }
})
