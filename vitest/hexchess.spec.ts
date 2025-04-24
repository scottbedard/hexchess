import { describe, expect, test } from 'vitest'
import {
  Hexchess,
  positions
} from '../src'

describe('Hexchess', () => {
  test('clone', () => {
    const hexchess = Hexchess.init()
    const clone = hexchess.clone()

    expect(clone.board).toEqual(hexchess.board)
    expect(clone.ep).toEqual(hexchess.ep)
    expect(clone.turn).toEqual(hexchess.turn)
    expect(clone.halfmove).toEqual(hexchess.halfmove)
    expect(clone.fullmove).toEqual(hexchess.fullmove)

    expect(clone.board).not.toBe(hexchess.board)
    expect(clone).not.toBe(hexchess)
  })

  describe('isCheck', () => {
    test('no king', () => {
      const hexchess = new Hexchess()

      expect(hexchess.isCheck()).toBe(false)
    })

    test('not in check', () => {
      const hexchess = Hexchess.init()

      expect(hexchess.isCheck()).toBe(false)
    })

    test('in check', () => {
      const hexchess = Hexchess.parse('K/3/5/7/9/5r5/11/11/11/11/11 w - 0 1')

      expect(hexchess.isCheck()).toBe(true)
    })
  })

  describe('isCheckmate', () => {
    test('checkmate', () => {
      const hexchess = Hexchess.parse('K/3/5/3q3/2q6/11/11/11/11/11/11 b - 0 1')

      expect(hexchess.isCheckmate()).toBe(false)

      hexchess.applyMove('d7f9')

      expect(hexchess.isCheckmate()).toBe(true)
    })
  })

  describe('isStalemate', () => {
    test('stalemate', () => {
      const hexchess = Hexchess.parse('k/1P1/5/3K3/9/11/11/11/11/11/11 w - 0 1')

      expect(hexchess.isStalemate()).toBe(false)

      hexchess.applyMove('f8f9')

      expect(hexchess.isStalemate()).toBe(true)
    })
  })

  describe('isThreatened', () => {
    test('unattacked position is not threatened', () => {
      const hexchess = Hexchess.parse('1/2K/5/7/9/11/11/11/11/11/11 w - 0 1')

      expect(hexchess.isThreatened('g10')).toBe(false)
    })

    test('threatened by enemy piece', () => {
      const hexchess = Hexchess.parse('1/2K/5/7/9/11/11/11/11/11/6r4 w - 0 1')

      expect(hexchess.isThreatened('g10')).toBe(true)
    })

    test('not threatened by friendly piece', () => {
      const hexchess = Hexchess.parse('1/2K/5/7/9/11/11/11/11/11/6R4 w - 0 1')

      expect(hexchess.isThreatened('g10')).toBe(false)
    })

    test('position is threatened in and out of turn', () => {
      const hexchess = Hexchess.parse('1/3/5/7/4q4/5K5/11/11/11/11/11 w - 0 1')

      hexchess.turn = 'b'
      expect(hexchess.isThreatened('f6')).toBe(true)

      hexchess.turn = 'w'
      expect(hexchess.isThreatened('f6')).toBe(true)
    })

    test('unoccupied position is not threatened', () => {
      const hexchess = new Hexchess()

      for (const position of positions) {
        expect(hexchess.isThreatened(position)).toBe(false)
      }
    })
  })

  describe('movesFrom', () => {
    test('returns empty array for empty position', () => {
      const hexchess = Hexchess.init()

      expect(hexchess.movesFrom('a4').length).toBe(0)
      expect(hexchess.movesFromUnsafe('a4').length).toBe(0)
    })

    test('cannot step out of a pin', () => {
      const hexchess = Hexchess.parse('1/3/5/7/4K4/5R5/5q5/11/11/11/11 w - 0 1')
      const moves = hexchess.movesFrom('f6')
      expect(moves.length).toBe(1)
      expect(moves[0].toString()).toBe('f6f5')
    })

    test('cannot self check on opponent\'s turn', () => {
      const hexchess = Hexchess.parse('1/3/5/7/4K4/5R5/5q5/11/11/11/11 b - 0 1')
      const moves = hexchess.movesFrom('f6')

      expect(moves.length).toBe(1)
      expect(moves[0].toString()).toBe('f6f5')
    })

    test('king cannot step into check', () => {
      const hexchess = Hexchess.parse('K/3/2q2/7/9/11/11/11/11/11/11 w - 0 1')
      const moves = hexchess.movesFrom('f11')

      expect(moves.length).toBe(0)
    })
  })

  describe('toString', () => {
    test('empty', () => {
      const hexchess = new Hexchess()

      expect(hexchess.toString()).toBe('1/3/5/7/9/11/11/11/11/11/11 w - 0 1')
    })

    test('initial position', () => {
      const hexchess = Hexchess.init()

      expect(hexchess.toString()).toBe('b/qbk/n1b1n/r5r/ppppppppp/11/5P5/4P1P4/3P1B1P3/2P2B2P2/1PRNQBKNRP1 w - 0 1')
    })

    test('with en passant', () => {
      const hexchess = Hexchess.init().applyMove('g4g6')

      expect(hexchess.toString()).toBe('b/qbk/n1b1n/r5r/ppppppppp/6P4/5P5/4P6/3P1B1P3/2P2B2P2/1PRNQBKNRP1 b g5 0 1')
    })
  })
})
