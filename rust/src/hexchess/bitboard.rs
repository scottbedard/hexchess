/*
    null,   null,   null,   null,   null,   'f11',  'g10',  'h9',   'i8',   'k7',   'l6',
    null,   null,   null,   null,   'e10',  'f10',  'g9',   'h8',   'i7',   'k6',   'l5',
    null,   null,   null,   'd9',   'e9',   'f9',   'g8',   'h7',   'i6',   'k5',   'l4',
    null,   null,   'c8',   'd8',   'e8',   'f8',   'g7',   'h6',   'i5',   'k4',   'l3',
    null,   'b7',   'c7',   'd7',   'e7',   'f7',   'g6',   'h5',   'i4',   'k3',   'l2',
    'a6',   'b6',   'c6',   'd6',   'e6',   'f6',   'g5',   'h4',   'i3',   'k2',   'l1',
    'a5',   'b5',   'c5',   'd5',   'e5',   'f5',   'g4',   'h3',   'i2',   'k1',   null,
    'a4',   'b4',   'c4',   'd4',   'e4',   'f4',   'g3',   'h2',   'i1',   null,   null,
    'a3',   'b3',   'c3',   'd3',   'e3',   'f3',   'g2',   'h1',   null,   null,   null,
    'a2',   'b2',   'c2',   'd2',   'e2',   'f2',   'g1',   null,   null,   null,   null,
    'a1',   'b1',   'c1',   'd1',   'e1',   'f1',   null,   null,   null,   null,   null,
*/

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Bitboard(pub u128);

impl Bitboard {
    /// Create a bitboard with all bits set.
    pub fn all() -> Self {
        Bitboard(u128::MAX)
    }

    /// Clear bit at the given index.
    /// Panics if `index` is out of bounds (0-127).
    pub fn clear_bit(&mut self, index: u8) {
        assert!(index < 128, "Bitboard index out of bounds");
        self.0 &= !(1u128 << index);
    }

    /// Returns the number of set bits.
    pub fn count_bits(&self) -> u32 {
        self.0.count_ones()
    }

    /// Checks if a specific bit at `index` is set (1).
    /// Panics if `index` is out of bounds (0-127).
    pub fn is_bit_set(&self, index: u8) -> bool {
        assert!(index < 128, "Index out of bounds for u128");
        (self.0 >> index) & 1 == 1
    }

    /// Iterates over the indices of the set bits.
    pub fn iter_set_bits(&self) -> SetBitsIterator {
        SetBitsIterator {
            bitboard: *self,
            current_index: 0,
        }
    }

    /// Returns the index of the least significant bit (LSB).
    /// Returns `None` if the bitboard is empty.
    pub fn lsb_index(&self) -> Option<u8> {
        if self.0 == 0 {
            None
        } else {
            Some(self.0.trailing_zeros() as u8)
        }
    }

    /// Returns the index of the most significant bit (MSB).
    /// Returns `None` if the bitboard is empty.
    pub fn msb_index(&self) -> Option<u8> {
        if self.0 == 0 {
            None
        } else {
            Some((127 - self.0.leading_zeros()) as u8)
        }
    }

    /// Create an empty bitboard.
    pub fn new() -> Self {
        Bitboard(0)
    }

    /// Set bit at the given index.
    /// Panics if `index` is out of bounds (0-127).
    pub fn set_bit(&mut self, index: u8) {
        assert!(index < 128, "Bitboard index out of bounds");
        self.0 |= 1u128 << index;
    }

    /// Toggles a specific bit at `index`.
    /// Panics if `index` is out of bounds (0-127).
    pub fn toggle_bit(&mut self, index: u8) {
        assert!(index < 128, "Index out of bounds for u128");
        self.0 ^= 1u128 << index;
    }
}

// Custom iterator for set bits
pub struct SetBitsIterator {
    bitboard: Bitboard,
    current_index: u8,
}

impl Iterator for SetBitsIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        while self.current_index < 128 {
            if self.bitboard.is_bit_set(self.current_index) {
                let index = self.current_index;
                self.current_index += 1;
                return Some(index);
            }
            self.current_index += 1;
        }
        None
    }
}

// Implement common bitwise operations
impl std::ops::BitAnd for Bitboard {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Bitboard(self.0 & other.0)
    }
}

impl std::ops::BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, other: Self) {
        self.0 &= other.0;
    }
}

impl std::ops::BitOr for Bitboard {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Bitboard(self.0 | other.0)
    }
}

impl std::ops::BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, other: Self) {
        self.0 |= other.0;
    }
}

impl std::ops::BitXor for Bitboard {
    type Output = Self;
    fn bitxor(self, other: Self) -> Self {
        Bitboard(self.0 ^ other.0)
    }
}

impl std::ops::BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, other: Self) {
        self.0 ^= other.0;
    }
}

impl std::ops::Not for Bitboard {
    type Output = Self;
    fn not(self) -> Self::Output {
        Bitboard(!self.0)
    }
}

