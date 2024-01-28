use crate::board::{File, Rank};
use crate::piece::Piece;
use crate::role::Role;

#[derive(Debug, Clone, Copy)]
pub struct Square(pub u8);

impl Square {
    pub fn from_algebraic(algebraic: &str) -> Square {
        let file = match algebraic.chars().nth(0).unwrap() {
            'a' => File::A,
            'b' => File::B,
            'c' => File::C,
            'd' => File::D,
            'e' => File::E,
            'f' => File::F,
            'g' => File::G,
            'h' => File::H,
            _ => panic!("Invalid file"),
        };

        let rank = match algebraic.chars().nth(1).unwrap() {
            '1' => Rank::First,
            '2' => Rank::Second,
            '3' => Rank::Third,
            '4' => Rank::Fourth,
            '5' => Rank::Fifth,
            '6' => Rank::Sixth,
            '7' => Rank::Seventh,
            '8' => Rank::Eighth,
            _ => panic!("Invalid rank"),
        };

        Square((rank as u8) * 8 + (file as u8))
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
    EnPassant,
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
