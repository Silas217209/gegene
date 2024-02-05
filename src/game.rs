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
use std::{i32, usize};

#[derive(Debug, Clone, Copy)]
pub struct CastlingRight {
    pub king_side: bool,
    pub queen_side: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum Outcome {
    Playing,
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
    pub fn perft(&self, depth: u32, current_depth: u32) -> u64 {
        if current_depth == 0 {
            return 1;
        }

        let mut nodes = 0;
        let (legal_moves, count) = self.get_legal_moves();
        for i in 0..count {
            let mut game = *self;
            game.play(legal_moves[i]);
            let child_nodes = game.perft(depth, current_depth - 1);
            if current_depth == depth {
                println!("{}: {}", legal_moves[i], child_nodes);
            }
            nodes += child_nodes;
        }

        if current_depth == depth {
            println!("count: {count}");
        }
        nodes
    }

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
        if capture_mask.0.count_ones() > 1 && capture_mask.0.count_zeros() > 0 {
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
            let mut double_pawn_push_bitboard = Bitboard(0);

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
                        let move_bitboard = Bitboard(
                            (current_square & Bitboard::from_rank_number(1))
                                .0
                                .wrapping_shl(16),
                        );
                        r |= move_bitboard;
                        double_pawn_push_bitboard |= move_bitboard;
                    }

                    r
                } else {
                    let mut tmp = Bitboard(0);
                    // simple pawn push
                    tmp |= Bitboard(current_square.0.wrapping_shr(8));

                    // double pawn push
                    if tmp & (self.board.by_color.white | self.board.by_color.black) == Bitboard(0)
                    {
                        let move_bitboard = Bitboard(
                            (current_square & Bitboard::from_rank_number(6))
                                .0
                                .wrapping_shr(16),
                        );

                        tmp |= move_bitboard;
                        double_pawn_push_bitboard |= move_bitboard;
                    }
                    tmp
                };

                let mut pawn_attacks = self.board.pawn_attacks(self.turn, Square(i as u8));

                // remove captures with no enemy to capture
                pawn_attacks &= enemy_bitboard;

                // remove pawn pushes where a piece is blocking
                pawn_moves &= !(self.board.by_color.white | self.board.by_color.black);

