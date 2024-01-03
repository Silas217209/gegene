use crate::board::{File, Rank};
use std::{
    fmt::write,
    ops::{BitAnd, BitAndAssign, BitOrAssign, BitXorAssign}, slice::RChunks,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bitboard(pub u64);

impl Bitboard {
    pub fn from_rank(rank: Rank) -> Bitboard {
        match rank {
            Rank::First => Bitboard(0x00_00_00_00_00_00_00_FF),
            Rank::Second => Bitboard(0x00_00_00_00_00_00_FF_00),
            Rank::Third => Bitboard(0x00_00_00_00_00_FF_00_00),
            Rank::Fourth => Bitboard(0x00_00_00_00_FF_00_00_00),
            Rank::Fifth => Bitboard(0x00_00_00_FF_00_00_00_00),
            Rank::Sixth => Bitboard(0x00_00_FF_00_00_00_00_00),
            Rank::Seventh => Bitboard(0x00_FF_00_00_00_00_00_00),
            Rank::Eighth => Bitboard(0xFF_00_00_00_00_00_00_00),
        }
    }

    pub fn from_file(file: File) -> Bitboard {
        match file {
            File::A => Bitboard(0x01_01_01_01_01_01_01_01),
            File::B => Bitboard(0x02_02_02_02_02_02_02_02),
            File::C => Bitboard(0x04_04_04_04_04_04_04_04),
            File::D => Bitboard(0x08_08_08_08_08_08_08_08),
            File::E => Bitboard(0x10_10_10_10_10_10_10_10),
            File::F => Bitboard(0x20_20_20_20_20_20_20_20),
            File::G => Bitboard(0x40_40_40_40_40_40_40_40),
            File::H => Bitboard(0x80_80_80_80_80_80_80_80),
        }
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

impl std::fmt::Display for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for rank in 0..8 {
            for file in 0..8 {
                let bitboard: Bitboard = Bitboard(0x01 << file + (rank * 8 as usize));
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

