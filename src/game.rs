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
enum Outcome {
    Win(Color),
    Draw,
}

#[derive(Debug)]
pub struct Game {
    pub board: Board,
    pub turn: Color,
    pub white_castlinright: CastlingRight,
    pub can_black_castle: CastlingRight,
    pub en_passant_target: Square,
    pub halfmove_clock: i32,
    pub fullmoves: i32,
    pub outcome: Outcome,
}

impl Game {
    fn get_legal_moves(&self) -> Vec<Move> {
        unimplemented!();
    }

    fn from_fen(&self, fen: &str) -> Game {
        // we only care about the information after the position
        let fen: Vec<&str> = fen.split(" ").collect();
        let fen = &fen[1..];

        let active_color = fen[0];
        let castling_rights = fen[1];
        let en_passant_target = fen[2];
        let halfmove_clock = fen[3];
        let fullmoves = fen[4];

        let active_color: Color = match active_color {
            "w" => Color::White,
            "b" => Color::Black,
            _ => panic!("invalid FEN: active color"),
        };

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

        let en_passent_target: Square = {
            let square = Square{

            }
        }

        unimplemented!();
    }
}
