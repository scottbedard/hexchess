import { describe, test, expect } from 'vitest'
import { Hexchess, index } from '../../src'
describe('knight', () => {
  test('near edge of board', () => {
    const result = Hexchess.parse('1/1N1/5/7/9/11/11/11/11/11/11 w - 0 1').movesFrom('f10')

    expect(result.length).toEqual(6)
    expect(result[0]).toEqual({ from: index('f10'), promotion: null, to: index('i8') })
    expect(result[1]).toEqual({ from: index('f10'), promotion: null, to: index('h7') })
    expect(result[2]).toEqual({ from: index('f10'), promotion: null, to: index('g7') })
    expect(result[3]).toEqual({ from: index('f10'), promotion: null, to: index('e7') })
    expect(result[4]).toEqual({ from: index('f10'), promotion: null, to: index('d7') })
    expect(result[5]).toEqual({ from: index('f10'), promotion: null, to: index('c8') })
  })
})
