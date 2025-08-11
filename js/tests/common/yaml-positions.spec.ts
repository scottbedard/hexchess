import { describe, expect, test } from 'vitest'
import { index, position as toPosition, type Position} from '../../src'
import { yaml } from '../utils'

const suite = yaml<string[]>('positions.yaml')

describe('positions', () => {
  let i = 0
  for (const position of suite) {
    test(position, () => {
      expect(index(position as Position)).toEqual(i)
      expect(toPosition(i)).toEqual(position)
      i++
    })
  }
})
