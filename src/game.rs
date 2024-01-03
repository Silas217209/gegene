use std::i32;

use crate::{
    board::{Board, Color},
    r#move::{Move, Square},
};

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
    pub fn get_legal_moves(&self) -> Vec<Move> {
        unimplemented!();
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
            println!("{:?}", fen_info);
            match fen_info[3].parse() {
                Ok(n) => n,
                Err(_) => panic!("invalid FEN: halfmove clock"),
            }
        };

        let fullmoves: i32 = if fen_info.len() <= 4 || fen_info[4] == "-" {
            1
        } else { match fen_info[4].parse() {
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
