use std::str;

use crate::board::{File, Rank};
use crate::piece::Piece;
use crate::role::Role;

#[derive(Debug, Clone, Copy)]
pub struct Square(pub u8);

impl Square {
    pub fn from_algebraic(algebraic: &str) -> Result<Square, &str> {
        if algebraic.len() != 2 {
            return Err("The input should have a lenght of exactly 2");
        }
        let first_char = algebraic.chars().nth(0);

        if first_char.is_none() {
            return Err("square string is invalid (first char)");
        }
        let second_char = algebraic.chars().nth(1);
        if second_char.is_none() {
            return Err("square string is invalid (second char)");
        }
        let file = match first_char.unwrap() {
            'a' => File::A,
            'b' => File::B,
            'c' => File::C,
            'd' => File::D,
            'e' => File::E,
            'f' => File::F,
            'g' => File::G,
            'h' => File::H,
            _ => return Err("invalid File"),
        };

        let rank = match second_char.unwrap() {
            '1' => Rank::First,
            '2' => Rank::Second,
            '3' => Rank::Third,
            '4' => Rank::Fourth,
            '5' => Rank::Fifth,
            '6' => Rank::Sixth,
            '7' => Rank::Seventh,
            '8' => Rank::Eighth,
            _ => return Err("invalid Rank"),
        };

        Ok(Square((rank as u8) * 8 + (file as u8)))
    }

    pub fn file(&self) -> u8 {
        self.0 % 8
    }

    pub fn rank(&self) -> u8 {
        self.0 / 8
    }
}
#[derive(Debug, Copy, Clone)]
pub enum MoveType {
    Quiet,
    KingSideCastle,
    QueenSideCastle,
    EnPassant(Square),
    Promotion(Role),
}

#[derive(Debug, Copy, Clone)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub piece: Piece,
    pub capture: Option<Piece>,
    pub move_type: MoveType,
}
