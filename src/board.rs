use crate::bitboard::Bitboard;
use crate::piece::Piece;
use crate::role::Role;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

#[derive(Debug)]
pub enum Rank {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eighth,
}

#[derive(Debug)]
pub struct ByRole<T> {
    pub pawns: T,
    pub bishops: T,
    pub knights: T,
    pub rooks: T,
    pub queens: T,
    pub kings: T,
}

#[derive(Debug)]
pub struct ByColor<T> {
    pub white: T,
    pub black: T,
}

#[derive(Debug)]
pub struct Board {
    pub by_color: ByColor<Bitboard>,
    pub by_role: ByRole<Bitboard>,
}

impl Board {
    pub fn new() -> Board {
        // starting position
        // rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
        // note: with a dark background, piece colors seem inverted, but they aren't
        // ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜
        // ♟︎︎ ♟︎︎ ♟︎︎ ♟︎︎ ♟︎︎ ♟︎︎ ♟︎︎ ♟︎︎
        // . . . . . . . .
        // . . . . . . . .
        // . . . . . . . .
        // . . . . . . . .
        // ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙
        // ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖
        Board{
            by_color: ByColor {
                white: Bitboard(0x00_00_00_00_00_00_FF_FF),
                black: Bitboard(0xFF_FF_00_00_00_00_00_00),
            },
            by_role: ByRole {
                pawns: Bitboard(0x00_FF_00_00_00_00_FF_00),
                bishops: Bitboard(0x24_00_00_00_00_00_00_24),
                knights: Bitboard(0x42_00_00_00_00_00_00_42),
                rooks: Bitboard(0x81_00_00_00_00_00_00_81),
                queens: Bitboard(0x08_00_00_00_00_00_00_08),
                kings: Bitboard(0x10_00_00_00_00_00_00_10),
            },
        }
    }

    pub fn from_fen(fen: &str) -> Board {
        let mut by_color = ByColor {
            white: Bitboard(0x00_00_00_00_00_00_00_00),
            black: Bitboard(0x00_00_00_00_00_00_00_00),
        };

        let mut by_role: ByRole<Bitboard> = ByRole {
            pawns: Bitboard(0x00_00_00_00_00_00_00_00),
            bishops: Bitboard(0x00_00_00_00_00_00_00_00),
            knights: Bitboard(0x00_00_00_00_00_00_00_00),
            rooks: Bitboard(0x00_00_00_00_00_00_00_00),
            queens: Bitboard(0x00_00_00_00_00_00_00_00),
            kings: Bitboard(0x00_00_00_00_00_00_00_00),
        };

        // split fen at / and then iterate over each rank
        let ranks = fen.split("/");
        'rank: for (i, rank) in ranks.enumerate() {
            let mut index = 0;
            // iterate over each character in the rank
            for c in rank.chars() {
                if c.is_whitespace() {
                    break 'rank;
                }
                if c.is_digit(10) {
                    index += c.to_digit(10).unwrap();
                    continue;
                }
                let bitboard: Bitboard = Bitboard(0x01u64.wrapping_shl(index + (i as u32 * 8)));
                let role = Role::from_char(c);
                match role {
                    Role::Pawn => {
                        by_role.pawns |= bitboard;
                    }
                    Role::Bishop => {
                        by_role.bishops |= bitboard;
                    }
                    Role::Knight => {
                        by_role.knights |= bitboard;
                    }
                    Role::Rook => {
                        by_role.rooks |= bitboard;
                    }
                    Role::Queen => {
                        by_role.queens |= bitboard;
                    }
                    Role::King => {
                        by_role.kings |= bitboard;
                    }
                }

                if c.is_uppercase() {
                    by_color.white |= bitboard;
                } else {
                    by_color.black |= bitboard;
                }

                index += 1;
            }
        }

        Board { by_color, by_role }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for rank in (0..8) {
            for file in 0..8 {
                let bitboard = Bitboard(0x01u64.wrapping_shl(file + (rank * 8)));

                let mut role: Option<Role> = None;
                let mut color: Option<Color> = None;

                if self.by_role.pawns & bitboard != Bitboard(0) {
                    role = Some(Role::Pawn);
                } else if self.by_role.bishops & bitboard != Bitboard(0) {
                    role = Some(Role::Bishop);
                } else if self.by_role.knights & bitboard != Bitboard(0) {
                    role = Some(Role::Knight);
                } else if self.by_role.rooks & bitboard != Bitboard(0) {
                    role = Some(Role::Rook);
                } else if self.by_role.queens & bitboard != Bitboard(0) {
                    role = Some(Role::Queen);
                } else if self.by_role.kings & bitboard != Bitboard(0) {
                    role = Some(Role::King);
                }

                if self.by_color.white & bitboard != Bitboard(0) {
                    color = Some(Color::White);
                } else if self.by_color.black & bitboard != Bitboard(0) {
                    color = Some(Color::Black);
                }

                if role.is_some() && color.is_some() {
                    let piece = Piece {
                        role: role.unwrap(),
                        color: color.unwrap(),
                    };
                    write!(f, "{} ", piece.get_unicode())?;
                } else {
                    write!(f, ". ")?;
                }
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
