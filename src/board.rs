use crate::lookup::bishop_mask::BISHOP_MASK;
use crate::lookup::bishop_moves::BISHOP_MOVES;
use crate::lookup::direction_mask::DIRECTION_MASK;
use crate::lookup::king::KING_MOVES;
use crate::lookup::knight::KNIGHT_MOVES;
use crate::lookup::rook_mask::ROOK_MASK;
use crate::lookup::rook_moves::ROOK_MOVES;
use crate::pext::Pext;
use crate::piece::Piece;
use crate::r#move::{MoveType, Square};
use crate::role::Role;
use crate::{bitboard::Bitboard, r#move::Move};
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Color {
    White,
    Black,
}

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
    First = 0,
    Second = 1,
    Third = 2,
    Fourth = 3,
    Fifth = 4,
    Sixth = 5,
    Seventh = 6,
    Eighth = 7,
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

    pub fn piece_at(&self, square: i32) -> Option<Piece> {
        let bitboard = Bitboard(0x01u64.wrapping_shl(square as u32));
        let color = if self.by_color.white & bitboard != Bitboard(0) {
            Some(Color::White)
        } else if self.by_color.black & bitboard != Bitboard(0) {
            Some(Color::Black)
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
                color: color.unwrap(),
                role: role.unwrap(),
            })
        } else {
            None
        }
    }

    pub fn update_bitboard(&mut self, piece: Piece, from_square: Bitboard, to_square: Bitboard) {
        let move_bitboard = from_square | to_square;

        match piece.role {
            Role::Pawn => {
                self.by_role.pawns ^= move_bitboard;
            }
            Role::Bishop => {
                self.by_role.bishops ^= move_bitboard;
            }
            Role::Knight => {
                self.by_role.knights ^= move_bitboard;
            }
            Role::Rook => {
                self.by_role.rooks ^= move_bitboard;
            }
            Role::Queen => {
                self.by_role.queens ^= move_bitboard;
            }
            Role::King => {
                self.by_role.kings ^= move_bitboard;
            }
        }

        match piece.color {
            Color::White => {
                self.by_color.white ^= move_bitboard;
            }
            Color::Black => {
                self.by_color.black ^= move_bitboard;
            }
        }
    }

    #[inline]
    pub fn rook_attacks(&self, square: usize, blockers: Bitboard) -> Bitboard {
        let (mask, offset) = ROOK_MASK[square];
        let index = (blockers.0.pext(mask.0) + offset) as usize;
        ROOK_MOVES[index]
    }

    #[inline]
    pub fn bishop_attacks(&self, square: usize, blockers: Bitboard) -> Bitboard {
        let (mask, offset) = BISHOP_MASK[square];
        let index = (blockers.0.pext(mask.0) + offset) as usize;
        BISHOP_MOVES[index]
    }
    #[inline]
    pub fn pawn_attacks(self, color: Color, square: Square) -> Bitboard {
        let square_bitboard = Bitboard(1u64 << square.0);

        let rank1 = Bitboard::from_rank_number(0);
        match color {
            Color::White => {
                (square_bitboard >> 7 | square_bitboard >> 9)
                    & Bitboard(rank1.0.wrapping_shl((8 * (square.rank() + 1)) as u32))
            }
            Color::Black => {
                (square_bitboard << 7 | square_bitboard << 9)
                    & Bitboard(rank1.0.wrapping_shl((8 * (square.rank() - 1)) as u32))
            }
        }
    }

    #[inline]
    pub fn get_en_passant_moves(self, en_passant_target: Square, turn: Color) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        let en_passant_attackers = self.pawn_attacks(
            match turn {
                Color::White => Color::Black,
                Color::Black => Color::White,
            },
            en_passant_target,
        );

        let my_bitboard = match turn {
            Color::White => self.by_color.white,
            Color::Black => self.by_color.black,
        };

        let en_passant_attackers = en_passant_attackers & my_bitboard & self.by_role.pawns;

        if en_passant_attackers.0.count_ones() == 0 {
            return moves;
        }
        let en_passant_square = match turn {
            Color::White => Square(en_passant_target.0 - 8),
            Color::Black => Square(en_passant_target.0 + 8),
        };

        let attacker = en_passant_attackers.0.trailing_zeros();
        let move1 = Move {
            from: Square(attacker as u8),
            to: en_passant_target,
            piece: Piece {
                role: Role::Pawn,
                color: turn,
            },
            capture: Option::from(Piece {
                color: match turn {
                    Color::White => Color::Black,
                    Color::Black => Color::White,
                },
                role: Role::Pawn,
            }),
            move_type: MoveType::EnPassant(en_passant_square),
        };

        moves.push(move1);

        if en_passant_attackers.0.count_ones() > 1 {
            return moves;
        }

        let attacker = 64 - en_passant_attackers.0.leading_zeros();
        let move1 = Move {
            from: Square(attacker as u8),
            to: en_passant_target,
            piece: Piece {
                role: Role::Pawn,
                color: turn,
            },
            capture: Option::from(Piece {
                color: match turn {
                    Color::White => Color::Black,
                    Color::Black => Color::White,
                },
                role: Role::Pawn,
            }),
            move_type: MoveType::EnPassant(en_passant_square),
        };

        moves.push(move1);

        moves
    }

    pub fn check_mask(self, turn: Color) -> (Bitboard, Bitboard) {
        let my_bitboard = match turn {
            Color::White => self.by_color.white,
            Color::Black => self.by_color.black,
        };

        let all_pieces = self.by_color.white | self.by_color.black;

        let enemy_bitboard = !my_bitboard & all_pieces;

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
        let king_reaches = self.rook_attacks(king_square.0 as usize, blockers);

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
                .wrapping_shl(king_square.file() as u32)
                + 1,
        ) & king_reaches;
        let king_reaches_east = king_reaches_east & horizontal_mask;

        let king_reaches_west = Bitboard(
            Bitboard::from_rank_number(king_square.rank() as usize)
                .0
                .wrapping_shr(8 - king_square.file() as u32),
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
                let attacker_moves =
                    self.rook_attacks(attack_square as usize, blockers) & direction;
                move_mask |= direction & attacker_moves;
                capture_mask |= Bitboard(1 << attack_square);
            }
        }

        // bishop moves
        let bishops = self.by_role.bishops & enemy_bitboard;
        let king_reaches = self.bishop_attacks(king_square.0 as usize, blockers);
        let (north_mask, east_mask, south_mask, west_mask) = DIRECTION_MASK[king_square.0 as usize];

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
                let attacker_moves = self.bishop_attacks(attack_square as usize, blockers);
                move_mask |= direction & attacker_moves;
                capture_mask |= Bitboard(1 << attack_square);
            }
        }

        // pawns
        let pawns = self.by_role.pawns & enemy_bitboard;
        let king_reaches = my_bitboard & self.by_role.kings;
        let king_reaches = Bitboard(king_reaches.0.wrapping_shl(9))
            | Bitboard(king_reaches.0.wrapping_shl(7)) * (turn == Color::White) as u64
                + Bitboard(king_reaches.0.wrapping_shr(9))
            | Bitboard(king_reaches.0.wrapping_shr(7)) * (turn == Color::Black) as u64;

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

    pub fn pin_mask(self, turn: Color) -> Bitboard {
        let all_pieces = self.by_color.white | self.by_color.black;
        let my_bitboard = match turn {
            Color::White => self.by_color.white,
            Color::Black => self.by_color.black,
        };

        let my_bitboard_without_king = my_bitboard & !(self.by_role.kings & my_bitboard);

        let enemy_bitboard = !my_bitboard & (all_pieces);

        let mut pin_mask = Bitboard(0);

        let blockers = enemy_bitboard;

        let king_square = (my_bitboard & self.by_role.kings).0.trailing_zeros();
        if king_square > 63 {
            return Bitboard(0);
        }
        let king_file = king_square as i32 % 8;
        let king_rank = king_square as i32 / 8;

        // rook moves
        let rooks = self.by_role.rooks & enemy_bitboard;
        let queen = self.by_role.queens & enemy_bitboard;
        let king_reaches = self.rook_attacks(king_square as usize, blockers);

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
                .wrapping_shl(king_file as u32)
                + 1,
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
                let attacker_moves =
                    self.rook_attacks(attack_square as usize, blockers) & direction;

                pin_mask |= (direction & (attacker_moves | Bitboard(1 << attack_square)))
                    * ((attacker_moves & my_bitboard_without_king).0.count_ones() == 1) as u64;
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

        for &direction in &[
            king_reaches_north_east,
            king_reaches_north_west,
            king_reaches_south_east,
            king_reaches_south_west,
        ] {
            let attack = direction & (bishops | queen);
            if attack != Bitboard(0) {
                let attack_square = attack.0.trailing_zeros();
                let attacker_moves =
                    self.bishop_attacks(attack_square as usize, blockers) & direction;

                pin_mask |= (direction & (attacker_moves | Bitboard(1 << attack_square)))
                    * ((attacker_moves & my_bitboard_without_king).0.count_ones() == 1) as u64;
            }
        }

        pin_mask
    }

    pub fn seen_by_enemy(self, turn: Color) -> Bitboard {
        let mut bitboard = Bitboard(0);
        let all_pieces = self.by_color.white | self.by_color.black;

        let my_bitboard = match turn {
            Color::White => self.by_color.white,
            Color::Black => self.by_color.black,
        };

        let enemy_bitboard = !my_bitboard & (all_pieces);
        let king = self.by_role.kings & my_bitboard;
        let blockers = all_pieces & !king;

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
                let seen = self.rook_attacks(i, blockers);
                bitboard |= seen;
            } else if is_bishop {
                let seen = self.bishop_attacks(i, blockers);
                bitboard |= seen;
            } else if is_queen {
                let seen = self.rook_attacks(i, blockers) | self.bishop_attacks(i, blockers);
                bitboard |= seen;
            } else if is_knight {
                let seen = KNIGHT_MOVES[i];
                bitboard |= seen;
            } else if is_king {
                let seen = KING_MOVES[i];
                bitboard |= seen;
            } else if is_pawn {
                let seen = match turn {
                    Color::White => {
                        (Bitboard(current_square.0.wrapping_shr(9))
                            | Bitboard(current_square.0.wrapping_shr(7)))
                            & Bitboard::from_rank_number(i / 8 - 1)
                    }
                    Color::Black => {
                        (Bitboard(current_square.0.wrapping_shl(9))
                            | Bitboard(current_square.0.wrapping_shl(7)))
                            & Bitboard::from_rank_number(i / 8 + 1)
                    }
                };
                bitboard |= seen;
            }
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
                    write!(f, "· ")?;
                }
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
