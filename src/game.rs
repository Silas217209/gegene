use std::{i32, usize};

use crate::bitboard::Bitboard;
use crate::lookup::king::KING_MOVES;
use crate::lookup::knight::KNIGHT_MOVES;
use crate::piece::Piece;
use crate::r#move::MoveType;
use crate::role::Role;
use crate::{
    board::{Board, Color},
    r#move::{Move, Square},
};

#[derive(Debug, Clone, Copy)]
pub struct CastlingRight {
    pub king_side: bool,
    pub queen_side: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum Outcome {
    Win(Color),
    Draw,
}

#[derive(Debug, Clone, Copy)]
pub struct Game {
    pub board: Board,
    pub turn: Color,
    pub white_castling_rights: CastlingRight,
    pub black_castling_rights: CastlingRight,
    pub en_passant_target: Option<Square>,
    pub halfmove_clock: i32,
    pub fullmoves: i32,
    pub outcome: Outcome,
}

impl Game {
    pub fn get_legal_moves(self) -> ([Move; 218], usize) {
        let mut index = 0;
        let mut moves: [Move; 218] = [Move {
            from: Square(0),
            to: Square(0),
            piece: Piece {
                color: Color::White,
                role: Role::King,
            },
            capture: None,
            move_type: MoveType::Quiet,
        }; 218];

        let mut all_moves_bitboard = Bitboard(0);

        let my_bitboard = Bitboard(
            (Color::White == self.turn) as u64 * self.board.by_color.white.0
                + (Color::Black == self.turn) as u64 * self.board.by_color.black.0,
        );

        let enemy_bitboard = !my_bitboard & (self.board.by_color.white | self.board.by_color.black);

        let blockers = self.board.by_color.white | self.board.by_color.black;

        let enemy_or_empty = !my_bitboard;
        let seen_by_enemy = self.board.seen_by_enemy(self.turn);

        let (move_mask, capture_mask) = self.board.check_mask(self.turn);

        // only the king can move if there is a check by more than one enemy piece
        if capture_mask.0.count_ones() > 1 {
            let king_bitboard = self.board.by_role.kings & my_bitboard;
            let king_square = king_bitboard.0.trailing_zeros() as usize;
            let mut king_moves = KING_MOVES[king_square];

            king_moves &= enemy_or_empty;
            king_moves &= !seen_by_enemy;

            for i in 0..64 {
                let current_square = Bitboard(1 << i);

                if current_square & king_moves == Bitboard(0) {
                    continue;
                }

                moves[index] = Move {
                    from: Square(king_square as u8),
                    to: Square(i as u8),
                    piece: Piece {
                        color: self.turn,
                        role: Role::King,
                    },
                    capture: self.board.piece_at(i),
                    move_type: MoveType::Quiet,
                };
                index += 1;
            }

            return (moves, index);
        }

        let pin_mask = self.board.pin_mask(self.turn);

        for i in 0..64 {
            let mut is_promotion = false;
            let mut moves_bitboard = Bitboard(0);

            let current_square = Bitboard(1 << i);

            if current_square & my_bitboard == Bitboard(0) {
                continue;
            }

            let is_pinned = current_square & pin_mask != Bitboard(0);

            if self.board.by_role.pawns & current_square != Bitboard(0) {
                let mut pawn_moves = if self.turn == Color::White {
                    let mut r = Bitboard(0);
                    // simple pawn push
                    r |= Bitboard(current_square.0.wrapping_shl(8));

                    // double pawn push
                    if r & (self.board.by_color.white | self.board.by_color.black) == Bitboard(0) {
                        r |= Bitboard(
                            (current_square & Bitboard::from_rank_number(1))
                                .0
                                .wrapping_shl(16),
                        );
                    }

                    r
                } else {
                    let mut r = Bitboard(0);
                    // simple pawn push
                    r |= Bitboard(current_square.0.wrapping_shr(8));

                    // double pawn push
                    if r & (self.board.by_color.white | self.board.by_color.black) == Bitboard(0) {
                        r |= Bitboard(
                            (current_square & Bitboard::from_rank_number(6))
                                .0
                                .wrapping_shr(16),
                        );
                    }

                    r
                };

                let mut pawn_captures = if self.turn == Color::White {
                    let mut r = Bitboard(0);
                    r |= Bitboard(current_square.0.wrapping_shl(7));
                    r |= Bitboard(current_square.0.wrapping_shl(9));

                    // filter out overshifted squares
                    if i < 56 {
                        let one_rank_up = Bitboard::from_rank_number(i / 8 + 1);
                        r &= one_rank_up;
                    }

                    // en passant
                    if let Some(en_passant_target) = self.en_passant_target {
                        let en_passant_square = Bitboard((i << en_passant_target.0) as u64)
                            & enemy_bitboard
                            & capture_mask;

                        // check if one of my pawns can capture the en passant target
                        let en_passant_attackers = (en_passant_square << Bitboard(1)
                            | en_passant_square >> Bitboard(1))
                            & Bitboard::from_rank_number(3)
                            & self.board.by_role.pawns
                            & my_bitboard;
                    }

                    r
                } else {
                    let mut r = Bitboard(0);

                    r |= Bitboard(current_square.0.wrapping_shr(7));
                    r |= Bitboard(current_square.0.wrapping_shr(9));

                    // filter out overshifted squares
                    if i < 7 {
                        let one_rank_down = Bitboard::from_rank_number(i / 8 - 1);
                        r &= one_rank_down;
                    }
                    r
                };

                // remove captures with no enemy to capture
                pawn_captures &= enemy_bitboard;

                // remove pawn pushes where a piece is blocking
                pawn_moves &= !(self.board.by_color.white | self.board.by_color.black);

                // combine pawn pushes and captures
                pawn_moves |= pawn_captures;

                // filter out checks
                pawn_moves &= move_mask | capture_mask;

                is_promotion = pawn_moves
                    & (Bitboard::from_rank_number(0) | Bitboard::from_rank_number(7))
                    != Bitboard(0);

                moves_bitboard |= pawn_moves;
            } else if self.board.by_role.bishops & current_square != Bitboard(0) {
                moves_bitboard |=
                    self.board.bishop_attacks(i, blockers) & (move_mask | capture_mask);
            } else if self.board.by_role.knights & current_square != Bitboard(0) {
                moves_bitboard = KNIGHT_MOVES[i] & (move_mask | capture_mask);
            } else if self.board.by_role.rooks & current_square != Bitboard(0) {
                moves_bitboard = self.board.rook_attacks(i, blockers) & (move_mask | capture_mask);
            } else if self.board.by_role.queens & current_square != Bitboard(0) {
                moves_bitboard = (self.board.bishop_attacks(i, blockers)
                    | self.board.rook_attacks(i, blockers))
                    & (move_mask | capture_mask);
            }

            moves_bitboard &= pin_mask | Bitboard((is_pinned as u64).wrapping_sub(1));

            moves_bitboard &= enemy_or_empty;
            all_moves_bitboard |= moves_bitboard;

            for square in 0..64 {
                let current_square = Bitboard(1 << square);

                if current_square & moves_bitboard == Bitboard(0) {
                    continue;
                }

                if is_promotion {
                    for role in [Role::Rook, Role::Bishop, Role::Rook, Role::Queen] {
                        moves[index] = Move {
                            from: Square(i as u8),
                            to: Square(square as u8),
                            piece: self.board.piece_at(i as i32).unwrap(),
                            capture: self.board.piece_at(square),
                            move_type: MoveType::Promotion(role),
                        };
                        index += 1;
                    }
                    continue;
                }

                moves[index] = Move {
                    from: Square(i as u8),
                    to: Square(square as u8),
                    piece: self.board.piece_at(i as i32).unwrap(),
                    capture: self.board.piece_at(square),
                    move_type: MoveType::Quiet,
                };

                index += 1;
            }
        }

        // castle
        let kingside_castle_mask = match self.turn {
            Color::White => Bitboard(112),
            Color::Black => Bitboard(8070450532247928832),
        };

        let kingside_castle_without_king = match self.turn {
            Color::White => Bitboard(96),
            Color::Black => Bitboard(6917529027641081856),
        };

        let queenside_castle_mask = match self.turn {
            Color::White => Bitboard(28),
            Color::Black => Bitboard(2017612633061982208),
        };
        let queenside_castle_mask_without_king = match self.turn {
            Color::White => Bitboard(12),
            Color::Black => Bitboard(864691128455135232),
        };

        if self.white_castling_rights.king_side
            && (kingside_castle_without_king
                & (self.board.by_color.black | self.board.by_color.white)
                == Bitboard(0)
                && kingside_castle_mask & seen_by_enemy == Bitboard(0))
        {
            moves[index] = Move {
                from: Square(4),
                to: Square(6),
                piece: Piece {
                    role: Role::King,
                    color: Color::White,
                },
                capture: None,
                move_type: MoveType::KingSideCastle,
            };
            index += 1;
        }

        if self.white_castling_rights.queen_side
            && (queenside_castle_mask_without_king
                & (self.board.by_color.black | self.board.by_color.white)
                == Bitboard(0)
                && queenside_castle_mask & seen_by_enemy == Bitboard(0))
        {
            moves[index] = Move {
                from: Square(4),
                to: Square(2),
                piece: Piece {
                    role: Role::King,
                    color: Color::White,
                },
                capture: None,
                move_type: MoveType::QueenSideCastle,
            };
            index += 1;
        }

        return (moves, index);
    }

