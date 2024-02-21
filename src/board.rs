use crate::bmi::Bmi;
use crate::lookup::bishop_mask::BISHOP_MASK;
use crate::lookup::bishop_moves::BISHOP_MOVES;
use crate::lookup::direction_mask::DIRECTION_MASK;
use crate::lookup::king::KING_MOVES;
use crate::lookup::knight::KNIGHT_MOVES;
use crate::lookup::pin_mask::PIN_MASK;
use crate::lookup::rook_mask::ROOK_MASK;
use crate::lookup::rook_moves::ROOK_MOVES;
use crate::piece::Piece;
use crate::r#move::Square;
use crate::role::Role;
use crate::{bitboard::Bitboard, lookup::zobrist::ZOBRIST_VALUES};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy)]
pub enum File {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
}

#[derive(Debug, Clone, Copy)]
pub enum Rank {
    First = 0,
    Second = 1,
    Third = 2,
    Fourth = 3,
    Fifth = 4,
    Sixth = 5,
    Seventh = 6,
    Eighth = 7,
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
    pub zobrist: u64,
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
            zobrist: 0,
        }
    }

    pub fn from_fen(fen: &str) -> Board {
        let mut zobrist: u64 = 0;
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
                let mut zobrist_offset = 0;
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
                        zobrist_offset = 0;
                    }
                    Role::Bishop => {
                        by_role.bishops |= bitboard;
                        zobrist_offset = 1;
                    }
                    Role::Knight => {
                        by_role.knights |= bitboard;
                        zobrist_offset = 2;
                    }
                    Role::Rook => {
                        by_role.rooks |= bitboard;
                        zobrist_offset = 3;
                    }
                    Role::Queen => {
                        by_role.queens |= bitboard;
                        zobrist_offset = 4;
                    }
                    Role::King => {
                        by_role.kings |= bitboard;
                        zobrist_offset = 5;
                    }
                }

                if c.is_uppercase() {
                    by_color.white |= bitboard;
                    zobrist ^= ZOBRIST_VALUES[i * 64 + zobrist_offset];
                } else {
                    by_color.black |= bitboard;
                    zobrist ^= ZOBRIST_VALUES[i * 64 + zobrist_offset + 6];
                }

                index += 1;
            }
        }

        Board {
            by_color,
            by_role,
            zobrist,
        }
    }

    pub fn piece_at(&self, square: i32) -> Option<Piece> {
        let bitboard = Bitboard(0x01u64.wrapping_shl(square as u32));
        let color = if self.by_color.white & bitboard != Bitboard(0) {
            Some(true)
        } else if self.by_color.black & bitboard != Bitboard(0) {
            Some(false)
        } else {
            None
        };

        let role = if self.by_role.pawns & bitboard != Bitboard(0) {
            Some(Role::Pawn)
        } else if self.by_role.bishops & bitboard != Bitboard(0) {
            Some(Role::Bishop)
        } else if self.by_role.knights & bitboard != Bitboard(0) {
            Some(Role::Knight)
        } else if self.by_role.rooks & bitboard != Bitboard(0) {
            Some(Role::Rook)
        } else if self.by_role.queens & bitboard != Bitboard(0) {
            Some(Role::Queen)
        } else if self.by_role.kings & bitboard != Bitboard(0) {
            Some(Role::King)
        } else {
            None
        };

        if color.is_some() && role.is_some() {
            Some(Piece {
                is_white: color.unwrap(),
                role: role.unwrap(),
            })
        } else {
            None
        }
    }

    #[inline]
    pub fn update_bitboard(&mut self, piece: Piece, from_square: Bitboard, to_square: Bitboard) {
        let move_bitboard = from_square | to_square;

        let mut zobrist_offset = 0;

        match piece.role {
            Role::Pawn => {
                self.by_role.pawns ^= move_bitboard;
                zobrist_offset = 0;
            }
            Role::Bishop => {
                self.by_role.bishops ^= move_bitboard;
                zobrist_offset = 1;
            }
            Role::Knight => {
                self.by_role.knights ^= move_bitboard;
                zobrist_offset = 2;
            }
            Role::Rook => {
                self.by_role.rooks ^= move_bitboard;
                zobrist_offset = 3;
            }
            Role::Queen => {
                self.by_role.queens ^= move_bitboard;
                zobrist_offset = 4;
            }
            Role::King => {
                self.by_role.kings ^= move_bitboard;
                zobrist_offset = 5;
            }
        }

        if piece.is_white {
            self.by_color.white ^= move_bitboard;
        } else {
            self.by_color.black ^= move_bitboard;
            zobrist_offset += 6;
        }

        self.zobrist ^=
            ZOBRIST_VALUES[from_square.0.trailing_zeros() as usize * 12 + zobrist_offset];
        if from_square.0 != to_square.0 && to_square.0.trailing_zeros() < 64 {
            self.zobrist ^=
                ZOBRIST_VALUES[to_square.0.trailing_zeros() as usize * 12 + zobrist_offset];
        }
    }

    #[inline]
    pub fn rook_attacks(square: usize, blockers: Bitboard) -> Bitboard {
        let (mask, offset) = ROOK_MASK[square];
        let index = (blockers.0.pext(mask.0) + offset) as usize;
        ROOK_MOVES[index]
    }

    #[inline]
    pub fn bishop_attacks(square: usize, blockers: Bitboard) -> Bitboard {
        let (mask, offset) = BISHOP_MASK[square];
        let index = (blockers.0.pext(mask.0) + offset) as usize;
        BISHOP_MOVES[index]
    }
    #[inline]
    pub fn pawn_attacks(is_white: bool, square: Bitboard) -> Bitboard {
        let rank = square.0.trailing_zeros() / 8;

        let rank1 = Bitboard::from_rank_number(0);
        if is_white {
            (square >> 7 | square >> 9) & Bitboard(rank1.0.wrapping_shl((8 * (rank + 1)) as u32))
        } else {
            (square << 7 | square << 9) & Bitboard(rank1.0.wrapping_shl((8 * (rank - 1)) as u32))
        }
    }

    pub fn check_mask(self, is_white: bool) -> (Bitboard, Bitboard) {
        let my_bitboard = self.my_bitboard(is_white);

        let all_pieces = self.by_color.white | self.by_color.black;

        let enemy_bitboard = self.enemy_bitboard(is_white);

        let king_square = Square((my_bitboard & self.by_role.kings).0.trailing_zeros() as u8);
        if king_square.0 > 63 {
            return (Bitboard(0), Bitboard(0));
        }
        let mut capture_mask = Bitboard(0);
        let mut move_mask = Bitboard(0);

        let blockers = all_pieces & !(self.by_role.kings & my_bitboard);

        // rook moves
        let rooks = self.by_role.rooks & enemy_bitboard;
        let queen = self.by_role.queens & enemy_bitboard;
        let king_reaches = Board::rook_attacks(king_square.0 as usize, blockers);

        let king_reaches_north = Bitboard(
            Bitboard::from_file_number(king_square.file() as usize)
                .0
                .wrapping_shl((king_square.rank() as u32 + 1) * 8),
        ) & king_reaches;
        let king_reaches_south = Bitboard(
            Bitboard::from_file_number(king_square.file() as usize)
                .0
                .wrapping_shr((7 - king_square.rank() as u32) * 8),
        ) & king_reaches;

        let horizontal_mask = Bitboard::from_rank_number(king_square.rank() as usize);

        let king_reaches_east = Bitboard(
            Bitboard::from_rank_number(king_square.rank() as usize)
                .0
                .wrapping_shl(king_square.file() as u32),
        ) & king_reaches;
        let king_reaches_east = king_reaches_east & horizontal_mask;

        let king_reaches_west = Bitboard(
            Bitboard::from_rank_number(king_square.rank() as usize)
                .0
                .wrapping_shr(8 - king_square.file() as u32),
        ) & king_reaches;
        let king_reaches_west = king_reaches_west & horizontal_mask;
        for direction in [
            king_reaches_north,
            king_reaches_south,
            king_reaches_east,
            king_reaches_west,
        ] {
            let attack = direction & (rooks | queen);
            if attack != Bitboard(0) {
                let attack_square = attack.0.trailing_zeros();
                let attacker_moves =
                    Board::rook_attacks(attack_square as usize, blockers) & direction;
                move_mask |= direction & attacker_moves;
                capture_mask |= Bitboard(1 << attack_square);
            }
        }

        // bishop moves
        let bishops = self.by_role.bishops & enemy_bitboard;
        let king_reaches = Board::bishop_attacks(king_square.0 as usize, blockers);
        let (north_mask, east_mask, south_mask, west_mask) = DIRECTION_MASK[king_square.0 as usize];

        let king_reaches_north_east = king_reaches & (north_mask & east_mask);
        let king_reaches_north_west = king_reaches & (north_mask & west_mask);
        let king_reaches_south_east = king_reaches & (south_mask & east_mask);
        let king_reaches_south_west = king_reaches & (south_mask & west_mask);
        for direction in [
            king_reaches_north_east,
            king_reaches_north_west,
            king_reaches_south_east,
            king_reaches_south_west,
        ] {
            let attack = direction & (bishops | queen);
            if attack != Bitboard(0) {
                let attack_square = attack.0.trailing_zeros();
                let attacker_moves = Board::bishop_attacks(attack_square as usize, blockers);
                move_mask |= direction & attacker_moves;
                capture_mask |= Bitboard(1 << attack_square);
            }
        }

        // pawns
        let pawns = self.by_role.pawns & enemy_bitboard;
        let king_reaches = Board::pawn_attacks(is_white, my_bitboard & self.by_role.kings);

        capture_mask |= king_reaches & pawns;

        // knights
        let knights = self.by_role.knights & enemy_bitboard;
        let king_reaches = KNIGHT_MOVES[king_square.0 as usize];

        capture_mask |= king_reaches & knights;
        if capture_mask == Bitboard(0) {
            return (Bitboard(0xFFFFFFFFFFFFFFFF), Bitboard(0xFFFFFFFFFFFFFFFF));
        }
        (move_mask, capture_mask)
    }

    pub fn pin_mask(self, is_white: bool) -> (Bitboard, Bitboard) {
        let all_pieces = self.by_color.white | self.by_color.black;
        let my_bitboard = self.my_bitboard(is_white);

        let my_bitboard_without_king = my_bitboard & !(self.by_role.kings & my_bitboard);

        let enemy_bitboard = self.enemy_bitboard(is_white);

        let mut pin_mask_vh = Bitboard(0);
        let mut pin_mask_diagonal = Bitboard(0);

        let blockers = enemy_bitboard;

        let king_square = (my_bitboard & self.by_role.kings).0.trailing_zeros();
        if king_square > 63 {
            return (Bitboard(0), Bitboard(0));
        }

        let king_file = king_square as i32 % 8;
        let king_rank = king_square as i32 / 8;

        // rook moves
        let rooks = self.by_role.rooks & enemy_bitboard;
        let queen = self.by_role.queens & enemy_bitboard;
        let king_reaches = Board::rook_attacks(king_square as usize, blockers);

        let king_reaches_north = Bitboard(
            Bitboard::from_file_number(king_file as usize)
                .0
                .wrapping_shl((king_rank as u32 + 1) * 8),
        ) & king_reaches;

        let king_reaches_south = Bitboard(
            Bitboard::from_file_number(king_file as usize)
                .0
                .wrapping_shr((7 - king_rank as u32) * 8),
        ) & king_reaches;

        let horizontal_mask = Bitboard::from_rank_number(king_rank as usize);

        let king_reaches_east = Bitboard(
            Bitboard::from_rank_number(king_rank as usize)
                .0
                .wrapping_shl(king_file as u32),
        ) & king_reaches;
        let king_reaches_east = king_reaches_east & horizontal_mask;

        let king_reaches_west = Bitboard(
            Bitboard::from_rank_number(king_rank as usize)
                .0
                .wrapping_shr(8 - king_file as u32),
        ) & king_reaches;
        let king_reaches_west = king_reaches_west & horizontal_mask;

        for &direction in &[
            king_reaches_north,
            king_reaches_south,
            king_reaches_east,
            king_reaches_west,
        ] {
            let attack = direction & (rooks | queen);

            if attack != Bitboard(0) {
                let attack_square = attack.0.trailing_zeros();
                let attacker_moves = PIN_MASK[(king_square * 64 + attack_square) as usize];

                pin_mask_vh |= attacker_moves
                    * ((attacker_moves & my_bitboard_without_king).0.count_ones() == 1) as u64;
            }
        }

        // bishop moves
        let bishops = self.by_role.bishops & enemy_bitboard;
        let king_reaches = Board::bishop_attacks(king_square as usize, blockers);
        let (north_mask, east_mask, south_mask, west_mask) = DIRECTION_MASK[king_square as usize];

        let king_reaches_north_east = king_reaches & (north_mask & east_mask);
        let king_reaches_north_west = king_reaches & (north_mask & west_mask);
        let king_reaches_south_east = king_reaches & (south_mask & east_mask);
        let king_reaches_south_west = king_reaches & (south_mask & west_mask);

        for &direction in &[
            king_reaches_north_east,
            king_reaches_north_west,
            king_reaches_south_east,
            king_reaches_south_west,
        ] {
            let attack = direction & (bishops | queen);
            if attack != Bitboard(0) {
                let attack_square = attack.0.trailing_zeros();

                let attacker_moves = PIN_MASK[(king_square * 64 + attack_square) as usize];
                pin_mask_diagonal |= attacker_moves
                    * ((attacker_moves & my_bitboard_without_king).0.count_ones() == 1) as u64;
            }
        }

        (pin_mask_vh, pin_mask_diagonal)
    }

    pub const fn my_bitboard(self, is_white: bool) -> Bitboard {
        if is_white {
            self.by_color.white
        } else {
            self.by_color.black
        }
    }

    pub const fn enemy_bitboard(self, is_white: bool) -> Bitboard {
        if is_white {
            self.by_color.black
        } else {
            self.by_color.white
        }
    }

    pub fn seen_by_enemy(self, is_white: bool) -> Bitboard {
        let mut bitboard = Bitboard(0);
        let all_pieces = self.by_color.white | self.by_color.black;

        let my_bitboard = self.my_bitboard(is_white);
        let enemy_bitboard = self.enemy_bitboard(is_white);

        let king = self.by_role.kings & my_bitboard;
        let blockers = all_pieces & !king;

        let mut loop_bibooard = enemy_bitboard;
        while loop_bibooard != Bitboard(0) {
            let i = loop_bibooard.0.trailing_zeros() as usize;
            let current_square = Bitboard(loop_bibooard.0.blsi());

            let is_queen = current_square & self.by_role.queens != Bitboard(0);
            let is_rook = current_square & self.by_role.rooks != Bitboard(0);
            let is_bishop = current_square & self.by_role.bishops != Bitboard(0);
            let is_knight = current_square & self.by_role.knights != Bitboard(0);
            let is_king = current_square & self.by_role.kings != Bitboard(0);
            let is_pawn = current_square & self.by_role.pawns != Bitboard(0);

            if is_rook {
                let seen = Board::rook_attacks(i, blockers);
                bitboard |= seen;
            } else if is_bishop {
                let seen = Board::bishop_attacks(i, blockers);
                bitboard |= seen;
            } else if is_queen {
                let seen = Board::rook_attacks(i, blockers) | Board::bishop_attacks(i, blockers);
                bitboard |= seen;
            } else if is_knight {
                let seen = KNIGHT_MOVES[i];
                bitboard |= seen;
            } else if is_king {
                let seen = KING_MOVES[i];
                bitboard |= seen;
            } else if is_pawn {
                let seen = ((Bitboard(current_square.0.wrapping_shr(9))
                    | Bitboard(current_square.0.wrapping_shr(7)))
                    & Bitboard::from_rank_number(i / 8 - 1))
                    * (is_white as u64)
                    + ((Bitboard(current_square.0.wrapping_shl(9))
                        | Bitboard(current_square.0.wrapping_shl(7)))
                        & Bitboard::from_rank_number(i / 8 + 1))
                        * (!is_white as u64);
                bitboard |= seen;
            }
            // clear lsb
            loop_bibooard = Bitboard(loop_bibooard.0.blsr());
        }
        bitboard
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for rank in 0..8 {
            for file in 0..8 {
                let bitboard = Bitboard(0x01u64.wrapping_shl(file + ((7 - rank) * 8)));

                let mut role: Option<Role> = None;
                let mut color: Option<bool> = None;

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
                    color = Some(true);
                } else if self.by_color.black & bitboard != Bitboard(0) {
                    color = Some(false);
                }

                if role.is_some() && color.is_some() {
                    let piece = Piece {
                        role: role.unwrap(),
                        is_white: color.unwrap(),
                    };
                    write!(f, "{} ", piece.get_unicode())?;
                } else {
                    write!(f, "· ")?;
                }
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