                // combine pawn pushes and captures
                pawn_moves |= pawn_attacks;

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
            } else if self.board.by_role.kings & current_square != Bitboard(0) {
                let mut king_moves = KING_MOVES[i];
                king_moves &= !seen_by_enemy;

                moves_bitboard |= king_moves;
            };

            moves_bitboard &= enemy_or_empty;
            if is_pinned {
                moves_bitboard &= pin_mask;
            }
        
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
                    move_type: if double_pawn_push_bitboard & current_square != Bitboard(0) {
                        MoveType::DoublePawnPush
                    } else {
                        MoveType::Quiet
                    },
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
            && kingside_castle_without_king
                & (self.board.by_color.black | self.board.by_color.white)
                == Bitboard(0)
            && kingside_castle_mask & seen_by_enemy == Bitboard(0)
            && self.turn == Color::White
        {
            moves[index] = Move {
                from: Square(4),
                to: Square(6),
                piece: Piece {
                    role: Role::King,
                    color: Color::White,
                },
                capture: None,
                move_type: MoveType::KingSideCastleWhite,
            };
            index += 1;
        }

        if self.white_castling_rights.queen_side
            && queenside_castle_mask_without_king
                & (self.board.by_color.black | self.board.by_color.white)
                == Bitboard(0)
            && queenside_castle_mask & seen_by_enemy == Bitboard(0)
            && self.turn == Color::White
        {
            moves[index] = Move {
                from: Square(4),
                to: Square(2),
                piece: Piece {
                    role: Role::King,
                    color: Color::White,
                },
                capture: None,
                move_type: MoveType::QueenSideCastleWhite,
            };
            index += 1;
        }

        if self.black_castling_rights.king_side
            && kingside_castle_without_king
                & (self.board.by_color.black | self.board.by_color.white)
                == Bitboard(0)
            && kingside_castle_mask & seen_by_enemy == Bitboard(0)
            && self.turn == Color::Black
        {
            moves[index] = Move {
                from: Square(60),
                to: Square(62),
                piece: Piece {
                    role: Role::King,
                    color: Color::White,
                },
                capture: None,
                move_type: MoveType::KingSideCastleBlack,
            };
            index += 1;
        }

        if self.black_castling_rights.queen_side
            && queenside_castle_mask_without_king
                & (self.board.by_color.black | self.board.by_color.white)
                == Bitboard(0)
            && queenside_castle_mask & seen_by_enemy == Bitboard(0)
            && self.turn == Color::Black
        {
            moves[index] = Move {
                from: Square(60),
                to: Square(58),
                piece: Piece {
                    role: Role::King,
                    color: Color::White,
                },
                capture: None,
                move_type: MoveType::QueenSideCastleBlack,
            };
            index += 1;
        }

        // en passant
        if let Some(en_passant_target) = self.en_passant_target {
            let en_passant_attackers = self.board.pawn_attacks(
                match self.turn {
                    Color::White => Color::Black,
                    Color::Black => Color::White,
                },
                en_passant_target,
            ) & my_bitboard
                & self.board.by_role.pawns;


            let en_passant_square = match self.turn {
                Color::White => Square(en_passant_target.0 - 8),
                Color::Black => Square(en_passant_target.0 + 8),
            };

            let en_passant_rank: Bitboard = match self.turn {
                Color::White => Bitboard::from_rank_number(4),
                Color::Black => Bitboard::from_rank_number(3),
            };


            let en_passant_piece_bitboard = Bitboard(1 << en_passant_square.0);

            let my_king = self.board.by_role.kings & my_bitboard;

            if en_passant_attackers.0.count_ones() == 0 {
                return (moves, index);
            }

            let king_blockers = (my_bitboard | enemy_bitboard)
                & !(en_passant_piece_bitboard | en_passant_attackers);

            let king_reaches = self
                .board
                .rook_attacks(my_king.0.trailing_zeros() as usize, king_blockers)
                & en_passant_rank;

            let is_discovered_check = king_reaches
                & ((self.board.by_role.queens | self.board.by_role.rooks) & enemy_bitboard)
                != Bitboard(0);

            // discovered check is only one attacker
            if is_discovered_check && en_passant_attackers.0.count_ones() == 1{
                return (moves, index);
            }


            let attacker = en_passant_attackers.0.trailing_zeros();
            let move1 = Move {
                from: Square(attacker as u8),
                to: en_passant_target,
                piece: Piece {
                    role: Role::Pawn,
                    color: self.turn,
                },
                capture: Option::from(Piece {
                    color: match self.turn {
                        Color::White => Color::Black,
                        Color::Black => Color::White,
                    },
                    role: Role::Pawn,
                }),
                move_type: MoveType::EnPassant(en_passant_square),
            };

            moves[index] = move1;
            index += 1;

            if en_passant_attackers.0.count_ones() == 1 {
                return (moves, index);
            }
            let attacker = 64 - en_passant_attackers.0.leading_zeros();
            let move1 = Move {
                from: Square(attacker as u8),
                to: en_passant_target,
                piece: Piece {
                    role: Role::Pawn,
                    color: self.turn,
                },
                capture: Option::from(Piece {
                    color: match self.turn {
                        Color::White => Color::Black,
                        Color::Black => Color::White,
                    },
                    role: Role::Pawn,
                }),
                move_type: MoveType::EnPassant(en_passant_square),
            };

            moves[index] = move1;
            index += 1;
        }

        return (moves, index);
    }

    pub fn play(&mut self, played_move: Move) {
        let from_square = Bitboard(1u64 << played_move.from.0 as u64);
        let to_square = Bitboard(1u64 << played_move.to.0 as u64);

        self.en_passant_target = Option::None;

        // update biboards
        match played_move.move_type {
            MoveType::Quiet => {
                self.board
                    .update_bitboard(played_move.piece, from_square, to_square);
            }
            MoveType::KingSideCastleWhite => {
                self.board.update_bitboard(
                    Piece {
                        color: self.turn,
                        role: Role::Rook,
                    },
                    Bitboard(0b10000000),
                    Bitboard(0b100000),
                );
                self.board.update_bitboard(
                    Piece {
                        color: self.turn,
                        role: Role::King,
                    },
                    Bitboard(0b10000),
                    Bitboard(0b1000000),
                );

                self.white_castling_rights.king_side = false;
                self.white_castling_rights.queen_side = false;
            }
            MoveType::QueenSideCastleWhite => {
                self.board.update_bitboard(
                    Piece {
                        color: self.turn,
                        role: Role::Rook,
                    },
                    Bitboard(0b1),
                    Bitboard(0b1000),
                );
                self.board.update_bitboard(
                    Piece {
                        color: self.turn,
                        role: Role::King,
                    },
                    Bitboard(0b10000),
                    Bitboard(0b100),
                );
                self.white_castling_rights.queen_side = false;
                self.white_castling_rights.king_side = false;
            }
            MoveType::KingSideCastleBlack => {
                self.board.update_bitboard(
                    Piece {
                        color: Color::Black,
                        role: Role::Rook,
                    },
                    Bitboard(0b1000000000000000000000000000000000000000000000000000000000000000),
                    Bitboard(0b10000000000000000000000000000000000000000000000000000000000000),
                );
                self.board.update_bitboard(
                    Piece {
                        color: Color::Black,
                        role: Role::King,
                    },
                    Bitboard(0b1000000000000000000000000000000000000000000000000000000000000),
                    Bitboard(0b100000000000000000000000000000000000000000000000000000000000000),
                );

                self.black_castling_rights.king_side = false;
                self.black_castling_rights.queen_side = false;
            }

            MoveType::QueenSideCastleBlack => {
                self.board.update_bitboard(
                    Piece {
                        color: Color::Black,
                        role: Role::Rook,
                    },
                    Bitboard(0b1),
                    Bitboard(0b1000),
                );
                self.board.update_bitboard(
                    Piece {
                        color: Color::Black,
                        role: Role::King,
                    },
                    Bitboard(0b1000000000000000000000000000000000000000000000000000000000000),
                    Bitboard(0b10000000000000000000000000000000000000000000000000000000000),
                );
                self.black_castling_rights.queen_side = false;
                self.black_castling_rights.king_side = false;
            }

            MoveType::EnPassant(en_passant_square) => {
                self.board.update_bitboard(
                    Piece {
                        color: self.turn,
                        role: Role::Pawn,
                    },
                    from_square,
                    to_square,
                );
                self.board.update_bitboard(
                    Piece {
                        color: match self.turn {
                            Color::White => Color::Black,
                            Color::Black => Color::White,
                        },
                        role: Role::Pawn,
                    },
                    Bitboard(1 << en_passant_square.0),
                    Bitboard(1 << en_passant_square.0),
                );
            }
            MoveType::Promotion(piece_type) => {
                self.board.update_bitboard(
                    Piece {
                        color: self.turn,
                        role: Role::Pawn,
                    },
                    from_square,
                    from_square,
                );
                self.board.update_bitboard(
                    Piece {
                        color: self.turn,
                        role: piece_type,
                    },
                    to_square,
                    to_square,
                );
            }
            MoveType::DoublePawnPush => {
                self.board
                    .update_bitboard(played_move.piece, from_square, to_square);
                self.en_passant_target = match self.turn {
                    Color::White => Some(Square(played_move.to.0 - 8)),
                    Color::Black => Some(Square(played_move.to.0 + 8)),
                };
            }
        }

        if let Some(capture) = played_move.capture {
            self.board.update_bitboard(capture, to_square, to_square);
        }

        self.turn = match self.turn {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    pub fn from_fen(fen: &str) -> Result<Game, &str> {
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
        let en_passent_target = if en_passant_target == "-" {
            None
        } else {
            match Square::from_algebraic(en_passant_target) {
                Ok(square) => Some(square),
                Err(e) => return Err(e),
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

        Ok(Game {
            board: Board::from_fen(fen),
            turn: active_color,
            white_castling_rights: castlig_rights.0,
            black_castling_rights: castlig_rights.1,
            en_passant_target: en_passent_target,
            halfmove_clock,
            fullmoves,
            outcome: Outcome::Draw,
        })
    }
}
