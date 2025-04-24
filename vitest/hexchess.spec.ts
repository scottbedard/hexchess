import { describe, expect, test } from 'vitest'
import {
  Hexchess,
  index,
  positions,
  San
} from '../src'

describe('Hexchess', () => {
  describe('applyMove', () => {
    // {
    //   "description": "",
    //   "error": false,
    //   "from": "",
    //   "sequence": "",
    //   "to": ""
    // }

    test('promote white and black pieces', () => {
      const hexchess = Hexchess.parse('1/3/1P1P1/7/1P5P1/11/11/11/11/2p1p1p1p2/11 w - 0 1')

      hexchess.applyMove('c7c8r')
      expect(hexchess.get('c8')).toBe('R')

      hexchess.applyMove('c2c1r')
      expect(hexchess.get('c1')).toBe('r')

      hexchess.applyMove('e9e10b')
      expect(hexchess.get('e10')).toBe('B')

      hexchess.applyMove('e2e1b')
      expect(hexchess.get('e1')).toBe('b')

      hexchess.applyMove('g9g10q')
      expect(hexchess.get('g10')).toBe('Q')

      hexchess.applyMove('g2g1q')
      expect(hexchess.get('g1')).toBe('q')

      hexchess.applyMove('i7i8n')
      expect(hexchess.get('i8')).toBe('N')

      hexchess.applyMove('i2i1n')
      expect(hexchess.get('i1')).toBe('n')
    })

    test('errors on illegal move', () => {
      const hexchess = Hexchess.init()

      expect(() => hexchess.applyMove('a4a5')).toThrow()
    })
  })

  describe('applyMoveUnsafe', () => {
    test('errors on empty positions', () => {
      const hexchess = Hexchess.init()

      expect(() => hexchess.applyMoveUnsafe('a4a5')).toThrow()
    })

    test('illegal move succeeds', () => {
      const hexchess = Hexchess.init().applyMoveUnsafe('b1b6') // <- illegal pawn move

      expect(hexchess.toString()).toEqual('b/qbk/n1b1n/r5r/ppppppppp/1P9/5P5/4P1P4/3P1B1P3/2P2B2P2/2RNQBKNRP1 b - 0 1')
    })
  })

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

  test('currentMoves', () => {
    const hexchess = Hexchess.init()
    const result = hexchess.currentMoves().map(san => san.toString())

    expect(result.length).toBe(51)
    expect(result[0]).toBe('f5f6')
    expect(result[1]).toBe('e4e5')
    expect(result[2]).toBe('e4e6')
    expect(result[3]).toBe('g4g5')
    expect(result[4]).toBe('g4g6')
    expect(result[5]).toBe('d3d4')
    expect(result[6]).toBe('d3d5')
    expect(result[7]).toBe('f3h2')
    expect(result[8]).toBe('f3d2')
    expect(result[9]).toBe('h3h4')
    expect(result[10]).toBe('h3h5')
    expect(result[11]).toBe('c2c3')
    expect(result[12]).toBe('c2c4')
    expect(result[13]).toBe('f2g3')
    expect(result[14]).toBe('f2h4')
    expect(result[15]).toBe('f2i5')
    expect(result[16]).toBe('f2k6')
    expect(result[17]).toBe('f2e3')
    expect(result[18]).toBe('f2d4')
    expect(result[19]).toBe('f2c5')
    expect(result[20]).toBe('f2b6')
    expect(result[21]).toBe('i2i3')
    expect(result[22]).toBe('i2i4')
    expect(result[23]).toBe('b1b2')
    expect(result[24]).toBe('b1b3')
    expect(result[25]).toBe('c1d2')
    expect(result[26]).toBe('c1e3')
    expect(result[27]).toBe('c1f4')
    expect(result[28]).toBe('d1f4')
    expect(result[29]).toBe('d1g2')
    expect(result[30]).toBe('d1b2')
    expect(result[31]).toBe('d1c3')
    expect(result[32]).toBe('e1e2')
    expect(result[33]).toBe('e1e3')
    expect(result[34]).toBe('e1d2')
    expect(result[35]).toBe('e1c3')
    expect(result[36]).toBe('e1b4')
    expect(result[37]).toBe('e1a5')
    expect(result[38]).toBe('f1g2')
    expect(result[39]).toBe('f1e2')
    expect(result[40]).toBe('g1g2')
    expect(result[41]).toBe('g1h2')
    expect(result[42]).toBe('h1i3')
    expect(result[43]).toBe('h1k2')
    expect(result[44]).toBe('h1e2')
    expect(result[45]).toBe('h1f4')
    expect(result[46]).toBe('i1h2')
    expect(result[47]).toBe('i1g3')
    expect(result[48]).toBe('i1f4')
    expect(result[49]).toBe('k1k2')
    expect(result[50]).toBe('k1k3')
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
