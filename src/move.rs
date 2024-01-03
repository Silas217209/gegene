use crate::piece::Piece;
use crate::board::{Rank, File};

#[derive(Debug)]
pub struct Square(File, Rank);

pub struct Move {
    pub from: Square,
    pub to: Square,
    pub piece: Piece,
    pub capture: Option<Piece>
}
