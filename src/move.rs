use crate::piece::Piece;
use crate::board::{Rank, File};

#[derive(Debug)]
pub struct Square(pub File, pub Rank);

impl Square {
    pub fn index(self) -> i32 {
        return self.0 as i32 + self.1 as i32 * 8
    }

    pub fn from_algebraic(notation: &str) -> Square {
        let mut square = Square(File::A, Rank::First);
        for c in notation.chars() {
            match c {
                'a' => square.0 = File::A,
                'b' => square.0 = File::B,
                'c' => square.0 = File::C,
                'd' => square.0 = File::D,
                'e' => square.0 = File::E,
                'f' => square.0 = File::F,
                'g' => square.0 = File::G,
                'h' => square.0 = File::H,
                '1' => square.1 = Rank::First,
                '2' => square.1 = Rank::Second,
                '3' => square.1 = Rank::Third,
                '4' => square.1 = Rank::Fourth,
                '5' => square.1 = Rank::Fifth,
                '6' => square.1 = Rank::Sixth,
                '7' => square.1 = Rank::Seventh,
                '8' => square.1 = Rank::Eighth,
                _ => panic!("Invalid square notation")
            };
        }
        return square;
    } 
}

pub struct Move {
    pub from: Square,
    pub to: Square,
    pub piece: Piece,
    pub capture: Option<Piece>
}
