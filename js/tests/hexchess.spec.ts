import { describe, expect, test } from 'vitest'
import { Hexchess } from '../src'

describe('Hexchess', () => {
  test('clone', () => {
    const hexchess = Hexchess.init()
    const clone = hexchess.clone()

    expect(clone).not.toBe(hexchess)
    expect(clone.board).toEqual(hexchess.board)
    expect(clone.board).not.toBe(hexchess.board)
    expect(clone.ep).toEqual(hexchess.ep)
    expect(clone.turn).toEqual(hexchess.turn)
    expect(clone.halfmove).toEqual(hexchess.halfmove)
    expect(clone.fullmove).toEqual(hexchess.fullmove)
  })
})