    pub fn from_fen(fen: &str) -> Game {
        // we only care about the information after the position
        let fen_info: Vec<&str> = fen.trim().split(" ").collect();
        let fen_info = &fen_info[1..];

        let active_color = fen_info[0];
        let active_color: Color = match active_color {
            "w" => Color::White,
            "b" => Color::Black,
            _ => panic!("invalid FEN: active color"),
        };

        let castling_rights = fen_info[1];
        let castlig_rights = {
            let mut white_castling = CastlingRight {
                king_side: false,
                queen_side: false,
            };

            let mut black_castling = CastlingRight {
                king_side: false,
                queen_side: false,
            };

            for c in castling_rights.chars() {
                match c {
                    'K' => white_castling.king_side = true,
                    'Q' => white_castling.queen_side = true,
                    'k' => black_castling.king_side = true,
                    'q' => black_castling.queen_side = true,
                    '-' => continue,
                    _ => panic!("invalid FEN: castling rights"),
                };
            }
            (white_castling, black_castling)
        };

        let en_passant_target = fen_info[2];
        let en_passent_target: Option<Square> = {
            if en_passant_target == "-" {
                None
            } else {
                Some(Square::from_algebraic(en_passant_target))
            }
        };

        let halfmove_clock: i32 = if fen_info.len() <= 3 {
            0
        } else {
            match fen_info[3].parse() {
                Ok(n) => n,
                Err(_) => panic!("invalid FEN: halfmove clock"),
            }
        };

        let fullmoves: i32 = if fen_info.len() <= 4 || fen_info[4] == "-" {
            1
        } else {
            match fen_info[4].parse() {
                Ok(n) => n,
                Err(_) => panic!("invalid FEN: fullmoves"),
            }
        };

        Game {
            board: Board::from_fen(fen),
            turn: active_color,
            white_castling_rights: castlig_rights.0,
            black_castling_rights: castlig_rights.1,
            en_passant_target: en_passent_target,
            halfmove_clock,
            fullmoves,
            outcome: Outcome::Draw,
        }
    }
}
