import { describe, expect, test } from 'vitest'
import {
  Hexchess,
  index,
  positions,
  San
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

  test('findKing', () => {
    const hexchess = Hexchess.init()

    expect(hexchess.findKing('b')).toBe(index('g10'))
    expect(hexchess.findKing('w')).toBe(index('g1'))
  })

  test('get', () => {
    const hexchess = Hexchess.init()

    expect(hexchess.get('g10')).toBe('k')
    expect(hexchess.get('g1')).toBe('K')
    expect(hexchess.get('a4')).toBe(null)
    // @ts-expect-error - invalid position
    expect(hexchess.get('whoops')).toBe(null)
  })

  test('getColor', () => {
    const hexchess = Hexchess.init()
    const results = hexchess.getColor('b')

    expect(results.length).toBe(18)
    expect(results[0]).toBe(index('f11'))
    expect(results[1]).toBe(index('e10'))
    expect(results[2]).toBe(index('f10'))
    expect(results[3]).toBe(index('g10'))
    expect(results[4]).toBe(index('d9'))
    expect(results[5]).toBe(index('f9'))
    expect(results[6]).toBe(index('h9'))
    expect(results[7]).toBe(index('c8'))
    expect(results[8]).toBe(index('i8'))
    expect(results[9]).toBe(index('b7'))
    expect(results[10]).toBe(index('c7'))
    expect(results[11]).toBe(index('d7'))
    expect(results[12]).toBe(index('e7'))
    expect(results[13]).toBe(index('f7'))
    expect(results[14]).toBe(index('g7'))
    expect(results[15]).toBe(index('h7'))
    expect(results[16]).toBe(index('i7'))
    expect(results[17]).toBe(index('k7'))
  })

  describe('isLegal', () => {
    test('legal move', () => {
      const hexchess = Hexchess.init()

      expect(hexchess.isLegal('g4g5')).toBe(true)
    })

    test('illegal move', () => {
      const hexchess = Hexchess.init()

      expect(hexchess.isLegal('b1b4')).toBe(false)
    })

    test('illegal move out of turn', () => {
      const hexchess = Hexchess.init()

      expect(hexchess.isLegal('g7g6')).toBe(false)

      hexchess.turn = 'b'

      expect(hexchess.isLegal('g7g6')).toBe(true)
    })

    test('white cannot promote on black positions', () => {
      const hexchess = Hexchess.parse('1/3/5/7/p7p/R9R/11/11/11/11/rP7Pr w - 0 1')

      expect(hexchess.isLegal(new San({ from: 'b1', to: 'b2' }))).toBe(true)
      expect(hexchess.isLegal(new San({ from: 'b1', to: 'b2', promotion: 'q' }))).toBe(false)

      expect(hexchess.isLegal(new San({ from: 'k1', to: 'l1' }))).toBe(true)
      expect(hexchess.isLegal(new San({ from: 'k1', to: 'l1', promotion: 'q' }))).toBe(false)
    })

    test('black cannot promote on white positions', () => {
      const hexchess = Hexchess.parse('1/3/5/7/p7p/R9R/11/11/11/11/rP7Pr b - 0 1')

      expect(hexchess.isLegal(new San({ from: 'b7', to: 'a6' }))).toBe(true)
      expect(hexchess.isLegal(new San({ from: 'b7', to: 'a6', promotion: 'q' }))).toBe(false)

      expect(hexchess.isLegal(new San({ from: 'k7', to: 'l6' }))).toBe(true)
      expect(hexchess.isLegal(new San({ from: 'k7', to: 'l6', promotion: 'q' }))).toBe(false)
    })

    test('pawn must promote on final rank', () => {
      const hexchess = Hexchess.parse('1/1P1/5/7/9/11/11/11/11/5p5/11 w - 0 1')

      expect(hexchess.isLegal(new San({ from: 'f10', to: 'f11' }))).toBe(false)
      expect(hexchess.isLegal(new San({ from: 'f10', to: 'f11', promotion: 'q' }))).toBe(true)

      hexchess.turn = 'b'

      expect(hexchess.isLegal(new San({ from: 'f2', to: 'f1' }))).toBe(false)
      expect(hexchess.isLegal(new San({ from: 'f2', to: 'f1', promotion: 'q' }))).toBe(true)
    })
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
