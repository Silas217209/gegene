use crate::game::Game;
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
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let game = Game::from_fen(fen);

    println!("{}", game.board);
    println!("{}", game.board.seen_by_enemy(game.turn));
}
