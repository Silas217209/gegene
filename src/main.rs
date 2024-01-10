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
    let fen = "8/4n3/4p3/3K4/8/8/8/3r3b w kq - 0 1";
    let game = Game::from_fen(fen);

    println!("{}", game.board);
    println!("{}", game.board.check_mask(Color::White));

    // get ray from index 0 with king as a blocker
    let king = (game.board.by_role.kings & game.board.by_color.white);
    let (mask, offset) = BISHOP_MASK[8];
    let index = (king.0.pext(mask.0) + offset) as usize;
    let ray = BISHOP_MOVES[index];
    println!("{}", ray);
}
