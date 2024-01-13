use crate::bitboard::Bitboard;
use crate::piece::Piece;
use crate::role::Role;
use std::fmt::{Display, Formatter};
use std::thread::current;
use crate::lookup::bishop_mask::BISHOP_MASK;
use crate::lookup::bishop_moves::BISHOP_MOVES;
use crate::lookup::direction_mask::DIRECTION_MASK;
use crate::lookup::king::KING_MOVES;
use crate::lookup::knight::KNIGHT_MOVES;
use crate::lookup::rook_mask::ROOK_MASK;
use crate::lookup::rook_moves::ROOK_MOVES;
use crate::pdep::Pdep;
use crate::pext::Pext;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, Copy)]
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

impl File {
    pub fn from_char(c: char) -> File {
        match c {
            'a' => File::A,
            'b' => File::B,
            'c' => File::C,
            'd' => File::D,
            'e' => File::E,
            'f' => File::F,
            'g' => File::G,
            'h' => File::H,
            _ => panic!("Invalid file character"),
        }
    }

    pub fn from_number(number: i32) -> File {
        match number {
            0 => File::A,
            1 => File::B,
            2 => File::C,
            3 => File::D,
            4 => File::E,
            5 => File::F,
            6 => File::G,
            7 => File::H,
            _ => panic!("Invalid file number"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
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

impl Rank {
    pub fn from_number(number: i32) -> Rank {
        match number {
            0 => Rank::First,
            1 => Rank::Second,
            2 => Rank::Third,
            3 => Rank::Fourth,
            4 => Rank::Fifth,
            5 => Rank::Sixth,
            6 => Rank::Seventh,
            7 => Rank::Eighth,
            _ => panic!("Invalid rank number"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ByRole<T> {
    pub pawns: T,
    pub bishops: T,
    pub knights: T,
    pub rooks: T,
    pub queens: T,
    pub kings: T,
}

#[derive(Debug, Copy, Clone)]
pub struct ByColor<T> {
    pub white: T,
    pub black: T,
}

#[derive(Debug, Copy, Clone)]
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
        Board {
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
        let ranks: Vec<&str> = fen.split("/").collect();
        let ranks = ranks.iter().rev().enumerate();
        'rank: for (i, rank) in ranks {
            let mut index = 0;
            let rank = rank.split(" ").collect::<Vec<&str>>()[0];
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

    pub fn enemy_rooks(&self, color: Color) -> Bitboard {
        match color {
            Color::White => self.by_color.black & self.by_role.rooks,
            Color::Black => self.by_color.white & self.by_role.rooks,
        }
    }

    pub fn rook_attacks(&self, square: usize, blockers: Bitboard) -> Bitboard {
        let (mask, offset) = ROOK_MASK[square];
        let index = (blockers.0.pext(mask.0) + offset) as usize;
        ROOK_MOVES[index]
    }

    pub fn bishop_attacks(&self, square: usize, blockers: Bitboard) -> Bitboard {
        let (mask, offset) = BISHOP_MASK[square];
        let index = (blockers.0.pext(mask.0) + offset) as usize;
        BISHOP_MOVES[index]
    }

    pub fn check_mask(self, turn: Color) -> (Bitboard, i32) {
        let my_bitboard = match turn {
            Color::White => self.by_color.white,
            Color::Black => self.by_color.black,
        };

        let mut count = 0;

        let enemy_bitboard = !my_bitboard & (self.by_color.white | self.by_color.black);

        let mut check_mask = Bitboard(0);

        let blockers = (self.by_color.white | self.by_color.black) & !(self.by_role.kings & my_bitboard);
        let king_square = (my_bitboard & self.by_role.kings).0.trailing_zeros();
        let king_file = File::from_number(king_square as i32 % 8);
        let king_rank = Rank::from_number(king_square as i32 / 8);

        // rook moves
        let rooks = self.by_role.rooks & enemy_bitboard;
        let queen = self.by_role.queens & enemy_bitboard;
        let king_reaches = self.rook_attacks(king_square as usize, blockers);

        let king_reaches_north = Bitboard(Bitboard::from_file(king_file).0.wrapping_shl((king_rank as u32 + 1) * 8)) & king_reaches;
        let king_reaches_south = Bitboard(Bitboard::from_file(king_file).0.wrapping_shr((7 - king_rank as u32) * 8)) & king_reaches;

        let king_rank_above = 0xFFu64.wrapping_shl((king_rank as u32 + 1) * 8);
        let king_rank_below = 0xFFu64.wrapping_shl((king_rank as u32 - 1) * 8);
        let horizontal_mask = Bitboard(!(king_rank_above | king_rank_below));

        let king_reaches_east = Bitboard(Bitboard::from_rank(king_rank).0.wrapping_shl(king_file as u32) + 1) & king_reaches;
        let king_reaches_east = king_reaches_east & horizontal_mask;

        let king_reaches_west = Bitboard(Bitboard::from_rank(king_rank).0.wrapping_shr(8 - king_file as u32)) & king_reaches;
        let king_reaches_west = king_reaches_west & horizontal_mask;

        for &direction in &[king_reaches_north, king_reaches_south, king_reaches_east, king_reaches_west] {
            let attack = direction & (rooks | queen);
            if attack != Bitboard(0) {
                count += 1;
                let attack_square = attack.0.trailing_zeros();
                let attacker_moves = self.rook_attacks(attack_square as usize, blockers);
                check_mask |= direction & (attacker_moves | Bitboard(1 << attack_square));
            }
        }

        // bishop moves
        let bishops = self.by_role.bishops & enemy_bitboard;
        let king_reaches = self.bishop_attacks(king_square as usize, blockers);
        let (north_mask, east_mask, south_mask, west_mask) = DIRECTION_MASK[king_square as usize];

        let king_reaches_north_east = king_reaches & (north_mask & east_mask);
        let king_reaches_north_west = king_reaches & (north_mask & west_mask);
        let king_reaches_south_east = king_reaches & (south_mask & east_mask);
        let king_reaches_south_west = king_reaches & (south_mask & west_mask);

        for &direction in &[king_reaches_north_east, king_reaches_north_west, king_reaches_south_east, king_reaches_south_west] {
            let attack = direction & (bishops | queen);
            if attack != Bitboard(0) {
                count += 1;
                let attack_square = attack.0.trailing_zeros();
                let attacker_moves = self.bishop_attacks(attack_square as usize, blockers);
                check_mask |= direction & (attacker_moves | Bitboard(1 << attack_square));
            }
        }

        // pawns
        let pawns = self.by_role.pawns & enemy_bitboard;
        let king_reaches = my_bitboard & self.by_role.kings;
        let king_reaches = match turn {
            Color::White => Bitboard(king_reaches.0.wrapping_shl(9)) | Bitboard(king_reaches.0.wrapping_shl(7)),
            Color::Black => Bitboard(king_reaches.0.wrapping_shr(9)) | Bitboard(king_reaches.0.wrapping_shr(7)),
        };

        let pawn_count = pawns.0.pext(king_reaches.0);

        match pawn_count {
            0b1 => {
                count += 1;
            }
            0b11 => {
                count += 2;
            }
            _ => {}
        }

        check_mask |= king_reaches & pawns;

        // knights
        let knights = self.by_role.knights & enemy_bitboard;
        let king_reaches = KNIGHT_MOVES[king_square as usize];

        let knight_count = knights.0.pext(king_reaches.0);

        match knight_count {
            0b1 => {
                count += 1;
            }
            0b11 => {
                count += 2;
            }
            0b111 => {
                count += 3;
            }
            0b1111 => {
                count += 4;
            }
            0b11111 => {
                count += 5;
            }
            0b111111 => {
                count += 6;
            }
            0b1111111 => {
                count += 7;
            }
            0b11111111 => {
                count += 8;
            }
            _ => {}
        }

        check_mask |= king_reaches & knights;

        (check_mask, count)
    }

    pub fn pin_mask(self, turn: Color) -> (Bitboard, Bitboard) {
        let my_bitboard = match turn {
            Color::White => self.by_color.white,
            Color::Black => self.by_color.black,
        };
//        let my_bitboard = Bitboard((Color::White == turn) as u64 * self.by_color.black.0 + (Color::Black == turn) as u64 * self.by_color.white.0);

        let my_bitboard_without_king = my_bitboard & !(self.by_role.kings & my_bitboard);


        let enemy_bitboard = !my_bitboard & (self.by_color.white | self.by_color.black);

        let mut pin_mask_vh = Bitboard(0);
        let mut pin_mask_diagonal = Bitboard(0);

        let blockers = enemy_bitboard & !(self.by_role.kings & my_bitboard);
        let king_square = (my_bitboard & self.by_role.kings).0.trailing_zeros();
        let king_file = File::from_number(king_square as i32 % 8);
        let king_rank = Rank::from_number(king_square as i32 / 8);

        // rook moves
        let rooks = self.by_role.rooks & enemy_bitboard;
        let queen = self.by_role.queens & enemy_bitboard;
        let king_reaches = self.rook_attacks(king_square as usize, blockers);

        let king_reaches_north = Bitboard(Bitboard::from_file(king_file).0.wrapping_shl((king_rank as u32 + 1) * 8)) & king_reaches;
        let king_reaches_south = Bitboard(Bitboard::from_file(king_file).0.wrapping_shr((7 - king_rank as u32) * 8)) & king_reaches;

        let king_rank_above = 0xFFu64.wrapping_shl((king_rank as u32 + 1) * 8);
        let king_rank_below = 0xFFu64.wrapping_shl((king_rank as u32 - 1) * 8);
        let horizontal_mask = Bitboard(!(king_rank_above | king_rank_below));

        let king_reaches_east = Bitboard(Bitboard::from_rank(king_rank).0.wrapping_shl(king_file as u32) + 1) & king_reaches;
        let king_reaches_east = king_reaches_east & horizontal_mask;

        let king_reaches_west = Bitboard(Bitboard::from_rank(king_rank).0.wrapping_shr(8 - king_file as u32)) & king_reaches;
        let king_reaches_west = king_reaches_west & horizontal_mask;

        for &direction in &[king_reaches_north, king_reaches_south, king_reaches_east, king_reaches_west] {
            let attack = direction & (rooks | queen);

            if attack != Bitboard(0) {
                let attack_square = attack.0.trailing_zeros();
                let attacker_moves = self.rook_attacks(attack_square as usize, blockers);

                if attacker_moves.0.pext(my_bitboard_without_king.0) != 1 {
                    continue;
                }

                pin_mask_vh |= direction & (attacker_moves | Bitboard(1 << attack_square));
            }
        }

        // bishop moves
        let bishops = self.by_role.bishops & enemy_bitboard;
        let king_reaches = self.bishop_attacks(king_square as usize, blockers);
        let (north_mask, east_mask, south_mask, west_mask) = DIRECTION_MASK[king_square as usize];

        let king_reaches_north_east = king_reaches & (north_mask & east_mask);
        let king_reaches_north_west = king_reaches & (north_mask & west_mask);
        let king_reaches_south_east = king_reaches & (south_mask & east_mask);
        let king_reaches_south_west = king_reaches & (south_mask & west_mask);

        for &direction in &[king_reaches_north_east, king_reaches_north_west, king_reaches_south_east, king_reaches_south_west] {
            let attack = direction & (bishops | queen);
            if attack != Bitboard(0) {
                let attack_square = attack.0.trailing_zeros();
                let attacker_moves = self.bishop_attacks(attack_square as usize, blockers);

                if attacker_moves.0.pext(my_bitboard_without_king.0) != 1 {
                    continue;
                }

                pin_mask_diagonal |= direction & (attacker_moves | Bitboard(1 << attack_square));
            }
        }

        (pin_mask_vh, pin_mask_diagonal)
    }

    pub fn seen_by_enemy(self, turn: Color) -> Bitboard {
        let mut bitboard = Bitboard(0);

        let my_bitboard = match turn {
            Color::White => self.by_color.white,
            Color::Black => self.by_color.black,
        };

        let enemy_bitboard = !my_bitboard & (self.by_color.white | self.by_color.black);

        let blockers = self.by_color.white | self.by_color.black;

        for i in 0..64 {
            let current_square = Bitboard(1 << i);

            if current_square & enemy_bitboard == Bitboard(0) {
                continue;
            }

            let is_queen = current_square & self.by_role.queens != Bitboard(0);
            let is_rook = current_square & self.by_role.rooks != Bitboard(0);
            let is_bishop = current_square & self.by_role.bishops != Bitboard(0);
            let is_knight = current_square & self.by_role.knights != Bitboard(0);
            let is_king = current_square & self.by_role.kings != Bitboard(0);
            let is_pawn = current_square & self.by_role.pawns != Bitboard(0);

            if is_rook {
                let seen = self.rook_attacks(i as usize, blockers);
                bitboard |= seen;
            } else if is_bishop {
                let seen = self.bishop_attacks(i as usize, blockers);
                bitboard |= seen;
            } else if is_queen {
                let seen = self.rook_attacks(i as usize, blockers) | self.bishop_attacks(i as usize, blockers);
                bitboard |= seen;
            } else if is_knight {
                let seen = KNIGHT_MOVES[i as usize];
                bitboard |= seen;
            } else if is_king {
                let seen = KING_MOVES[i as usize];
                bitboard |= seen;
            } else if is_pawn {
                let seen = match turn {
                    Color::White => (Bitboard(current_square.0.wrapping_shr(9)) | Bitboard(current_square.0.wrapping_shr(7))) & Bitboard::from_rank(Rank::from_number(i / 8 - 1)),
                    Color::Black => (Bitboard(current_square.0.wrapping_shl(9)) | Bitboard(current_square.0.wrapping_shl(7))) & Bitboard::from_rank(Rank::from_number(i / 8 + 1)),
                };
                bitboard |= seen;
            }
        }

        bitboard & !enemy_bitboard
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for rank in 0..8 {
            for file in 0..8 {
                let bitboard = Bitboard(0x01u64.wrapping_shl(file + ((7 - rank) * 8)));

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
