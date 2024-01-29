use duplicate::duplicate_item;

use std::ops::{Add, BitOr, BitXor, Mul, Not, Shl, Shr, Sub};
use std::ops::{BitAnd, BitAndAssign, BitOrAssign, BitXorAssign};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bitboard(pub u64);

// 56 57 58 59 60 61 62 63
// 48 49 50 51 52 53 54 55
// 40 41 42 43 44 45 46 47
// 32 33 34 35 36 37 38 39
// 24 25 26 27 28 29 30 31
// 16 17 18 19 20 21 22 23
// 08 09 10 11 12 13 14 15
// 00 01 02 03 04 05 06 07
impl Bitboard {
    pub fn from_rank_number(rank: usize) -> Bitboard {
        const RANK_TABLE: [Bitboard; 8] = [
            Bitboard(0x00_00_00_00_00_00_00_FF),
            Bitboard(0x00_00_00_00_00_00_FF_00),
            Bitboard(0x00_00_00_00_00_FF_00_00),
            Bitboard(0x00_00_00_00_FF_00_00_00),
            Bitboard(0x00_00_00_FF_00_00_00_00),
            Bitboard(0x00_00_FF_00_00_00_00_00),
            Bitboard(0x00_FF_00_00_00_00_00_00),
            Bitboard(0xFF_00_00_00_00_00_00_00),
        ];
        RANK_TABLE[rank]
    }

    pub fn from_file_number(file: usize) -> Bitboard {
        const FILE_TABLE: [Bitboard; 8] = [
            Bitboard(0x01_01_01_01_01_01_01_01),
            Bitboard(0x02_02_02_02_02_02_02_02),
            Bitboard(0x04_04_04_04_04_04_04_04),
            Bitboard(0x08_08_08_08_08_08_08_08),
            Bitboard(0x10_10_10_10_10_10_10_10),
            Bitboard(0x20_20_20_20_20_20_20_20),
            Bitboard(0x40_40_40_40_40_40_40_40),
            Bitboard(0x80_80_80_80_80_80_80_80),
        ];

        FILE_TABLE[file]
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Bitboard) {
        self.0 |= rhs.0;
    }
}

impl BitAnd for Bitboard {
    type Output = Bitboard;

    fn bitand(self, rhs: Self) -> Self::Output {
        return Bitboard(self.0 & rhs.0);
    }
}

impl BitOr for Bitboard {
    type Output = Bitboard;

    fn bitor(self, rhs: Self) -> Self::Output {
        return Bitboard(self.0 | rhs.0);
    }
}

impl BitXor for Bitboard {
    type Output = Bitboard;

    fn bitxor(self, rhs: Self) -> Self::Output {
        return Bitboard(self.0 ^ rhs.0);
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0
    }
}

impl BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0
    }
}

impl Not for Bitboard {
    type Output = Bitboard;

    fn not(self) -> Self::Output {
        Bitboard(!self.0)
    }
}

impl Shr for Bitboard {
    type Output = Bitboard;

    fn shr(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 >> rhs.0)
    }
}

impl Shl for Bitboard {
    type Output = Bitboard;

    fn shl(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 << rhs.0)
    }
}

impl Mul<u64> for Bitboard {
    type Output = Bitboard;

    fn mul(self, rhs: u64) -> Self::Output {
        Bitboard(self.0 * rhs)
    }
}

impl Add for Bitboard {
    type Output = Bitboard;

    fn add(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 + rhs.0)
    }
}

impl Sub<u64> for Bitboard {
    type Output = Bitboard;

    fn sub(self, rhs: u64) -> Self::Output {
        Bitboard(self.0 - rhs)
    }
}

#[duplicate_item(int_type; [u8]; [u16]; [u32]; [u64]; [u128]; [usize]; [i8]; [i16]; [i32]; [i64])]
impl Shl<int_type> for Bitboard {
    type Output = Bitboard;

    fn shl(self, rhs: int_type) -> Self::Output {
        Bitboard(self.0 >> rhs)
    }
}
#[duplicate_item(int_type; [u8]; [u16]; [u32]; [u64]; [u128]; [usize]; [i8]; [i16]; [i32]; [i64])]
impl Shr<int_type> for Bitboard {
    type Output = Bitboard;

    fn shr(self, rhs: int_type) -> Self::Output {
        Bitboard(self.0 << rhs)
    }
}

impl std::fmt::Display for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for rank in (0..8).rev() {
            for file in 0..8 {
                let bitboard: Bitboard = Bitboard(0x01 << file + (rank * 8usize));
                if *self & bitboard == Bitboard(0x0) {
                    write!(f, "○ ")?;
                } else {
                    write!(f, "● ")?;
                }
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
