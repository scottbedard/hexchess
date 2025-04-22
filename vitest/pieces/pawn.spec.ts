import { describe, expect, test } from 'vitest'
import { Hexchess, index } from '../../src'

describe('pawn', () => {
  test('black en passant portside', () => {
    const result = Hexchess.parse('1/3/5/7/4P4/4p6/11/11/11/11/11 b f6 0 1').movesFrom('e6')
    expect(result).toEqual([
      { from: index('e6'), promotion: null, to: index('e5') },
      { from: index('e6'), promotion: null, to: index('f6') },
    ])
  })

  test('black en passant portside out of turn', () => {
    const result = Hexchess.parse('1/3/5/7/4P4/4p6/11/11/11/11/11 w f6 0 1').movesFrom('e6')
    expect(result).toEqual([
      { from: index('e6'), promotion: null, to: index('e5') },
      // f6 is out of turn
    ])
  })

  test('black en passant starboard', () => {
    const result = Hexchess.parse('1/3/5/7/4P4/6p4/11/11/11/11/11 b f6 0 1').movesFrom('g6')
    expect(result).toEqual([
      { from: index('g6'), promotion: null, to: index('g5') },
      { from: index('g6'), promotion: null, to: index('f6') },
    ])
  })

  test('black en passant starboard out of turn', () => {
    const result = Hexchess.parse('1/3/5/7/4P4/6p4/11/11/11/11/11 w f6 0 1').movesFrom('g6')
    expect(result).toEqual([
      { from: index('g6'), promotion: null, to: index('g5') },
      // f6 is out of turn
    ])
  })

  test('black promote forward', () => {
    const result = Hexchess.parse('1/3/5/7/9/11/11/11/11/5p5/11 b - 0 1').movesFrom('f2')
    expect(result).toEqual([
      { from: index('f2'), promotion: 'b', to: index('f1') },
      { from: index('f2'), promotion: 'n', to: index('f1') },
      { from: index('f2'), promotion: 'q', to: index('f1') },
      { from: index('f2'), promotion: 'r', to: index('f1') },
    ])
  })

  test('black promote capture portside', () => {
    const result = Hexchess.parse('1/3/5/7/9/11/11/11/11/5p5/4rrK4 w - 0 1').movesFrom('f2')
    expect(result).toEqual([
      { from: index('f2'), promotion: 'b', to: index('g1') },
      { from: index('f2'), promotion: 'n', to: index('g1') },
      { from: index('f2'), promotion: 'q', to: index('g1') },
      { from: index('f2'), promotion: 'r', to: index('g1') },
    ])
  })

  test('black promote capture starboard', () => {
    const result = Hexchess.parse('1/3/5/7/9/11/11/11/11/5p5/4Krr4 w - 0 1').movesFrom('f2')
    expect(result).toEqual([
      { from: index('f2'), promotion: 'b', to: index('e1') },
      { from: index('f2'), promotion: 'n', to: index('e1') },
      { from: index('f2'), promotion: 'q', to: index('e1') },
      { from: index('f2'), promotion: 'r', to: index('e1') },
    ])
  })

  test('white promote forward', () => {
    const result = Hexchess.parse('1/1P1/5/7/9/11/11/11/11/11/11 w - 0 1').movesFrom('f10')
    expect(result).toEqual([
      { from: index('f10'), promotion: 'b', to: index('f11') },
      { from: index('f10'), promotion: 'n', to: index('f11') },
      { from: index('f10'), promotion: 'q', to: index('f11') },
      { from: index('f10'), promotion: 'r', to: index('f11') },
    ])
  })

  test('white promote capture portside', () => {
    const result = Hexchess.parse('R/kPR/5/7/9/11/11/11/11/11/11 w - 0 1').movesFrom('f10')
    expect(result).toEqual([
      { from: index('f10'), promotion: 'b', to: index('e10') },
      { from: index('f10'), promotion: 'n', to: index('e10') },
      { from: index('f10'), promotion: 'q', to: index('e10') },
      { from: index('f10'), promotion: 'r', to: index('e10') },
    ])
  })

  test('white promote capture starboard', () => {
    const result = Hexchess.parse('R/RPk/5/7/9/11/11/11/11/11/11 w - 0 1').movesFrom('f10')
    expect(result).toEqual([
      { from: index('f10'), promotion: 'b', to: index('g10') },
      { from: index('f10'), promotion: 'n', to: index('g10') },
      { from: index('f10'), promotion: 'q', to: index('g10') },
      { from: index('f10'), promotion: 'r', to: index('g10') },
    ])
  })
})
