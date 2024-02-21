use crate::bitboard::Bitboard;
use crate::bmi::Bmi;
use crate::lookup::king::KING_MOVES;
use crate::lookup::knight::KNIGHT_MOVES;
use crate::piece::Piece;
use crate::r#move::MoveType;
use crate::role::{PromotionRole, Role};
use crate::{
    board::Board,
    r#move::{Move, Square},
};
use std::{i32, usize};

#[derive(Debug, Clone, Copy)]
pub struct CastlingRight {
    pub king_side: bool,
    pub queen_side: bool,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum DrawType {
    FitftyMoveRule,
    Stalemate,
    ThreefoldRepitition,
    InsufficientMaterial,
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum Outcome {
    Playing,
    Win(bool), //is_white
    Draw(DrawType),
}

#[derive(Debug, Clone, Copy)]
pub struct Game {
    pub board: Board,
    pub is_white: bool,
    pub white_castling_rights: CastlingRight,
    pub black_castling_rights: CastlingRight,
    pub en_passant_target: Option<Square>,
    pub halfmove_clock: i32,
    pub fullmoves: i32,
    pub outcome: Outcome,
    pub history: ([u64; 200], usize),
}

impl Game {
    pub fn perft(&self, depth: u32, current_depth: u32, debug: bool) -> u64 {
        if current_depth == 0 {
            return 1;
        }

        let mut nodes = 0;
        let (legal_moves, count) = self.get_legal_moves();
        for i in 0..count {
            let mut game = *self;
            game.play(legal_moves[i]);
            let child_nodes = game.perft(depth, current_depth - 1, debug);
            if current_depth == depth && debug {
                println!("{}: {}", legal_moves[i], child_nodes);
            }
            nodes += child_nodes;
        }

        nodes
    }

    pub fn get_legal_moves(self) -> ([Move; 218], usize) {
        let mut index = 0;
        let mut moves: [Move; 218] = [Move::null(); 218];

        let my_bitboard = Bitboard(
            (self.is_white) as u64 * self.board.by_color.white.0
                + (!self.is_white) as u64 * self.board.by_color.black.0,
        );

        let enemy_bitboard = !my_bitboard & (self.board.by_color.white | self.board.by_color.black);

        let blockers = self.board.by_color.white | self.board.by_color.black;

        let enemy_or_empty = !my_bitboard;
        let seen_by_enemy = self.board.seen_by_enemy(self.is_white);

        let (move_mask, capture_mask) = self.board.check_mask(self.is_white);
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
                let capture = self.board.piece_at(i);
                let move1 = Move::new(
                    self.is_white,
                    Square(king_square as u8),
                    Square(i as u8),
                    Role::King,
                    capture.is_some(),
                    capture
                        .unwrap_or(Piece {
                            is_white: true,
                            role: Role::Pawn,
                        })
                        .role,
                    crate::role::PromotionRole::Queen,
                    MoveType::Quiet,
                );

                moves[index] = move1;
                index += 1;
            }

            return (moves, index);
        }

        let (pin_mask_vh, pin_mask_diagonal) = self.board.pin_mask(self.is_white);

        let mut loop_bitboard = my_bitboard;
        while loop_bitboard.0 != 0 {
            let mut is_promotion = false;
            let mut moves_bitboard = Bitboard(0);
            let mut double_pawn_push_bitboard = Bitboard(0);

            let current_square = Bitboard(loop_bitboard.0.blsi());
            let i = current_square.0.trailing_zeros() as usize;

            let is_pinned_vh = current_square & pin_mask_vh != Bitboard(0);
            let is_pinned_diagoal = current_square & pin_mask_diagonal != Bitboard(0);

            if self.board.by_role.pawns & current_square != Bitboard(0) {
                let mut pawn_moves = if self.is_white {
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

                let mut pawn_attacks = Board::pawn_attacks(self.is_white, current_square);

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
                moves_bitboard |= Board::bishop_attacks(i, blockers) & (move_mask | capture_mask);
            } else if self.board.by_role.knights & current_square != Bitboard(0) {
                moves_bitboard = KNIGHT_MOVES[i] & (move_mask | capture_mask);
            } else if self.board.by_role.rooks & current_square != Bitboard(0) {
                moves_bitboard = Board::rook_attacks(i, blockers) & (move_mask | capture_mask);
            } else if self.board.by_role.queens & current_square != Bitboard(0) {
                moves_bitboard = if is_pinned_diagoal {
                    Board::bishop_attacks(i, blockers) & pin_mask_diagonal
                } else if is_pinned_vh {
                    Board::rook_attacks(i, blockers) & pin_mask_vh
                } else {
                    Board::bishop_attacks(i, blockers) | Board::rook_attacks(i, blockers)
                } & (move_mask | capture_mask);
            } else if self.board.by_role.kings & current_square != Bitboard(0) {
                let mut king_moves = KING_MOVES[i];
                king_moves &= !seen_by_enemy;

                moves_bitboard |= king_moves;
            };

            moves_bitboard &= enemy_or_empty;
            if is_pinned_vh {
                moves_bitboard &= pin_mask_vh;
            }

            if is_pinned_diagoal {
                moves_bitboard &= pin_mask_diagonal;
            }

            while moves_bitboard != Bitboard(0) {
                let current_square = Bitboard(moves_bitboard.0.blsi());
                let square = current_square.0.trailing_zeros() as i32;

                if current_square & moves_bitboard == Bitboard(0) {
                    continue;
                }

                if is_promotion {
                    for role in [
                        PromotionRole::Rook,
                        PromotionRole::Bishop,
                        PromotionRole::Knight,
                        PromotionRole::Queen,
                    ] {
                        let capture = self.board.piece_at(square);
                        let move1 = Move::new(
                            self.is_white,
                            Square(i as u8),
                            Square(square as u8),
                            Role::Pawn,
                            capture.is_some(),
                            capture
                                .unwrap_or(Piece {
                                    is_white: true,
                                    role: Role::Pawn,
                                })
                                .role,
                            role,
                            MoveType::Promotion,
                        );
                        moves[index] = move1;
                        index += 1;
                    }
                    moves_bitboard = Bitboard(moves_bitboard.0.blsr());
                    continue;
                }

                let capture = self.board.piece_at(square);
                let move1 = Move::new(
                    self.is_white,
                    Square(i as u8),
                    Square(square as u8),
                    self.board.piece_at(i as i32).unwrap().role,
                    capture.is_some(),
                    capture
                        .unwrap_or(Piece {
                            is_white: true,
                            role: Role::Pawn,
                        })
                        .role,
                    PromotionRole::Queen,
                    if double_pawn_push_bitboard & current_square != Bitboard(0) {
                        MoveType::DoublePawnPush
                    } else {
                        MoveType::Quiet
                    },
                );

                moves[index] = move1;

                index += 1;
                moves_bitboard = Bitboard(moves_bitboard.0.blsr());
            }

            loop_bitboard = Bitboard(loop_bitboard.0.blsr());
        }

        // castle
        let kingside_castle_mask =
            Bitboard(112 * (self.is_white as u64) + 8070450532247928832 * (!self.is_white as u64));

        let kingside_castle_without_king =
            Bitboard(96 * (self.is_white as u64) + 6917529027641081856 * (!self.is_white as u64));

        let queenside_castle_mask =
            Bitboard(28 * (self.is_white as u64) + 2017612633061982208 * (!self.is_white as u64));

        let queenside_castle_mask_without_king =
            Bitboard(12 * (self.is_white as u64) + 864691128455135232 * (!self.is_white as u64));

        let queenside_extension =
            Bitboard(2 * (self.is_white as u64) + 144115188075855872 * (!self.is_white as u64));
        if self.white_castling_rights.king_side
            && kingside_castle_without_king
                & (self.board.by_color.black | self.board.by_color.white)
                == Bitboard(0)
            && kingside_castle_mask & seen_by_enemy == Bitboard(0)
            && self.is_white
        {
            let move1 = Move::new(
                true,
                Square(4),
                Square(6),
                Role::King,
                false,
                Role::Pawn,
                PromotionRole::Queen,
                MoveType::KingsideCastle,
            );
            moves[index] = move1;
            index += 1;
        }

        if self.white_castling_rights.queen_side
            && queenside_castle_mask_without_king
                & (self.board.by_color.black | self.board.by_color.white)
                == Bitboard(0)
            && queenside_castle_mask & seen_by_enemy == Bitboard(0)
            && queenside_extension & (self.board.by_color.black | self.board.by_color.white)
                == Bitboard(0)
            && self.is_white
        {
            let move1 = Move::new(
                true,
                Square(4),
                Square(2),
                Role::King,
                false,
                Role::Pawn,
                PromotionRole::Queen,
                MoveType::QueensideCastle,
            );
            moves[index] = move1;
            index += 1;
        }

        if self.black_castling_rights.king_side
            && kingside_castle_without_king
                & (self.board.by_color.black | self.board.by_color.white)
                == Bitboard(0)
            && kingside_castle_mask & seen_by_enemy == Bitboard(0)
            && !self.is_white
        {
            let move1 = Move::new(
                false,
                Square(60),
                Square(62),
                Role::King,
                false,
                Role::Pawn,
                PromotionRole::Queen,
                MoveType::KingsideCastle,
            );
            moves[index] = move1;
            index += 1;
        }

        if self.black_castling_rights.queen_side
            && queenside_castle_mask_without_king
                & (self.board.by_color.black | self.board.by_color.white)
                == Bitboard(0)
            && queenside_castle_mask & seen_by_enemy == Bitboard(0)
            && queenside_extension & (self.board.by_color.black | self.board.by_color.white)
                == Bitboard(0)
            && !self.is_white
        {
            let move1 = Move::new(
                false,
                Square(60),
                Square(58),
                Role::King,
                false,
                Role::Pawn,
                PromotionRole::Queen,
                MoveType::QueensideCastle,
            );
            moves[index] = move1;
            index += 1;
        }

        // en passant
        if let Some(en_passant_target) = self.en_passant_target {
            let en_passant_target_bitboard = Bitboard(1 << en_passant_target.0);

            let en_passant_attackers =
                Board::pawn_attacks(!self.is_white, en_passant_target_bitboard)
                    & my_bitboard
                    & self.board.by_role.pawns;

            if en_passant_attackers.0.count_ones() == 0 {
                return (moves, index);
            }

            let en_passant_square = Square(
                (en_passant_target.0 as i8
                    + (-8 as i8) * (self.is_white as i8)
                    + 8 * (!self.is_white as i8)) as u8,
            );

            let en_passant_rank: Bitboard = Bitboard::from_rank_number(
                4 * (self.is_white as usize) + 3 * (!self.is_white as usize),
            );

            let en_passant_piece_bitboard = Bitboard(1 << en_passant_square.0);

            if en_passant_target_bitboard & move_mask == Bitboard(0)
                && en_passant_piece_bitboard & capture_mask == Bitboard(0)
            {
                return (moves, index);
            }

            let my_king = self.board.by_role.kings & my_bitboard;

            let king_blockers = (my_bitboard | enemy_bitboard)
                & !(en_passant_piece_bitboard | en_passant_attackers);
            let king_reaches =
                Board::rook_attacks(my_king.0.trailing_zeros() as usize, king_blockers)
                    & en_passant_rank;

            let is_discovered_check = king_reaches
                & ((self.board.by_role.queens | self.board.by_role.rooks) & enemy_bitboard)
                != Bitboard(0);

            // discovered check is only one attacker
            if is_discovered_check && en_passant_attackers.0.count_ones() == 1 {
                return (moves, index);
            }

            let attacker = en_passant_attackers.0.trailing_zeros();
            let attcker_bitboard = Bitboard(1 << attacker);

            let is_pinned_diagonal = attcker_bitboard & pin_mask_diagonal != Bitboard(0);
            let is_target_in_pin_mask = en_passant_target_bitboard & pin_mask_vh != Bitboard(0);

            let can_i_en_passant = if is_pinned_diagonal {
                is_target_in_pin_mask
            } else {
                true
            };

            if pin_mask_vh & attcker_bitboard == Bitboard(0) && can_i_en_passant {
                let move1 = Move::new(
                    self.is_white,
                    Square(attacker as u8),
                    en_passant_target,
                    Role::Pawn,
                    false,
                    Role::Pawn,
                    PromotionRole::Queen,
                    MoveType::EnPassant,
                );
                moves[index] = move1;
                index += 1;
            }
            if en_passant_attackers.0.count_ones() == 1 {
                return (moves, index);
            }
            let attacker = 63 - en_passant_attackers.0.leading_zeros();
            let attcker_bitboard = Bitboard(1 << attacker);

            let is_pinned_diagonal = attcker_bitboard & pin_mask_diagonal != Bitboard(0);
            let is_target_in_pin_mask = en_passant_target_bitboard & pin_mask_vh != Bitboard(0);

            let can_i_en_passant = if is_pinned_diagonal {
                is_target_in_pin_mask
            } else {
                true
            };

            if pin_mask_vh & attcker_bitboard == Bitboard(0) && can_i_en_passant {
                let move1 = Move::new(
                    self.is_white,
                    Square(attacker as u8),
                    en_passant_target,
                    Role::Pawn,
                    false,
                    Role::Pawn,
                    PromotionRole::Queen,
                    MoveType::EnPassant,
                );

                moves[index] = move1;
                index += 1;
            }
        }

        return (moves, index);
    }

    pub fn play_uci(&mut self, uci: &str) -> Result<(), &str> {
        if uci.len() != 4 && uci.len() != 5 {
            return Err("invalid move");
        }

        let from = Square::from_algebraic(&uci[0..2]);
        if let Err(_) = from {
            return Err("invalid move (from)");
        }
        let to = Square::from_algebraic(&uci[2..4]);
        if let Err(_) = to {
            return Err("invalid move (to)");
        }

        let from = from.unwrap();
        let to = to.unwrap();

        let role = self.board.piece_at(from.0 as i32).unwrap().role;

        // castling
        if uci == "e1g1" && matches!(role, Role::King) {
            self.play(Move::kingside_castle(true));
            return Ok(());
        }
        if uci == "e1c1" && matches!(role, Role::King) {
            self.play(Move::queenside_castle(true));
            return Ok(());
        }
        if uci == "e8g8"  && matches!(role, Role::King){
            self.play(Move::kingside_castle(false));
            return Ok(());
        }
        if uci == "e8c8"  && matches!(role, Role::King){
            self.play(Move::queenside_castle(false));
            return Ok(());
        }
        let capture = self.board.piece_at(to.0 as i32);

        // promotion
        if uci.len() == 5 {
            let promotion = match &uci[4..5] {
                "q" => PromotionRole::Queen,
                "r" => PromotionRole::Rook,
                "b" => PromotionRole::Bishop,
                "n" => PromotionRole::Knight,
                _ => return Err("invalid promotion"),
            };
            let played_move = Move::new(
                self.is_white,
                from,
                to,
                Role::Pawn,
                capture.is_some(),
                capture
                    .unwrap_or(Piece {
                        is_white: true,
                        role: Role::Pawn,
                    })
                    .role,
                promotion,
                MoveType::Promotion,
            );
            self.play(played_move);
            return Ok(());
        }

        let role = self.board.piece_at(from.0 as i32);
        if role.is_none() {
            return Err("invalid move (no piece at from)");
        }

        let role = role.unwrap().role;

        // double pawn push
        if matches!(role, Role::Pawn)
            && ((from.rank() == 1 && to.0.wrapping_sub(from.0) == 2)
                || (from.rank() == 6 && from.0.wrapping_sub(to.0) == 2))
        {
            let played_move = Move::new(
                self.is_white,
                from,
                to,
                Role::Pawn,
                capture.is_some(),
                capture
                    .unwrap_or(Piece {
                        is_white: true,
                        role: Role::Pawn,
                    })
                    .role,
                PromotionRole::Queen,
                MoveType::DoublePawnPush,
            );
            self.play(played_move);
            return Ok(());
        }

        // en passant
        if matches!(role, Role::Pawn)
            && capture.is_none()
            && self.en_passant_target.is_some()
            && to.0 == self.en_passant_target.unwrap().0
        {
            let move1 = Move::new(
                self.is_white,
                from,
                to,
                Role::Pawn,
                true,
                Role::Pawn,
                PromotionRole::Queen,
                MoveType::EnPassant,
            );
            self.play(move1);
            return Ok(());
        }

        let move1 = Move::new(
            self.is_white,
            from,
            to,
            self.board.piece_at(from.0 as i32).unwrap().role,
            capture.is_some(),
            capture
                .unwrap_or(Piece {
                    is_white: true,
                    role: Role::Pawn,
                })
                .role,
            PromotionRole::Queen,
            MoveType::Quiet,
        );

        self.play(move1);
        Ok(())
    }

    pub fn play(&mut self, played_move: Move) {
        let from_square = Bitboard(1u64 << played_move.from().0 as u64);
        let to_square = Bitboard(1u64 << played_move.to().0 as u64);

        self.en_passant_target = Option::None;
        self.halfmove_clock += 1;

        // 50 move rule
        if self.halfmove_clock >= 50 {
            self.outcome = Outcome::Draw(DrawType::FitftyMoveRule);
        }

        // update biboards
        match played_move.move_type() {
            MoveType::Quiet => {
                self.board
                    .update_bitboard(played_move.piece(), from_square, to_square);

                match played_move.role() {
                    Role::King => {
                        if played_move.is_white() {
                            self.white_castling_rights.king_side = false;
                            self.white_castling_rights.queen_side = false;
                        } else {
                            self.black_castling_rights.king_side = false;
                            self.black_castling_rights.queen_side = false;
                        }
                    }
                    Role::Rook => {
                        if played_move.is_white() {
                            match played_move.from() {
                                Square(0) => self.white_castling_rights.queen_side = false,
                                Square(7) => self.white_castling_rights.king_side = false,
                                _ => {}
                            }
                        } else {
                            match played_move.from() {
                                Square(56) => self.black_castling_rights.queen_side = false,
                                Square(63) => self.black_castling_rights.king_side = false,
                                _ => {}
                            }
                        }
                    }
                    Role::Pawn => {
                        self.halfmove_clock = 0;
                    }
                    _ => {}
                }
            }
            MoveType::KingsideCastle => {
                if played_move.is_white() {
                    self.board.update_bitboard(
                        Piece {
                            is_white: true,
                            role: Role::Rook,
                        },
                        Bitboard(0b1 << 7),
                        Bitboard(0b1 << 5),
                    );
                    self.board.update_bitboard(
                        Piece {
                            is_white: self.is_white,
                            role: Role::King,
                        },
                        Bitboard(0b1 << 4),
                        Bitboard(0b1 << 6),
                    );

                    self.white_castling_rights.king_side = false;
                    self.white_castling_rights.queen_side = false;
                } else {
                    self.board.update_bitboard(
                        Piece {
                            is_white: false,
                            role: Role::Rook,
                        },
                        Bitboard(0b1 << 63),
                        Bitboard(0b1 << 61),
                    );
                    self.board.update_bitboard(
                        Piece {
                            is_white: false,
                            role: Role::King,
                        },
                        Bitboard(0b1 << 60),
                        Bitboard(0b1 << 62),
                    );

                    self.black_castling_rights.king_side = false;
                    self.black_castling_rights.queen_side = false;
                }
            }
            MoveType::QueensideCastle => {
                if played_move.is_white() {
                    self.board.update_bitboard(
                        Piece {
                            is_white: self.is_white,
                            role: Role::Rook,
                        },
                        Bitboard(0b1),
                        Bitboard(0b1 << 3),
                    );
                    self.board.update_bitboard(
                        Piece {
                            is_white: self.is_white,
                            role: Role::King,
                        },
                        Bitboard(0b1 << 4),
                        Bitboard(0b1 << 2),
                    );
                    self.white_castling_rights.queen_side = false;
                    self.white_castling_rights.king_side = false;
                } else {
                    self.board.update_bitboard(
                        Piece {
                            is_white: false,
                            role: Role::Rook,
                        },
                        Bitboard(0b1 << 56),
                        Bitboard(0b1 << 59),
                    );
                    self.board.update_bitboard(
                        Piece {
                            is_white: false,
                            role: Role::King,
                        },
                        Bitboard(0b1 << 60),
                        Bitboard(0b1) << 58,
                    );
                    self.black_castling_rights.queen_side = false;
                    self.black_castling_rights.king_side = false;
                }
            }

            MoveType::EnPassant => {
                let en_passant_square = Square(
                    (played_move.to().0 as i8
                        + (-8 as i8) * (self.is_white as i8)
                        + 8 * (!self.is_white as i8)) as u8,
                );
                self.board.update_bitboard(
                    Piece {
                        is_white: self.is_white,
                        role: Role::Pawn,
                    },
                    from_square,
                    to_square,
                );
                self.board.update_bitboard(
                    Piece {
                        is_white: !self.is_white,
                        role: Role::Pawn,
                    },
                    Bitboard(1 << en_passant_square.0),
                    Bitboard(1 << en_passant_square.0),
                );
            }
            MoveType::Promotion => {
                self.board.update_bitboard(
                    Piece {
                        is_white: self.is_white,
                        role: Role::Pawn,
                    },
                    from_square,
                    from_square,
                );
                self.board.update_bitboard(
                    Piece {
                        is_white: self.is_white,
                        role: match played_move.promotion_role() {
                            PromotionRole::Queen => Role::Queen,
                            PromotionRole::Rook => Role::Rook,
                            PromotionRole::Bishop => Role::Bishop,
                            PromotionRole::Knight => Role::Knight,
                        },
                    },
                    to_square,
                    to_square,
                );
            }
            MoveType::DoublePawnPush => {
                self.board
                    .update_bitboard(played_move.piece(), from_square, to_square);
                self.en_passant_target = Some(Square(
                    (played_move.to().0 as i8
                        + (-8 as i8) * (self.is_white as i8)
                        + 8 * (!self.is_white as i8)) as u8,
                ));
            }
        }

        if played_move.is_capture() {
            let capture = played_move.capture_role();
            self.halfmove_clock = 0;
            if !matches!(played_move.move_type(), MoveType::EnPassant) {
                self.board.update_bitboard(
                    Piece {
                        is_white: !played_move.is_white(),
                        role: capture,
                    },
                    to_square,
                    to_square,
                );

                // if rook is captured, remove castling rights
                match capture {
                    Role::Rook => {
                        if !played_move.is_white() {
                            // capture is white
                            match played_move.to() {
                                Square(0) => self.white_castling_rights.queen_side = false,
                                Square(7) => self.white_castling_rights.king_side = false,
                                _ => {}
                            }
                        } else {
                            match played_move.to() {
                                Square(56) => self.black_castling_rights.queen_side = false,
                                Square(63) => self.black_castling_rights.king_side = false,
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        if !self.is_white {
            self.fullmoves += 1;
        }

        self.is_white = !self.is_white;

        self.history.0[self.history.1] = self.board.zobrist;
        self.history.1 += 1;

        if self.history.1 >= 200 {
            self.history.1 = 0;
        }

        // 3 fold repetition
        let mut count = 0;
        for i in 0..self.history.1 {
            if self.history.0[i] == self.board.zobrist {
                count += 1;
            }
        }

        if count >= 3 {
            self.outcome = Outcome::Draw(DrawType::ThreefoldRepitition);
        }
    }

    pub fn from_fen(fen: &str) -> Result<Game, &str> {
        // we only care about the information after the position
        let fen_info: Vec<&str> = fen.trim().split(" ").collect();
        let fen_info = &fen_info[1..];

        let is_white = fen_info[0];
        let is_white = match is_white {
            "w" => true,
            "b" => false,
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
            is_white,
            white_castling_rights: castlig_rights.0,
            black_castling_rights: castlig_rights.1,
            en_passant_target: en_passent_target,
            halfmove_clock,
            fullmoves,
            outcome: Outcome::Playing,
            history: ([0; 200], 0),
        })
    }
}
