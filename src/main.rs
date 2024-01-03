use crate::board::Board;
use crate::game::Game;

mod bitboard;
mod board;
mod role;
mod piece;
mod r#move;
mod game;

fn main() {
    let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -";

    let game = Game::from_fen(fen);


    println!("{:#?}", game);
    println!("{}", game.board);
}
