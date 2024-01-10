use std::i32;

use crate::{
    board::{Board, Color},
    r#move::{Move, Square},
};
use crate::bitboard::Bitboard;
use crate::lookup::knight::KNIGHT_MOVES;
use crate::lookup::rook_mask::ROOK_MASK;
use crate::lookup::rook_moves::ROOK_MOVES;
use crate::pext::Pext;

#[derive(Debug)]
pub struct CastlingRight {
    pub king_side: bool,
    pub queen_side: bool,
}

#[derive(Debug)]
pub enum Outcome {
    Win(Color),
    Draw,
}

#[derive(Debug)]
pub struct Game {
    pub board: Board,
    pub turn: Color,
    pub white_castlinright: CastlingRight,
    pub can_black_castle: CastlingRight,
    pub en_passant_target: Option<Square>,
    pub halfmove_clock: i32,
    pub fullmoves: i32,
    pub outcome: Outcome,
}


impl Game {
    pub fn get_legal_moves(self) -> Vec<Move> {
        let my_bitboard = match self.turn {
            Color::White => self.board.by_color.white,
            Color::Black => self.board.by_color.black,
        };

        let enemy_bitboard = match self.turn {
            Color::White => self.board.by_color.black,
            Color::Black => self.board.by_color.white,
        };

        let enemy_or_empty = !my_bitboard;

        let blockers = self.board.by_color.white | self.board.by_color.black;
        println!("{}", blockers);

        let check_mask = {
            let mut check_mask = Bitboard(0);

            let king_square = (my_bitboard & self.board.by_role.kings).0.trailing_zeros();

            // rooks
            let rooks = self.board.by_role.rooks & enemy_bitboard;
            let rook1_square = rooks.0.trailing_zeros();
            let rook2_square = if rooks.0.leading_zeros() > 63 {
                64
            } else {
                63 - rooks.0.leading_zeros()
            };

            let king_reaches = self.board.rook_attacks(king_square as usize, blockers);

            if rook1_square <= 63 {
                let rook1_attack = self.board.rook_attacks(rook1_square as usize, blockers);
                check_mask |= king_reaches & (rook1_attack | Bitboard(1 << rook1_square));
            }

            if rook2_square <= 63 {
                let rook2_attack = self.board.rook_attacks(rook2_square as usize, blockers);
                check_mask |= king_reaches & (rook2_attack | Bitboard(1 << rook2_square));
            }

            // bishops
            let bishops = self.board.by_role.bishops & enemy_bitboard;
            let bishop1_square = bishops.0.trailing_zeros();
            let bishop2_square = if bishops.0.leading_zeros() > 63 {
                64
            } else {
                63 - bishops.0.leading_zeros()
            };

            let king_reaches = self.board.bishop_attacks(king_square as usize, blockers);

            if bishop1_square <= 63 {
                let bishop1_attack = self.board.bishop_attacks(bishop1_square as usize, blockers);
                check_mask |= king_reaches & (bishop1_attack | Bitboard(1 << bishop1_square));
            }

            if bishop2_square <= 63 {
                let bishop2_attack = self.board.bishop_attacks(bishop2_square as usize, blockers);
                check_mask |= king_reaches & (bishop2_attack | Bitboard(1 << bishop2_square));
            }


            let knights = self.board.by_role.knights & enemy_bitboard;
            check_mask |= knights & KNIGHT_MOVES[king_square as usize];


            // pawns
            let king_reaches = match self.turn {
                Color::White => my_bitboard << Bitboard(7) | my_bitboard << Bitboard(9),
                Color::Black => my_bitboard >> Bitboard(7) | my_bitboard >> Bitboard(9),
            };
            println!("{}", king_reaches);
            check_mask |= king_reaches & (enemy_bitboard & self.board.by_role.pawns);

            check_mask
        };

        println!("{}", check_mask);

        unimplemented!("get_legal_moves");
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
            white_castlinright: castlig_rights.0,
            can_black_castle: castlig_rights.1,
            en_passant_target: en_passent_target,
            halfmove_clock,
            fullmoves,
            outcome: Outcome::Draw,
        }
    }
}