impl std::ops::Shl<u8> for Bitboard {
    type Output = Self;
    fn shl(self, rhs: u8) -> Self::Output {
        Bitboard(self.0 << rhs)
    }
}

impl std::ops::ShlAssign<u8> for Bitboard {
    fn shl_assign(&mut self, rhs: u8) {
        self.0 <<= rhs;
    }
}

impl std::ops::Shr<u8> for Bitboard {
    type Output = Self;
    fn shr(self, rhs: u8) -> Self::Output {
        Bitboard(self.0 >> rhs)
    }
}

impl std::ops::ShrAssign<u8> for Bitboard {
    fn shr_assign(&mut self, rhs: u8) {
        self.0 >>= rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let bb = Bitboard::new();
        assert_eq!(bb.0, 0);
    }
    
    #[test]
    fn test_all() {
        let bb = Bitboard::all();
        assert_eq!(bb.0, u128::MAX);
    }

    #[test]
    fn test_set_bit_and_is_bit_set() {
        let mut bb = Bitboard::new();
        bb.set_bit(0);
        assert_eq!(bb.0, 1);
        bb.set_bit(63);
        assert_eq!(bb.0, 1 | (1u128 << 63));
        bb.set_bit(127);
        assert_eq!(bb.0, 1 | (1u128 << 63) | (1u128 << 127));
    }

    #[test]
    fn test_clear_bit() {
        let mut bb = Bitboard::all();
        bb.clear_bit(0);
        assert_eq!(bb.0, u128::MAX ^ 1);
        bb.clear_bit(63);
        assert_eq!(bb.0, u128::MAX ^ 1 ^ (1u128 << 63));
    }

    #[test]
    fn test_toggle_bit() {
        let mut bb = Bitboard::new();
        bb.toggle_bit(5);
        assert_eq!(bb.0, 1u128 << 5);
        bb.toggle_bit(5);
        assert_eq!(bb.0, 0);
    }

    #[test]
    fn test_is_bit_set() {
        let mut bb = Bitboard::new();
        bb.set_bit(10);
        assert!(bb.is_bit_set(10));
        assert!(!bb.is_bit_set(9));
    }

    #[test]
    fn test_count_bits() {
        let mut bb = Bitboard::new();
        assert_eq!(bb.count_bits(), 0);
        bb.set_bit(0);
        bb.set_bit(1);
        bb.set_bit(127);
        assert_eq!(bb.count_bits(), 3);
    }

    #[test]
    fn test_lsb_index() {
        let mut bb = Bitboard::new();
        assert_eq!(bb.lsb_index(), None);
        bb.set_bit(5);
        assert_eq!(bb.lsb_index(), Some(5));
        bb.set_bit(0);
        assert_eq!(bb.lsb_index(), Some(0));
    }

    #[test]
    fn test_msb_index() {
        let mut bb = Bitboard::new();
        assert_eq!(bb.msb_index(), None);
        bb.set_bit(5);
        assert_eq!(bb.msb_index(), Some(5));
        bb.set_bit(127);
        assert_eq!(bb.msb_index(), Some(127));
    }

    #[test]
    fn test_bit_operations() {
        let bb1 = Bitboard(0b1010);
        let bb2 = Bitboard(0b0110);

        assert_eq!((bb1 & bb2).0, 0b0010);
        assert_eq!((bb1 | bb2).0, 0b1110);
        assert_eq!((bb1 ^ bb2).0, 0b1100);
        assert_eq!((!bb1).0, !0b1010);

        let mut bb_assign = Bitboard(0b1111);
        bb_assign &= bb2;
        assert_eq!(bb_assign.0, 0b0110);

        let mut bb_assign = Bitboard(0b1001);
        bb_assign |= bb2;
        assert_eq!(bb_assign.0, 0b1111);

        let mut bb_assign = Bitboard(0b1111);
        bb_assign ^= bb2;
        assert_eq!(bb_assign.0, 0b1001);

        assert_eq!((bb1 << 2).0, 0b101000);
        assert_eq!((bb1 >> 1).0, 0b0101);

        let mut bb_shift = Bitboard(0b1010);
        bb_shift <<= 2;
        assert_eq!(bb_shift.0, 0b101000);

        let mut bb_shift = Bitboard(0b1010);
        bb_shift >>= 1;
        assert_eq!(bb_shift.0, 0b0101);
    }

    #[test]
    fn test_iter_set_bits() {
        let mut bb = Bitboard::new();
        bb.set_bit(3);
        bb.set_bit(7);
        bb.set_bit(100);

        let mut set_bits = Vec::new();
        for index in bb.iter_set_bits() {
            set_bits.push(index);
        }
        assert_eq!(set_bits, vec![3, 7, 100]);

        let empty_bb = Bitboard::new();
        assert!(empty_bb.iter_set_bits().next().is_none());
    }
}
