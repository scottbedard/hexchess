import { expect, test } from 'vitest'
import { index, position } from '../../src/utils'
import { json } from '../utils'

test('position', () => {
  const positions = json('positions.json')

  positions.forEach((name, i) => {
    expect(index(name)).toEqual(i)
    expect(position(i)).toEqual(name)
  })
})
