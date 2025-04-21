import { describe, expect, test } from 'vitest'
import { Hexchess } from '../src'
import { index, walk } from '../src/utils'

describe('walk', () => {
  test('stop at board edge', () => {
    const hexchess = new Hexchess()

    expect(walk(hexchess, index('f6'), 0, 'w')).toEqual([
      index('f7'),
      index('f8'),
      index('f9'),
      index('f10'),
      index('f11'),
    ])
  })

  test('stop before friendly piece', () => {
    const hexchess = Hexchess.parse('1/3/2P2/7/9/5R5/11/11/11/11/11 w - 0 1')

    expect(walk(hexchess, index('f6'), 0, 'w')).toEqual([
      index('f7'),
      index('f8'),
      // f9 is a friendly pawn
    ])
  })

  test('stop on hostile piece', () => {
    const hexchess = Hexchess.parse('1/3/2p2/7/9/5R5/11/11/11/11/11 w - 0 1')

    expect(walk(hexchess, index('f6'), 0, 'w')).toEqual([
      index('f7'),
      index('f8'),
      index('f9'), // <- f9 is a hostile pawn
    ])
  })
})
