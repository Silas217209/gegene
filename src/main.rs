use crate::board::Color;
use crate::game::Game;
use crate::lookup::bishop_mask::BISHOP_MASK;
use crate::lookup::bishop_moves::{BISHOP_MOVES};
use crate::lookup::{generate_bishop_moves, generate_ray_mask};
use crate::lookup::rook_mask::ROOK_MASK;
use crate::lookup::rook_moves::ROOK_MOVES;
use crate::pdep::Pdep;
use crate::pext::Pext;

mod bitboard;
mod board;
mod role;
mod piece;
mod r#move;
mod game;
mod lookup;
mod pext;
mod pdep;

fn main() {
    let fen = "8/3K4/8/3R4/6b1/8/k7/3q4 w kq - 0 1";
    let game = Game::from_fen(fen);

    println!("{}", game.board);
    println!("{}", game.board.check_mask(Color::White));
    println!("{}", game.board.pin_mask(Color::White));

}
